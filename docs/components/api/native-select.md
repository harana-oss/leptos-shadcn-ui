# Native Select Component API

> **Stub page.** Generated from `packages/leptos/native-select/src`; expand with prose and examples as the component matures.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-native-select = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_native_select::*;
```

---

## Exported Components

### NativeSelect

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `value` | `MaybeProp<String>` | optional |
| `on_change` | `Option<Callback<String>>` | optional |
| `size` | `MaybeProp<String>` | optional |
| `name` | `MaybeProp<String>` | optional |
| `disabled` | `Signal<bool>` | optional |
| `required` | `Signal<bool>` | optional |
| `invalid` | `Signal<bool>` | optional |
| `aria_label` | `MaybeProp<String>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### NativeSelectOption

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `value` | `String` | **required** |
| `disabled` | `Signal<bool>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### NativeSelectOptGroup

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `label` | `String` | **required** |
| `disabled` | `Signal<bool>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

---

## Minimal Example

```rust
use leptos::prelude::*;
use leptos_shadcn_native_select::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <NativeSelect>"Content"</NativeSelect>
    }
}
```

---

## Source

[packages/leptos/native-select](https://github.com/harana-oss/leptos-shadcn-ui/tree/main/packages/leptos/native-select)
