//! New York style hover card. Shares state/context with the default variant
//! and only adjusts styling.
use leptos::prelude::*;
use leptos_style::Style;

pub use crate::default::HoverCardContext;

const NEW_YORK_TRIGGER_CLASS: &str = "text-sm";
const NEW_YORK_CONTENT_CLASS: &str = "shadow-md";

fn merge(base: &str, extra: Option<String>) -> String {
    format!("{} {}", base, extra.unwrap_or_default())
}

#[component]
pub fn HoverCard(
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
    crate::default::HoverCard(crate::default::HoverCardProps {
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
    })
}

#[component]
pub fn HoverCardTrigger(
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] role: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<String>,
    #[prop(into, optional)] lang: MaybeProp<String>,
    #[prop(into, optional)] tabindex: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    crate::default::HoverCardTrigger(crate::default::HoverCardTriggerProps {
        disabled,
        class: MaybeProp::derive(move || Some(merge(NEW_YORK_TRIGGER_CLASS, class.get()))),
        id,
        style,
        role,
        dir,
        lang,
        tabindex,
        children,
    })
}

#[component]
pub fn HoverCardContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] role: MaybeProp<String>,
    #[prop(into, optional)] dir: MaybeProp<String>,
    #[prop(into, optional)] lang: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    crate::default::HoverCardContent(crate::default::HoverCardContentProps {
        class: MaybeProp::derive(move || Some(merge(NEW_YORK_CONTENT_CLASS, class.get()))),
        id,
        style,
        role,
        dir,
        lang,
        children,
    })
}
