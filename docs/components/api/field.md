# Field Component API

> **Stub page.** Generated from `packages/leptos/field/src`; expand with prose and examples as the component matures.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-field = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_field::*;
```

---

## Exported Components

### FieldSet

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `disabled` | `Signal<bool>` | optional |
| `children` | `Option<Children>` | optional |

### FieldLegend

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `variant` | `MaybeProp<String>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### FieldGroup

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### Field

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `invalid` | `Signal<bool>` | optional |
| `disabled` | `Signal<bool>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### FieldContent

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### FieldLabel

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `html_for` | `MaybeProp<String>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### FieldTitle

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### FieldDescription

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### FieldSeparator

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### FieldError

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `errors` | `Signal<Vec<String>>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

---

## Minimal Example

```rust
use leptos::prelude::*;
use leptos_shadcn_field::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <FieldSet>"Content"</FieldSet>
    }
}
```

---

## Source

[packages/leptos/field](https://github.com/harana-oss/leptos-shadcn-ui/tree/main/packages/leptos/field)
