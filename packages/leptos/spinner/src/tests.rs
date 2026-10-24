use crate::default::*;
use leptos::prelude::*;

#[test]
fn classes() {
    assert!(SPINNER_CLASS.contains("animate-spin"));
    assert_eq!(
        cn(SPINNER_CLASS, Some("size-8".into())),
        "size-4 animate-spin size-8"
    );
}

#[test]
fn renders_html() {
    let html = view! { <Spinner/> }.to_html();
    assert!(html.contains("<svg"));
    assert!(html.contains(r#"role="status""#));
    assert!(html.contains(r#"aria-label="Loading""#));
    assert!(html.contains("animate-spin"));
}
