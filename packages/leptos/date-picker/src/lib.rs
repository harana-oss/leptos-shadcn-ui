#[cfg(feature = "new_york")]
pub use new_york::*;

#[cfg(not(feature = "new_york"))]
pub use default::*;

#[cfg(feature = "new_york")]
mod new_york;

#[cfg(not(feature = "new_york"))]
mod default;

pub mod signal_managed;

mod tests;

#[cfg(test)]
mod tdd_tests;

#[cfg(test)]
mod advanced_date_picker_tests;
#[cfg(test)]
pub mod advanced_tests;

// Signal-managed exports
pub use signal_managed::*;

// Leptos effects spawn on the async executor; native tests need one.
#[cfg(test)]
#[ctor::ctor]
unsafe fn init_test_executor() {
    let _ = any_spawner::Executor::init_futures_executor();
}
