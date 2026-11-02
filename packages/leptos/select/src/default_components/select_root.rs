use leptos::{
    ev::{KeyboardEvent, MouseEvent},
    prelude::*,
    wasm_bindgen::JsCast,
};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::keyboard::{SelectKeyAction, key_action};

static SELECT_INSTANCE: AtomicUsize = AtomicUsize::new(0);
pub(crate) static SELECT_ITEM_KEY: AtomicUsize = AtomicUsize::new(0);

/// An option registered with its Select root, in render (DOM) order.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectItemEntry {
    pub(crate) key: usize,
    pub value: String,
    pub disabled: Signal<bool>,
    pub id: String,
}

/// Shared state for one Select instance.
#[derive(Clone, Copy)]
pub struct SelectContext {
    pub open: Signal<bool>,
    pub set_open: Callback<bool>,
    pub value: Signal<String>,
    pub set_value: Callback<String>,
    pub disabled: Signal<bool>,
    pub required: Signal<bool>,
    pub name: MaybeProp<String>,
    /// Value of the keyboard/pointer highlighted option.
    pub active: RwSignal<Option<String>>,
    /// Registered options in render order.
    pub items: RwSignal<Vec<SelectItemEntry>>,
    pub trigger_id: StoredValue<String>,
    pub listbox_id: StoredValue<String>,
}

impl SelectContext {
    /// The id of the currently highlighted option, if any.
    pub fn active_descendant(&self) -> Option<String> {
        let active = self.active.get()?;
        self.items
            .get()
            .into_iter()
            .find(|item| item.value == active)
            .map(|item| item.id)
    }
}

pub(crate) fn element_by_id(id: &str) -> Option<web_sys::Element> {
    document().get_element_by_id(id)
}

pub(crate) fn focus_by_id(id: &str) {
    if let Some(el) = element_by_id(id) {
        let _ = el.unchecked_into::<web_sys::HtmlElement>().focus();
    }
}

// Select Root Provider
#[component]
pub fn Select(
    #[prop(into, optional)] open: Signal<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] value: Signal<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] required: Signal<bool>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let internal_open = RwSignal::new(false);
    let internal_value = RwSignal::new(default_value.get_untracked().unwrap_or_default());

    let open_state = Signal::derive(move || {
        if open.get() != internal_open.get() {
            open.get()
        } else {
            internal_open.get()
        }
    });

    let value_state = Signal::derive(move || {
        if !value.get().is_empty() && value.get() != internal_value.get() {
            value.get()
        } else {
            internal_value.get()
        }
    });

    let set_open = Callback::new(move |new_open: bool| {
        internal_open.set(new_open);
        if let Some(callback) = &on_open_change {
            callback.run(new_open);
        }
    });

    let set_value = Callback::new(move |new_value: String| {
        internal_value.set(new_value.clone());
        if let Some(callback) = &on_value_change {
            callback.run(new_value);
        }
    });

    let instance = SELECT_INSTANCE.fetch_add(1, Ordering::Relaxed);
    let ctx = SelectContext {
        open: open_state,
        set_open,
        value: value_state,
        set_value,
        disabled,
        required,
        name,
        active: RwSignal::new(None),
        items: RwSignal::new(Vec::new()),
        trigger_id: StoredValue::new(format!("select-{instance}-trigger")),
        listbox_id: StoredValue::new(format!("select-{instance}-listbox")),
    };
    provide_context(ctx);

    // Kept for backwards compatibility with code reading these contexts directly.
    provide_context(value_state);
    provide_context(set_open);
    provide_context(set_value);
    provide_context(name);

    view! {
        {children.map(|c| c())}
        // Native form integration: submits `name=value` with the enclosing form.
        <input
            type="hidden"
            name=move || name.get()
            prop:value=move || value_state.get()
            value=move || value_state.get()
            required=move || required.get()
            disabled=move || disabled.get()
            aria-hidden="true"
            data-select-input=""
        />
    }
}

