# Date Picker Component API

> **Stub page.** Generated from `packages/leptos/date-picker/src`; expand with prose and examples as the component matures.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-date-picker = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_date_picker::*;
```

---

## Exported Components

### DatePicker

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `selected` | `MaybeProp<CalendarDate>` | optional |
| `on_select` | `Option<Callback<CalendarDate>>` | optional |
| `disabled` | `MaybeProp<Vec<CalendarDate>>` | optional |
| `placeholder` | `MaybeProp<String>` | optional |
| `class` | `MaybeProp<String>` | optional |

### DatePickerWithRange

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `from` | `MaybeProp<CalendarDate>` | optional |
| `to` | `MaybeProp<CalendarDate>` | optional |
| `on_select` | `Option<Callback<(Option<CalendarDate>, Option<CalendarDate>)>>` | optional |
| `disabled` | `MaybeProp<Vec<CalendarDate>>` | optional |
| `placeholder` | `MaybeProp<String>` | optional |
| `class` | `MaybeProp<String>` | optional |

---

## Minimal Example

```rust
use leptos::prelude::*;
use leptos_shadcn_date_picker::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <DatePicker />
    }
}
```

---

## Source

[packages/leptos/date-picker](https://github.com/harana-oss/leptos-shadcn-ui/tree/main/packages/leptos/date-picker)
