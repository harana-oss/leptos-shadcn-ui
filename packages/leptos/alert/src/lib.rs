//! Leptos port of shadcn/ui alert

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{Alert, AlertDescription, AlertTitle, AlertVariant};
pub use new_york::{
    Alert as AlertNewYork, AlertDescription as AlertDescriptionNewYork,
    AlertTitle as AlertTitleNewYork, AlertVariant as AlertVariantNewYork,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
