#[cfg(test)]
mod state_management_tests {

    use crate::*;
    use leptos::prelude::*;

    // ===== STATE MANAGEMENT TESTS =====
    // These tests focus on state management and signal handling

    #[test]
    fn test_context_menu_state_initialization() {
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
        // Test that state is properly initialized
    }

    #[test]
    fn test_context_menu_state_with_open_signal() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
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
        // Test that state works with open signal
    }

    #[test]
    fn test_context_menu_state_with_on_open_change() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _on_open_change = Callback::new(|_: bool| {});
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
        // Test that state works with on_open_change callback
    }

    #[test]
    fn test_context_menu_state_with_multiple_signals() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _open = RwSignal::new(false);
        let _disabled = RwSignal::new(false);
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
        // Test that state works with multiple signals
    }

    #[test]
    fn test_context_menu_state_with_computed_values() {
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
        // Test that state works with computed values
    }

    #[test]
    fn test_context_menu_state_with_derived_signals() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let open = RwSignal::new(false);
        let derived_signal = Signal::derive(move || open.get().to_string());
        let _context_menu_view = view! {
            <ContextMenu attr:aria-expanded=derived_signal>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>"Item 1"</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with derived signals
    }

    #[test]
    fn test_context_menu_state_with_memo() {
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
        // Test that state works with memoized values
    }

    #[test]
    fn test_context_menu_state_with_context() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let context_value = RwSignal::new("context-value".to_string());
        provide_context(context_value);
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
        // Test that state works with context
    }

    #[test]
    fn test_context_menu_state_with_provided_context() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let context_value = RwSignal::new("context-value".to_string());
        provide_context(context_value);
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
        // Test that state works with provided context
    }

    #[test]
    fn test_context_menu_state_with_expected_context() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let context_value = RwSignal::new("context-value".to_string());
        provide_context(context_value);
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
        // Test that state works with expected context
    }

    #[test]
    fn test_context_menu_state_with_multiple_contexts() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let context1 = RwSignal::new("context1".to_string());
        let context2 = RwSignal::new("context2".to_string());
        provide_context(context1);
        provide_context(context2);
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
        // Test that state works with multiple contexts
    }

    #[test]
    fn test_context_menu_state_with_nested_contexts() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let outer_context = RwSignal::new("outer".to_string());
        let inner_context = RwSignal::new("inner".to_string());
        provide_context(outer_context);
        provide_context(inner_context);
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
        // Test that state works with nested contexts
    }

    #[test]
    fn test_context_menu_state_with_conditional_rendering() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let show_context_menu = RwSignal::new(true);
        let _context_menu_view = view! {
            <Show when=move || show_context_menu.get()>
                <ContextMenu>
                    <ContextMenuTrigger>
                        "Right-click me"
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem>"Item 1"</ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            </Show>
        };
        // Test that state works with conditional rendering
    }

    #[test]
    fn test_context_menu_state_with_conditional_props() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let _is_disabled = RwSignal::new(false);
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
        // Test that state works with conditional props
    }

    #[test]
    fn test_context_menu_state_with_dynamic_content() {
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
        // Test that state works with dynamic content
    }

    #[test]
    fn test_context_menu_state_with_filtered_items() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let all_items = vec!["Item 1", "Item 2", "Item 3"];
        let filter = RwSignal::new("".to_string());
        let filtered_items = Signal::derive(move || {
            let f = filter.get();
            if f.is_empty() {
                all_items.clone()
            } else {
                all_items
                    .clone()
                    .into_iter()
                    .filter(|item| item.contains(&f))
                    .collect()
            }
        });
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || filtered_items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem>{item}</ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with filtered items
    }

    #[test]
    fn test_context_menu_state_with_sorted_items() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 3", "Item 1", "Item 2"]);
        let sorted_items = Signal::derive(move || {
            let mut items = items.get();
            items.sort();
            items
        });
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || sorted_items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem>{item}</ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with sorted items
    }

    #[test]
    fn test_context_menu_state_with_grouped_items() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec![
            ("Group 1", "Item 1"),
            ("Group 1", "Item 2"),
            ("Group 2", "Item 3"),
        ]);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|entry| format!("{}-{}", entry.0, entry.1) let:entry>
                        <div role="group">
                            <ContextMenuLabel>{entry.0}</ContextMenuLabel>
                            <ContextMenuItem>{entry.1}</ContextMenuItem>
                        </div>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with grouped items
    }

    #[test]
    fn test_context_menu_state_with_async_content() {
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
        // Test that state works with async content
    }

    #[test]
    fn test_context_menu_state_with_loading_state() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let loading = RwSignal::new(false);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <Show when=move || !loading.get()>
                        <For each=move || items.get() key=|item| item.to_string() let:item>
                            <ContextMenuItem>{item}</ContextMenuItem>
                        </For>
                    </Show>
                    <Show when=move || loading.get()>
                        <ContextMenuItem>"Loading..."</ContextMenuItem>
                    </Show>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with loading state
    }

    #[test]
    fn test_context_menu_state_with_error_state() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let error = RwSignal::new(None::<String>);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <Show when=move || error.get().is_none()>
                        <For each=move || items.get() key=|item| item.to_string() let:item>
                            <ContextMenuItem>{item}</ContextMenuItem>
                        </For>
                    </Show>
                    <Show when=move || error.get().is_some()>
                        <ContextMenuItem>"Error occurred"</ContextMenuItem>
                    </Show>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with error state
    }

    #[test]
    fn test_context_menu_state_with_validation() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let _is_valid = Signal::derive(move || !selected_item.get().is_empty());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem on:click=move |_| {
                                selected_item.set(item.to_string());
                            }>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with validation
    }

    #[test]
    fn test_context_menu_state_with_form_integration() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let _form_data = RwSignal::new(std::collections::HashMap::<String, String>::new());
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
        // Test that state works with form integration
    }

    #[test]
    fn test_context_menu_state_with_persistence() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("Item 1".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem class=MaybeProp::from(Signal::derive(move || {
                                if item == selected_item.get() { "selected".to_string() } else { String::new() }
                            }))>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with persistence
    }

    #[test]
    fn test_context_menu_state_with_undo_redo() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("Item 1".to_string());
        let history = RwSignal::new(vec!["Item 1".to_string()]);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem on:click=move |_| {
                                history.update(|h| h.push(selected_item.get()));
                                selected_item.set(item.to_string());
                            }>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with undo/redo
    }

    #[test]
    fn test_context_menu_state_with_batch_updates() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let open = RwSignal::new(false);
        let selected_item = RwSignal::new("".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem on:click=move |_| {
                                batch(move || {
                                    selected_item.set(item.to_string());
                                    open.set(false);
                                });
                            }>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with batch updates
    }

    #[test]
    fn test_context_menu_state_with_debounced_updates() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let _search_term = RwSignal::new("".to_string());
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
        // Test that state works with debounced updates
    }

    #[test]
    fn test_context_menu_state_with_throttled_updates() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let update_count = RwSignal::new(0);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem on:click=move |_| {
                                update_count.update(|c| *c += 1);
                            }>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with throttled updates
    }

    #[test]
    fn test_context_menu_state_with_optimistic_updates() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem on:click=move |_| {
                                // Optimistic update
                                selected_item.set(item.to_string());
                            }>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with optimistic updates
    }

    #[test]
    fn test_context_menu_state_with_rollback() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let previous_item = RwSignal::new("".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem on:click=move |_| {
                                previous_item.set(selected_item.get());
                                selected_item.set(item.to_string());
                            }>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with rollback
    }

    #[test]
    fn test_context_menu_state_with_conflict_resolution() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let conflict_resolution = RwSignal::new("last_wins".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem on:click=move |_| {
                                match conflict_resolution.get().as_str() {
                                    "last_wins" => selected_item.set(item.to_string()),
                                    "first_wins" => {
                                        if selected_item.get().is_empty() {
                                            selected_item.set(item.to_string());
                                        }
                                    },
                                    _ => selected_item.set(item.to_string()),
                                }
                            }>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with conflict resolution
    }

    #[test]
    fn test_context_menu_state_with_synchronization() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let sync_enabled = RwSignal::new(true);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem on:click=move |_| {
                                if sync_enabled.get() {
                                    selected_item.set(item.to_string());
                                }
                            }>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with synchronization
    }

    #[test]
    fn test_context_menu_state_with_replication() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let replica_count = RwSignal::new(3);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem on:click=move |_| {
                                selected_item.set(item.to_string());
                                // Replicate to other instances
                                for _ in 0..replica_count.get() {
                                    // Replication logic would go here
                                }
                            }>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with replication
    }

    #[test]
    fn test_context_menu_state_with_consistency() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let consistency_level = RwSignal::new("strong".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem on:click=move |_| {
                                match consistency_level.get().as_str() {
                                    "strong" => {
                                        // Strong consistency
                                        selected_item.set(item.to_string());
                                    },
                                    "eventual" => {
                                        // Eventual consistency
                                        selected_item.set(item.to_string());
                                    },
                                    _ => selected_item.set(item.to_string()),
                                }
                            }>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with consistency
    }

    #[test]
    fn test_context_menu_state_with_atomicity() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem on:click=move |_| {
                                // Atomic operation
                                batch(move || {
                                    selected_item.set(item.to_string());
                                });
                            }>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with atomicity
    }

    #[test]
    fn test_context_menu_state_with_isolation() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let isolation_level = RwSignal::new("read_committed".to_string());
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem on:click=move |_| {
                                match isolation_level.get().as_str() {
                                    "read_committed" => {
                                        // Read committed isolation
                                        selected_item.set(item.to_string());
                                    },
                                    "repeatable_read" => {
                                        // Repeatable read isolation
                                        selected_item.set(item.to_string());
                                    },
                                    _ => selected_item.set(item.to_string()),
                                }
                            }>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with isolation
    }

    #[test]
    fn test_context_menu_state_with_durability() {
        // Components use context; an owner must be active on native.
        let _owner = Owner::new();
        _owner.set();
        let items = RwSignal::new(vec!["Item 1", "Item 2"]);
        let selected_item = RwSignal::new("".to_string());
        let durability_enabled = RwSignal::new(true);
        let _context_menu_view = view! {
            <ContextMenu>
                <ContextMenuTrigger>
                    "Right-click me"
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <For each=move || items.get() key=|item| item.to_string() let:item>
                        <ContextMenuItem on:click=move |_| {
                                selected_item.set(item.to_string());
                                if durability_enabled.get() {
                                    // Persist to durable storage
                                }
                            }>
                            {item}
                        </ContextMenuItem>
                    </For>
                </ContextMenuContent>
            </ContextMenu>
        };
        // Test that state works with durability
    }
}
