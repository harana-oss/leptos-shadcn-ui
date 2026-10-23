use crate::default::*;
use ToggleGroupType::{Multiple, Single};

fn s(v: &[&str]) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

#[test]
fn single_select_from_empty() {
    assert_eq!(toggle_value(Single, &[], "a"), s(&["a"]));
}

#[test]
fn single_replaces_selection() {
    assert_eq!(toggle_value(Single, &s(&["a"]), "b"), s(&["b"]));
}

#[test]
fn single_toggle_off_clears() {
    assert!(toggle_value(Single, &s(&["a"]), "a").is_empty());
}

#[test]
fn single_sequence() {
    let mut v = Vec::new();
    for (item, expect) in [("a", vec!["a"]), ("b", vec!["b"]), ("b", vec![]), ("c", vec!["c"])] {
        v = toggle_value(Single, &v, item);
        assert_eq!(v, s(&expect));
    }
}

#[test]
fn multiple_add_and_remove() {
    let v = toggle_value(Multiple, &[], "a");
    assert_eq!(v, s(&["a"]));
    let v = toggle_value(Multiple, &v, "b");
    assert_eq!(v, s(&["a", "b"]));
    let v = toggle_value(Multiple, &v, "c");
    assert_eq!(v, s(&["a", "b", "c"]));
    let v = toggle_value(Multiple, &v, "b");
    assert_eq!(v, s(&["a", "c"]));
    let v = toggle_value(Multiple, &v, "a");
    let v = toggle_value(Multiple, &v, "c");
    assert!(v.is_empty());
}

#[test]
fn multiple_removes_duplicates_of_item() {
    assert_eq!(toggle_value(Multiple, &s(&["a", "b", "a"]), "a"), s(&["b"]));
}

#[test]
fn toggle_does_not_mutate_input() {
    let cur = s(&["a"]);
    let _ = toggle_value(Multiple, &cur, "b");
    assert_eq!(cur, s(&["a"]));
}

#[test]
fn normalize() {
    assert_eq!(normalize_value(Single, &s(&["a", "b"])), s(&["a"]));
    assert!(normalize_value(Single, &[]).is_empty());
    assert_eq!(normalize_value(Multiple, &s(&["a", "b", "a"])), s(&["a", "b"]));
}

#[test]
fn pressed_and_state() {
    let v = s(&["x", "y"]);
    assert!(is_pressed(&v, "x"));
    assert!(!is_pressed(&v, "z"));
    assert!(!is_pressed(&[], "x"));
    assert_eq!(data_state(true), "on");
    assert_eq!(data_state(false), "off");
}

#[test]
fn enum_parsing() {
    assert_eq!(ToggleGroupType::parse("multiple"), Multiple);
    assert_eq!(ToggleGroupType::parse("single"), Single);
    assert_eq!(ToggleGroupType::parse(""), Single);
    assert_eq!(ToggleGroupVariant::parse("outline"), ToggleGroupVariant::Outline);
    assert_eq!(ToggleGroupVariant::parse("x"), ToggleGroupVariant::Default);
    assert_eq!(ToggleGroupSize::parse("sm"), ToggleGroupSize::Sm);
    assert_eq!(ToggleGroupSize::parse("lg"), ToggleGroupSize::Lg);
    assert_eq!(ToggleGroupSize::parse(""), ToggleGroupSize::Default);
    for t in [Single, Multiple] {
        assert_eq!(ToggleGroupType::parse(t.as_str()), t);
    }
    for sz in [ToggleGroupSize::Default, ToggleGroupSize::Sm, ToggleGroupSize::Lg] {
        assert_eq!(ToggleGroupSize::parse(sz.as_str()), sz);
    }
}

#[test]
fn variant_size_class_mapping() {
    assert_eq!(ToggleGroupVariant::Default.class(), TOGGLE_VARIANT_DEFAULT_CLASS);
    assert_eq!(ToggleGroupVariant::Outline.class(), TOGGLE_VARIANT_OUTLINE_CLASS);
    assert_eq!(ToggleGroupSize::Default.class(), "h-9 px-2 min-w-9");
    assert_eq!(ToggleGroupSize::Sm.class(), "h-8 px-1.5 min-w-8");
    assert_eq!(ToggleGroupSize::Lg.class(), "h-10 px-2.5 min-w-10");
}

#[test]
fn item_class_composition() {
    let c = toggle_group_item_class(ToggleGroupVariant::Outline, ToggleGroupSize::Lg, "extra");
    assert!(c.starts_with(TOGGLE_BASE_CLASS));
    assert!(c.contains(TOGGLE_VARIANT_OUTLINE_CLASS));
    assert!(c.contains(TOGGLE_SIZE_LG_CLASS));
    assert!(c.contains(TOGGLE_GROUP_ITEM_CLASS));
    assert!(c.ends_with("extra"));
    assert_eq!(toggle_group_class(""), TOGGLE_GROUP_CLASS);
    assert!(TOGGLE_BASE_CLASS.contains("data-[state=on]:bg-accent"));
}

#[test]
fn context_toggle_updates_signal() {
    use leptos::prelude::*;
    let owner = Owner::new();
    owner.with(|| {
        let ctx = ToggleGroupContext {
            kind: Signal::stored(Multiple),
            variant: Signal::stored(ToggleGroupVariant::Default),
            size: Signal::stored(ToggleGroupSize::Default),
            disabled: Signal::stored(false),
            value: RwSignal::new(Vec::new()),
            on_value_change: None,
        };
        ctx.toggle("a");
        ctx.toggle("b");
        assert_eq!(ctx.value.get_untracked(), s(&["a", "b"]));
        ctx.toggle("a");
        assert_eq!(ctx.value.get_untracked(), s(&["b"]));
    });
}
