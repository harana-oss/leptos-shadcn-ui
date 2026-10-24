//! Leptos port of shadcn/ui Form component
//!
//! Provides form building blocks with validation and accessibility features.

pub mod default;
pub mod new_york;
pub mod signal_managed;

// Re-export common types
pub use default::{
    Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage,
};

mod tests;

mod implementation_tests;

// Signal-managed exports
pub use signal_managed::*;
