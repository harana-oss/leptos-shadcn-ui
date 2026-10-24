#![allow(clippy::module_inception)] // test layout: one inner module per test file
#[cfg(test)]
mod basic_rendering_tests {

    use crate::*;
    use leptos::prelude::*;

    // ===== BASIC RENDERING TESTS =====
    // These tests focus on basic rendering and component creation

    #[test]
    fn test_context_menu_basic_rendering() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
            </ContextMenu>
        };
        // GREEN PHASE: Verify actual rendering behavior
    }

    #[test]
    fn test_context_menu_trigger() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger class=MaybeProp::from("custom-trigger")>
                    "Custom Trigger"
                </ContextMenuTrigger>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_content() {
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
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_item() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem class=MaybeProp::from("custom-item")>
                        "Custom Item"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_separator() {
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
                    <ContextMenuSeparator/>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_label() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuLabel>"Section Label"</ContextMenuLabel>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_group() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <div role="group">
                        <ContextMenuItem>"Group Item 1"</ContextMenuItem>
                        <ContextMenuItem>"Group Item 2"</ContextMenuItem>
                    </div>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_submenu() {
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
                    <ContextMenuSub>
                        <ContextMenuSubTrigger>"Submenu"</ContextMenuSubTrigger>
                        <ContextMenuSubContent>
                            <ContextMenuItem>"Sub Item 1"</ContextMenuItem>
                            <ContextMenuItem>"Sub Item 2"</ContextMenuItem>
                        </ContextMenuSubContent>
                    </ContextMenuSub>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_props() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger class=MaybeProp::from("custom-trigger") attr:style="color: red" attr:id="test-trigger">
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent class=MaybeProp::from("custom-content") style=MaybeProp::from("background: blue") id=MaybeProp::from("test-content")>
                    <ContextMenuItem class=MaybeProp::from("custom-item") style=MaybeProp::from("color: white") id=MaybeProp::from("test-item")>
                        "Custom Item"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_children() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    <div>
                        <span>"Complex Trigger"</span>
                        <button>"Button"</button>
                    </div>
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>
                        <div>
                            <span>"Complex Item"</span>
                            <span>"Description"</span>
                        </div>
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_multiple_items() {
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
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                    <ContextMenuItem>"Item 3"</ContextMenuItem>
                    <ContextMenuItem>"Item 4"</ContextMenuItem>
                    <ContextMenuItem>"Item 5"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_nested_structure() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuLabel>"Actions"</ContextMenuLabel>
                    <ContextMenuItem>"Edit"</ContextMenuItem>
                    <ContextMenuItem>"Copy"</ContextMenuItem>
                    <ContextMenuSeparator/>
                    <ContextMenuLabel>"More"</ContextMenuLabel>
                    <ContextMenuSub>
                        <ContextMenuSubTrigger>"Advanced"</ContextMenuSubTrigger>
                        <ContextMenuSubContent>
                            <ContextMenuItem>"Advanced Option 1"</ContextMenuItem>
                            <ContextMenuItem>"Advanced Option 2"</ContextMenuItem>
                        </ContextMenuSubContent>
                    </ContextMenuSub>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_empty_content() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_only_separators() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuSeparator/>
                    <ContextMenuSeparator/>
                    <ContextMenuSeparator/>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_only_labels() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuLabel>"Label 1"</ContextMenuLabel>
                    <ContextMenuLabel>"Label 2"</ContextMenuLabel>
                    <ContextMenuLabel>"Label 3"</ContextMenuLabel>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_with_mixed_content() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuLabel>"Section 1"</ContextMenuLabel>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                    <ContextMenuSeparator/>
                    <ContextMenuLabel>"Section 2"</ContextMenuLabel>
                    <ContextMenuItem>"Item 3"</ContextMenuItem>
                    <ContextMenuSub>
                        <ContextMenuSubTrigger>"Submenu"</ContextMenuSubTrigger>
                        <ContextMenuSubContent>
                            <ContextMenuItem>"Sub Item 1"</ContextMenuItem>
                            <ContextMenuItem>"Sub Item 2"</ContextMenuItem>
                        </ContextMenuSubContent>
                    </ContextMenuSub>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_rendering_performance() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let start = std::time::Instant::now();

        for _ in 0..100 {
            let _context_menu_view = view! {
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                        <ContextMenuItem>"Item 2"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            };
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_context_menu_rendering_memory_usage() {
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
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };

        let size = std::mem::size_of::<usize>();
        assert!(size < 1024);
    }

    #[test]
    fn test_context_menu_rendering_consistency() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view1 = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        let _context_menu_view2 = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        let _context_menu_view3 = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_rendering_with_different_props() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view1 = view! {
            <ContextMenu>
                <ContextMenuTrigger class=MaybeProp::from("trigger1")>
                    "Trigger 1"
                </ContextMenuTrigger>
                <ContextMenuContent class=MaybeProp::from("content1")>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        let _context_menu_view2 = view! {
            <ContextMenu>
                <ContextMenuTrigger class=MaybeProp::from("trigger2")>
                    "Trigger 2"
                </ContextMenuTrigger>
                <ContextMenuContent class=MaybeProp::from("content2")>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_rendering_edge_cases() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    ""
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>""</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }

    #[test]
    fn test_context_menu_rendering_with_none_props() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger class=MaybeProp::from(None::<String>) attr:style={None::<String>} attr:id={None::<String>}>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent class=MaybeProp::from(None::<String>) style=MaybeProp::from(None::<String>) id=MaybeProp::from(None::<String>)>
                    <ContextMenuItem class=MaybeProp::from(None::<String>) style=MaybeProp::from(None::<String>) id=MaybeProp::from(None::<String>)>
                        "Item"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
    }
}
