//! Default style input-group components.

use leptos::prelude::*;

pub const INPUT_GROUP_CLASS: &str = "group/input-group border-input dark:bg-input/30 relative flex w-full items-center rounded-md border shadow-xs transition-[color,box-shadow] outline-none h-9 min-w-0 has-[>textarea]:h-auto has-[>[data-align=inline-start]]:[&>input]:pl-2 has-[>[data-align=inline-end]]:[&>input]:pr-2 has-[>[data-align=block-start]]:h-auto has-[>[data-align=block-start]]:flex-col has-[>[data-align=block-start]]:[&>input]:pb-3 has-[>[data-align=block-end]]:h-auto has-[>[data-align=block-end]]:flex-col has-[>[data-align=block-end]]:[&>input]:pt-3 has-[[data-slot=input-group-control]:focus-visible]:border-ring has-[[data-slot=input-group-control]:focus-visible]:ring-ring/50 has-[[data-slot=input-group-control]:focus-visible]:ring-[3px] has-[[data-slot][aria-invalid=true]]:ring-destructive/20 has-[[data-slot][aria-invalid=true]]:border-destructive dark:has-[[data-slot][aria-invalid=true]]:ring-destructive/40";

pub const INPUT_GROUP_ADDON_CLASS: &str = "text-muted-foreground flex h-auto cursor-text items-center justify-center gap-2 py-1.5 text-sm font-medium select-none [&>svg:not([class*='size-'])]:size-4 [&>kbd]:rounded-[calc(var(--radius)-5px)] group-data-[disabled=true]/input-group:opacity-50";
pub const INPUT_GROUP_ADDON_INLINE_START_CLASS: &str =
    "order-first pl-3 has-[>button]:ml-[-0.45rem] has-[>kbd]:ml-[-0.35rem]";
pub const INPUT_GROUP_ADDON_INLINE_END_CLASS: &str =
    "order-last pr-3 has-[>button]:mr-[-0.45rem] has-[>kbd]:mr-[-0.35rem]";
pub const INPUT_GROUP_ADDON_BLOCK_START_CLASS: &str = "order-first w-full justify-start px-3 pt-3 [.border-b]:pb-3 group-has-[>input]/input-group:pt-2.5";
pub const INPUT_GROUP_ADDON_BLOCK_END_CLASS: &str = "order-last w-full justify-start px-3 pb-3 [.border-t]:pt-3 group-has-[>input]/input-group:pb-2.5";

pub const BUTTON_BASE_CLASS: &str = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive";
pub const INPUT_GROUP_BUTTON_CLASS: &str = "text-sm shadow-none flex gap-2 items-center";
pub const INPUT_GROUP_BUTTON_SIZE_XS_CLASS: &str = "h-6 gap-1 px-2 rounded-[calc(var(--radius)-5px)] [&>svg:not([class*='size-'])]:size-3.5 has-[>svg]:px-2";
pub const INPUT_GROUP_BUTTON_SIZE_SM_CLASS: &str =
    "h-8 px-2.5 gap-1.5 rounded-md has-[>svg]:px-2.5";
pub const INPUT_GROUP_BUTTON_SIZE_ICON_XS_CLASS: &str =
    "size-6 rounded-[calc(var(--radius)-5px)] p-0 has-[>svg]:p-0";
pub const INPUT_GROUP_BUTTON_SIZE_ICON_SM_CLASS: &str = "size-8 p-0 has-[>svg]:p-0";

pub const BUTTON_VARIANT_DEFAULT_CLASS: &str =
    "bg-primary text-primary-foreground hover:bg-primary/90";
pub const BUTTON_VARIANT_DESTRUCTIVE_CLASS: &str = "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60";
pub const BUTTON_VARIANT_OUTLINE_CLASS: &str = "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50";
pub const BUTTON_VARIANT_SECONDARY_CLASS: &str =
    "bg-secondary text-secondary-foreground hover:bg-secondary/80";
pub const BUTTON_VARIANT_GHOST_CLASS: &str =
    "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50";
pub const BUTTON_VARIANT_LINK_CLASS: &str = "text-primary underline-offset-4 hover:underline";

pub const INPUT_GROUP_TEXT_CLASS: &str = "text-muted-foreground flex items-center gap-2 text-sm [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4";