// Select Trigger
#[component]
pub fn SelectTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<SelectContext>();
    if let Some(custom) = id.get_untracked() {
        ctx.trigger_id.set_value(custom);
    }

    let trigger_class = format!(
        "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1 {}",
        class.get_untracked().unwrap_or_default()
    );

    let open_with = move |active: Option<String>| {
        ctx.active.set(active);
        ctx.set_open.run(true);
    };

    let handle_click = move |_e: MouseEvent| {
        if ctx.disabled.get_untracked() {
            return;
        }
        if ctx.open.get_untracked() {
            ctx.set_open.run(false);
        } else {
            let current = ctx.value.get_untracked();
            open_with((!current.is_empty()).then_some(current));
        }
    };

    let handle_keydown = move |ev: KeyboardEvent| {
        if ctx.disabled.get_untracked() {
            return;
        }
        let items = ctx.items.get_untracked();
        let disabled: Vec<bool> = items.iter().map(|i| i.disabled.get_untracked()).collect();
        let active = ctx.active.get_untracked();
        let active_idx = active
            .as_ref()
            .and_then(|a| items.iter().position(|i| &i.value == a));
        let selected = ctx.value.get_untracked();
        let selected_idx = items.iter().position(|i| i.value == selected);

        let action = key_action(
            &ev.key(),
            ctx.open.get_untracked(),
            active_idx,
            selected_idx,
            &disabled,
        );
        if action.prevents_default() {
            ev.prevent_default();
        }
        match action {
            SelectKeyAction::Open(idx) => open_with(idx.map(|i| items[i].value.clone())),
            SelectKeyAction::Highlight(idx) => {
                ctx.active.set(Some(items[idx].value.clone()));
                if let Some(el) = element_by_id(&items[idx].id) {
                    el.scroll_into_view_with_bool(false);
                }
            }
            SelectKeyAction::SelectActive => {
                if let Some(idx) = active_idx.filter(|&i| !disabled[i]) {
                    ctx.set_value.run(items[idx].value.clone());
                }
                ctx.set_open.run(false);
                focus_by_id(&ctx.trigger_id.get_value());
            }
            SelectKeyAction::Close => {
                ctx.set_open.run(false);
                focus_by_id(&ctx.trigger_id.get_value());
            }
            SelectKeyAction::CloseAndContinue => ctx.set_open.run(false),
            SelectKeyAction::None => {}
        }
    };

    // Space activates <button> on keyup; suppress it since keydown handled it.
    let handle_keyup = move |ev: KeyboardEvent| {
        if ev.key() == " " {
            ev.prevent_default();
        }
    };

    view! {
        <button
            type="button"
            class=trigger_class
            id=ctx.trigger_id.get_value()
            style=style.get_untracked()
            on:click=handle_click
            on:keydown=handle_keydown
            on:keyup=handle_keyup
            disabled=ctx.disabled
            data-disabled=move || ctx.disabled.get().then_some("")
            data-state=move || if ctx.open.get() { "open" } else { "closed" }
            aria-expanded=move || ctx.open.get().to_string()
            aria-haspopup="listbox"
            aria-controls=ctx.listbox_id.get_value()
            aria-required=move || ctx.required.get().to_string()
            aria-activedescendant=move || {
                if ctx.open.get() { ctx.active_descendant() } else { None }
            }
            role="combobox"
        >
            {children.map(|c| c())}
        </button>
    }
}

// Select Value
#[component]
pub fn SelectValue(
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let ctx = expect_context::<SelectContext>();

    let value_class = format!(
        "placeholder:text-muted-foreground {}",
        class.get_untracked().unwrap_or_default()
    );

    view! {
        <span
            class=value_class
            id=id.get_untracked()
            style=style.get_untracked()
        >
            {move || {
                let current_value = ctx.value.get();
                if current_value.is_empty() {
                    placeholder.get().unwrap_or_default()
                } else {
                    current_value
                }
            }}
        </span>
    }
}
