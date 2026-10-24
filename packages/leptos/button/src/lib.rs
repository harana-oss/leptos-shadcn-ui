//! Leptos port of shadcn/ui button

pub mod default;
pub mod new_york;
pub mod signal_managed;
// TODO: Enable when API standards crate is ready for v1.0
// pub mod standardized;

pub use default::{
    BUTTON_TOUCH_CLASS, Button, ButtonChildProps, ButtonResponsiveSize, ButtonSize, ButtonVariant,
};
pub use new_york::{
    Button as ButtonNewYork, ButtonChildProps as ButtonChildPropsNewYork,
    ButtonSize as ButtonSizeNewYork, ButtonVariant as ButtonVariantNewYork,
};
pub use signal_managed::{
    EnhancedButton, SignalManagedButton, SignalManagedButtonChildProps, SignalManagedButtonState,
};
// TODO: Enable when API standards crate is ready for v1.0
// pub use standardized::{StandardizedButton, StandardizedButtonProps};

// #[cfg(test)]
// mod tests;

// #[cfg(test)]

// Real working tests (replaces placeholders)

mod tests_simple;

// Keep legacy tests for now (will phase out)

#[cfg(test)]
mod tdd_tests;

mod implementation_tests;

mod new_york_tests;

mod variant_comparison_tests;
