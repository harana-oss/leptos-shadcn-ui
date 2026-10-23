use crate::default::*;
use leptos::prelude::*;

#[test]
fn classes() {
    assert!(KBD_CLASS.contains("bg-muted"));
    assert!(KBD_CLASS.contains("text-muted-foreground"));
    assert_eq!(cn(KBD_GROUP_CLASS, None), KBD_GROUP_CLASS);
    assert_eq!(cn("a", Some("  ".into())), "a");
}

#[test]
fn renders_html() {
    let html = view! { <KbdGroup><Kbd class="x">"Ctrl"</Kbd></KbdGroup> }.to_html();
    assert!(html.contains("<kbd"));
    assert!(html.contains(r#"data-slot="kbd-group""#));
    assert!(html.contains(r#"data-slot="kbd""#));
    assert!(html.contains("Ctrl"));
}
