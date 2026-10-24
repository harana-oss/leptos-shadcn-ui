//! Leptos port of shadcn/ui popover

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{Popover, PopoverContent, PopoverContext, PopoverTrigger};
pub use new_york::{
    Popover as PopoverNewYork, PopoverContent as PopoverContentNewYork,
    PopoverTrigger as PopoverTriggerNewYork,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