pub const INPUT_BASE_CLASS: &str = "file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive";
pub const INPUT_GROUP_INPUT_CLASS: &str = "flex-1 rounded-none border-0 bg-transparent shadow-none focus-visible:ring-0 dark:bg-transparent";
pub const TEXTAREA_BASE_CLASS: &str = "border-input placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 flex field-sizing-content min-h-16 w-full rounded-md border bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm";
pub const INPUT_GROUP_TEXTAREA_CLASS: &str = "flex-1 resize-none rounded-none border-0 bg-transparent py-3 shadow-none focus-visible:ring-0 dark:bg-transparent";

/// Joins class fragments, skipping empty ones.
pub(crate) fn cn(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Placement of an [`InputGroupAddon`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputGroupAddonAlign {
    #[default]
    InlineStart,
    InlineEnd,
    BlockStart,
    BlockEnd,
}

impl InputGroupAddonAlign {
    pub fn parse(value: &str) -> Self {
        match value {
            "inline-end" => Self::InlineEnd,
            "block-start" => Self::BlockStart,
            "block-end" => Self::BlockEnd,
            _ => Self::InlineStart,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InlineStart => "inline-start",
            Self::InlineEnd => "inline-end",
            Self::BlockStart => "block-start",
            Self::BlockEnd => "block-end",
        }
    }

    pub fn class(&self) -> &'static str {
        match self {
            Self::InlineStart => INPUT_GROUP_ADDON_INLINE_START_CLASS,
            Self::InlineEnd => INPUT_GROUP_ADDON_INLINE_END_CLASS,
            Self::BlockStart => INPUT_GROUP_ADDON_BLOCK_START_CLASS,
            Self::BlockEnd => INPUT_GROUP_ADDON_BLOCK_END_CLASS,
        }
    }
}

pub fn addon_class(align: InputGroupAddonAlign, extra: &str) -> String {
    cn(&[INPUT_GROUP_ADDON_CLASS, align.class(), extra])
}

/// Size class for [`InputGroupButton`]: `xs` (default), `sm`, `icon-xs`, `icon-sm`.
pub fn button_size_class(size: &str) -> &'static str {
    match size {
        "sm" => INPUT_GROUP_BUTTON_SIZE_SM_CLASS,
        "icon-xs" => INPUT_GROUP_BUTTON_SIZE_ICON_XS_CLASS,
        "icon-sm" => INPUT_GROUP_BUTTON_SIZE_ICON_SM_CLASS,
        _ => INPUT_GROUP_BUTTON_SIZE_XS_CLASS,
    }
}

/// Variant class for [`InputGroupButton`]; defaults to `ghost`.
pub fn button_variant_class(variant: &str) -> &'static str {
    match variant {
        "default" => BUTTON_VARIANT_DEFAULT_CLASS,
        "destructive" => BUTTON_VARIANT_DESTRUCTIVE_CLASS,
        "outline" => BUTTON_VARIANT_OUTLINE_CLASS,
        "secondary" => BUTTON_VARIANT_SECONDARY_CLASS,
        "link" => BUTTON_VARIANT_LINK_CLASS,
        _ => BUTTON_VARIANT_GHOST_CLASS,
    }
}

pub fn button_class(variant: &str, size: &str, extra: &str) -> String {
    cn(&[
        BUTTON_BASE_CLASS,
        button_variant_class(variant),
        INPUT_GROUP_BUTTON_CLASS,
        button_size_class(size),
        extra,
    ])
}

