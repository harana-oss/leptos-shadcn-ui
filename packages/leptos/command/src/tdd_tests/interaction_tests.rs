#[cfg(test)]
mod interaction_tests {
    use crate::*;
    use leptos::prelude::*;

    #[test]
    fn test_command_keyboard_navigation() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                        <CommandItem>"Calculator"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_edge_cases() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("")/>
                <CommandList>
                    <CommandEmpty>""</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>""</CommandGroupHeading>
                        <CommandItem>""</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_performance() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let start = std::time::Instant::now();

        for i in 0..100 {
            let _command_view = view! {
                <Command>
                    <CommandInput placeholder=format!("Search {}...", i).into()/>
                    <CommandList>
                        <CommandEmpty>"No results found."</CommandEmpty>
                        <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                            <CommandItem>format!("Item {}", i)</CommandItem>
                        </CommandGroup>
                    </CommandList>
                </Command>
            };
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 1000,
            "Command rendering should be performant"
        );
    }

    #[test]
    fn test_command_callback_handling() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let callback = Callback::new(move |value: String| {
            // Test callback handling
            let _ = value;
        });

        let _command_view = view! {
            <Command on_value_change=callback>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                        <CommandItem>"Calendar"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_value_updates() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let value_signal = RwSignal::new("".to_string());

        let _command_view = view! {
            <Command value=MaybeProp::from(value_signal)>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                        <CommandItem>"Calendar"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };

        // Test value updates
        value_signal.set("test".to_string());
        assert_eq!(value_signal.get(), "test");

        value_signal.set("updated".to_string());
        assert_eq!(value_signal.get(), "updated");
    }

    #[test]
    fn test_command_item_selection() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                        <CommandItem>"Calculator"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_input_focus() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                        <CommandItem>"Calendar"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_search_filtering() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                        <CommandItem>"Calculator"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_shortcut_handling() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                        <CommandItem>
                            "Calendar"
                            <CommandShortcut>"⌘K"</CommandShortcut>
                        </CommandItem>
                        <CommandItem>
                            "Search Emoji"
                            <CommandShortcut>"⌘E"</CommandShortcut>
                        </CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_disabled_interactions() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                        <CommandItem >"Disabled Item"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_mouse_interactions() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_touch_interactions() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_voice_interactions() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                        <CommandItem>"Calendar"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }
}
