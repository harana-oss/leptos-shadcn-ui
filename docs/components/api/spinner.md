# Spinner Component API

> **Stub page.** Generated from `packages/leptos/spinner/src`; expand with prose and examples as the component matures.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-spinner = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_spinner::*;
```

---

## Exported Components

### Spinner

Defined in `src/default.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

---

## Minimal Example

```rust
use leptos::prelude::*;
use leptos_shadcn_spinner::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Spinner />
    }
}
```

---

## Source

[packages/leptos/spinner](https://github.com/harana-oss/leptos-shadcn-ui/tree/main/packages/leptos/spinner)
