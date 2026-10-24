//! Drawer nested components
//!
//! This module contains nested drawer components for complex drawer hierarchies.

use super::types::DrawerDirection;
use leptos::prelude::*;

#[component]
pub fn DrawerNestedRoot(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] direction: Signal<DrawerDirection>,
    #[prop(into, optional)] should_scale_background: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    provide_context(open);
    provide_context(on_open_change);
    provide_context(direction);
    provide_context(should_scale_background);

    view! {
        <div class="drawer-nested-root">
            {children.map(|c| c())}
        </div>
    }
}
