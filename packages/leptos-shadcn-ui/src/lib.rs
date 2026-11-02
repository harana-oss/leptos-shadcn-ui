//! # Leptos ShadCN UI
//!
//! A comprehensive collection of beautiful, accessible UI components built for
//! [Leptos](https://leptos.dev/) 0.9, inspired by [shadcn/ui](https://ui.shadcn.com/).
//!
//! ## Features
//!
//! - **60 component crates**: every component crate in the workspace is re-exported here
//! - **Leptos 0.9**: built against Leptos `0.9.0-rc0`
//! - **Accessibility First**: components follow WAI-ARIA authoring practices
//! - **Tailwind CSS**: seamless integration with Tailwind CSS
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! leptos-shadcn-ui = { git = "https://github.com/harana-oss/leptos-shadcn-ui", features = ["button", "input", "card"] }
//! ```
//!
//! Every component whose feature is enabled is available at the crate root
//! (`use leptos_shadcn_ui::Button;`) and through the prelude
//! (`use leptos_shadcn_ui::prelude::*;`). The `all-components` feature (on by default)
//! enables all of them.
//!
//! ## License
//!
//! MIT License - see the [LICENSE](../LICENSE) file for details.

// Several component crates export overlapping helper names (e.g. `signal_managed` items);
// the component names themselves are unique and are checked by `tests/reexports.rs`.
#![allow(ambiguous_glob_reexports)]

// Component re-exports, one per feature. Keep in sync with `all-components` in Cargo.toml.
#[cfg(feature = "accordion")]
pub use leptos_shadcn_accordion::default::*;
#[cfg(feature = "alert")]
pub use leptos_shadcn_alert::default::*;
#[cfg(feature = "alert-dialog")]
pub use leptos_shadcn_alert_dialog::default::*;
#[cfg(feature = "aspect-ratio")]
pub use leptos_shadcn_aspect_ratio::default::*;
#[cfg(feature = "avatar")]
pub use leptos_shadcn_avatar::default::*;
#[cfg(feature = "badge")]
pub use leptos_shadcn_badge::default::*;
#[cfg(feature = "breadcrumb")]
pub use leptos_shadcn_breadcrumb::default::*;
#[cfg(feature = "button")]
pub use leptos_shadcn_button::default::*;
#[cfg(feature = "button-group")]
pub use leptos_shadcn_button_group::default::*;
#[cfg(feature = "calendar")]
pub use leptos_shadcn_calendar::default::*;
#[cfg(feature = "card")]
pub use leptos_shadcn_card::default::*;
#[cfg(feature = "carousel")]
pub use leptos_shadcn_carousel::default::*;
#[cfg(feature = "chart")]
pub use leptos_shadcn_chart::*;
#[cfg(feature = "checkbox")]
pub use leptos_shadcn_checkbox::default::*;
#[cfg(feature = "collapsible")]
pub use leptos_shadcn_collapsible::default::*;
#[cfg(feature = "combobox")]
pub use leptos_shadcn_combobox::default::*;
#[cfg(feature = "command")]
pub use leptos_shadcn_command::default::*;
#[cfg(feature = "context-menu")]
pub use leptos_shadcn_context_menu::default::*;
#[cfg(feature = "date-picker")]
pub use leptos_shadcn_date_picker::default::*;
#[cfg(feature = "dialog")]
pub use leptos_shadcn_dialog::default::*;
#[cfg(feature = "drawer")]
pub use leptos_shadcn_drawer::default::*;
#[cfg(feature = "dropdown-menu")]
pub use leptos_shadcn_dropdown_menu::default::*;
#[cfg(feature = "empty")]
pub use leptos_shadcn_empty::default::*;
#[cfg(feature = "error-boundary")]
pub use leptos_shadcn_error_boundary::default::*;
#[cfg(feature = "field")]
pub use leptos_shadcn_field::default::*;
#[cfg(feature = "form")]
pub use leptos_shadcn_form::default::*;
#[cfg(feature = "hover-card")]
pub use leptos_shadcn_hover_card::default::*;
#[cfg(feature = "input")]
pub use leptos_shadcn_input::default::*;
#[cfg(feature = "input-group")]
pub use leptos_shadcn_input_group::default::*;
#[cfg(feature = "input-otp")]
pub use leptos_shadcn_input_otp::*;
#[cfg(feature = "item")]
pub use leptos_shadcn_item::default::*;
#[cfg(feature = "kbd")]
pub use leptos_shadcn_kbd::default::*;
#[cfg(feature = "label")]
pub use leptos_shadcn_label::default::*;
#[cfg(feature = "menubar")]
pub use leptos_shadcn_menubar::default::*;
#[cfg(feature = "native-select")]
pub use leptos_shadcn_native_select::default::*;
#[cfg(feature = "navigation-menu")]
pub use leptos_shadcn_navigation_menu::default::*;
#[cfg(feature = "pagination")]
pub use leptos_shadcn_pagination::default::*;
#[cfg(feature = "popover")]
pub use leptos_shadcn_popover::default::*;
#[cfg(feature = "progress")]
pub use leptos_shadcn_progress::default::*;
#[cfg(feature = "radio-group")]
pub use leptos_shadcn_radio_group::default::*;
#[cfg(feature = "resizable")]
pub use leptos_shadcn_resizable::default::*;
#[cfg(feature = "scroll-area")]
pub use leptos_shadcn_scroll_area::default::*;
#[cfg(feature = "select")]
pub use leptos_shadcn_select::default::*;
#[cfg(feature = "separator")]
pub use leptos_shadcn_separator::default::*;
#[cfg(feature = "sheet")]
pub use leptos_shadcn_sheet::default::*;
#[cfg(feature = "sidebar")]
pub use leptos_shadcn_sidebar::*;
#[cfg(feature = "skeleton")]
pub use leptos_shadcn_skeleton::default::*;
#[cfg(feature = "slider")]
pub use leptos_shadcn_slider::default::*;
#[cfg(feature = "sonner")]
pub use leptos_shadcn_sonner::*;
#[cfg(feature = "spinner")]
pub use leptos_shadcn_spinner::default::*;
#[cfg(feature = "switch")]
pub use leptos_shadcn_switch::default::*;
#[cfg(feature = "table")]
pub use leptos_shadcn_table::default::*;
#[cfg(feature = "tabs")]
pub use leptos_shadcn_tabs::default::*;
#[cfg(feature = "textarea")]
pub use leptos_shadcn_textarea::default::*;
#[cfg(feature = "toast")]
pub use leptos_shadcn_toast::default::*;
#[cfg(feature = "toggle")]
pub use leptos_shadcn_toggle::default::*;
#[cfg(feature = "toggle-group")]
pub use leptos_shadcn_toggle_group::default::*;
#[cfg(feature = "tooltip")]
pub use leptos_shadcn_tooltip::default::*;
#[cfg(feature = "typography")]
pub use leptos_shadcn_typography::default::*;

// Non-visual / infrastructure crates
#[cfg(feature = "analytics")]
pub use leptos_shadcn_analytics::*;
#[cfg(feature = "lazy-loading")]
pub use leptos_shadcn_lazy_loading::*;
#[cfg(feature = "performance-audit")]
pub use leptos_shadcn_performance_audit as performance_audit;
#[cfg(feature = "registry")]
pub use leptos_shadcn_registry::*;

// Re-export common types and utilities
pub use tailwind_fuse::tw_merge;

pub mod prelude;
