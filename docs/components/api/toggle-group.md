# Toggle Group Component API

> **Stub page.** Generated from `packages/leptos/toggle-group/src`; expand with prose and examples as the component matures.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-toggle-group = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_toggle_group::*;
```

---

## Exported Components

### ToggleGroup

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `type` | `MaybeProp<String>` | optional |
| `variant` | `MaybeProp<String>` | optional |
| `value` | `Option<RwSignal<Vec<String>>>` | optional |
| `default_value` | `Vec<String>` | optional |
| `on_value_change` | `Option<Callback<Vec<String>>>` | optional |
| `disabled` | `Signal<bool>` | optional |
| `aria_label` | `MaybeProp<String>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### ToggleGroupItem

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `value` | `String` | **required** |
| `variant` | `MaybeProp<String>` | optional |
| `size` | `MaybeProp<String>` | optional |
| `disabled` | `Signal<bool>` | optional |
| `aria_label` | `MaybeProp<String>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

---

## Minimal Example

```rust
use leptos::prelude::*;
use leptos_shadcn_toggle_group::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <ToggleGroup>"Content"</ToggleGroup>
    }
}
```

---

## Source

[packages/leptos/toggle-group](https://github.com/harana-oss/leptos-shadcn-ui/tree/main/packages/leptos/toggle-group)
