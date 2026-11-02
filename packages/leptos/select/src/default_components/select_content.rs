use leptos::{ev::MouseEvent, prelude::*, wasm_bindgen::JsCast};

use super::select_root::{SELECT_ITEM_KEY, SelectContext, SelectItemEntry, element_by_id};

// Select Content
#[component]
pub fn SelectContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<SelectContext>();
    if let Some(custom) = id.get_untracked() {
        ctx.listbox_id.set_value(custom);
    }

    let content_class = format!(
        "relative z-50 max-h-96 min-w-[8rem] overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 {}",
        class.get_untracked().unwrap_or_default()
    );

    // Close when a pointer goes down outside both the trigger and the listbox.
    // The listener is removed when this component is disposed.
    let handle_click_outside = move |e: MouseEvent| {
        if !ctx.open.get_untracked() {
            return;
        }
        let target = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
        let inside = |id: String| element_by_id(&id).is_some_and(|el| el.contains(target.as_ref()));
        if !inside(ctx.trigger_id.get_value()) && !inside(ctx.listbox_id.get_value()) {
            ctx.set_open.run(false);
        }
    };
    let listener = window_event_listener(leptos::ev::mousedown, handle_click_outside);
    on_cleanup(move || listener.remove());

    view! {
        <div
            class=content_class
            id=ctx.listbox_id.get_value()
            style=style.get_untracked()
            data-state=move || if ctx.open.get() { "open" } else { "closed" }
            hidden=move || !ctx.open.get()
            role="listbox"
            aria-labelledby=ctx.trigger_id.get_value()
            aria-required=move || ctx.required.get().to_string()
        >
            {children.map(|c| c())}
        </div>
    }
}

// Select Item
#[component]
pub fn SelectItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = expect_context::<SelectContext>();

    let item_class = format!(
        "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none focus:bg-accent focus:text-accent-foreground data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 {}",
        class.get_untracked().unwrap_or_default()
    );

    let key = SELECT_ITEM_KEY.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let item_id = id
        .get_untracked()
        .unwrap_or_else(|| format!("{}-option-{key}", ctx.listbox_id.get_value()));
    ctx.items.update(|items| {
        items.push(SelectItemEntry {
            key,
            value: value.clone(),
            disabled,
            id: item_id.clone(),
        })
    });
    on_cleanup(move || {
        let _ = ctx.items.try_update(|items| items.retain(|i| i.key != key));
    });

    let value = StoredValue::new(value);
    let is_selected = Signal::derive(move || ctx.value.get() == value.get_value());
    let is_active =
        Signal::derive(move || ctx.active.get().is_some_and(|a| a == value.get_value()));

    let handle_click = move |_e: MouseEvent| {
        if disabled.get_untracked() {
            return;
        }
        ctx.set_value.run(value.get_value());
        ctx.set_open.run(false);
        super::select_root::focus_by_id(&ctx.trigger_id.get_value());
    };

    let handle_pointer = move |_e: MouseEvent| {
        if !disabled.get_untracked() {
            ctx.active.set(Some(value.get_value()));
        }
    };

    view! {
        <div
            class=item_class
            id=item_id
            style=style.get_untracked()
            on:click=handle_click
            on:mousemove=handle_pointer
            role="option"
            aria-selected=move || is_selected.get().to_string()
            aria-disabled=move || disabled.get().then_some("true")
            data-disabled=move || disabled.get().then_some("")
            data-highlighted=move || is_active.get().then_some("")
            data-state=move || if is_selected.get() { "checked" } else { "unchecked" }
            data-value=value.get_value()
        >
            {children.map(|c| c())}
        </div>
    }
}

// Select Separator
#[component]
pub fn SelectSeparator(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let separator_class = format!(
        "-mx-1 my-1 h-px bg-muted {}",
        class.get().unwrap_or_default()
    );

    view! {
        <div
            class=separator_class
            id=id.get()
            style=style.get()
            role="separator"
        />
    }
}

// Select Label
#[component]
pub fn SelectLabel(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let label_class = format!(
        "py-1.5 pl-8 pr-2 text-sm font-semibold {}",
        class.get().unwrap_or_default()
    );

    view! {
        <div
            class=label_class
            id=id.get()
            style=style.get()
            role="group"
        >
            {children.map(|c| c())}
        </div>
    }
}

// Select Group
#[component]
pub fn SelectGroup(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let group_class = format!(
        "p-1 text-muted-foreground {}",
        class.get().unwrap_or_default()
    );

    view! {
        <div
            class=group_class
            id=id.get()
            style=style.get()
            role="group"
        >
            {children.map(|c| c())}
        </div>
    }
}
