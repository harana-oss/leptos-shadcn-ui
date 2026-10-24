// Component migration module for signal management
// Split from original 541-line file into focused modules

pub mod migration_core;
pub mod component_helpers;
pub mod component_types;
pub mod validation;

pub use migration_core::*;
pub use component_helpers::*;
pub use component_types::*;
pub use validation::*;
