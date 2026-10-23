use crate::default::*;

#[test]
fn align_parse_and_roundtrip() {
    assert_eq!(InputGroupAddonAlign::parse(""), InputGroupAddonAlign::InlineStart);
    assert_eq!(InputGroupAddonAlign::parse("nope"), InputGroupAddonAlign::InlineStart);
    for a in [
        InputGroupAddonAlign::InlineStart,
        InputGroupAddonAlign::InlineEnd,
        InputGroupAddonAlign::BlockStart,
        InputGroupAddonAlign::BlockEnd,
    ] {
        assert_eq!(InputGroupAddonAlign::parse(a.as_str()), a);
    }
}

#[test]
fn addon_class_mapping() {
    assert!(addon_class(InputGroupAddonAlign::InlineStart, "").contains("order-first pl-3"));
    assert!(addon_class(InputGroupAddonAlign::InlineEnd, "").contains("order-last pr-3"));
    assert!(addon_class(InputGroupAddonAlign::BlockStart, "").contains("order-first w-full"));
    assert!(addon_class(InputGroupAddonAlign::BlockEnd, "").contains("order-last w-full"));
    let c = addon_class(InputGroupAddonAlign::InlineEnd, "extra");
    assert!(c.starts_with(INPUT_GROUP_ADDON_CLASS));
    assert!(c.ends_with("extra"));
}

#[test]
fn button_size_mapping() {
    assert_eq!(button_size_class(""), INPUT_GROUP_BUTTON_SIZE_XS_CLASS);
    assert_eq!(button_size_class("xs"), INPUT_GROUP_BUTTON_SIZE_XS_CLASS);
    assert_eq!(button_size_class("sm"), INPUT_GROUP_BUTTON_SIZE_SM_CLASS);
    assert_eq!(button_size_class("icon-xs"), INPUT_GROUP_BUTTON_SIZE_ICON_XS_CLASS);
    assert_eq!(button_size_class("icon-sm"), INPUT_GROUP_BUTTON_SIZE_ICON_SM_CLASS);
}

#[test]
fn button_variant_mapping() {
    assert_eq!(button_variant_class(""), BUTTON_VARIANT_GHOST_CLASS);
    assert_eq!(button_variant_class("ghost"), BUTTON_VARIANT_GHOST_CLASS);
    assert_eq!(button_variant_class("default"), BUTTON_VARIANT_DEFAULT_CLASS);
    assert_eq!(button_variant_class("outline"), BUTTON_VARIANT_OUTLINE_CLASS);
    assert_eq!(button_variant_class("secondary"), BUTTON_VARIANT_SECONDARY_CLASS);
    assert_eq!(button_variant_class("destructive"), BUTTON_VARIANT_DESTRUCTIVE_CLASS);
    assert_eq!(button_variant_class("link"), BUTTON_VARIANT_LINK_CLASS);
}

#[test]
fn button_class_composition() {
    let c = button_class("outline", "sm", "x");
    assert!(c.starts_with(BUTTON_BASE_CLASS));
    assert!(c.contains(BUTTON_VARIANT_OUTLINE_CLASS));
    assert!(c.contains(INPUT_GROUP_BUTTON_SIZE_SM_CLASS));
    assert!(c.ends_with(" x"));
}

#[test]
fn control_classes_use_theme_tokens() {
    assert!(INPUT_GROUP_CLASS.contains("border-input"));
    assert!(INPUT_GROUP_CLASS.contains("has-[[data-slot][aria-invalid=true]]:border-destructive"));
    assert!(INPUT_GROUP_INPUT_CLASS.contains("border-0"));
    assert!(INPUT_GROUP_TEXTAREA_CLASS.contains("resize-none"));
    assert_eq!(cn(&["a", "", "b"]), "a b");
}
