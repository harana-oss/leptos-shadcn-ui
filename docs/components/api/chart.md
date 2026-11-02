# Chart Component API

> **Stub page.** Generated from `packages/leptos/chart/src`; expand with prose and examples as the component matures.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-chart = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_chart::*;
```

---

## Exported Components

### ChartContainer

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `config` | `Signal<ChartConfig>` | **required** |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Children` | **required** |

### ChartTooltipContent

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `label` | `MaybeProp<String>` | optional |
| `items` | `Signal<Vec<(String, f64)>>` | **required** |
| `indicator` | `MaybeProp<ChartIndicator>` | optional |
| `hide_label` | `MaybeProp<bool>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

### ChartLegendContent

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

### BarChart

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `data` | `Signal<Vec<ChartRow>>` | **required** |
| `show_tooltip` | `MaybeProp<bool>` | optional |
| `width` | `MaybeProp<f64>` | optional |
| `height` | `MaybeProp<f64>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

### LineChart

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `data` | `Signal<Vec<ChartRow>>` | **required** |
| `show_dots` | `MaybeProp<bool>` | optional |
| `width` | `MaybeProp<f64>` | optional |
| `height` | `MaybeProp<f64>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

---

## Minimal Example

```rust
use leptos::prelude::*;
use leptos_shadcn_chart::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <ChartContainer config=ChartConfig::new().series("desktop", "Desktop", "var(--chart-1)")>
            // BarChart / LineChart go here
        </ChartContainer>
    }
}
```

---

## Source

[packages/leptos/chart](https://github.com/harana-oss/leptos-shadcn-ui/tree/main/packages/leptos/chart)
