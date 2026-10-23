//! Leptos port of shadcn/ui progress

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{Progress, ProgressIndicator, ProgressLabel, ProgressRoot, ProgressVariant};
pub use new_york::{
    Progress as ProgressNewYork, ProgressIndicator as ProgressIndicatorNewYork,
    ProgressLabel as ProgressLabelNewYork, ProgressRoot as ProgressRootNewYork,
    ProgressVariant as ProgressVariantNewYork,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
