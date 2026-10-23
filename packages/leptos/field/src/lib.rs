//! Leptos port of shadcn/ui field (v4).
//!
//! Composable primitives for building accessible form fields: `Field`, `FieldSet`,
//! `FieldLegend`, `FieldGroup`, `FieldContent`, `FieldLabel`, `FieldTitle`,
//! `FieldDescription`, `FieldError` and `FieldSeparator`.

pub mod default;
pub mod new_york;

pub use default::*;

#[cfg(test)]
mod tests;
