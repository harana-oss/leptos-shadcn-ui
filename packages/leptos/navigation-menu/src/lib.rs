//! Leptos port of shadcn/ui navigation-menu

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{
    NavigationMenu, NavigationMenuContent, NavigationMenuContext, NavigationMenuItem,
    NavigationMenuItemContext, NavigationMenuLink, NavigationMenuList, NavigationMenuTrigger,
};
pub use new_york::NavigationMenu as NavigationMenuNewYork;

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
