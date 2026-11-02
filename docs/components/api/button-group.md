# Button Group Component API

> **Stub page.** Generated from `packages/leptos/button-group/src`; expand with prose and examples as the component matures.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-button-group = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_button_group::*;
```

---

## Exported Components

### ButtonGroup

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `orientation` | `MaybeProp<ButtonGroupOrientation>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### ButtonGroupSeparator

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `orientation` | `MaybeProp<ButtonGroupOrientation>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

### ButtonGroupText

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

---

## Minimal Example

```rust
use leptos::prelude::*;
use leptos_shadcn_button_group::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <ButtonGroup>"Content"</ButtonGroup>
    }
}
```

---

## Source

[packages/leptos/button-group](https://github.com/harana-oss/leptos-shadcn-ui/tree/main/packages/leptos/button-group)
