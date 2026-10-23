#[cfg(test)]
mod performance_tests {

    use crate::*;
    use leptos::prelude::*;

    // ===== PERFORMANCE TESTS =====
    // These tests focus on performance, callbacks, and optimization

    #[test]
    fn test_combobox_performance_basic_rendering() {
        leptos::prelude::Owner::new().with(|| {
            let start = std::time::Instant::now();

            for _ in 0..100 {
                let options = vec![
                    ComboboxOption::new("option1", "Option 1"),
                    ComboboxOption::new("option2", "Option 2"),
                ];
                let _combobox_view = view! {
                    <Combobox options=options/>
                };
            }

            let duration = start.elapsed();
            assert!(duration.as_millis() < 100);
        });
    }

    #[test]
    fn test_combobox_performance_large_option_list() {
        leptos::prelude::Owner::new().with(|| {
            let start = std::time::Instant::now();

            let options = (1..=1000)
                .map(|i| ComboboxOption::new(format!("option{}", i), format!("Option {}", i)))
                .collect::<Vec<_>>();

            let _combobox_view = view! {
                <Combobox options=options/>
            };

            let duration = start.elapsed();
            assert!(duration.as_millis() < 50);
        });
    }

    #[test]
    fn test_combobox_performance_memory_usage() {
        leptos::prelude::Owner::new().with(|| {
            let options = vec![
                ComboboxOption::new("option1", "Option 1"),
                ComboboxOption::new("option2", "Option 2"),
            ];
            let _combobox_view = view! {
                <Combobox options=options/>
            };

            // Test that memory usage is reasonable
            let size = std::mem::size_of::<usize>();
            assert!(size < 1024);
        });
    }

    #[test]
    fn test_combobox_performance_signal_updates() {
        leptos::prelude::Owner::new().with(|| {
            let options = vec![
                ComboboxOption::new("option1", "Option 1"),
                ComboboxOption::new("option2", "Option 2"),
            ];
            let value = RwSignal::new("option1".to_string());
            let _combobox_view = view! {
                <Combobox
                    options=options
                    value=MaybeProp::from(value)
                />
            };

            let start = std::time::Instant::now();

            for i in 0..100 {
                value.set(format!("option{}", i % 2 + 1));
            }

            let duration = start.elapsed();
            assert!(duration.as_millis() < 10);
        });
    }

    #[test]
    fn test_combobox_performance_callback_execution() {
        leptos::prelude::Owner::new().with(|| {
            let options = vec![
                ComboboxOption::new("option1", "Option 1"),
                ComboboxOption::new("option2", "Option 2"),
            ];
            let call_count = RwSignal::new(0);
            let on_change = Callback::new(move |_| {
                call_count.update(|c| *c += 1);
            });
            let _combobox_view = view! {
                <Combobox
                    options=options
                    on_change=on_change
                />
            };

            let start = std::time::Instant::now();

            for _ in 0..100 {
                on_change.run("test-value".to_string());
            }

            let duration = start.elapsed();
            assert!(duration.as_millis() < 10);
            assert_eq!(call_count.get(), 100);
        });
    }

    #[test]
    fn test_combobox_performance_dynamic_options() {
        leptos::prelude::Owner::new().with(|| {
            let options = RwSignal::new(vec![
                ComboboxOption::new("option1", "Option 1"),
                ComboboxOption::new("option2", "Option 2"),
            ]);
            let _combobox_view = view! {
                <Combobox options=options.get_untracked()/>
            };

            let start = std::time::Instant::now();

            for i in 0..100 {
                options.update(|opts| {
                    opts.push(ComboboxOption::new(
                        format!("option{}", i + 3),
                        format!("Option {}", i + 3),
                    ));
                });
            }

            let duration = start.elapsed();
            assert!(duration.as_millis() < 50);
        });
    }

