# Table Component API

A table component for displaying structured data in rows and columns.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-table = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_table::Table;
```

---

## Import

```rust
use leptos_shadcn_table::Table;
// Data-table API:
use leptos_shadcn_table::{DataTable, DataColumn, DataRow, SortDirection};
```

---

## Component API

> Prop tables are verified against `packages/leptos/table/src`.

The crate exports a single `Table` component (a styled `<table>` wrapper) plus the `DataTable` API (`DataTable`, `DataColumn`, `DataRow`, `DataTableState`, `SortDirection`, `FilterType`, `FilterOperator`, `SelectionMode`, `ExportFormat`). `TableHeader`/`TableBody`/`TableRow`/`TableHead`/`TableCell`/`TableCaption`/`TableFooter` are **not** exported; use plain `<thead>`, `<tbody>`, `<tr>`, `<th>`, `<td>`, `<caption>` elements inside `Table`.

### Table

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

---

## Usage Examples

### Basic Table

```rust
use leptos::prelude::*;
use leptos_shadcn_table::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <Table>
            <caption class="mt-4 text-sm text-muted-foreground">"A list of your recent invoices."</caption>
            <thead>
                <tr class="border-b">
                    <th class="h-12 px-4 text-left font-medium">"Invoice"</th>
                    <th class="h-12 px-4 text-left font-medium">"Status"</th>
                    <th class="h-12 px-4 text-left font-medium">"Amount"</th>
                </tr>
            </thead>
            <tbody>
                <tr class="border-b">
                    <td class="p-4">"INV001"</td>
                    <td class="p-4">"Paid"</td>
                    <td class="p-4">"$250.00"</td>
                </tr>
                <tr class="border-b">
                    <td class="p-4">"INV002"</td>
                    <td class="p-4">"Pending"</td>
                    <td class="p-4">"$150.00"</td>
                </tr>
            </tbody>
        </Table>
    }
}
```

---

## CSS Classes

```css
.table {
    w-full caption-bottom text-sm
}

.table-header {
    border-b
}

.table-body {
    &[data-truncate=true] tr {
        @apply last:border-b;
    }
}

.table-footer {
    border-t bg-muted/50 font-medium
}

.table-row {
    border-b transition-colors hover:bg-muted/50
}

.table-head {
    h-10 px-2 text-left align-middle font-medium
    text-muted-foreground [&:has([role=checkbox])]:pr-0
}

.table-cell {
    p-2 align-middle [&:has([role=checkbox])]:pr-0
}

.table-caption {
    mt-4 text-sm text-muted-foreground
}
```

---

## Accessibility

### ARIA Attributes

- `role="table"` - Table role
- `role="row"` - Row role
- `role="columnheader"` - Header cell role
- `role="cell"` - Data cell role
- Scope attributes for headers

---

## TypeScript API

```typescript
interface TableProps {
  className?: string;
  children?: React.ReactNode;
}

export const Table: React.FC<TableProps>;
```

---

*Source: [packages/leptos/table/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/table/src/default.rs)*
