//! Leptos port of shadcn/ui switch

pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use default::{Switch, SwitchLabel, SwitchRoot, SwitchSize, SwitchThumb, SwitchVariant};
pub use new_york::{
    Switch as SwitchNewYork, SwitchLabel as SwitchLabelNewYork, SwitchRoot as SwitchRootNewYork,
    SwitchSize as SwitchSizeNewYork, SwitchThumb as SwitchThumbNewYork,
    SwitchVariant as SwitchVariantNewYork,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

pub mod implementation_tests;

// Signal-managed exports
pub use signal_managed::*;

// Leptos effects spawn on the async executor; native tests need one.
#[cfg(test)]
#[ctor::ctor]
unsafe fn init_test_executor() {
    let _ = any_spawner::Executor::init_futures_executor();
}
