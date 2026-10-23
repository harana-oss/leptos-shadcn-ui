//! Default style for item.

use leptos::prelude::*;

/// Merge a base class string with an optional user class.
pub(crate) fn cn(base: &str, extra: Option<String>) -> String {
    match extra {
        Some(e) if !e.trim().is_empty() => format!("{base} {}", e.trim()),
        _ => base.to_string(),
    }
}

pub const ITEM_CLASS: &str = "group/item flex items-center border border-transparent text-sm rounded-md transition-colors [a]:hover:bg-accent/50 [a]:transition-colors duration-100 flex-wrap outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]";
pub const ITEM_VARIANT_DEFAULT_CLASS: &str = "bg-transparent";
pub const ITEM_VARIANT_OUTLINE_CLASS: &str = "border-border";
pub const ITEM_VARIANT_MUTED_CLASS: &str = "bg-muted/50";
pub const ITEM_SIZE_DEFAULT_CLASS: &str = "p-4 gap-4";
pub const ITEM_SIZE_SM_CLASS: &str = "py-3 px-4 gap-2.5";
pub const ITEM_GROUP_CLASS: &str = "group/item-group flex flex-col";
pub const ITEM_SEPARATOR_CLASS: &str = "bg-border shrink-0 h-px w-full my-0";
pub const ITEM_MEDIA_CLASS: &str = "flex shrink-0 items-center justify-center gap-2 group-has-[[data-slot=item-description]]/item:self-start [&_svg]:pointer-events-none group-has-[[data-slot=item-description]]/item:translate-y-0.5";
pub const ITEM_MEDIA_DEFAULT_CLASS: &str = "bg-transparent";
pub const ITEM_MEDIA_ICON_CLASS: &str = "size-8 border rounded-sm bg-muted [&_svg:not([class*='size-'])]:size-4";
pub const ITEM_MEDIA_IMAGE_CLASS: &str = "size-10 rounded-sm overflow-hidden [&_img]:size-full [&_img]:object-cover";
pub const ITEM_CONTENT_CLASS: &str = "flex flex-1 flex-col gap-1 [&+[data-slot=item-content]]:flex-none";
pub const ITEM_TITLE_CLASS: &str = "flex w-fit items-center gap-2 text-sm leading-snug font-medium";
pub const ITEM_DESCRIPTION_CLASS: &str = "text-muted-foreground line-clamp-2 text-sm leading-normal font-normal text-balance [&>a:hover]:text-primary [&>a]:underline [&>a]:underline-offset-4";
pub const ITEM_ACTIONS_CLASS: &str = "flex items-center gap-2";
pub const ITEM_HEADER_CLASS: &str = "flex basis-full items-center justify-between gap-2";
pub const ITEM_FOOTER_CLASS: &str = "flex basis-full items-center justify-between gap-2";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

impl ItemVariant {
    /// Tailwind classes for this variant.
    pub fn class(&self) -> &'static str {
        match self {
            ItemVariant::Default => ITEM_VARIANT_DEFAULT_CLASS,
            ItemVariant::Outline => ITEM_VARIANT_OUTLINE_CLASS,
            ItemVariant::Muted => ITEM_VARIANT_MUTED_CLASS,
        }
    }

    /// shadcn/ui variant name (used in data attributes).
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemVariant::Default => "default",
            ItemVariant::Outline => "outline",
            ItemVariant::Muted => "muted",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

impl ItemSize {
    /// Tailwind classes for this variant.
    pub fn class(&self) -> &'static str {
        match self {
            ItemSize::Default => ITEM_SIZE_DEFAULT_CLASS,
            ItemSize::Sm => ITEM_SIZE_SM_CLASS,
        }
    }

    /// shadcn/ui variant name (used in data attributes).
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemSize::Default => "default",
            ItemSize::Sm => "sm",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

impl ItemMediaVariant {
    /// Tailwind classes for this variant.
    pub fn class(&self) -> &'static str {
        match self {
            ItemMediaVariant::Default => ITEM_MEDIA_DEFAULT_CLASS,
            ItemMediaVariant::Icon => ITEM_MEDIA_ICON_CLASS,
            ItemMediaVariant::Image => ITEM_MEDIA_IMAGE_CLASS,
        }
    }

    /// shadcn/ui variant name (used in data attributes).
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemMediaVariant::Default => "default",
            ItemMediaVariant::Icon => "icon",
            ItemMediaVariant::Image => "image",
        }
    }
}

/// Full class for an Item.
pub fn item_class(variant: ItemVariant, size: ItemSize, extra: Option<String>) -> String {
    cn(&format!("{} {} {}", ITEM_CLASS, variant.class(), size.class()), extra)
}

/// Full class for ItemMedia.
pub fn item_media_class(variant: ItemMediaVariant, extra: Option<String>) -> String {
    cn(&format!("{} {}", ITEM_MEDIA_CLASS, variant.class()), extra)
}

/// A list item with media, content and actions.
#[component]
pub fn Item(
    #[prop(into, optional)] variant: MaybeProp<ItemVariant>,
    #[prop(into, optional)] size: MaybeProp<ItemSize>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let v = move || variant.get().unwrap_or_default();
    let s = move || size.get().unwrap_or_default();
    view! {
        <div
            data-slot="item"
            data-variant=move || v().as_str()
            data-size=move || s().as_str()
            class=move || item_class(v(), s(), class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Media slot of an Item.
#[component]
pub fn ItemMedia(
    #[prop(into, optional)] variant: MaybeProp<ItemMediaVariant>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let v = move || variant.get().unwrap_or_default();
    view! {
        <div
            data-slot="item-media"
            data-variant=move || v().as_str()
            class=move || item_media_class(v(), class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Separator between items.
#[component]
pub fn ItemSeparator(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <div
            role="separator"
            aria-orientation="horizontal"
            data-orientation="horizontal"
            data-slot="item-separator"
            class=move || cn(ITEM_SEPARATOR_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        ></div>
    }
}

/// `ItemGroup` component (data-slot="item-group").
#[component]
pub fn ItemGroup(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="item-group"
            role="list"
            class=move || cn(ITEM_GROUP_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// `ItemContent` component (data-slot="item-content").
#[component]
pub fn ItemContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="item-content"
            class=move || cn(ITEM_CONTENT_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// `ItemTitle` component (data-slot="item-title").
#[component]
pub fn ItemTitle(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="item-title"
            class=move || cn(ITEM_TITLE_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// `ItemDescription` component (data-slot="item-description").
#[component]
pub fn ItemDescription(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <p
            data-slot="item-description"
            class=move || cn(ITEM_DESCRIPTION_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </p>
    }
}

/// `ItemActions` component (data-slot="item-actions").
#[component]
pub fn ItemActions(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="item-actions"
            class=move || cn(ITEM_ACTIONS_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// `ItemHeader` component (data-slot="item-header").
#[component]
pub fn ItemHeader(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="item-header"
            class=move || cn(ITEM_HEADER_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// `ItemFooter` component (data-slot="item-footer").
#[component]
pub fn ItemFooter(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="item-footer"
            class=move || cn(ITEM_FOOTER_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}
