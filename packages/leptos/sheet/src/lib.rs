//! Leptos port of shadcn/ui sheet
//!
//! Compound API: `Sheet`, `SheetTrigger`, `SheetContent`, `SheetHeader`,
//! `SheetFooter`, `SheetTitle`, `SheetDescription`, `SheetClose`.
//!
//! Follow-up: portal rendering is not supported yet; `SheetContent` renders
//! in place as a fixed-position overlay.

pub mod default;
pub mod new_york;
pub mod sheet_core;
pub mod signal_managed;

pub use crate::sheet_core::{SheetContext, SheetSide, SheetStyles};
pub use default::{
    Sheet, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetTitle,
    SheetTrigger,
};
pub use new_york::{
    Sheet as SheetNewYork, SheetClose as SheetCloseNewYork, SheetContent as SheetContentNewYork,
    SheetDescription as SheetDescriptionNewYork, SheetFooter as SheetFooterNewYork,
    SheetHeader as SheetHeaderNewYork, SheetTitle as SheetTitleNewYork,
    SheetTrigger as SheetTriggerNewYork,
};

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
