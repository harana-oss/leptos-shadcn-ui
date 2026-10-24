use crate::default::*;

fn s(v: &[&str]) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

#[test]
fn orientation_parse() {
    assert_eq!(
        FieldOrientation::parse("horizontal"),
        FieldOrientation::Horizontal
    );
    assert_eq!(
        FieldOrientation::parse("responsive"),
        FieldOrientation::Responsive
    );
    assert_eq!(
        FieldOrientation::parse("vertical"),
        FieldOrientation::Vertical
    );
    assert_eq!(FieldOrientation::parse(""), FieldOrientation::Vertical);
    assert_eq!(FieldOrientation::parse("bogus"), FieldOrientation::Vertical);
    assert_eq!(FieldOrientation::default(), FieldOrientation::Vertical);
}

#[test]
fn orientation_roundtrip() {
    for o in [
        FieldOrientation::Vertical,
        FieldOrientation::Horizontal,
        FieldOrientation::Responsive,
    ] {
        assert_eq!(FieldOrientation::parse(o.as_str()), o);
    }
}

#[test]
fn field_class_mapping() {
    let v = field_class(FieldOrientation::Vertical, "");
    assert!(v.starts_with(FIELD_CLASS));
    assert!(v.contains("flex-col"));
    let h = field_class(FieldOrientation::Horizontal, "my-x");
    assert!(h.contains("flex-row items-center"));
    assert!(h.ends_with("my-x"));
    let r = field_class(FieldOrientation::Responsive, "");
    assert!(r.contains("@md/field-group:flex-row"));
    assert!(FIELD_CLASS.contains("data-[invalid=true]:text-destructive"));
}

#[test]
fn cn_skips_empty() {
    assert_eq!(cn(&["a", "", "  ", "b "]), "a b");
    assert_eq!(cn(&[]), "");
}

#[test]
fn legend_variants() {
    assert_eq!(legend_variant("label"), "label");
    assert_eq!(legend_variant("legend"), "legend");
    assert_eq!(legend_variant(""), "legend");
}

#[test]
fn error_content_none() {
    assert_eq!(field_error_content(&[]), FieldErrorContent::None);
    assert_eq!(
        field_error_content(&s(&["", "  "])),
        FieldErrorContent::None
    );
}

#[test]
fn error_content_single_and_dedupe() {
    assert_eq!(
        field_error_content(&s(&["Required"])),
        FieldErrorContent::Single("Required".into())
    );
    assert_eq!(
        field_error_content(&s(&["Required", "Required", ""])),
        FieldErrorContent::Single("Required".into())
    );
}

#[test]
fn error_content_list_preserves_order() {
    assert_eq!(
        field_error_content(&s(&["b", "a", "b", "c"])),
        FieldErrorContent::List(s(&["b", "a", "c"]))
    );
}
