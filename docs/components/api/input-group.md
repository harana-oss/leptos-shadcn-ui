# Input Group Component API

> **Stub page.** Generated from `packages/leptos/input-group/src`; expand with prose and examples as the component matures.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-input-group = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_input_group::*;
```

---

## Exported Components

### InputGroup

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `disabled` | `Signal<bool>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### InputGroupAddon

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### InputGroupButton

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `variant` | `MaybeProp<String>` | optional |
| `button_type` | `MaybeProp<String>` | optional |
| `disabled` | `Signal<bool>` | optional |
| `aria_label` | `MaybeProp<String>` | optional |
| `on_click` | `Option<Callback<()>>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### InputGroupText

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### InputGroupInput

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `value` | `MaybeProp<String>` | optional |
| `placeholder` | `MaybeProp<String>` | optional |
| `input_type` | `MaybeProp<String>` | optional |
| `name` | `MaybeProp<String>` | optional |
| `disabled` | `Signal<bool>` | optional |
| `invalid` | `Signal<bool>` | optional |
| `on_change` | `Option<Callback<String>>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

### InputGroupTextarea

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `value` | `MaybeProp<String>` | optional |
| `placeholder` | `MaybeProp<String>` | optional |
| `name` | `MaybeProp<String>` | optional |
| `rows` | `MaybeProp<u32>` | optional |
| `disabled` | `Signal<bool>` | optional |
| `invalid` | `Signal<bool>` | optional |
| `on_change` | `Option<Callback<String>>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

---

## Minimal Example

```rust
use leptos::prelude::*;
use leptos_shadcn_input_group::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <InputGroup>"Content"</InputGroup>
    }
}
```

---

## Source

[packages/leptos/input-group](https://github.com/harana-oss/leptos-shadcn-ui/tree/main/packages/leptos/input-group)
