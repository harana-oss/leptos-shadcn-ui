//! Default style for button-group.

use leptos::prelude::*;

/// Merge a base class string with an optional user class.
pub(crate) fn cn(base: &str, extra: Option<String>) -> String {
    match extra {
        Some(e) if !e.trim().is_empty() => format!("{base} {}", e.trim()),
        _ => base.to_string(),
    }
}

pub const BUTTON_GROUP_CLASS: &str = "flex w-fit items-stretch has-[>[data-slot=button-group]]:gap-2 [&>*]:focus-visible:relative [&>*]:focus-visible:z-10 has-[select[aria-hidden=true]:last-child]:[&>[data-slot=select-trigger]:last-of-type]:rounded-r-md [&>[data-slot=select-trigger]:not([class*='w-'])]:w-fit [&>input]:flex-1";
pub const BUTTON_GROUP_HORIZONTAL_CLASS: &str = "[&>*:not(:first-child)]:rounded-l-none [&>*:not(:first-child)]:border-l-0 [&>*:not(:last-child)]:rounded-r-none";
pub const BUTTON_GROUP_VERTICAL_CLASS: &str = "flex-col [&>*:not(:first-child)]:rounded-t-none [&>*:not(:first-child)]:border-t-0 [&>*:not(:last-child)]:rounded-b-none";
pub const BUTTON_GROUP_TEXT_CLASS: &str = "bg-muted flex items-center gap-2 rounded-md border px-4 text-sm font-medium shadow-xs [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4";
pub const BUTTON_GROUP_SEPARATOR_CLASS: &str = "bg-input relative !m-0 self-stretch shrink-0 data-[orientation=vertical]:h-auto data-[orientation=vertical]:w-px data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl ButtonGroupOrientation {
    /// Tailwind classes for this variant.
    pub fn class(&self) -> &'static str {
        match self {
            ButtonGroupOrientation::Horizontal => BUTTON_GROUP_HORIZONTAL_CLASS,
            ButtonGroupOrientation::Vertical => BUTTON_GROUP_VERTICAL_CLASS,
        }
    }

    /// shadcn/ui variant name (used in data attributes).
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonGroupOrientation::Horizontal => "horizontal",
            ButtonGroupOrientation::Vertical => "vertical",
        }
    }
}

/// Full class for a button group with the given orientation.
pub fn button_group_class(orientation: ButtonGroupOrientation, extra: Option<String>) -> String {
    cn(&format!("{} {}", BUTTON_GROUP_CLASS, orientation.class()), extra)
}

/// Groups related buttons together (data-slot="button-group").
#[component]
pub fn ButtonGroup(
    #[prop(into, optional)] orientation: MaybeProp<ButtonGroupOrientation>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let o = move || orientation.get().unwrap_or_default();
    view! {
        <div
            role="group"
            data-slot="button-group"
            data-orientation=move || o().as_str()
            class=move || button_group_class(o(), class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Separator between buttons in a group. Orientation defaults to vertical.
#[component]
pub fn ButtonGroupSeparator(
    #[prop(into, optional)] orientation: MaybeProp<ButtonGroupOrientation>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let o = move || orientation.get().unwrap_or(ButtonGroupOrientation::Vertical);
    view! {
        <div
            role="separator"
            data-slot="button-group-separator"
            data-orientation=move || o().as_str()
            aria-orientation=move || o().as_str()
            class=move || cn(BUTTON_GROUP_SEPARATOR_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        ></div>
    }
}

/// `ButtonGroupText` component (data-slot="button-group-text").
#[component]
pub fn ButtonGroupText(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="button-group-text"
            class=move || cn(BUTTON_GROUP_TEXT_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}
