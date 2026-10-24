//! Leptos port of shadcn/ui drawer

pub mod default;
pub mod default_components;
pub mod new_york;
pub mod signal_managed;

pub use default::{
    Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerDirection, DrawerFooter,
    DrawerHeader, DrawerNestedRoot, DrawerOverlay, DrawerPortal, DrawerTitle, DrawerTrigger,
};

pub use new_york::{
    Drawer as DrawerNewYork, DrawerClose as DrawerCloseNewYork,
    DrawerContent as DrawerContentNewYork, DrawerDescription as DrawerDescriptionNewYork,
    DrawerDirection as DrawerDirectionNewYork, DrawerFooter as DrawerFooterNewYork,
    DrawerHeader as DrawerHeaderNewYork, DrawerNestedRoot as DrawerNestedRootNewYork,
    DrawerOverlay as DrawerOverlayNewYork, DrawerPortal as DrawerPortalNewYork,
    DrawerTitle as DrawerTitleNewYork, DrawerTrigger as DrawerTriggerNewYork,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
