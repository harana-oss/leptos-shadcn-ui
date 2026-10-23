//! Default style for kbd.

use leptos::prelude::*;

/// Merge a base class string with an optional user class.
pub fn cn(base: &str, extra: Option<String>) -> String {
    match extra {
        Some(e) if !e.trim().is_empty() => format!("{base} {}", e.trim()),
        _ => base.to_string(),
    }
}

pub const KBD_CLASS: &str = "bg-muted text-muted-foreground pointer-events-none inline-flex h-5 w-fit min-w-5 items-center justify-center gap-1 rounded-sm px-1 font-sans text-xs font-medium select-none [&_svg:not([class*='size-'])]:size-3 [[data-slot=tooltip-content]_&]:bg-background/20 [[data-slot=tooltip-content]_&]:text-background dark:[[data-slot=tooltip-content]_&]:bg-background/10";
pub const KBD_GROUP_CLASS: &str = "inline-flex items-center gap-1";

/// `Kbd` component (data-slot="kbd").
#[component]
pub fn Kbd(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <kbd
            data-slot="kbd"
            class=move || cn(KBD_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </kbd>
    }
}

/// `KbdGroup` component (data-slot="kbd-group").
#[component]
pub fn KbdGroup(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <kbd
            data-slot="kbd-group"
            class=move || cn(KBD_GROUP_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </kbd>
    }
}
