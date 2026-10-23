//! Component Migration Types Module
//!
//! This module provides the supporting types for component migrations.

use serde::{Deserialize, Serialize};

// Button component types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ButtonState {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub loading: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

// Input component types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InputState {
    pub value: String,
    pub placeholder: String,
    pub disabled: bool,
    pub error: Option<String>,
    pub focused: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationState {
    pub is_valid: bool,
    pub has_error: bool,
    pub error_message: Option<String>,
}

// Card component types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CardState {
    pub title: String,
    pub description: String,
    pub expanded: bool,
    pub loading: bool,
}

// Form component types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FormState {
    pub fields: std::collections::HashMap<String, String>,
    pub is_submitting: bool,
    pub is_valid: bool,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FormValidation {
    pub can_submit: bool,
    pub has_errors: bool,
    pub error_count: usize,
}

// Table component types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableState {
    pub data: Vec<String>,
    pub sort_column: Option<String>,
    pub sort_direction: SortDirection,
    pub selected_rows: std::collections::HashSet<usize>,
    pub page: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

// Dialog component types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DialogState {
    pub is_open: bool,
    pub title: String,
    pub content: String,
    pub can_close: bool,
}

// Navigation component types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NavigationState {
    pub items: Vec<String>,
    pub active_item: Option<String>,
    pub collapsed: bool,
    pub mobile_open: bool,
}

// Toast component types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToastState {
    pub message: String,
    pub variant: ToastVariant,
    pub duration: u64,
    pub is_visible: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ToastVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

// Calendar component types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalendarState {
    pub selected_date: Option<chrono::NaiveDate>,
    pub current_month: chrono::NaiveDate,
    pub events: Vec<String>,
    pub view_mode: CalendarView,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum CalendarView {
    #[default]
    Month,
    Week,
    Day,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalendarData {
    pub month: chrono::NaiveDate,
    pub selected: Option<chrono::NaiveDate>,
    pub event_count: usize,
}

// Default implementations for all state types

impl Default for TableState {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            sort_column: None,
            sort_direction: SortDirection::default(),
            selected_rows: std::collections::HashSet::new(),
            page: 1,
            page_size: 10,
        }
    }
}

impl Default for DialogState {
    fn default() -> Self {
        Self {
            is_open: false,
            title: String::new(),
            content: String::new(),
            can_close: true,
        }
    }
}

impl Default for ToastState {
    fn default() -> Self {
        Self {
            message: String::new(),
            variant: ToastVariant::default(),
            duration: 5000,
            is_visible: false,
        }
    }
}

impl Default for CalendarState {
    fn default() -> Self {
        Self {
            selected_date: None,
            current_month: chrono::Local::now().date_naive(),
            events: Vec::new(),
            view_mode: CalendarView::default(),
        }
    }
}

impl Default for CalendarData {
    fn default() -> Self {
        Self {
            month: chrono::Local::now().date_naive(),
            selected: None,
            event_count: 0,
        }
    }
}