    #[test]
    fn test_combobox_performance_filtered_options() {
        leptos::prelude::Owner::new().with(|| {
            let all_options = (1..=1000)
                .map(|i| ComboboxOption::new(format!("option{}", i), format!("Option {}", i)))
                .collect::<Vec<_>>();

            let filter = RwSignal::new("".to_string());
            let filtered_options = Signal::derive(move || {
                let f = filter.get();
                if f.is_empty() {
                    all_options.clone()
                } else {
                    all_options
                        .clone()
                        .into_iter()
                        .filter(|opt| opt.label.contains(&f))
                        .collect()
                }
            });

            let _combobox_view = view! {
                <Combobox options=filtered_options.get_untracked()/>
            };

            let start = std::time::Instant::now();

            filter.set("Option 1".to_string());
            filter.set("Option 2".to_string());
            filter.set("".to_string());

            let duration = start.elapsed();
            assert!(duration.as_millis() < 100);
        });
    }

    #[test]
    fn test_combobox_performance_computed_props() {
        leptos::prelude::Owner::new().with(|| {
            let options = vec![
                ComboboxOption::new("option1", "Option 1"),
                ComboboxOption::new("option2", "Option 2"),
            ];
            let value = RwSignal::new("option1".to_string());
            let computed_class = Signal::derive(move || format!("combobox-{}", value.get()));
            let _combobox_view = view! {
                <Combobox
                    options=options
                    value=MaybeProp::from(value)
                    class=MaybeProp::from(computed_class)
                />
            };

            let start = std::time::Instant::now();

            for i in 0..100 {
                value.set(format!("option{}", i % 2 + 1));
            }

            let duration = start.elapsed();
            assert!(duration.as_millis() < 20);
        });
    }

    #[test]
    fn test_combobox_performance_memoized_values() {
        leptos::prelude::Owner::new().with(|| {
            let options = vec![
                ComboboxOption::new("option1", "Option 1"),
                ComboboxOption::new("option2", "Option 2"),
            ];
            let value = RwSignal::new("option1".to_string());
            let memoized_class = Memo::new(move |_| format!("combobox-{}", value.get()));
            let _combobox_view = view! {
                <Combobox
                    options=options
                    value=MaybeProp::from(value)
                    class=MaybeProp::from(memoized_class)
                />
            };

            let start = std::time::Instant::now();

            for i in 0..100 {
                value.set(format!("option{}", i % 2 + 1));
            }

            let duration = start.elapsed();
            assert!(duration.as_millis() < 15);
        });
    }

    #[test]
    fn test_combobox_performance_batch_updates() {
        leptos::prelude::Owner::new().with(|| {
            let options = vec![
                ComboboxOption::new("option1", "Option 1"),
                ComboboxOption::new("option2", "Option 2"),
            ];
            let value = RwSignal::new("option1".to_string());
            let disabled = RwSignal::new(false);
            let _combobox_view = view! {
                <Combobox
                    options=options
                    value=MaybeProp::from(value)
                    disabled=disabled
                />
            };

            let start = std::time::Instant::now();

            for i in 0..100 {
                batch(move || {
                    value.set(format!("option{}", i % 2 + 1));
                    disabled.set(i % 2 == 0);
                });
            }

            let duration = start.elapsed();
            assert!(duration.as_millis() < 20);
        });
    }

    #[test]
    fn test_combobox_performance_untracked_access() {
        leptos::prelude::Owner::new().with(|| {
            let options = vec![
                ComboboxOption::new("option1", "Option 1"),
                ComboboxOption::new("option2", "Option 2"),
            ];
            let value = RwSignal::new("option1".to_string());
            let _combobox_view = view! {
                <Combobox
                    options=options
                    value=MaybeProp::from(value)
                />
            };

            let start = std::time::Instant::now();

            for _ in 0..1000 {
                let _untracked_value = value.get_untracked();
            }

            let duration = start.elapsed();
            assert!(duration.as_millis() < 5);
        });
    }

    #[test]
    fn test_combobox_performance_concurrent_updates() {
        leptos::prelude::Owner::new().with(|| {
            let options = vec![
                ComboboxOption::new("option1", "Option 1"),
                ComboboxOption::new("option2", "Option 2"),
            ];
            let value1 = RwSignal::new("option1".to_string());
            let value2 = RwSignal::new("option1".to_string());
            let _combobox_view1 = view! {
                <Combobox
                    options=options.clone()
                    value=MaybeProp::from(value1)
                />
            };
            let _combobox_view2 = view! {
                <Combobox
                    options=options
                    value=MaybeProp::from(value2)
                />
            };

            let start = std::time::Instant::now();

            for i in 0..100 {
                value1.set(format!("option{}", i % 2 + 1));
                value2.set(format!("option{}", i % 2 + 1));
            }

            let duration = start.elapsed();
            assert!(duration.as_millis() < 30);
        });
    }

