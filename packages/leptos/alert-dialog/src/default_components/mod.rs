//! AlertDialog default components
//!
//! This module contains all the default alert dialog components organized into focused sub-modules
//! for better maintainability and readability.

pub mod action_cancel;
pub mod alert_dialog;
pub mod content;
pub mod header_footer;
pub mod overlay;
pub mod title_description;
pub mod trigger;

// Re-export all components for easy access
pub use action_cancel::{AlertDialogAction, AlertDialogCancel};
pub use alert_dialog::AlertDialog;
pub use content::AlertDialogContent;
pub use header_footer::{AlertDialogFooter, AlertDialogHeader};
pub use overlay::AlertDialogOverlay;
pub use title_description::{AlertDialogDescription, AlertDialogTitle};
pub use trigger::AlertDialogTrigger;
