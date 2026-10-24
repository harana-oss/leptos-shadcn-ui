//! Drawer component types and enums
//!
//! This module contains the type definitions and enums used by the Drawer component.

#[derive(Debug, Clone, PartialEq, Default)]
pub enum DrawerDirection {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}
