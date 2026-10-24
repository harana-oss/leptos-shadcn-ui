//! Leptos port of shadcn/ui context menu

pub mod default;
pub mod default_components;
pub mod global;
pub mod new_york;
pub mod signal_managed;

pub use default::{
    ContextMenu, ContextMenuCheckboxItem, ContextMenuContent, ContextMenuItem, ContextMenuLabel,
    ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuSeparator, ContextMenuShortcut,
    ContextMenuSub, ContextMenuSubContent, ContextMenuSubTrigger, ContextMenuTrigger,
};

pub use new_york::{
    ContextMenu as ContextMenuNewYork, ContextMenuCheckboxItem as ContextMenuCheckboxItemNewYork,
    ContextMenuContent as ContextMenuContentNewYork, ContextMenuItem as ContextMenuItemNewYork,
    ContextMenuLabel as ContextMenuLabelNewYork,
    ContextMenuRadioGroup as ContextMenuRadioGroupNewYork,
    ContextMenuRadioItem as ContextMenuRadioItemNewYork,
    ContextMenuSeparator as ContextMenuSeparatorNewYork,
    ContextMenuShortcut as ContextMenuShortcutNewYork, ContextMenuSub as ContextMenuSubNewYork,
    ContextMenuSubContent as ContextMenuSubContentNewYork,
    ContextMenuSubTrigger as ContextMenuSubTriggerNewYork,
    ContextMenuTrigger as ContextMenuTriggerNewYork,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;

// Leptos effects spawn on the async executor; native tests need one.
#[cfg(test)]
#[ctor::ctor]
unsafe fn init_test_executor() {
    let _ = any_spawner::Executor::init_futures_executor();
}
