//! Default style toggle-group components.

use leptos::prelude::*;

pub const TOGGLE_GROUP_CLASS: &str = "group/toggle-group flex w-fit items-center rounded-md data-[variant=outline]:shadow-xs";
pub const TOGGLE_BASE_CLASS: &str = "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium hover:bg-muted hover:text-muted-foreground disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 [&_svg]:shrink-0 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] outline-none transition-[color,box-shadow] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive whitespace-nowrap";
pub const TOGGLE_VARIANT_DEFAULT_CLASS: &str = "bg-transparent";
pub const TOGGLE_VARIANT_OUTLINE_CLASS: &str = "border border-input bg-transparent shadow-xs hover:bg-accent hover:text-accent-foreground";
pub const TOGGLE_SIZE_DEFAULT_CLASS: &str = "h-9 px-2 min-w-9";
pub const TOGGLE_SIZE_SM_CLASS: &str = "h-8 px-1.5 min-w-8";
pub const TOGGLE_SIZE_LG_CLASS: &str = "h-10 px-2.5 min-w-10";
pub const TOGGLE_GROUP_ITEM_CLASS: &str = "min-w-0 flex-1 shrink-0 rounded-none shadow-none first:rounded-l-md last:rounded-r-md focus:z-10 focus-visible:z-10 data-[variant=outline]:border-l-0 data-[variant=outline]:first:border-l";

