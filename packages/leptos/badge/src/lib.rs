//! Leptos port of shadcn/ui badge

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{Badge, BadgeVariant};
pub use new_york::{Badge as BadgeNewYork, BadgeVariant as BadgeVariantNewYork};

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
