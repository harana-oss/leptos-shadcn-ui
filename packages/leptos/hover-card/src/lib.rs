//! Leptos port of shadcn/ui hover-card

pub mod signal_managed;
pub mod default;
pub mod new_york;

pub use default::{HoverCard, HoverCardContent, HoverCardContext, HoverCardTrigger};
pub use new_york::{
    HoverCard as HoverCardNewYork, HoverCardContent as HoverCardContentNewYork,
    HoverCardTrigger as HoverCardTriggerNewYork,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
