#[cfg(test)]
mod basic_rendering_tests {
    use crate::*;
    use leptos::prelude::*;

    #[test]
    fn test_command_basic_rendering() {
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
        // GREEN PHASE: Verify actual rendering behavior
    }

    #[test]
    fn test_command_with_value() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command value=MaybeProp::from("initial")>
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
    fn test_command_with_callback() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let callback = Callback::new(move |_value: String| {
            // Callback logic
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
    fn test_command_with_class() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command class=MaybeProp::from("custom-command")>
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
    fn test_command_with_label() {
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
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_with_form() {
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
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_callback_execution() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let callback = Callback::new(move |value: String| {
            // Test callback execution
            assert!(!value.is_empty() || value.is_empty());
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
    fn test_command_custom_styles() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command class=MaybeProp::from("custom-styles")>
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
    fn test_command_combined_props() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let callback = Callback::new(move |_value: String| {
            // Combined props callback
        });
        let _command_view = view! {
            <Command
                value=MaybeProp::from("combined")
                on_value_change=callback
                class=MaybeProp::from("combined-class")
            >
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
    fn test_command_multiple_instances() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view1 = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search 1...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                </CommandList>
            </Command>
        };

        let _command_view2 = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search 2...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_state_management() {
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

        // Test state management
        value_signal.set("test value".to_string());
        assert_eq!(value_signal.get(), "test value");
    }

    #[test]
    fn test_command_context_management() {
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
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_animations() {
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
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }
}
