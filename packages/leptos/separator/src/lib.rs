//! Leptos port of shadcn/ui separator

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::Separator;
pub use new_york::Separator as SeparatorNewYork;

mod tests;

// Signal-managed exports
pub use signal_managed::*;
