//! Leptos port of shadcn/ui dialog

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{
    Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
pub use new_york::{
    Dialog as DialogNewYork, DialogClose as DialogCloseNewYork,
    DialogContent as DialogContentNewYork, DialogDescription as DialogDescriptionNewYork,
    DialogFooter as DialogFooterNewYork, DialogHeader as DialogHeaderNewYork,
    DialogTitle as DialogTitleNewYork, DialogTrigger as DialogTriggerNewYork,
};

mod tests;

// Signal-managed exports
pub use signal_managed::*;
