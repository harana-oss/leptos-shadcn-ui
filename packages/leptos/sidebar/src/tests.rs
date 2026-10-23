use crate::*;

#[test]
fn state_strings() {
    assert_eq!(sidebar_state(true), "expanded");
    assert_eq!(sidebar_state(false), "collapsed");
}

#[test]
fn shortcut_detection() {
    assert!(is_toggle_shortcut("b", true, false));
    assert!(is_toggle_shortcut("B", false, true));
    assert!(!is_toggle_shortcut("b", false, false));
    assert!(!is_toggle_shortcut("c", true, false));
}

#[test]
fn enum_strings() {
    assert_eq!(SidebarSide::default().as_str(), "left");
    assert_eq!(SidebarSide::Right.as_str(), "right");
    assert_eq!(SidebarVariant::Floating.as_str(), "floating");
    assert_eq!(SidebarVariant::Inset.as_str(), "inset");
    assert_eq!(SidebarCollapsible::default().as_str(), "offcanvas");
    assert_eq!(SidebarCollapsible::Icon.as_str(), "icon");
    assert_eq!(SidebarCollapsible::None.as_str(), "none");
    assert_eq!(SidebarMenuButtonSize::Lg.as_str(), "lg");
}

#[test]
fn menu_button_classes() {
    let c = sidebar_menu_button_class(SidebarMenuButtonVariant::Outline, SidebarMenuButtonSize::Sm);
    assert!(c.contains("bg-background"));
    assert!(c.contains("h-7 text-xs"));
    let d = sidebar_menu_button_class(SidebarMenuButtonVariant::Default, SidebarMenuButtonSize::Lg);
    assert!(d.contains("h-12"));
    assert!(d.contains("data-[active=true]:bg-sidebar-accent"));
}

#[test]
fn container_classes() {
    let c = sidebar_container_class(SidebarSide::Right, SidebarVariant::Floating);
    assert!(c.contains("right-0"));
    assert!(c.contains("p-2"));
    let l = sidebar_container_class(SidebarSide::Left, SidebarVariant::Sidebar);
    assert!(l.contains("left-0"));
    assert!(l.contains("border-r"));
    assert!(sidebar_gap_class(SidebarVariant::Inset).contains("--spacing(4)"));
}

#[test]
fn menu_action_hover() {
    assert!(sidebar_menu_action_class(true).contains("md:opacity-0"));
    assert!(!sidebar_menu_action_class(false).contains("md:opacity-0"));
}

#[test]
fn context_toggle_desktop_and_mobile() {
    let owner = Owner::new();
    owner.with(|| {
        let ctx = SidebarContext::new(true);
        assert_eq!(ctx.state(), "expanded");
        ctx.toggle();
        assert_eq!(ctx.state(), "collapsed");
        ctx.is_mobile.set(true);
        ctx.toggle();
        assert!(ctx.open_mobile.get_untracked());
        assert!(!ctx.open.get_untracked());
    });
}

#[test]
fn mobile_breakpoint_logic() {
    assert_eq!(MOBILE_BREAKPOINT, 768);
    assert_eq!(mobile_media_query(), "(max-width: 767px)");
    assert!(is_mobile_width(767.0));
    assert!(is_mobile_width(767.5));
    assert!(!is_mobile_width(768.0));
    assert!(!is_mobile_width(1024.0));
}

#[test]
fn override_wins_over_detection() {
    assert!(resolve_is_mobile(Some(true), false));
    assert!(!resolve_is_mobile(Some(false), true));
    assert!(resolve_is_mobile(None, true));
    assert!(!resolve_is_mobile(None, false));
}

#[test]
fn use_is_mobile_defaults_false_off_browser() {
    let owner = Owner::new();
    owner.with(|| assert!(!use_is_mobile().get_untracked()));
}
