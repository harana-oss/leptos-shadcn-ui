//! Resizable components (default style)

use leptos::prelude::*;
use leptos_style::Style;

use crate::shared::{render_group, render_handle, render_panel, GroupProps, HandleProps, PanelProps};
pub use crate::resizable::ResizeDirection;

/// Resizable panel group component
#[component]
pub fn ResizablePanelGroup(
    #[prop(into, optional)] direction: MaybeProp<ResizeDirection>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] keyboard_resize: MaybeProp<bool>,
    #[prop(into, optional)] touch_support: MaybeProp<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_resize: MaybeProp<Callback<Vec<f64>>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    render_group(GroupProps {
        base_class: "resizable-panel-group",
        direction,
        class,
        id,
        style,
        keyboard_resize,
        touch_support,
        aria_label,
        on_resize,
        children,
    })
}

/// Individual resizable panel component
#[component]
pub fn ResizablePanel(
    #[prop(into, optional)] default_size: MaybeProp<f64>,
    #[prop(into, optional)] min_size: MaybeProp<f64>,
    #[prop(into, optional)] max_size: MaybeProp<f64>,
    #[prop(into, optional)] collapsible: MaybeProp<bool>,
    #[prop(into, optional)] collapsed_size: MaybeProp<f64>,
    #[prop(into, optional)] collapsed: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_resize: MaybeProp<Callback<f64>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    render_panel(PanelProps {
        base_class: "resizable-panel",
        button_class: "collapse-button absolute top-2 right-2 z-10 p-1 rounded hover:bg-gray-200",
        content_class: "panel-content",
        default_size,
        min_size,
        max_size,
        collapsible,
        collapsed_size,
        collapsed,
        class,
        id,
        style,
        aria_label,
        on_resize,
        children,
    })
}

/// Resizable handle component
#[component]
pub fn ResizableHandle(
    #[prop(into, optional)] with_handle: MaybeProp<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] role: MaybeProp<String>,
    #[prop(into, optional)] keyboard_resize: MaybeProp<bool>,
    #[prop(into, optional)] touch_support: MaybeProp<bool>,
) -> impl IntoView {
    render_handle(HandleProps {
        base_class: "resizable-handle",
        grip_class: "handle-grip",
        dots_class: "grip-dots",
        with_handle,
        class,
        disabled,
        aria_label,
        role,
        keyboard_resize,
        touch_support,
    })
}
