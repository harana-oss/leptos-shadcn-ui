//! Sonner toast component
//!
//! This module contains the complete Sonner toast system,
//! organized into focused sub-modules for better maintainability and readability.

pub mod api;
pub mod builder;
pub mod context;
pub mod toast_component;
pub mod types;

// Re-export the main components and types
pub use api::toast;
/// Deprecated alias kept for compatibility.
pub use api::toast as api_toast;
pub use builder::ToastBuilder;
pub use context::{SonnerContextValue, SonnerProvider, SonnerViewport};
pub use toast_component::SonnerToast;
pub use types::*;
