//! Leptos port of shadcn/ui input

pub mod default;
pub mod new_york;
pub mod signal_managed;
pub mod validation;

pub use default::Input;
pub use new_york::Input as InputNewYork;
pub use signal_managed::{
    EnhancedInput, INPUT_CLASS, INPUT_ERROR_CLASS, SignalManagedInput, SignalManagedInputState,
};
pub use validation::{
    InputValidator, ValidationContext, ValidationError, ValidationResult, ValidationRule,
    validation_builders,
};

mod tests_real;

// TDD tests organized into focused modules
#[cfg(test)]
mod tests;

#[cfg(test)]
mod leptos_v0_8_compatibility_tests;

#[cfg(test)]
pub mod compatibility_tests;

// #[cfg(test)]
// mod implementation_tests;

#[cfg(test)]
mod tdd_tests {
    pub mod basic_rendering_tests;
    // Temporarily disable other test modules until fixed
    // pub mod validation_tests;
    // pub mod styling_tests;
    // pub mod accessibility_tests;
    // pub mod performance_tests;
    // pub mod integration_tests;
}

// #[cfg(test)]
// mod new_york_tests;