#[component]
pub fn InputGroup(
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = move || cn(&[INPUT_GROUP_CLASS, &class.get().unwrap_or_default()]);
    view! {
        <div
            role="group"
            data-slot="input-group"
            data-disabled=move || disabled.get().then_some("true")
            class=class
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

/// Selector for the enclosing input group root.
pub const INPUT_GROUP_SELECTOR: &str = "[data-slot=input-group]";
/// Selector for the control focused when an addon is clicked.
pub const INPUT_GROUP_CONTROL_SELECTOR: &str = "input, textarea";
/// Clicks originating inside a matching element do not move focus.
pub const ADDON_FOCUS_IGNORE_SELECTOR: &str = "button";

/// Whether an addon click should focus the group's control (shadcn skips
/// clicks that land inside a button).
pub fn addon_click_should_focus(target_inside_button: bool) -> bool {
    !target_inside_button
}

/// Focus the input/textarea of the group containing the clicked addon.
#[cfg(target_arch = "wasm32")]
fn focus_group_control(ev: &leptos::ev::MouseEvent) {
    use wasm_bindgen::JsCast;

    let Some(target) = ev
        .target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
    else {
        return;
    };
    let inside_button = target
        .closest(ADDON_FOCUS_IGNORE_SELECTOR)
        .ok()
        .flatten()
        .is_some();
    if !addon_click_should_focus(inside_button) {
        return;
    }
    let control = target
        .closest(INPUT_GROUP_SELECTOR)
        .ok()
        .flatten()
        .and_then(|group| {
            group
                .query_selector(INPUT_GROUP_CONTROL_SELECTOR)
                .ok()
                .flatten()
        })
        .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok());
    if let Some(control) = control {
        let _ = control.focus();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_group_control(_ev: &leptos::ev::MouseEvent) {}

#[component]
pub fn InputGroupAddon(
    /// `inline-start` (default), `inline-end`, `block-start` or `block-end`.
    #[prop(into, optional)]
    align: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let align = move || InputGroupAddonAlign::parse(&align.get().unwrap_or_default());
    let class = move || addon_class(align(), &class.get().unwrap_or_default());
    view! {
        <div
            role="group"
            data-slot="input-group-addon"
            data-align=move || align().as_str()
            class=class
            id=move || id.get()
            style=move || style.get()
            on:click=move |ev| focus_group_control(&ev)
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn InputGroupButton(
    /// Button variant; defaults to `ghost`.
    #[prop(into, optional)]
    variant: MaybeProp<String>,
    /// `xs` (default), `sm`, `icon-xs` or `icon-sm`.
    #[prop(into, optional)]
    size: MaybeProp<String>,
    /// Button `type` attribute; defaults to `button`.
    #[prop(into, optional)]
    button_type: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let size_attr = move || size.get().unwrap_or_else(|| "xs".to_string());
    let class = move || {
        button_class(
            &variant.get().unwrap_or_default(),
            &size_attr(),
            &class.get().unwrap_or_default(),
        )
    };
    view! {
        <button
            type=move || button_type.get().unwrap_or_else(|| "button".to_string())
            data-slot="button"
            data-size=size_attr
            aria-label=move || aria_label.get()
            disabled=disabled
            class=class
            id=move || id.get()
            style=move || style.get()
            on:click=move |_| {
                if let Some(cb) = on_click {
                    cb.run(());
                }
            }
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn InputGroupText(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = move || cn(&[INPUT_GROUP_TEXT_CLASS, &class.get().unwrap_or_default()]);
    view! {
        <span class=class id=move || id.get() style=move || style.get()>
            {children.map(|c| c())}
        </span>
    }
}

#[component]
pub fn InputGroupInput(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    /// Input `type`; defaults to `text`.
    #[prop(into, optional)]
    input_type: MaybeProp<String>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] invalid: Signal<bool>,
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let class = move || {
        cn(&[
            INPUT_BASE_CLASS,
            INPUT_GROUP_INPUT_CLASS,
            &class.get().unwrap_or_default(),
        ])
    };
    view! {
        <input
            data-slot="input-group-control"
            type=move || input_type.get().unwrap_or_else(|| "text".to_string())
            prop:value=move || value.get().unwrap_or_default()
            placeholder=move || placeholder.get()
            name=move || name.get()
            disabled=disabled
            aria-invalid=move || invalid.get().then_some("true")
            class=class
            id=move || id.get()
            style=move || style.get()
            on:input=move |ev| {
                if let Some(cb) = on_change {
                    cb.run(event_target_value(&ev));
                }
            }
        />
    }
}

#[component]
pub fn InputGroupTextarea(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] rows: MaybeProp<u32>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] invalid: Signal<bool>,
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    let class = move || {
        cn(&[
            TEXTAREA_BASE_CLASS,
            INPUT_GROUP_TEXTAREA_CLASS,
            &class.get().unwrap_or_default(),
        ])
    };
    view! {
        <textarea
            data-slot="input-group-control"
            prop:value=move || value.get().unwrap_or_default()
            placeholder=move || placeholder.get()
            name=move || name.get()
            rows=move || rows.get()
            disabled=disabled
            aria-invalid=move || invalid.get().then_some("true")
            class=class
            id=move || id.get()
            style=move || style.get()
            on:input=move |ev| {
                if let Some(cb) = on_change {
                    cb.run(event_target_value(&ev));
                }
            }
        ></textarea>
    }
}
