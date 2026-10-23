//! Data table component for the Table package
//!
//! This module contains a comprehensive data table component with sorting,
//! filtering, pagination, and selection capabilities.

pub mod component;
pub mod types;

// Re-export main types and components
pub use component::DataTable;
pub use types::*;
