//! Leptos port of shadcn/ui scroll-area

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::ScrollArea;
pub use new_york::ScrollArea as ScrollAreaNewYork;

mod tests;

// Signal-managed exports
pub use signal_managed::*;
