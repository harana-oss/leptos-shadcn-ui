//! Leptos port of [shadcn/ui Tooltip](https://ui.shadcn.com/docs/components/tooltip).
//!
//! A tooltip component for displaying additional information on hover or focus.
//!
//! See [the Rust shadcn/ui book](https://shadcn-ui.rustforweb.org/components/tooltip.html) for more documentation.

pub mod default;
pub mod new_york;
pub mod signal_managed;

mod tests;

#[cfg(test)]
mod tdd_tests;

// Re-export the components for easy access
pub use default::*;

#[cfg(feature = "new_york")]
pub use new_york as tooltip;

// Signal-managed exports
pub use signal_managed::*;

// Leptos effects spawn on the async executor; native tests need one.
#[cfg(test)]
#[ctor::ctor]
unsafe fn init_test_executor() {
    let _ = any_spawner::Executor::init_futures_executor();
}
