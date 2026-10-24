//! ContextMenu submenu components
//! 
//! This module contains the ContextMenuSub, ContextMenuSubTrigger,
//! and ContextMenuSubContent components for nested context menus.

use leptos::prelude::*;
use web_sys::MouseEvent;

/// Open state of a submenu. A newtype so it does not shadow the root menu's
/// `RwSignal<bool>`, which items use to close the whole menu.
#[derive(Clone, Copy)]
struct SubMenuOpen(RwSignal<bool>);

#[component]
pub fn ContextMenuSub(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(SubMenuOpen(open));

    // Closing on leaving the whole submenu (trigger and content) lets the
    // pointer travel from the trigger into the content without it closing.
    view! {
        <div class="context-menu-sub" on:mouseleave=move |_| open.set(false)>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuSubTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let SubMenuOpen(open) = expect_context::<SubMenuOpen>();

    let handle_mouse_enter = move |_| {
        if !disabled.get().unwrap_or(false) {
            open.set(true);
        }
    };

    let trigger_class = move || {
        let base_class = "context-menu-sub-trigger";
        let disabled_class = if disabled.get().unwrap_or(false) { " disabled" } else { "" };
        let custom_class = class.get().unwrap_or_default();
        format!("{}{} {}", base_class, disabled_class, custom_class)
    };

    view! {
        <div
            class=trigger_class
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
            on:mouseenter=handle_mouse_enter
            role="menuitem"
            aria-haspopup="menu"
            aria-expanded=move || open.get().to_string()
            aria-disabled=disabled.get().unwrap_or(false)
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuSubContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let SubMenuOpen(open) = expect_context::<SubMenuOpen>();

    let handle_click = move |e: MouseEvent| {
        e.stop_propagation();
    };

    view! {
        <div
            class=move || format!("context-menu-sub-content {}", class.get().unwrap_or_default())
            id=move || id.get().unwrap_or_default()
            style=move || style.get().unwrap_or_default()
            style:display=move || (!open.get()).then_some("none")
            data-state=move || if open.get() { "open" } else { "closed" }
            on:click=handle_click
            role="menu"
            aria-orientation="vertical"
        >
            {children.map(|c| c())}
        </div>
    }
}
