//! Leptos port of shadcn/ui checkbox

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::Checkbox;
pub use new_york::Checkbox as CheckboxNewYork;

mod tests;

#[cfg(test)]
mod tdd_tests;

#[cfg(test)]
mod implementation_tests;

// Signal-managed exports
pub use signal_managed::*;
