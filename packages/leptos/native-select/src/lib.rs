//! Leptos port of shadcn/ui native-select (v4).
//!
//! A styled native `<select>` with a chevron icon, plus `NativeSelectOption`
//! and `NativeSelectOptGroup`.

pub mod default;
pub mod new_york;

pub use default::*;

#[cfg(test)]
mod tests;
