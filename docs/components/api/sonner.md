# Sonner Component API

> **Stub page.** Generated from `packages/leptos/sonner/src`; expand with prose and examples as the component matures.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-sonner = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_sonner::*;
```

---

## Exported Components

### Toaster

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `position` | `MaybeProp<ToasterPosition>` | optional |
| `max_visible` | `MaybeProp<usize>` | optional |
| `duration_ms` | `MaybeProp<u64>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

---

## Minimal Example

```rust
use leptos::prelude::*;
use leptos_shadcn_sonner::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Toaster />
    }
}
```

---

## Source

[packages/leptos/sonner](https://github.com/harana-oss/leptos-shadcn-ui/tree/main/packages/leptos/sonner)
