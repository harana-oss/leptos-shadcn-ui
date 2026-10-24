//! Signal-Managed Input Components
//!
//! This module contains signal-managed versions of the Input component with advanced
//! signal management capabilities using leptos-shadcn-signal-management.

pub mod basic_input;
pub mod enhanced_input;
pub mod types;

// Re-export the main components and types
pub use basic_input::SignalManagedInput;
pub use enhanced_input::EnhancedInput;
pub use types::{INPUT_CLASS, INPUT_ERROR_CLASS, SignalManagedInputState};
