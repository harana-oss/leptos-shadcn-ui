//! Leptos port of shadcn/ui Combobox component
//!
//! Provides an autocomplete input component with a list of suggestions.

pub mod default;
pub mod new_york;
pub mod signal_managed;

// Re-export common types
pub use default::{Combobox, ComboboxOption};

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;

// Leptos effects spawn on the async executor; native tests need one.
#[cfg(test)]
#[ctor::ctor]
unsafe fn init_test_executor() {
    let _ = any_spawner::Executor::init_futures_executor();
}
