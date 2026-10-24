//! Leptos port of shadcn/ui alert dialog

pub mod default;
pub mod default_components;
pub mod new_york;
pub mod signal_managed;

pub use default::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogOverlay, AlertDialogTitle, AlertDialogTrigger,
};

pub use new_york::{
    AlertDialog as AlertDialogNewYork, AlertDialogAction as AlertDialogActionNewYork,
    AlertDialogCancel as AlertDialogCancelNewYork, AlertDialogContent as AlertDialogContentNewYork,
    AlertDialogDescription as AlertDialogDescriptionNewYork,
    AlertDialogFooter as AlertDialogFooterNewYork, AlertDialogHeader as AlertDialogHeaderNewYork,
    AlertDialogOverlay as AlertDialogOverlayNewYork, AlertDialogTitle as AlertDialogTitleNewYork,
    AlertDialogTrigger as AlertDialogTriggerNewYork,
};

mod tests;

// Signal-managed exports
pub use signal_managed::*;
