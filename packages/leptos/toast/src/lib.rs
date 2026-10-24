//! Leptos port of shadcn/ui toast

pub mod default;
pub mod new_york;
pub mod signal_managed;
pub mod sonner;

pub use default::Toast;
pub use new_york::Toast as ToastNewYork;
pub use sonner::{
    SonnerProvider, SonnerToast, SonnerViewport, ToastAction, ToastBuilder, ToastData,
    ToastPosition, ToastTheme, ToastVariant, toast,
};

mod tests;

#[cfg(test)]
mod tdd_tests;

mod sonner_tests;

mod sonner_advanced_tests;

// Signal-managed exports
pub use signal_managed::*;
