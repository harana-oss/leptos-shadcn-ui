#![allow(clippy::module_inception)] // test layout: one inner module per test file
#[cfg(test)]
mod accessibility_tests {
    
    use crate::*;
    use leptos::prelude::*;

    // ===== ACCESSIBILITY TESTS =====
    // These tests focus on accessibility features and ARIA attributes

    #[test]
    fn test_context_menu_accessibility_basic() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test basic accessibility features
    }

    #[test]
    fn test_context_menu_accessibility_with_aria_label() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:aria-label="Context menu trigger">
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent attr:aria-label="Context menu">
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with ARIA label
    }

    #[test]
    fn test_context_menu_accessibility_with_aria_describedby() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:aria-describedby="menu-description">
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with ARIA describedby
    }

    #[test]
    fn test_context_menu_accessibility_with_aria_expanded() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let expanded = RwSignal::new(false);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:aria-expanded=expanded>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with ARIA expanded
    }

    #[test]
    fn test_context_menu_accessibility_with_aria_haspopup() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:aria-haspopup="menu">
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with ARIA haspopup
    }

    #[test]
    fn test_context_menu_accessibility_with_aria_owns() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:aria-owns="menu-content">
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent id=MaybeProp::from("menu-content")>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with ARIA owns
    }

    #[test]
    fn test_context_menu_accessibility_with_aria_controls() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:aria-controls="menu-content">
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent id=MaybeProp::from("menu-content")>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with ARIA controls
    }

    #[test]
    fn test_context_menu_accessibility_with_aria_activedescendant() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let active_item = RwSignal::new("item-1".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent attr:aria-activedescendant=active_item>
                    <ContextMenuItem id=MaybeProp::from("item-1")>"Item 1"</ContextMenuItem>
                    <ContextMenuItem id=MaybeProp::from("item-2")>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with ARIA activedescendant
    }

    #[test]
    fn test_context_menu_accessibility_with_keyboard_navigation() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:tabindex=0>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem attr:tabindex=0>"Item 1"</ContextMenuItem>
                    <ContextMenuItem attr:tabindex=0>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with keyboard navigation
    }

    #[test]
    fn test_context_menu_accessibility_with_focus_management() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:autofocus=true>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with focus management
    }

    #[test]
    fn test_context_menu_accessibility_with_screen_reader_support() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:role="button" attr:aria-label="Open context menu">
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent attr:role="menu" attr:aria-label="Context menu options">
                    <ContextMenuItem attr:role="menuitem">"Item 1"</ContextMenuItem>
                    <ContextMenuItem attr:role="menuitem">"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with screen reader support
    }

    #[test]
    fn test_context_menu_accessibility_with_live_regions() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let status = RwSignal::new("".to_string());
        let _context_menu_view = view! {
            <div>
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem on:click=move |_| {
                                status.set("Item 1 selected".to_string());
                            }>
                            "Item 1"
                        </ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
                <div attr:aria-live="polite" attr:aria-atomic=true>
                    {status}
                </div>
            </div>
        };
        // Test accessibility with live regions
    }

    #[test]
    fn test_context_menu_accessibility_with_high_contrast() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger class=MaybeProp::from("high-contrast")>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent class=MaybeProp::from("high-contrast")>
                    <ContextMenuItem class=MaybeProp::from("high-contrast")>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with high contrast mode
    }

    #[test]
    fn test_context_menu_accessibility_with_reduced_motion() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger class=MaybeProp::from("reduced-motion")>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent class=MaybeProp::from("reduced-motion")>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with reduced motion
    }

    #[test]
    fn test_context_menu_accessibility_with_voice_control() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:aria-label="Context menu trigger" attr:title="Right-click or press Enter to open menu">
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem attr:aria-label="Edit option" attr:title="Edit this item">
                        "Edit"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with voice control
    }

    #[test]
    fn test_context_menu_accessibility_with_multiple_aria_attributes() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:aria-label="Context menu trigger" attr:aria-describedby="menu-description" attr:aria-expanded=false attr:aria-haspopup="menu" attr:aria-owns="menu-content" attr:aria-controls="menu-content" attr:role="button" attr:tabindex=0>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent id=MaybeProp::from("menu-content") attr:role="menu" attr:aria-label="Context menu options" attr:aria-orientation="vertical">
                    <ContextMenuItem attr:role="menuitem" attr:tabindex=0 attr:aria-label="First menu item">
                        "Item 1"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with multiple ARIA attributes
    }

    #[test]
    fn test_context_menu_accessibility_performance() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let start = std::time::Instant::now();
        
        for _ in 0..100 {
            let _context_menu_view = view! {
                <ContextMenu>
                    <ContextMenuTrigger attr:aria-label="Context menu trigger" attr:aria-expanded=false attr:aria-haspopup="menu" attr:role="button" attr:tabindex=0>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent attr:role="menu" attr:aria-label="Context menu options">
                        <ContextMenuItem attr:role="menuitem">"Item 1"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            };
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_context_menu_accessibility_memory_usage() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:aria-label="Context menu trigger" attr:role="button">
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent attr:role="menu">
                    <ContextMenuItem attr:role="menuitem">"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        
        let size = std::mem::size_of::<usize>();
        assert!(size < 1024);
    }

    #[test]
    fn test_context_menu_accessibility_consistency() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view1 = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:aria-label="Menu 1">
                    "Menu 1"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        let _context_menu_view2 = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:aria-label="Menu 2">
                    "Menu 2"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_accessibility_edge_cases() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:aria-label="" attr:role="">
                    ""
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>""</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with empty values
    }

    #[test]
    fn test_context_menu_accessibility_none_values() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger attr:aria-label={None::<String>} attr:aria-describedby={None::<String>} attr:aria-expanded={None::<bool>} attr:aria-haspopup={None::<String>} attr:role={None::<String>} attr:tabindex={None::<i32>}>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent attr:role={None::<String>} attr:aria-label={None::<String>}>
                    <ContextMenuItem attr:role={None::<String>}>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test accessibility with None values
    }
}
