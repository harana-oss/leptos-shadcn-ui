//! Main ContextMenu component
//!
//! This module contains the main ContextMenu component that provides context
//! for the context menu system.

use leptos::prelude::*;

/// Identifies the enclosing context menu in the shared open-menu registry.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContextMenuId(pub u64);

#[component]
pub fn ContextMenu(#[prop(optional)] children: Option<Children>) -> impl IntoView {
    let open = RwSignal::new(false);
    let position = RwSignal::new((0, 0));
    let menu_id = crate::global::next_menu_id();

    // Keep the shared registry in sync when the menu is closed locally
    // (item selection, trigger click) and when it unmounts.
    Effect::new(move |_| {
        if !open.get() {
            crate::global::release_menu(menu_id);
        }
    });
    on_cleanup(move || crate::global::release_menu(menu_id));

    provide_context(open);
    provide_context(position);
    provide_context(ContextMenuId(menu_id));

    view! {
        <div class="relative">
            {children.map(|c| c())}
        </div>
    }
}