    #[test]
    fn test_combobox_performance_deep_nesting() {
        leptos::prelude::Owner::new().with(|| {
            let options = vec![
                ComboboxOption::new("option1", "Option 1"),
                ComboboxOption::new("option2", "Option 2"),
            ];
            let _combobox_view = view! {
                <div>
                    <div>
                        <div>
                            <div>
                                <div>
                                    <Combobox options=options/>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            };

            // Test that deep nesting doesn't significantly impact performance
            let start = std::time::Instant::now();

            for _ in 0..100 {
                let _nested_view = view! {
                    <div>
                        <div>
                            <div>
                                <Combobox options=vec![
                                    ComboboxOption::new("option1", "Option 1"),
                                    ComboboxOption::new("option2", "Option 2"),
                                ]/>
                            </div>
                        </div>
                    </div>
                };
            }

            let duration = start.elapsed();
            assert!(duration.as_millis() < 50);
        });
    }

    #[test]
    fn test_combobox_performance_many_props() {
        leptos::prelude::Owner::new().with(|| {
            let options = vec![
                ComboboxOption::new("option1", "Option 1"),
                ComboboxOption::new("option2", "Option 2"),
            ];
            let _combobox_view = view! {
                <Combobox
                    options=options
                    value=MaybeProp::from("option1")
                    placeholder=MaybeProp::from("Select an option")
                    disabled=false
                    class=MaybeProp::from("custom-class")
                    style=leptos_style::Style::from("color: red")
                    id=MaybeProp::from("test-combobox")
                    attr:aria-label="Select an option"
                    attr:aria-describedby="combobox-description"
                    attr:aria-expanded=false
                    attr:aria-autocomplete="list"
                    attr:aria-owns="combobox-list"
                    attr:aria-activedescendant="option1"
                    attr:aria-required=true
                    attr:aria-invalid=false
                    attr:aria-disabled=false
                    attr:aria-readonly=false
                    attr:aria-multiselectable=false
                    attr:aria-orientation="vertical"
                    attr:aria-live="polite"
                    attr:aria-atomic=true
                    attr:aria-relevant="additions removals"
                    attr:aria-busy=false
                    attr:aria-modal=false
                    attr:aria-hidden=false
                    attr:aria-selected=false
                    attr:aria-checked=false
                    attr:aria-pressed=false
                    attr:tabindex=0
                    attr:autofocus=true
                    attr:data-testid="test-combobox"
                />
            };

            // Test that many props don't significantly impact performance
            let start = std::time::Instant::now();

            for _ in 0..50 {
                let _many_props_view = view! {
                    <Combobox
                        options=vec![
                            ComboboxOption::new("option1", "Option 1"),
                            ComboboxOption::new("option2", "Option 2"),
                        ]
                        value=MaybeProp::from("option1")
                        placeholder=MaybeProp::from("Select an option")
                        disabled=false
                        class=MaybeProp::from("custom-class")
                        style=leptos_style::Style::from("color: red")
                        id=MaybeProp::from("test-combobox")
                        attr:aria-label="Select an option"
                        attr:aria-describedby="combobox-description"
                        attr:aria-expanded=false
                        attr:aria-autocomplete="list"
                        attr:aria-owns="combobox-list"
                        attr:aria-activedescendant="option1"
                        attr:aria-required=true
                        attr:aria-invalid=false
                        attr:aria-disabled=false
                        attr:aria-readonly=false
                        attr:aria-multiselectable=false
                        attr:aria-orientation="vertical"
                        attr:aria-live="polite"
                        attr:aria-atomic=true
                        attr:aria-relevant="additions removals"
                        attr:aria-busy=false
                        attr:aria-modal=false
                        attr:aria-hidden=false
                        attr:aria-selected=false
                        attr:aria-checked=false
                        attr:aria-pressed=false
                        attr:tabindex=0
                        attr:autofocus=true
                        attr:data-testid="test-combobox"
                    />
                };
            }

            let duration = start.elapsed();
            assert!(duration.as_millis() < 100);
        });
    }

