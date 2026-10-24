//! # Leptos ShadCN Signal Management
//!
//! Signal lifecycle management utilities for Leptos 0.8.8+ with tailwind-rs integration.
//!
//! This crate provides utilities for managing signal lifecycles, implementing batched updates,
//! and ensuring proper memory management in Leptos applications.
// SignalManagementError is public API; boxing its context would be a breaking change.
#![allow(clippy::result_large_err)]

pub mod advanced_memory;
pub mod batched_updates;
pub mod component_migration;
pub mod error;
pub mod lifecycle;
pub mod memory_management;

pub use advanced_memory::*;
pub use batched_updates::*;
pub use component_migration::*;
pub use error::*;
pub use lifecycle::*;
pub use memory_management::*;

// Re-export commonly used types for convenience
// Note: We don't re-export leptos::prelude::* to avoid conflicts with standard Leptos types

// Include test modules
#[cfg(test)]
mod simple_tests;

#[cfg(test)]
mod signal_management_tests;

#[cfg(test)]
mod lifecycle_tests;

#[cfg(test)]
mod memory_management_tests;
