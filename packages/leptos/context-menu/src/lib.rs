//! Leptos port of shadcn/ui context menu

pub mod signal_managed;
pub mod default;
pub mod new_york;
pub mod default_components;
pub mod global;

pub use default::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
    ContextMenuSeparator, ContextMenuLabel, ContextMenuCheckboxItem,
    ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuSub,
    ContextMenuSubContent, ContextMenuSubTrigger, ContextMenuShortcut,
};

pub use new_york::{
    ContextMenu as ContextMenuNewYork,
    ContextMenuContent as ContextMenuContentNewYork,
    ContextMenuItem as ContextMenuItemNewYork,
    ContextMenuTrigger as ContextMenuTriggerNewYork,
    ContextMenuSeparator as ContextMenuSeparatorNewYork,
    ContextMenuLabel as ContextMenuLabelNewYork,
    ContextMenuCheckboxItem as ContextMenuCheckboxItemNewYork,
    ContextMenuRadioGroup as ContextMenuRadioGroupNewYork,
    ContextMenuRadioItem as ContextMenuRadioItemNewYork,
    ContextMenuSub as ContextMenuSubNewYork,
    ContextMenuSubContent as ContextMenuSubContentNewYork,
    ContextMenuSubTrigger as ContextMenuSubTriggerNewYork,
    ContextMenuShortcut as ContextMenuShortcutNewYork,
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
