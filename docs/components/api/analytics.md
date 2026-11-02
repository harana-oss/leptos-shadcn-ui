# Analytics Component API

> **Stub page.** Generated from `packages/leptos/analytics/src`; expand with prose and examples as the component matures.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-analytics = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_analytics::*;
```

---

## Exported Components

### AnalyticsProvider

Defined in `src/context.rs`.

| Prop | Type | Default |
|------|------|---------|
| `config` | `Option<AnalyticsConfig>` | optional |
| `children` | `Children` | **required** |

### AnalyticsDashboard

Defined in `src/dashboard.rs`.

| Prop | Type | Default |
|------|------|---------|
| `limit` | `usize` | `20` |
| `show_actions` | `bool` | `true` |

### AnalyticsBadge

Defined in `src/dashboard.rs`.

| Prop | Type | Default |
|------|------|---------|
| `position` | `String` | `"bottom-right".to_string()` |

---

## Minimal Example

```rust
use leptos::prelude::*;
use leptos_shadcn_analytics::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <AnalyticsProvider>"Content"</AnalyticsProvider>
    }
}
```

---

## Source

[packages/leptos/analytics](https://github.com/harana-oss/leptos-shadcn-ui/tree/main/packages/leptos/analytics)
