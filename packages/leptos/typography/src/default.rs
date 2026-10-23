//! Default style for typography.

use leptos::prelude::*;

/// Merge a base class string with an optional user class.
pub fn cn(base: &str, extra: Option<String>) -> String {
    match extra {
        Some(e) if !e.trim().is_empty() => format!("{base} {}", e.trim()),
        _ => base.to_string(),
    }
}
pub const TYPOGRAPHY_H1_CLASS: &str = "scroll-m-20 text-center text-4xl font-extrabold tracking-tight text-balance";
pub const TYPOGRAPHY_H2_CLASS: &str = "scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0";
pub const TYPOGRAPHY_H3_CLASS: &str = "scroll-m-20 text-2xl font-semibold tracking-tight";
pub const TYPOGRAPHY_H4_CLASS: &str = "scroll-m-20 text-xl font-semibold tracking-tight";
pub const TYPOGRAPHY_P_CLASS: &str = "leading-7 [&:not(:first-child)]:mt-6";
pub const TYPOGRAPHY_BLOCKQUOTE_CLASS: &str = "mt-6 border-l-2 pl-6 italic";
pub const TYPOGRAPHY_LIST_CLASS: &str = "my-6 ml-6 list-disc [&>li]:mt-2";
pub const TYPOGRAPHY_INLINE_CODE_CLASS: &str = "bg-muted relative rounded px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold";
pub const TYPOGRAPHY_LEAD_CLASS: &str = "text-muted-foreground text-xl";
pub const TYPOGRAPHY_LARGE_CLASS: &str = "text-lg font-semibold";
pub const TYPOGRAPHY_SMALL_CLASS: &str = "text-sm leading-none font-medium";
pub const TYPOGRAPHY_MUTED_CLASS: &str = "text-muted-foreground text-sm";

/// `TypographyH1` component (data-slot="typography-h1").
#[component]
pub fn TypographyH1(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <h1
            data-slot="typography-h1"
            class=move || cn(TYPOGRAPHY_H1_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </h1>
    }
}

/// `TypographyH2` component (data-slot="typography-h2").
#[component]
pub fn TypographyH2(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <h2
            data-slot="typography-h2"
            class=move || cn(TYPOGRAPHY_H2_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </h2>
    }
}

/// `TypographyH3` component (data-slot="typography-h3").
#[component]
pub fn TypographyH3(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <h3
            data-slot="typography-h3"
            class=move || cn(TYPOGRAPHY_H3_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </h3>
    }
}

/// `TypographyH4` component (data-slot="typography-h4").
#[component]
pub fn TypographyH4(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <h4
            data-slot="typography-h4"
            class=move || cn(TYPOGRAPHY_H4_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </h4>
    }
}

/// `TypographyP` component (data-slot="typography-p").
#[component]
pub fn TypographyP(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <p
            data-slot="typography-p"
            class=move || cn(TYPOGRAPHY_P_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </p>
    }
}

/// `TypographyBlockquote` component (data-slot="typography-blockquote").
#[component]
pub fn TypographyBlockquote(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <blockquote
            data-slot="typography-blockquote"
            class=move || cn(TYPOGRAPHY_BLOCKQUOTE_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </blockquote>
    }
}

/// `TypographyList` component (data-slot="typography-list").
#[component]
pub fn TypographyList(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <ul
            data-slot="typography-list"
            class=move || cn(TYPOGRAPHY_LIST_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </ul>
    }
}

/// `TypographyInlineCode` component (data-slot="typography-inline-code").
#[component]
pub fn TypographyInlineCode(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <code
            data-slot="typography-inline-code"
            class=move || cn(TYPOGRAPHY_INLINE_CODE_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </code>
    }
}

/// `TypographyLead` component (data-slot="typography-lead").
#[component]
pub fn TypographyLead(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <p
            data-slot="typography-lead"
            class=move || cn(TYPOGRAPHY_LEAD_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </p>
    }
}

/// `TypographyLarge` component (data-slot="typography-large").
#[component]
pub fn TypographyLarge(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="typography-large"
            class=move || cn(TYPOGRAPHY_LARGE_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// `TypographySmall` component (data-slot="typography-small").
#[component]
pub fn TypographySmall(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <small
            data-slot="typography-small"
            class=move || cn(TYPOGRAPHY_SMALL_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </small>
    }
}

/// `TypographyMuted` component (data-slot="typography-muted").
#[component]
pub fn TypographyMuted(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <p
            data-slot="typography-muted"
            class=move || cn(TYPOGRAPHY_MUTED_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </p>
    }
}
