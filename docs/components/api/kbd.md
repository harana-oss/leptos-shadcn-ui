# Kbd Component API

> **Stub page.** Generated from `packages/leptos/kbd/src`; expand with prose and examples as the component matures.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-kbd = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_kbd::*;
```

---

## Exported Components

### Kbd

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### KbdGroup

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
use leptos_shadcn_kbd::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Kbd>"Content"</Kbd>
    }
}
```

---

## Source

[packages/leptos/kbd](https://github.com/harana-oss/leptos-shadcn-ui/tree/main/packages/leptos/kbd)
