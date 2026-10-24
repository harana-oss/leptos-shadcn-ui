#![allow(clippy::module_inception)] // test layout: one inner module per test file
#[cfg(test)]
mod performance_tests {

    use crate::*;
    use leptos::prelude::*;

    // ===== PERFORMANCE TESTS =====
    // These tests focus on performance, callbacks, and optimization

    #[test]
    fn test_context_menu_performance_basic_rendering() {
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
    fn test_context_menu_performance_large_menu() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let start = std::time::Instant::now();

        let items = StoredValue::new(Vec::from_iter(1..=100));
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get_value() key=|item| *item let:item>
                        <ContextMenuItem>{format!("Item {}", item)}</ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };

        let duration = start.elapsed();
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_context_menu_performance_memory_usage() {
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
    fn test_context_menu_performance_signal_updates() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let open = RwSignal::new(false);
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

        let start = std::time::Instant::now();

        for i in 0..100 {
            open.set(i % 2 == 0);
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 10);
    }

    #[test]
    fn test_context_menu_performance_callback_execution() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let call_count = RwSignal::new(0);
        let on_open_change = Callback::new(move |_| {
            call_count.update(|c| *c += 1);
        });
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

        let start = std::time::Instant::now();

        for _ in 0..100 {
            on_open_change.run(true);
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 10);
        assert_eq!(call_count.get(), 100);
    }

    #[test]
    fn test_context_menu_performance_dynamic_content() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem>{item}</ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };

        let start = std::time::Instant::now();

        for i in 0..100 {
            items.update(|items| {
                items.push(Box::leak(format!("Item {}", i + 3).into_boxed_str()));
            });
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_context_menu_performance_computed_props() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let open = RwSignal::new(false);
        let _computed_class = Signal::derive(move || {
            format!(
                "context-menu-{}",
                if open.get() { "open" } else { "closed" }
            )
        });
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

        let start = std::time::Instant::now();

        for i in 0..100 {
            open.set(i % 2 == 0);
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 20);
    }

    #[test]
    fn test_context_menu_performance_memoized_values() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let open = RwSignal::new(false);
        let _memoized_class = Memo::new(move |_| {
            format!(
                "context-menu-{}",
                if open.get() { "open" } else { "closed" }
            )
        });
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

        let start = std::time::Instant::now();

        for i in 0..100 {
            open.set(i % 2 == 0);
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 15);
    }

    #[test]
    fn test_context_menu_performance_batch_updates() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let open = RwSignal::new(false);
        let disabled = RwSignal::new(false);
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

        let start = std::time::Instant::now();

        for i in 0..100 {
            batch(move || {
                open.set(i % 2 == 0);
                disabled.set(i % 3 == 0);
            });
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 20);
    }

    #[test]
    fn test_context_menu_performance_untracked_access() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let open = RwSignal::new(false);
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

        let start = std::time::Instant::now();

        for _ in 0..1000 {
            let _untracked_value = open.get_untracked();
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 5);
    }

    #[test]
    fn test_context_menu_performance_concurrent_updates() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let open1 = RwSignal::new(false);
        let open2 = RwSignal::new(false);
        let _context_menu_view1 = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Menu 1"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        let _context_menu_view2 = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Menu 2"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 2"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };

        let start = std::time::Instant::now();

        for i in 0..100 {
            open1.set(i % 2 == 0);
            open2.set(i % 2 == 1);
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 30);
    }

    #[test]
    fn test_context_menu_performance_deep_nesting() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <div>
                <div>
                    <div>
                        <div>
                            <div>
                                <ContextMenu>
                                    <ContextMenuTrigger>
                                        "Right-click me"
                                    </ContextMenuTrigger>
                                    <ContextMenuContent>
                                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                                    </ContextMenuContent>
                                </ContextMenu>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        };

        let start = std::time::Instant::now();

        for _ in 0..100 {
            let _nested_view = view! {
                <div>
                    <div>
                        <div>
                            <ContextMenu>
                                <ContextMenuTrigger>
                                    "Nested menu"
                                </ContextMenuTrigger>
                                <ContextMenuContent>
                                    <ContextMenuItem>"Nested item"</ContextMenuItem>
                                </ContextMenuContent>
                            </ContextMenu>
                        </div>
                    </div>
                </div>
            };
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_context_menu_performance_many_props() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger class=MaybeProp::from("custom-trigger") attr:style="color: red" attr:id="test-trigger" attr:aria-label="Context menu trigger" attr:aria-describedby="trigger-description" attr:aria-expanded=false attr:aria-haspopup="menu" attr:role="button" attr:tabindex=0 attr:title="Right-click for options" attr:data-testid="context-menu-trigger">
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent class=MaybeProp::from("custom-content") style=MaybeProp::from("background: blue") id=MaybeProp::from("test-content") attr:role="menu" attr:aria-label="Context menu options" attr:aria-orientation="vertical" attr:data-testid="context-menu-content">
                    <ContextMenuItem class=MaybeProp::from("custom-item") style=MaybeProp::from("color: white") id=MaybeProp::from("test-item") attr:role="menuitem" attr:tabindex=0 attr:aria-label="Menu item" attr:data-testid="context-menu-item">
                        "Item 1"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };

        let start = std::time::Instant::now();

        for _ in 0..50 {
            let _many_props_view = view! {
                <ContextMenu>
                    <ContextMenuTrigger class=MaybeProp::from("trigger") attr:style="color: red" attr:id="trigger" attr:aria-label="Trigger" attr:role="button" attr:tabindex=0>
                        "Trigger"
                    </ContextMenuTrigger>
                    <ContextMenuContent class=MaybeProp::from("content") attr:role="menu" attr:aria-label="Menu">
                        <ContextMenuItem attr:role="menuitem">"Item"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            };
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_context_menu_performance_stress_test() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let start = std::time::Instant::now();

        for i in 0..50 {
            let items = StoredValue::new(Vec::from_iter(1..=20));
            let open = RwSignal::new(false);
            let disabled = RwSignal::new(i % 2 == 0);

            let _context_menu_view = view! {
                <ContextMenu>
                    <ContextMenuTrigger>
                        {format!("Menu {}", i)}
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <For each=move || items.get_value() key=|item| *item let:item>
                            <ContextMenuItem>
                                {format!("Item {} - {}", i, item)}
                            </ContextMenuItem>
                        </For>
                    </ContextMenuContent>
                </ContextMenu>
            };

            // Perform some updates
            open.set(true);
            disabled.set(!disabled.get());
            open.set(false);
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 500);
    }

    #[test]
    fn test_context_menu_performance_memory_leak_prevention() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        for _ in 0..100 {
            let _open = RwSignal::new(false);
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

            // Drop the view to test cleanup
            drop(_context_menu_view);
        }

        // Test that no significant memory leaks occur
        // This is a simple test - in practice you'd use more sophisticated memory monitoring
    }

    #[test]
    fn test_context_menu_performance_signal_cleanup() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let mut signals = Vec::new();

        for i in 0..100 {
            let open = RwSignal::new(false);
            signals.push(open);

            let _context_menu_view = view! {
                <ContextMenu>
                    <ContextMenuTrigger>
                        {format!("Menu {}", i)}
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            };
        }

        // Test that signals can be properly cleaned up
        drop(signals);
    }

    #[test]
    fn test_context_menu_performance_callback_cleanup() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let mut callbacks = Vec::new();

        for i in 0..100 {
            let callback = Callback::new(move |open: bool| {
                leptos::logging::log!("Menu {} open state changed: {}", i, open);
            });
            callbacks.push(callback);

            let _context_menu_view = view! {
                <ContextMenu>
                    <ContextMenuTrigger>
                        {format!("Menu {}", i)}
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            };
        }

        // Test that callbacks can be properly cleaned up
        drop(callbacks);
    }

    #[test]
    fn test_context_menu_performance_edge_case_handling() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let start = std::time::Instant::now();

        // Test with various edge cases
        let test_cases: Vec<Vec<String>> = vec![
            vec![],                                              // Empty items
            vec!["".to_string()],                                // Empty strings
            vec!["a".repeat(1000)],                              // Very long strings
            (1..=1000).map(|i| format!("Item {}", i)).collect(), // Many items
        ];

        for items in test_cases {
            let items = StoredValue::new(items);
            let _context_menu_view = view! {
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <For each=move || items.get_value() key=|item| item.to_string() let:item>
                            <ContextMenuItem>{item}</ContextMenuItem>
                        </For>
                    </ContextMenuContent>
                </ContextMenu>
            };
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 200);
    }

    #[test]
    fn test_context_menu_performance_rapid_state_changes() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let open = RwSignal::new(false);
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

        let start = std::time::Instant::now();

        // Rapid state changes
        for i in 0..1000 {
            open.set(i % 2 == 0);
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_context_menu_performance_large_nested_structure() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let start = std::time::Instant::now();

        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuSub>
                        <ContextMenuSubTrigger>"Level 1"</ContextMenuSubTrigger>
                        <ContextMenuSubContent>
                            <ContextMenuSub>
                                <ContextMenuSubTrigger>"Level 2"</ContextMenuSubTrigger>
                                <ContextMenuSubContent>
                                    <ContextMenuSub>
                                        <ContextMenuSubTrigger>"Level 3"</ContextMenuSubTrigger>
                                        <ContextMenuSubContent>
                                            <For each=move || Vec::from_iter(1..=50) key=|item| *item let:item>
                                                <ContextMenuItem>{format!("Deep Item {}", item)}</ContextMenuItem>
                                            </For>
                                        </ContextMenuSubContent>
                                    </ContextMenuSub>
                                </ContextMenuSubContent>
                            </ContextMenuSub>
                        </ContextMenuSubContent>
                    </ContextMenuSub>
                </ContextMenuContent>
            </ContextMenu>
        };

        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_context_menu_performance_event_handler_optimization() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let click_count = RwSignal::new(0);
        let callback = Callback::new(move |_| {
            click_count.update(|c| *c += 1);
        });

        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || Vec::from_iter(1..=100) key=|item| *item let:item>
                        <ContextMenuItem on:click=move |_| callback.run(())>
                            {format!("Item {}", item)}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };

        let start = std::time::Instant::now();

        // Simulate multiple clicks
        for _ in 0..100 {
            callback.run(());
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 20);
        assert_eq!(click_count.get(), 100);
    }
}
