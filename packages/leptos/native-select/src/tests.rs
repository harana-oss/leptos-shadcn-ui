use crate::default::*;

#[test]
fn size_normalization() {
    assert_eq!(native_select_size("sm"), "sm");
    assert_eq!(native_select_size("default"), "default");
    assert_eq!(native_select_size(""), "default");
    assert_eq!(native_select_size("xl"), "default");
}

#[test]
fn class_mapping() {
    assert_eq!(native_select_class("", ""), NATIVE_SELECT_CLASS);
    let sm = native_select_class("sm", "w-40");
    assert!(sm.starts_with(NATIVE_SELECT_CLASS));
    assert!(sm.contains(NATIVE_SELECT_SIZE_SM_CLASS));
    assert!(sm.ends_with("w-40"));
}

#[test]
fn class_tokens() {
    assert!(NATIVE_SELECT_CLASS.contains("appearance-none"));
    assert!(NATIVE_SELECT_CLASS.contains("pr-9"));
    assert!(NATIVE_SELECT_CLASS.contains("aria-invalid:border-destructive"));
    assert!(NATIVE_SELECT_ICON_CLASS.contains("pointer-events-none"));
    assert!(NATIVE_SELECT_WRAPPER_CLASS.contains("has-[select:disabled]:opacity-50"));
}
