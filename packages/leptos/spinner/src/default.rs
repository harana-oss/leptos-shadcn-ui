//! Default style for spinner.

use leptos::prelude::*;

/// Merge a base class string with an optional user class.
pub(crate) fn cn(base: &str, extra: Option<String>) -> String {
    match extra {
        Some(e) if !e.trim().is_empty() => format!("{base} {}", e.trim()),
        _ => base.to_string(),
    }
}

pub const SPINNER_CLASS: &str = "size-4 animate-spin";

/// Loading spinner (lucide Loader2Icon), role="status".
#[component]
pub fn Spinner(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            role="status"
            aria-label="Loading"
            data-slot="spinner"
            class=move || cn(SPINNER_CLASS, class.get())
            id=move || id.get()
            style=move || style.get()
        >
            <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
        </svg>
    }
}
