//! New York style popover. Shares state/context with the default variant and
//! only adjusts styling.
use leptos::prelude::*;
use leptos_style::Style;

pub use crate::default::PopoverContext;

const NEW_YORK_TRIGGER_CLASS: &str = "shadow-sm";
const NEW_YORK_CONTENT_CLASS: &str = "shadow-md";

fn merge(base: &str, extra: Option<String>) -> String {
    format!("{} {}", base, extra.unwrap_or_default())
}

#[component]
pub fn Popover(
    #[prop(into, optional)] variant: MaybeProp<String>,
    #[prop(into, optional)] size: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] open: Option<RwSignal<bool>>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    crate::default::Popover(
        crate::default::PopoverProps {
            variant,
            size,
            on_click,
            disabled,
            open,
            on_open_change,
            class,
            id,
            style,
            children,
        },
    )
}

#[component]
pub fn PopoverTrigger(
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    crate::default::PopoverTrigger(crate::default::PopoverTriggerProps {
        disabled,
        class: MaybeProp::derive(move || Some(merge(NEW_YORK_TRIGGER_CLASS, class.get()))),
        id,
        style,
        children,
    })
}

#[component]
pub fn PopoverContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    crate::default::PopoverContent(crate::default::PopoverContentProps {
        class: MaybeProp::derive(move || Some(merge(NEW_YORK_CONTENT_CLASS, class.get()))),
        id,
        style,
        children,
    })
}
