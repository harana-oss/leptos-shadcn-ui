//! Sonner toast component
//! 
//! This module contains the complete Sonner toast system,
//! organized into focused sub-modules for better maintainability and readability.

pub mod types;
pub mod builder;
pub mod context;
pub mod toast_component;
pub mod api;

// Re-export the main components and types
pub use types::*;
pub use builder::ToastBuilder;
pub use context::{SonnerProvider, SonnerViewport, SonnerContextValue};
pub use toast_component::SonnerToast;
pub use api::toast;
/// Deprecated alias kept for compatibility.
pub use api::toast as api_toast;
