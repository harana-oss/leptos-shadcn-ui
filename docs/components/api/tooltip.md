# Tooltip Component API

A floating label that displays information when hovering over an element.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-tooltip = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_tooltip::Tooltip;
```

---

## Import

```rust
use leptos_shadcn_tooltip::{
    Tooltip,
    TooltipTrigger,
    TooltipContent
};
```

---

## Component API

> Prop tables are verified against `packages/leptos/tooltip/src`.

Wrap tooltips in `TooltipProvider`. `side` on `TooltipContent` is a `TooltipSide`.

### TooltipProvider

| Prop | Type | Default |
|------|------|---------|
| `children` | `Option<Children>` | optional |

### Tooltip

| Prop | Type | Default |
|------|------|---------|
| `open` | `Signal<bool>` | optional |
| `on_open_change` | `Option<Callback<bool>>` | optional |
| `delay_duration` | `Signal<u32>` | optional |
| `children` | `Option<Children>` | optional |

### TooltipTrigger

| Prop | Type | Default |
|------|------|---------|
| `as_child` | `Option<Callback<TooltipTriggerChildProps, AnyView>>` | optional |
| `node_ref` | `AnyNodeRef` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

### TooltipContent

| Prop | Type | Default |
|------|------|---------|
| `side` | `TooltipSide` | optional |
| `side_offset` | `i32` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `node_ref` | `AnyNodeRef` | optional |
| `as_child` | `Option<Callback<TooltipContentChildProps, AnyView>>` | optional |
| `children` | `Option<Children>` | optional |

### TooltipSide

```rust
pub enum TooltipSide {
    Top,
    Right,
    Bottom,
    Left,
}
```

---

## Usage Examples

### Basic Tooltip

```rust
use leptos::prelude::*;
use leptos_shadcn_tooltip::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <TooltipProvider>
        <Tooltip>
            <TooltipTrigger>
                <button>"Hover me"</button>
            </TooltipTrigger>
            <TooltipContent>
                <p>"Tooltip content"</p>
            </TooltipContent>
        </Tooltip>
        </TooltipProvider>
    }
}
```

---

## CSS Classes

```css
.tooltip-content {
    z-50 overflow-hidden rounded-md bg-primary
    px-3 py-1.5 text-xs text-primary-foreground
    animate-in fade-in-0 zoom-in-95
}
```

---

## Accessibility

### ARIA Attributes

- `role="tooltip"` - Tooltip role
- `aria-describedby` - References tooltip content

---

## TypeScript API

```typescript
interface TooltipProps {
  open: boolean;
  delayDuration?: number;
  children?: React.ReactNode;
}

export const Tooltip: React.FC<TooltipProps>;
export const TooltipTrigger: React.FC<ComponentProps>;
export const TooltipContent: React.FC<ComponentProps>;
```

---

*Source: [packages/leptos/tooltip/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/tooltip/src/default.rs)*
