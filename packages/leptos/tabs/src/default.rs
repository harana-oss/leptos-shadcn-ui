use leptos::{
    ev::{KeyboardEvent, MouseEvent},
    prelude::*,
    wasm_bindgen::JsCast,
};
use leptos_node_ref::AnyNodeRef;
use leptos_struct_component::StructComponent;
use std::sync::atomic::{AtomicUsize, Ordering};

use leptos_style::Style;

use crate::keyboard::{Orientation, next_index, sanitize_id_part, tab_stop_index};

static TABS_INSTANCE: AtomicUsize = AtomicUsize::new(0);
static TRIGGER_KEY: AtomicUsize = AtomicUsize::new(0);

/// A trigger registered with its Tabs root, in render (DOM) order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TabTriggerEntry {
    key: usize,
    pub value: Signal<String>,
    pub disabled: Signal<bool>,
    pub id: Signal<String>,
}

// Tabs Root Provider
#[component]
pub fn Tabs(
    #[prop(into, optional)] value: Signal<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    /// `"horizontal"` (default) or `"vertical"`.
    #[prop(into, optional)]
    orientation: MaybeProp<String>,
    /// `"automatic"` (default: focus moves activate the tab) or `"manual"`.
    #[prop(into, optional)]
    activation_mode: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let internal_value = RwSignal::new(default_value.get_untracked().unwrap_or_default());

    let value_state = Signal::derive(move || {
        if !value.get().is_empty() && value.get() != internal_value.get() {
            value.get()
        } else {
            internal_value.get()
        }
    });

    let set_value = Callback::new(move |new_value: String| {
        internal_value.set(new_value.clone());
        if let Some(callback) = &on_value_change {
            callback.run(new_value);
        }
    });

    let orientation =
        Signal::derive(move || Orientation::parse(&orientation.get().unwrap_or_default()));
    let manual = Signal::derive(move || {
        activation_mode
            .get()
            .is_some_and(|m| m.eq_ignore_ascii_case("manual"))
    });

    provide_context(TabsContextValue {
        value: value_state,
        set_value,
        base_id: StoredValue::new(format!(
            "tabs-{}",
            TABS_INSTANCE.fetch_add(1, Ordering::Relaxed)
        )),
        triggers: RwSignal::new(Vec::new()),
        orientation,
        manual,
    });

    let tabs_class = Signal::derive(move || class.get().unwrap_or_default().to_string());

    view! {
        <div class={tabs_class} data-orientation=move || orientation.get().as_str()>
            {children.map(|c| c())}
        </div>
    }
}

#[derive(Clone, Copy)]
pub struct TabsContextValue {
    pub value: Signal<String>,
    pub set_value: Callback<String>,
    /// Unique per Tabs instance; prefix of generated trigger/panel ids.
    pub base_id: StoredValue<String>,
    /// Registered triggers in render order.
    pub triggers: RwSignal<Vec<TabTriggerEntry>>,
    pub orientation: Signal<Orientation>,
    pub manual: Signal<bool>,
}

impl TabsContextValue {
    pub fn trigger_id(&self, value: &str) -> String {
        format!(
            "{}-trigger-{}",
            self.base_id.get_value(),
            sanitize_id_part(value)
        )
    }

    pub fn content_id(&self, value: &str) -> String {
        format!(
            "{}-content-{}",
            self.base_id.get_value(),
            sanitize_id_part(value)
        )
    }
}

fn focus_element_by_id(id: &str) {
    if let Some(el) = document().get_element_by_id(id) {
        let _ = el.unchecked_into::<web_sys::HtmlElement>().focus();
    }
}

// Tabs List
#[component]
pub fn TabsList(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<TabsContextValue>();

    let list_class = Signal::derive(move || {
        format!(
            "inline-flex h-10 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground {}",
            class.get().unwrap_or_default()
        )
    });

    let on_keydown = move |ev: KeyboardEvent| {
        let triggers = ctx.triggers.get_untracked();
        let disabled: Vec<bool> = triggers
            .iter()
            .map(|t| t.disabled.get_untracked())
            .collect();
        let target_id = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .map(|el| el.id());
        let current = target_id
            .as_ref()
            .and_then(|id| triggers.iter().position(|t| &t.id.get_untracked() == id));
        if current.is_none() {
            // Key pressed on something other than a trigger (e.g. as_child content).
            return;
        }
        let key = ev.key();
        if let Some(next) = next_index(current, &key, &disabled, ctx.orientation.get_untracked()) {
            ev.prevent_default();
            let entry = triggers[next];
            focus_element_by_id(&entry.id.get_untracked());
            if !ctx.manual.get_untracked() {
                ctx.set_value.run(entry.value.get_untracked());
            }
        }
    };

    view! {
        <div
            class={list_class}
            role="tablist"
            aria-orientation=move || ctx.orientation.get().as_str()
            data-orientation=move || ctx.orientation.get().as_str()
            on:keydown=on_keydown
        >
            {children.map(|c| c())}
        </div>
    }
}