/// Joins class fragments, skipping empty ones.
pub(crate) fn cn(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Selection mode of a [`ToggleGroup`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupType {
    #[default]
    Single,
    Multiple,
}

impl ToggleGroupType {
    pub fn parse(value: &str) -> Self {
        match value {
            "multiple" => Self::Multiple,
            _ => Self::Single,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Multiple => "multiple",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupVariant {
    #[default]
    Default,
    Outline,
}

impl ToggleGroupVariant {
    pub fn parse(value: &str) -> Self {
        match value {
            "outline" => Self::Outline,
            _ => Self::Default,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Outline => "outline",
        }
    }

    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => TOGGLE_VARIANT_DEFAULT_CLASS,
            Self::Outline => TOGGLE_VARIANT_OUTLINE_CLASS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleGroupSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl ToggleGroupSize {
    pub fn parse(value: &str) -> Self {
        match value {
            "sm" => Self::Sm,
            "lg" => Self::Lg,
            _ => Self::Default,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
            Self::Lg => "lg",
        }
    }

    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => TOGGLE_SIZE_DEFAULT_CLASS,
            Self::Sm => TOGGLE_SIZE_SM_CLASS,
            Self::Lg => TOGGLE_SIZE_LG_CLASS,
        }
    }
}

/// Returns the new selection after toggling `item`.
///
/// * Single: selecting an unselected item replaces the selection; toggling the
///   selected item clears it.
/// * Multiple: adds or removes `item`, preserving the order of the others.
pub fn toggle_value(kind: ToggleGroupType, current: &[String], item: &str) -> Vec<String> {
    let selected = current.iter().any(|v| v == item);
    match kind {
        ToggleGroupType::Single => {
            if selected {
                Vec::new()
            } else {
                vec![item.to_string()]
            }
        }
        ToggleGroupType::Multiple => {
            if selected {
                current.iter().filter(|v| *v != item).cloned().collect()
            } else {
                let mut next = current.to_vec();
                next.push(item.to_string());
                next
            }
        }
    }
}

/// Normalizes an incoming value for the mode: single keeps at most the first
/// entry; multiple drops duplicates.
pub fn normalize_value(kind: ToggleGroupType, value: &[String]) -> Vec<String> {
    match kind {
        ToggleGroupType::Single => value.iter().take(1).cloned().collect(),
        ToggleGroupType::Multiple => {
            let mut out: Vec<String> = Vec::new();
            for v in value {
                if !out.contains(v) {
                    out.push(v.clone());
                }
            }
            out
        }
    }
}

pub fn is_pressed(current: &[String], item: &str) -> bool {
    current.iter().any(|v| v == item)
}

pub fn data_state(pressed: bool) -> &'static str {
    if pressed {
        "on"
    } else {
        "off"
    }
}

pub fn toggle_group_class(extra: &str) -> String {
    cn(&[TOGGLE_GROUP_CLASS, extra])
}

pub fn toggle_group_item_class(variant: ToggleGroupVariant, size: ToggleGroupSize, extra: &str) -> String {
    cn(&[TOGGLE_BASE_CLASS, variant.class(), size.class(), TOGGLE_GROUP_ITEM_CLASS, extra])
}

/// Context shared from [`ToggleGroup`] to its items.
#[derive(Debug, Clone, Copy)]
pub struct ToggleGroupContext {
    pub kind: Signal<ToggleGroupType>,
    pub variant: Signal<ToggleGroupVariant>,
    pub size: Signal<ToggleGroupSize>,
    pub disabled: Signal<bool>,
    pub value: RwSignal<Vec<String>>,
    pub on_value_change: Option<Callback<Vec<String>>>,
}

impl ToggleGroupContext {
    pub fn toggle(&self, item: &str) {
        let next = toggle_value(self.kind.get_untracked(), &self.value.get_untracked(), item);
        self.value.set(next.clone());
        if let Some(cb) = self.on_value_change {
            cb.run(next);
        }
    }
}

#[component]
pub fn ToggleGroup(
    /// `single` (default) or `multiple`.
    #[prop(into, optional)] r#type: MaybeProp<String>,
    /// `default` or `outline`.
    #[prop(into, optional)] variant: MaybeProp<String>,
    /// `default`, `sm` or `lg`.
    #[prop(into, optional)] size: MaybeProp<String>,
    /// Selected values (controlled when provided). Single mode uses at most one entry.
    #[prop(optional)] value: Option<RwSignal<Vec<String>>>,
    /// Initial selection when `value` is not provided.
    #[prop(into, optional)] default_value: Vec<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let kind = Signal::derive(move || ToggleGroupType::parse(&r#type.get().unwrap_or_default()));
    let variant = Signal::derive(move || ToggleGroupVariant::parse(&variant.get().unwrap_or_default()));
    let size = Signal::derive(move || ToggleGroupSize::parse(&size.get().unwrap_or_default()));
    let value = value.unwrap_or_else(|| {
        RwSignal::new(normalize_value(kind.get_untracked(), &default_value))
    });
    provide_context(ToggleGroupContext {
        kind,
        variant,
        size,
        disabled,
        value,
        on_value_change,
    });
    let class = move || toggle_group_class(&class.get().unwrap_or_default());
    view! {
        <div
            role="group"
            data-slot="toggle-group"
            data-type=move || kind.get().as_str()
            data-variant=move || variant.get().as_str()
            data-size=move || size.get().as_str()
            aria-label=move || aria_label.get()
            class=class
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ToggleGroupItem(
    #[prop(into)] value: String,
    /// Overrides the group variant.
    #[prop(into, optional)] variant: MaybeProp<String>,
    /// Overrides the group size.
    #[prop(into, optional)] size: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let ctx = use_context::<ToggleGroupContext>().unwrap_or_else(|| ToggleGroupContext {
        kind: Signal::stored(ToggleGroupType::Single),
        variant: Signal::stored(ToggleGroupVariant::Default),
        size: Signal::stored(ToggleGroupSize::Default),
        disabled: Signal::stored(false),
        value: RwSignal::new(Vec::new()),
        on_value_change: None,
    });
    let item = StoredValue::new(value);
    let variant = move || {
        variant
            .get()
            .map(|v| ToggleGroupVariant::parse(&v))
            .unwrap_or_else(|| ctx.variant.get())
    };
    let size = move || size.get().map(|s| ToggleGroupSize::parse(&s)).unwrap_or_else(|| ctx.size.get());
    let pressed = move || item.with_value(|v| is_pressed(&ctx.value.get(), v));
    let is_disabled = move || disabled.get() || ctx.disabled.get();
    let class = move || toggle_group_item_class(variant(), size(), &class.get().unwrap_or_default());
    view! {
        <button
            type="button"
            data-slot="toggle-group-item"
            data-variant=move || variant().as_str()
            data-size=move || size().as_str()
            data-state=move || data_state(pressed())
            aria-pressed=move || if pressed() { "true" } else { "false" }
            aria-label=move || aria_label.get()
            data-disabled=move || is_disabled().then_some("")
            disabled=is_disabled
            value=move || item.get_value()
            class=class
            id=move || id.get()
            style=move || style.get()
            on:click=move |_| {
                if !is_disabled() {
                    item.with_value(|v| ctx.toggle(v));
                }
            }
        >
            {children.map(|c| c())}
        </button>
    }
}
