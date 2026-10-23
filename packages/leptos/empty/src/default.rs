//! Default style for empty.

use leptos::prelude::*;

/// Merge a base class string with an optional user class.
pub(crate) fn cn(base: &str, extra: Option<String>) -> String {
    match extra {
        Some(e) if !e.trim().is_empty() => format!("{base} {}", e.trim()),
        _ => base.to_string(),
    }
}

pub const EMPTY_CLASS: &str = "flex min-w-0 flex-1 flex-col items-center justify-center gap-6 rounded-lg border-dashed p-6 text-center text-balance md:p-12";
pub const EMPTY_HEADER_CLASS: &str = "flex max-w-sm flex-col items-center gap-2 text-center";
pub const EMPTY_MEDIA_CLASS: &str = "flex shrink-0 items-center justify-center mb-2 [&_svg]:pointer-events-none [&_svg]:shrink-0";
pub const EMPTY_MEDIA_DEFAULT_CLASS: &str = "bg-transparent";
pub const EMPTY_MEDIA_ICON_CLASS: &str = "bg-muted text-foreground flex size-10 shrink-0 items-center justify-center rounded-lg [&_svg:not([class*='size-'])]:size-6";
pub const EMPTY_TITLE_CLASS: &str = "text-lg font-medium tracking-tight";
pub const EMPTY_DESCRIPTION_CLASS: &str = "text-muted-foreground [&>a:hover]:text-primary text-sm/relaxed [&>a]:underline [&>a]:underline-offset-4";
pub const EMPTY_CONTENT_CLASS: &str = "flex w-full max-w-sm min-w-0 flex-col items-center gap-4 text-sm text-balance";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EmptyMediaVariant {
    #[default]
    Default,
    Icon,
}

impl EmptyMediaVariant {
    /// Tailwind classes for this variant.
    pub fn class(&self) -> &'static str {
        match self {
            EmptyMediaVariant::Default => EMPTY_MEDIA_DEFAULT_CLASS,
            EmptyMediaVariant::Icon => EMPTY_MEDIA_ICON_CLASS,
        }
    }

    /// shadcn/ui variant name (used in data attributes).
    pub fn as_str(&self) -> &'static str {
        match self {
            EmptyMediaVariant::Default => "default",
            EmptyMediaVariant::Icon => "icon",
        }
    }
}

/// Full class for EmptyMedia.
pub fn empty_media_class(variant: EmptyMediaVariant, extra: Option<String>) -> String {
    cn(&format!("{} {}", EMPTY_MEDIA_CLASS, variant.class()), extra)
}

/// Media (icon/avatar) area of an empty state.
#[component]
pub fn EmptyMedia(
    #[prop(into, optional)] variant: MaybeProp<EmptyMediaVariant>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let v = move || variant.get().unwrap_or_default();
    view! {
        <div
            data-slot="empty-icon"
            data-variant=move || v().as_str()
            class=move || empty_media_class(v(), class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// `Empty` component (data-slot="empty").
#[component]
pub fn Empty(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="empty"
            class=move || cn(EMPTY_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// `EmptyHeader` component (data-slot="empty-header").
#[component]
pub fn EmptyHeader(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="empty-header"
            class=move || cn(EMPTY_HEADER_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// `EmptyTitle` component (data-slot="empty-title").
#[component]
pub fn EmptyTitle(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="empty-title"
            class=move || cn(EMPTY_TITLE_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// `EmptyDescription` component (data-slot="empty-description").
#[component]
pub fn EmptyDescription(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="empty-description"
            class=move || cn(EMPTY_DESCRIPTION_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// `EmptyContent` component (data-slot="empty-content").
#[component]
pub fn EmptyContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="empty-content"
            class=move || cn(EMPTY_CONTENT_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}
