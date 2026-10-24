use crate::default::*;
use leptos::prelude::*;

#[test]
fn orientation_classes() {
    assert_eq!(
        ButtonGroupOrientation::default(),
        ButtonGroupOrientation::Horizontal
    );
    assert!(
        ButtonGroupOrientation::Horizontal
            .class()
            .contains("rounded-l-none")
    );
    assert!(
        ButtonGroupOrientation::Vertical
            .class()
            .contains("flex-col")
    );
    assert_eq!(ButtonGroupOrientation::Vertical.as_str(), "vertical");
}

#[test]
fn group_class_merges() {
    let c = button_group_class(ButtonGroupOrientation::Vertical, Some("my-x".into()));
    assert!(c.starts_with(BUTTON_GROUP_CLASS));
    assert!(c.contains("flex-col"));
    assert!(c.ends_with("my-x"));
}

#[test]
fn tokens() {
    assert!(BUTTON_GROUP_TEXT_CLASS.contains("bg-muted"));
    assert!(BUTTON_GROUP_SEPARATOR_CLASS.contains("bg-input"));
}

#[test]
fn renders_html() {
    let html = view! { <ButtonGroup orientation=ButtonGroupOrientation::Vertical><ButtonGroupText>"A"</ButtonGroupText><ButtonGroupSeparator/></ButtonGroup> }.to_html();
    assert!(html.contains(r#"data-slot="button-group""#));
    assert!(html.contains(r#"role="group""#));
    assert!(html.contains(r#"data-orientation="vertical""#));
    assert!(html.contains(r#"data-slot="button-group-text""#));
    assert!(html.contains(r#"data-slot="button-group-separator""#));
}
