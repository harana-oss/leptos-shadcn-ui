//! Behavior tests for the Sheet compound component.
//!
//! These exercise the pure functions and `SheetContext` state logic directly,
//! so they run under plain `cargo test` without a browser.

#[cfg(test)]
#[allow(clippy::module_inception)] // test module layout mirrors the file name
mod tdd_tests {
    use crate::sheet_core::*;
    use leptos::prelude::*;
    use std::sync::{Arc, Mutex};

    fn with_owner<T>(f: impl FnOnce() -> T) -> T {
        Owner::new().with(f)
    }

    // ===== Side mapping =====

    #[test]
    fn side_defaults_to_right() {
        assert_eq!(SheetSide::default(), SheetSide::Right);
    }

    #[test]
    fn side_maps_to_shadcn_variant_classes() {
        let cases = [
            (SheetSide::Top, "top", "top-0", "border-b"),
            (SheetSide::Right, "right", "right-0", "border-l"),
            (SheetSide::Bottom, "bottom", "bottom-0", "border-t"),
            (SheetSide::Left, "left", "left-0", "border-r"),
        ];
        for (side, name, pos, border) in cases {
            assert_eq!(side.as_str(), name);
            let class = side.class();
            assert!(class.contains(pos), "{name}: {class}");
            assert!(class.contains(border), "{name}: {class}");
            assert!(
                class.contains(&format!("data-[state=open]:slide-in-from-{name}")),
                "{name}: {class}"
            );
            assert!(
                class.contains(&format!("data-[state=closed]:slide-out-to-{name}")),
                "{name}: {class}"
            );
        }
        assert!(SheetSide::Left.class().contains("sm:max-w-sm"));
        assert!(!SheetSide::Top.class().contains("w-3/4"));
    }

    #[test]
    fn content_class_combines_theme_side_and_user_class() {
        for styles in [&crate::default::STYLES, &crate::new_york::STYLES] {
            let c = content_class(styles, SheetSide::Left, Some("my-sheet".into()));
            assert!(c.starts_with(styles.content));
            assert!(c.contains(SheetSide::Left.class()));
            assert!(c.ends_with("my-sheet"));
            assert_eq!(
                content_class(styles, SheetSide::Top, None),
                format!("{} {}", styles.content, SheetSide::Top.class())
            );
        }
    }

    #[test]
    fn merge_class_ignores_blank_extra() {
        assert_eq!(merge_class("a", None), "a");
        assert_eq!(merge_class("a", Some("  ".into())), "a");
        assert_eq!(merge_class("a", Some(" b ".into())), "a b");
    }

    #[test]
    fn themes_differ_but_share_structure() {
        assert_ne!(crate::default::STYLES, crate::new_york::STYLES);
        assert!(crate::new_york::STYLES.content.contains("fixed z-50"));
    }

    // ===== Pure state helpers =====

    #[test]
    fn resolve_open_prefers_controlled_value() {
        assert!(resolve_open(Some(true), false));
        assert!(!resolve_open(Some(false), true));
        assert!(resolve_open(None, true));
        assert!(!resolve_open(None, false));
    }

    #[test]
    fn only_escape_is_a_close_key() {
        assert!(is_close_key("Escape"));
        assert!(is_close_key("Esc"));
        assert!(!is_close_key("Enter"));
        assert!(!is_close_key(" "));
    }

    #[test]
    fn data_state_values() {
        assert_eq!(data_state(true), "open");
        assert_eq!(data_state(false), "closed");
    }

    #[test]
    fn generated_ids_are_unique() {
        let a = next_sheet_id();
        let b = next_sheet_id();
        assert_ne!(a, b);
        assert!(a.starts_with("sheet-"));
    }

    // ===== SheetContext =====

    #[test]
    fn uncontrolled_open_and_close() {
        with_owner(|| {
            let ctx = SheetContext::new(None, false, None);
            assert!(!ctx.open.get_untracked());
            ctx.open();
            assert!(ctx.open.get_untracked());
            ctx.close();
            assert!(!ctx.open.get_untracked());
        });
    }

    #[test]
    fn default_open_seeds_uncontrolled_state() {
        with_owner(|| {
            let ctx = SheetContext::new(None, true, None);
            assert!(ctx.open.get_untracked());
        });
    }

    #[test]
    fn controlled_state_follows_signal_and_reports_changes() {
        with_owner(|| {
            let (open, set_open) = signal(false);
            let calls = Arc::new(Mutex::new(Vec::new()));
            let calls_cb = calls.clone();
            let ctx = SheetContext::new(
                Some(open.into()),
                true, // ignored when controlled
                Some(Callback::new(move |v| calls_cb.lock().unwrap().push(v))),
            );
            assert!(!ctx.open.get_untracked());

            // Requesting open notifies the parent but does not override it.
            ctx.open();
            assert_eq!(*calls.lock().unwrap(), vec![true]);
            assert!(!ctx.open.get_untracked());

            set_open.set(true);
            assert!(ctx.open.get_untracked());
        });
    }

    #[test]
    fn escape_closes_open_sheet() {
        with_owner(|| {
            let ctx = SheetContext::new(None, true, None);
            assert!(!ctx.handle_key("Enter"));
            assert!(ctx.open.get_untracked());
            assert!(ctx.handle_key("Escape"));
            assert!(!ctx.open.get_untracked());
            // Already closed: no-op.
            assert!(!ctx.handle_key("Escape"));
        });
    }

    #[test]
    fn overlay_click_closes_and_notifies() {
        with_owner(|| {
            let calls = Arc::new(Mutex::new(Vec::new()));
            let calls_cb = calls.clone();
            let ctx = SheetContext::new(
                None,
                true,
                Some(Callback::new(move |v| calls_cb.lock().unwrap().push(v))),
            );
            ctx.handle_overlay_click();
            assert!(!ctx.open.get_untracked());
            assert_eq!(*calls.lock().unwrap(), vec![false]);
        });
    }

    #[test]
    fn each_instance_gets_distinct_aria_ids() {
        with_owner(|| {
            let a = SheetContext::new(None, false, None);
            let b = SheetContext::new(None, false, None);
            let at = a.title_id.get_untracked();
            let ad = a.description_id.get_untracked();
            assert!(at.ends_with("-title"));
            assert!(ad.ends_with("-description"));
            assert_eq!(
                at.trim_end_matches("-title"),
                ad.trim_end_matches("-description")
            );
            assert_ne!(at, b.title_id.get_untracked());
            assert_ne!(ad, b.description_id.get_untracked());
        });
    }

    // ===== Views construct with the compound API =====

    #[test]
    fn compound_api_builds_for_both_themes() {
        with_owner(|| {
            use crate::*;
            let (open, set_open) = signal(false);
            let _ = view! {
                <Sheet
                    open=open.into()
                    on_open_change=Some(Callback::new(move |v| set_open.set(v)))
                >
                    <SheetTrigger>"Open"</SheetTrigger>
                    <SheetContent side=SheetSide::Left class="x">
                        <SheetHeader>
                            <SheetTitle>"Title"</SheetTitle>
                            <SheetDescription>"Desc"</SheetDescription>
                        </SheetHeader>
                        <SheetFooter>
                            <SheetClose>"Close"</SheetClose>
                        </SheetFooter>
                    </SheetContent>
                </Sheet>
            };
            let _ = view! {
                <SheetNewYork default_open=true>
                    <SheetTriggerNewYork>"Open"</SheetTriggerNewYork>
                    <SheetContentNewYork>
                        <SheetTitleNewYork>"T"</SheetTitleNewYork>
                    </SheetContentNewYork>
                </SheetNewYork>
            };
        });
    }
}
