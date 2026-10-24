//! Leptos port of shadcn/ui collapsible

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{Collapsible, CollapsibleContent, CollapsibleTrigger};

pub use new_york::{
    Collapsible as CollapsibleNewYork, CollapsibleContent as CollapsibleContentNewYork,
    CollapsibleTrigger as CollapsibleTriggerNewYork,
};

mod tests;

// Signal-managed exports
pub use signal_managed::*;