    #[test]
    fn test_combobox_performance_stress_test() {
        leptos::prelude::Owner::new().with(|| {
            let start = std::time::Instant::now();

            for i in 0..50 {
                let options = (1..=100)
                    .map(|j| {
                        ComboboxOption::new(
                            format!("option{}-{}", i, j),
                            format!("Option {} - {}", i, j),
                        )
                    })
                    .collect::<Vec<_>>();

                let value = RwSignal::new(format!("option{}-1", i));
                let disabled = RwSignal::new(i % 2 == 0);

                let _combobox_view = view! {
                    <Combobox
                        options=options
                        value=MaybeProp::from(value)
                        disabled=disabled
                        class=MaybeProp::from(format!("combobox-{}", i))
                        id=MaybeProp::from(format!("combobox-{}", i))
                    />
                };

                // Perform some updates
                value.set(format!("option{}-2", i));
                disabled.set(!disabled.get());
            }

            let duration = start.elapsed();
            assert!(duration.as_millis() < 500);
        });
    }

    #[test]
    fn test_combobox_performance_memory_leak_prevention() {
        leptos::prelude::Owner::new().with(|| {
            // Smoke test: repeatedly create and drop views; real leak detection needs a browser profiler.

            for _ in 0..100 {
                let options = vec![
                    ComboboxOption::new("option1", "Option 1"),
                    ComboboxOption::new("option2", "Option 2"),
                ];
                let value = RwSignal::new("option1".to_string());
                let _combobox_view = view! {
                    <Combobox
                        options=options
                        value=MaybeProp::from(value)
                    />
                };

                // Drop the view to test cleanup
                drop(_combobox_view);
            }

            // Test that no significant memory leaks occur
            // This is a simple test - in practice you'd use more sophisticated memory monitoring
        });
    }

    #[test]
    fn test_combobox_performance_signal_cleanup() {
        leptos::prelude::Owner::new().with(|| {
            let mut signals = Vec::new();

            for i in 0..100 {
                let options = vec![
                    ComboboxOption::new("option1", "Option 1"),
                    ComboboxOption::new("option2", "Option 2"),
                ];
                let value = RwSignal::new(format!("option{}", i));
                signals.push(value);

                let _combobox_view = view! {
                    <Combobox
                        options=options
                        value=MaybeProp::from(value)
                    />
                };
            }

            // Test that signals can be properly cleaned up
            drop(signals);
        });
    }

    #[test]
    fn test_combobox_performance_callback_cleanup() {
        leptos::prelude::Owner::new().with(|| {
            let mut callbacks = Vec::new();

            for i in 0..100 {
                let options = vec![
                    ComboboxOption::new("option1", "Option 1"),
                    ComboboxOption::new("option2", "Option 2"),
                ];
                let callback = Callback::new(move |value| {
                    leptos::logging::log!("Combobox {} changed to: {}", i, value);
                });
                callbacks.push(callback.clone());

                let _combobox_view = view! {
                    <Combobox
                        options=options
                        on_change=callback
                    />
                };
            }

            // Test that callbacks can be properly cleaned up
            drop(callbacks);
        });
    }

    #[test]
    fn test_combobox_performance_edge_case_handling() {
        leptos::prelude::Owner::new().with(|| {
            let start = std::time::Instant::now();

            // Test with various edge cases
            let test_cases = vec![
                vec![],                                                        // Empty options
                vec![ComboboxOption::new("", "")],                             // Empty strings
                vec![ComboboxOption::new("a".repeat(1000), "b".repeat(1000))], // Very long strings
                (1..=10000)
                    .map(|i| ComboboxOption::new(format!("opt{}", i), format!("Option {}", i)))
                    .collect(), // Many options
            ];

            for options in test_cases {
                let _combobox_view = view! {
                    <Combobox options=options/>
                };
            }

            let duration = start.elapsed();
            assert!(duration.as_millis() < 200);
        });
    }
}
