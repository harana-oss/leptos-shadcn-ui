//! Leptos port of shadcn/ui input-group (v4).
//!
//! `InputGroup` wraps an input or textarea with addons (icons, text, buttons)
//! aligned inline or as block rows.

pub mod default;
pub mod new_york;

pub use default::*;

#[cfg(test)]
mod tests;
