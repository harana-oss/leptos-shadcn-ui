//! ContextMenu trigger component
//!
//! This module contains the ContextMenuTrigger component for handling
//! right-click events and positioning.

use crate::default_components::context_menu::ContextMenuId;
use leptos::prelude::*;
use web_sys::MouseEvent;

#[component]
pub fn ContextMenuTrigger(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let open = expect_context::<RwSignal<bool>>();
    let position = expect_context::<RwSignal<(i32, i32)>>();
    let menu_id = expect_context::<ContextMenuId>().0;

    let handle_context_menu = move |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        position.set((e.client_x(), e.client_y()));
        // Closes any other open context menu; a single shared document
        // listener handles outside clicks and Escape.
        crate::global::open_menu(menu_id, open);
    };

    let handle_click = move |_| {
        crate::global::close_menu(menu_id);
    };

    view! {
        <div
            class=move || format!("cursor-pointer {}", class.get().unwrap_or_default())
            on:contextmenu=handle_context_menu
            on:click=handle_click
        >
            {children.map(|c| c())}
        </div>
    }
}
