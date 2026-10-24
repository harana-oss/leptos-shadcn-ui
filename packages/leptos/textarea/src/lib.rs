//! Leptos port of shadcn/ui textarea

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::Textarea;
pub use new_york::Textarea as TextareaNewYork;

mod tests;

#[cfg(test)]
mod tdd_tests;

mod implementation_tests;

// Signal-managed exports
pub use signal_managed::*;
