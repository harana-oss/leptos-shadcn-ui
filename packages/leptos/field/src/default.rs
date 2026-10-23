//! Default style field components.

use leptos::prelude::*;

pub const FIELD_SET_CLASS: &str = "flex flex-col gap-6 has-[>[data-slot=checkbox-group]]:gap-3 has-[>[data-slot=radio-group]]:gap-3";
pub const FIELD_LEGEND_CLASS: &str = "mb-3 font-medium data-[variant=legend]:text-base data-[variant=label]:text-sm";
pub const FIELD_GROUP_CLASS: &str = "group/field-group @container/field-group flex w-full flex-col gap-7 data-[slot=checkbox-group]:gap-3 [&>[data-slot=field-group]]:gap-4";
pub const FIELD_CLASS: &str = "group/field flex w-full gap-3 data-[invalid=true]:text-destructive";
pub const FIELD_VERTICAL_CLASS: &str = "flex-col [&>*]:w-full [&>.sr-only]:w-auto";
pub const FIELD_HORIZONTAL_CLASS: &str = "flex-row items-center [&>[data-slot=field-label]]:flex-auto has-[>[data-slot=field-content]]:items-start has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px";
pub const FIELD_RESPONSIVE_CLASS: &str = "flex-col [&>*]:w-full [&>.sr-only]:w-auto @md/field-group:flex-row @md/field-group:items-center @md/field-group:[&>*]:w-auto @md/field-group:[&>[data-slot=field-label]]:flex-auto @md/field-group:has-[>[data-slot=field-content]]:items-start @md/field-group:has-[>[data-slot=field-content]]:[&>[role=checkbox],[role=radio]]:mt-px";
pub const FIELD_CONTENT_CLASS: &str = "group/field-content flex flex-1 flex-col gap-1.5 leading-snug";
pub const FIELD_LABEL_CLASS: &str = "flex items-center gap-2 text-sm font-medium select-none group-data-[disabled=true]:pointer-events-none peer-disabled:cursor-not-allowed peer-disabled:opacity-50 group/field-label peer/field-label w-fit leading-snug group-data-[disabled=true]/field:opacity-50 has-[>[data-slot=field]]:w-full has-[>[data-slot=field]]:flex-col has-[>[data-slot=field]]:rounded-md has-[>[data-slot=field]]:border [&>*]:data-[slot=field]:p-4 has-data-[state=checked]:bg-primary/5 has-data-[state=checked]:border-primary dark:has-data-[state=checked]:bg-primary/10";
pub const FIELD_TITLE_CLASS: &str = "flex w-fit items-center gap-2 text-sm leading-snug font-medium group-data-[disabled=true]/field:opacity-50";
pub const FIELD_DESCRIPTION_CLASS: &str = "text-muted-foreground text-sm leading-normal font-normal group-has-[[data-orientation=horizontal]]/field:text-balance last:mt-0 nth-last-2:-mt-1 [[data-variant=legend]+&]:-mt-1.5 [&>a:hover]:text-primary [&>a]:underline [&>a]:underline-offset-4";
pub const FIELD_SEPARATOR_CLASS: &str = "relative -my-2 h-5 text-sm group-data-[variant=outline]/field-group:-mb-2";
pub const FIELD_SEPARATOR_LINE_CLASS: &str = "bg-border absolute inset-0 top-1/2 h-px w-full shrink-0";
pub const FIELD_SEPARATOR_CONTENT_CLASS: &str = "bg-background text-muted-foreground relative mx-auto block w-fit px-2";
pub const FIELD_ERROR_CLASS: &str = "text-destructive text-sm font-normal";
pub const FIELD_ERROR_LIST_CLASS: &str = "ml-4 flex list-disc flex-col gap-1";

/// Joins class fragments, skipping empty ones.
pub fn cn(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Layout orientation of a [`Field`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FieldOrientation {
    #[default]
    Vertical,
    Horizontal,
    Responsive,
}

impl FieldOrientation {
    /// Parses an orientation, falling back to `Vertical` for unknown values.
    pub fn parse(value: &str) -> Self {
        match value {
            "horizontal" => Self::Horizontal,
            "responsive" => Self::Responsive,
            _ => Self::Vertical,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Responsive => "responsive",
        }
    }

    pub fn class(&self) -> &'static str {
        match self {
            Self::Vertical => FIELD_VERTICAL_CLASS,
            Self::Horizontal => FIELD_HORIZONTAL_CLASS,
            Self::Responsive => FIELD_RESPONSIVE_CLASS,
        }
    }
}

/// Full class string for a [`Field`].
pub fn field_class(orientation: FieldOrientation, extra: &str) -> String {
    cn(&[FIELD_CLASS, orientation.class(), extra])
}

/// Legend variant: `legend` (default) or `label`.
pub fn legend_variant(value: &str) -> &'static str {
    match value {
        "label" => "label",
        _ => "legend",
    }
}

/// What a [`FieldError`] should render, computed from its error list.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldErrorContent {
    None,
    Single(String),
    List(Vec<String>),
}

