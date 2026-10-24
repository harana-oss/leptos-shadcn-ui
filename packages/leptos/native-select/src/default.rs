//! Default style native-select components.

use leptos::prelude::*;

pub const NATIVE_SELECT_WRAPPER_CLASS: &str =
    "group/native-select relative w-fit has-[select:disabled]:opacity-50";
pub const NATIVE_SELECT_CLASS: &str = "border-input placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 dark:hover:bg-input/50 h-9 w-full min-w-0 appearance-none rounded-md border bg-transparent px-3 py-2 pr-9 text-sm shadow-xs transition-[color,box-shadow] outline-none disabled:pointer-events-none disabled:cursor-not-allowed focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive";
pub const NATIVE_SELECT_SIZE_SM_CLASS: &str = "h-8 py-1";
pub const NATIVE_SELECT_ICON_CLASS: &str = "text-muted-foreground pointer-events-none absolute top-1/2 right-3.5 size-4 -translate-y-1/2 opacity-50 select-none";

/// Joins class fragments, skipping empty ones.
pub(crate) fn cn(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Normalized size: `sm` or `default`.
pub fn native_select_size(size: &str) -> &'static str {
    match size {
        "sm" => "sm",
        _ => "default",
    }
}

pub fn native_select_class(size: &str, extra: &str) -> String {
    let size_class = match native_select_size(size) {
        "sm" => NATIVE_SELECT_SIZE_SM_CLASS,
        _ => "",
    };
    cn(&[NATIVE_SELECT_CLASS, size_class, extra])
}

#[component]
pub fn NativeSelect(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    /// `default` or `sm`.
    #[prop(into, optional)]
    size: MaybeProp<String>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] required: Signal<bool>,
    #[prop(into, optional)] invalid: Signal<bool>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let size_attr = move || native_select_size(&size.get().unwrap_or_default());
    let class = move || native_select_class(size_attr(), &class.get().unwrap_or_default());
    view! {
        <div data-slot="native-select-wrapper" class=NATIVE_SELECT_WRAPPER_CLASS>
            <select
                data-slot="native-select"
                data-size=size_attr
                prop:value=move || value.get().unwrap_or_default()
                name=move || name.get()
                disabled=disabled
                required=required
                aria-invalid=move || invalid.get().then_some("true")
                aria-label=move || aria_label.get()
                class=class
                id=move || id.get()
                style=move || style.get()
                on:change=move |ev| {
                    if let Some(cb) = on_change {
                        cb.run(event_target_value(&ev));
                    }
                }
            >
                {children.map(|c| c())}
            </select>
            <svg
                data-slot="native-select-icon"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                class=NATIVE_SELECT_ICON_CLASS
            >
                <path d="m6 9 6 6 6-6"></path>
            </svg>
        </div>
    }
}

#[component]
pub fn NativeSelectOption(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <option
            data-slot="native-select-option"
            value=value
            disabled=disabled
            class=move || class.get()
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </option>
    }
}

#[component]
pub fn NativeSelectOptGroup(
    #[prop(into)] label: String,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <optgroup
            data-slot="native-select-optgroup"
            label=label
            disabled=disabled
            class=move || class.get()
            id=move || id.get()
            style=move || style.get()
        >
            {children.map(|c| c())}
        </optgroup>
    }
}
