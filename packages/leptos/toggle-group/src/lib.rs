//! Leptos port of shadcn/ui toggle-group (v4).
//!
//! `ToggleGroup` provides selection context (single or multiple) to its
//! `ToggleGroupItem` children. Selection logic lives in pure functions in
//! [`default`] so it can be unit tested natively.

pub mod default;
pub mod new_york;

pub use default::*;

#[cfg(test)]
mod tests;
