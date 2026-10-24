//! Drawer default components
//!
//! This module contains all the default drawer components organized into focused sub-modules
//! for better maintainability and readability.

pub mod close;
pub mod content;
pub mod drawer;
pub mod header_footer;
pub mod nested;
pub mod portal_overlay;
pub mod title_description;
pub mod trigger;
pub mod types;

// Re-export all components and types for easy access
pub use close::{DrawerClose, DrawerCloseChildProps};
pub use content::DrawerContent;
pub use drawer::Drawer;
pub use header_footer::{DrawerFooter, DrawerHeader};
pub use nested::DrawerNestedRoot;
pub use portal_overlay::{DrawerOverlay, DrawerPortal};
pub use title_description::{DrawerDescription, DrawerTitle};
pub use trigger::{DrawerTrigger, DrawerTriggerChildProps};
pub use types::*;
