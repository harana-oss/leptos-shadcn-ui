//! Leptos port of shadcn/ui toggle

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::Toggle;
pub use new_york::Toggle as ToggleNewYork;

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
