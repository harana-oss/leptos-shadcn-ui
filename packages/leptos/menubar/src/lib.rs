//! Leptos port of shadcn/ui menubar

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarMenuContext, MenubarTrigger,
};
pub use new_york::Menubar as MenubarNewYork;

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
