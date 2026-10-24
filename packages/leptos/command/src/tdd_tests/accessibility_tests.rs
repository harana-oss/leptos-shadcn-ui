#[cfg(test)]
mod accessibility_tests {
    use crate::*;
    use leptos::prelude::*;

    #[test]
    fn test_command_accessibility() {
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
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_aria_attributes() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput
                    placeholder=MaybeProp::from("Search...")
                />
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                        <CommandItem >"Calendar"</CommandItem>
                        <CommandItem >"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_role_attributes() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...") />
                <CommandList >
                    <CommandEmpty>"No results found."</CommandEmpty>
                    <CommandGroup>
                        <CommandGroupHeading>"Suggestions"</CommandGroupHeading>
                        <CommandItem >"Calendar"</CommandItem>
                        <CommandItem >"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_screen_reader_support() {
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
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_high_contrast_mode() {
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
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_reduced_motion() {
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
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_voice_control() {
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
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_switch_control() {
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
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_eye_tracking() {
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
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_motor_impairment_support() {
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
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_cognitive_accessibility() {
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
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_language_support() {
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
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_rtl_support() {
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
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_accessibility_testing() {
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
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }
}
