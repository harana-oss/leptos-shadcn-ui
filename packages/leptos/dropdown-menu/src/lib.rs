//! Leptos port of shadcn/ui dropdown-menu

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{
    DropdownMenu, DropdownMenuContent, DropdownMenuContext, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuShortcut, DropdownMenuTrigger,
};
pub use new_york::DropdownMenu as DropdownMenuNewYork;

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
