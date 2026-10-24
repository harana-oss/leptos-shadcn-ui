#![allow(clippy::module_inception)] // test layout: one inner module per test file
#[cfg(test)]
mod component_tests {
    use crate::*;
    use leptos::prelude::*;

    #[test]
    fn test_command_input_basic() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_input_with_placeholder() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Enter search term...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_list_basic() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_list_with_items() {
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
    fn test_command_empty() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_empty_custom_message() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"Custom empty message"</CommandEmpty>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_group_basic() {
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
                        <CommandItem>"Calendar"</CommandItem>
                        <CommandItem>"Search Emoji"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_group_with_heading() {
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
    fn test_command_group_multiple() {
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
                    <CommandGroup>
                        <CommandGroupHeading>"Recent"</CommandGroupHeading>
                        <CommandItem>"Recent Item 1"</CommandItem>
                        <CommandItem>"Recent Item 2"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_item_basic() {
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
    fn test_command_item_with_shortcut() {
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
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_item_disabled() {
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
                        <CommandItem >"Disabled Item"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_shortcut() {
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
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_separator() {
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
                    <CommandSeparator />
                    <CommandGroup>
                        <CommandGroupHeading>"Recent"</CommandGroupHeading>
                        <CommandItem>"Recent Item"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_complex_structure() {
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
                        <CommandItem>"Search Emoji"</CommandItem>
                        <CommandItem >"Disabled Item"</CommandItem>
                    </CommandGroup>
                    <CommandSeparator />
                    <CommandGroup>
                        <CommandGroupHeading>"Recent"</CommandGroupHeading>
                        <CommandItem>"Recent Item 1"</CommandItem>
                        <CommandItem>"Recent Item 2"</CommandItem>
                    </CommandGroup>
                </CommandList>
            </Command>
        };
    }

    #[test]
    fn test_command_empty_list() {
        // Native tests need an executor for effects and an owner for context.
        let _ = any_spawner::Executor::init_futures_executor();
        let _owner = Owner::new();
        _owner.set();
        let _command_view = view! {
            <Command>
                <CommandInput placeholder=MaybeProp::from("Search...")/>
                <CommandList>
                    <CommandEmpty>"No results found."</CommandEmpty>
                </CommandList>
            </Command>
        };
    }
}
