//! Leptos port of shadcn/ui table

pub mod data_table;
pub mod default;
pub mod new_york;
pub mod signal_managed;

pub use data_table::{
    DataColumn, DataRow, DataTable, DataTableState, ExportFormat, FilterOperator, FilterType,
    SelectionMode, SortDirection,
};
pub use default::Table;
pub use new_york::Table as TableNewYork;

mod tests;

mod tdd_tests;

mod data_table_tests;

// Signal-managed exports
pub use signal_managed::*;
