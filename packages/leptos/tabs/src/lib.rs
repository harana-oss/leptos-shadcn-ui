//! Leptos port of shadcn/ui tabs

pub mod default;
pub mod keyboard;
pub mod new_york;
pub mod signal_managed;

pub use default::{Tabs, TabsContent, TabsList, TabsTrigger};
pub use keyboard::Orientation as TabsOrientation;
pub use new_york::{
    Tabs as TabsNewYork, TabsContent as TabsContentNewYork, TabsList as TabsListNewYork,
    TabsTrigger as TabsTriggerNewYork,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

// Signal-managed exports
pub use signal_managed::*;