// Tabs Trigger
#[derive(Clone, StructComponent)]
#[struct_component(tag = "button")]
pub struct TabsTriggerChildProps {
    pub node_ref: AnyNodeRef,
    pub class: Signal<String>,
    pub id: Signal<String>,
    pub style: Signal<Style>,
    pub disabled: Signal<bool>,
    pub r#type: MaybeProp<String>,
    pub role: Signal<String>,
    pub aria_selected: Signal<String>,
    pub aria_controls: Signal<String>,
    pub tabindex: Signal<String>,
    pub onclick: Option<Callback<MouseEvent>>,
}

#[component]
pub fn TabsTrigger(
    #[prop(into)] value: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: Option<Callback<TabsTriggerChildProps, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<TabsContextValue>();

    let value_sig = Signal::derive(move || value.get().unwrap_or_default());
    let trigger_id =
        Signal::derive(move || id.get().unwrap_or_else(|| ctx.trigger_id(&value_sig.get())));
    let content_id = Signal::derive(move || ctx.content_id(&value_sig.get()));

    let key = TRIGGER_KEY.fetch_add(1, Ordering::Relaxed);
    ctx.triggers.update(|list| {
        list.push(TabTriggerEntry {
            key,
            value: value_sig,
            disabled,
            id: trigger_id,
        })
    });
    on_cleanup(move || {
        let _ = ctx
            .triggers
            .try_update(|list| list.retain(|entry| entry.key != key));
    });

    let is_selected = Signal::derive(move || ctx.value.get() == value_sig.get());

    let tabindex = Signal::derive(move || {
        let triggers = ctx.triggers.get();
        let disabled_list: Vec<bool> = triggers.iter().map(|t| t.disabled.get()).collect();
        let selected = ctx.value.get();
        let selected_idx = triggers.iter().position(|t| t.value.get() == selected);
        let stop = tab_stop_index(selected_idx, &disabled_list);
        let mine = triggers.iter().position(|t| t.key == key);
        if stop.is_some() && stop == mine {
            "0".to_string()
        } else {
            "-1".to_string()
        }
    });

    let trigger_class = Signal::derive(move || {
        let base_class = "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";
        let selected_class = if is_selected.get() {
            "bg-background text-foreground shadow-sm"
        } else {
            "hover:bg-background hover:text-foreground"
        };
        format!(
            "{} {} {}",
            base_class,
            selected_class,
            class.get().unwrap_or_default()
        )
    });

    let onclick = Callback::new(move |_: MouseEvent| {
        if !disabled.get_untracked() {
            ctx.set_value.run(value_sig.get_untracked());
        }
    });

    if let Some(as_child) = as_child.as_ref() {
        let child_props = TabsTriggerChildProps {
            node_ref,
            class: trigger_class,
            id: trigger_id,
            style,
            disabled,
            r#type: "button".to_string().into(),
            role: "tab".to_string().into(),
            aria_selected: Signal::derive(move || is_selected.get().to_string()),
            aria_controls: content_id,
            tabindex,
            onclick: Some(onclick),
        };
        as_child.run(child_props)
    } else {
        view! {
            <button
                node_ref=node_ref
                type="button"
                role="tab"
                class=trigger_class
                id=trigger_id
                style=style
                disabled=disabled
                data-disabled=move || disabled.get().then_some("")
                data-state=move || if is_selected.get() { "active" } else { "inactive" }
                data-orientation=move || ctx.orientation.get().as_str()
                aria-selected=move || is_selected.get().to_string()
                aria-controls=content_id
                tabindex=tabindex
                on:click=move |ev| onclick.run(ev)
            >
                {children.map(|c| c())}
            </button>
        }
        .into_any()
    }
}

// Tabs Content
#[component]
pub fn TabsContent(
    #[prop(into)] value: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<TabsContextValue>();

    let value_sig = Signal::derive(move || value.get().unwrap_or_default());
    let is_selected = Signal::derive(move || ctx.value.get() == value_sig.get());
    let content_id = Signal::derive(move || ctx.content_id(&value_sig.get()));
    let labelled_by = Signal::derive(move || {
        let v = value_sig.get();
        ctx.triggers
            .get()
            .iter()
            .find(|t| t.value.get() == v)
            .map(|t| t.id.get())
            .unwrap_or_else(|| ctx.trigger_id(&v))
    });

    let content_class = Signal::derive(move || {
        format!(
            "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 {}",
            class.get().unwrap_or_default()
        )
    });

    view! {
        <div
            class={content_class}
            role="tabpanel"
            id=content_id
            aria-labelledby=labelled_by
            tabindex="0"
            data-state=move || if is_selected.get() { "active" } else { "inactive" }
            data-orientation=move || ctx.orientation.get().as_str()
            hidden=move || !is_selected.get()
            style={move || if is_selected.get() { "" } else { "display: none;" }}
        >
            {children.map(|c| c())}
        </div>
    }
}