/// Deduplicates (preserving order) and drops blank messages, mirroring shadcn's FieldError.
pub fn field_error_content(errors: &[String]) -> FieldErrorContent {
    let mut unique: Vec<String> = Vec::new();
    for e in errors {
        let e = e.trim();
        if !e.is_empty() && !unique.iter().any(|u| u == e) {
            unique.push(e.to_string());
        }
    }
    match unique.len() {
        0 => FieldErrorContent::None,
        1 => FieldErrorContent::Single(unique.remove(0)),
        _ => FieldErrorContent::List(unique),
    }
}

#[component]
pub fn FieldSet(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = move || cn(&[FIELD_SET_CLASS, &class.get().unwrap_or_default()]);
    view! {
        <fieldset data-slot="field-set" class=class id=move || id.get() style=move || style.get() disabled=disabled>
            {children.map(|c| c())}
        </fieldset>
    }
}

#[component]
pub fn FieldLegend(
    /// `legend` (default) or `label`.
    #[prop(into, optional)] variant: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = move || cn(&[FIELD_LEGEND_CLASS, &class.get().unwrap_or_default()]);
    let variant = move || legend_variant(&variant.get().unwrap_or_default());
    view! {
        <legend data-slot="field-legend" data-variant=variant class=class id=move || id.get() style=move || style.get()>
            {children.map(|c| c())}
        </legend>
    }
}

#[component]
pub fn FieldGroup(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = move || cn(&[FIELD_GROUP_CLASS, &class.get().unwrap_or_default()]);
    view! {
        <div data-slot="field-group" class=class id=move || id.get() style=move || style.get()>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn Field(
    /// `vertical` (default), `horizontal` or `responsive`.
    #[prop(into, optional)] orientation: MaybeProp<String>,
    #[prop(into, optional)] invalid: Signal<bool>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let orient = move || FieldOrientation::parse(&orientation.get().unwrap_or_default());
    let class = move || field_class(orient(), &class.get().unwrap_or_default());
    view! {
        <div
            role="group"
            data-slot="field"
            data-orientation=move || orient().as_str()
            data-invalid=move || invalid.get().then_some("true")
            data-disabled=move || disabled.get().then_some("true")
            class=class
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn FieldContent(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = move || cn(&[FIELD_CONTENT_CLASS, &class.get().unwrap_or_default()]);
    view! {
        <div data-slot="field-content" class=class id=move || id.get() style=move || style.get()>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn FieldLabel(
    /// Id of the control this label is for.
    #[prop(into, optional)] html_for: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = move || cn(&[FIELD_LABEL_CLASS, &class.get().unwrap_or_default()]);
    view! {
        <label data-slot="field-label" for=move || html_for.get() class=class id=move || id.get() style=move || style.get()>
            {children.map(|c| c())}
        </label>
    }
}

#[component]
pub fn FieldTitle(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = move || cn(&[FIELD_TITLE_CLASS, &class.get().unwrap_or_default()]);
    view! {
        <div data-slot="field-label" class=class id=move || id.get() style=move || style.get()>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn FieldDescription(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = move || cn(&[FIELD_DESCRIPTION_CLASS, &class.get().unwrap_or_default()]);
    view! {
        <p data-slot="field-description" class=class id=move || id.get() style=move || style.get()>
            {children.map(|c| c())}
        </p>
    }
}

#[component]
pub fn FieldSeparator(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = move || cn(&[FIELD_SEPARATOR_CLASS, &class.get().unwrap_or_default()]);
    let has_content = if children.is_some() { "true" } else { "false" };
    view! {
        <div
            data-slot="field-separator"
            data-content=has_content
            class=class
            id=move || id.get()
            style=move || style.get()
        >
            <div data-slot="separator" role="none" data-orientation="horizontal" class=FIELD_SEPARATOR_LINE_CLASS></div>
            {children.map(|c| view! {
                <span data-slot="field-separator-content" class=FIELD_SEPARATOR_CONTENT_CLASS>{c()}</span>
            })}
        </div>
    }
}

#[component]
pub fn FieldError(
    /// Error messages; duplicates and blanks are dropped. Ignored when children are given.
    #[prop(into, optional)] errors: Signal<Vec<String>>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = move || cn(&[FIELD_ERROR_CLASS, &class.get().unwrap_or_default()]);
    if let Some(children) = children {
        return view! {
            <div role="alert" data-slot="field-error" class=class id=move || id.get() style=move || style.get()>
                {children()}
            </div>
        }
        .into_any();
    }
    (move || match field_error_content(&errors.get()) {
        FieldErrorContent::None => ().into_any(),
        FieldErrorContent::Single(msg) => view! {
            <div role="alert" data-slot="field-error" class=class id=id.get() style=style.get()>{msg}</div>
        }
        .into_any(),
        FieldErrorContent::List(list) => view! {
            <div role="alert" data-slot="field-error" class=class id=id.get() style=style.get()>
                <ul class=FIELD_ERROR_LIST_CLASS>
                    {list.into_iter().map(|m| view! { <li>{m}</li> }).collect_view()}
                </ul>
            </div>
        }
        .into_any(),
    })
    .into_any()
}
