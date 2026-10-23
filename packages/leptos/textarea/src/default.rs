use leptos::{ev::Event, prelude::*};
use leptos_style::Style;

const TEXTAREA_CLASS: &str = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

#[component]
pub fn Textarea(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] on_change: Option<Callback<String>>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] input_type: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] style: Signal<Style>,
) -> impl IntoView {
    // <textarea> has no `type` attribute; prop kept for API compatibility with Input.
    let _ = input_type;
    let handle_input = move |event: Event| {
        if let Some(callback) = &on_change {
            callback.run(event_target_value(&event));
        }
    };

    let computed_class = Signal::derive(move || {
        format!("{} {}", TEXTAREA_CLASS, class.get().unwrap_or_default())
    });

    view! {
        <textarea
            placeholder=move || placeholder.get().unwrap_or_default()
            disabled=move || disabled.get()
            class=move || computed_class.get()
            id=move || id.get().unwrap_or_default()
            style=move || style.get().to_string()
            prop:value=move || value.get().unwrap_or_default()
            on:input=handle_input
        ></textarea>
    }
}
