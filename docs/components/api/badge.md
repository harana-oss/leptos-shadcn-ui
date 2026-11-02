# Badge Component API

A small status indicator component for displaying labels and counts.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-badge = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_badge::Badge;
```

---

## Import

```rust
use leptos_shadcn_badge::{
    Badge,
    BadgeVariant
};
```

---

## Component API

> Prop tables are verified against `packages/leptos/badge/src`.

### Badge

| Prop | Type | Default |
|------|------|---------|
| `variant` | `MaybeProp<BadgeVariant>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

### BadgeVariant

```rust
pub enum BadgeVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
}
```

---

## Usage Examples

### Basic Badge

```rust
use leptos::prelude::*;
use leptos_shadcn_badge::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <Badge variant=BadgeVariant::Default>
            "New"
        </Badge>
    }
}
```

### Badge Variants

```rust
view! {
    <div class="flex gap-2">
        <Badge variant=BadgeVariant::Default>"Default"</Badge>
        <Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
        <Badge variant=BadgeVariant::Destructive>"Error"</Badge>
        <Badge variant=BadgeVariant::Outline>"Outline"</Badge>
    </div>
}
```

### Count Badge

```rust
view! {
    <div class="relative">
        <button>"Notifications"</button>
        <Badge class="absolute -top-2 -right-2 px-2 py-0.5">
            "3"
        </Badge>
    </div>
}
```

---

## CSS Classes

```css
.badge {
    inline-flex items-center rounded-full border
    px-2.5 py-0.5 text-xs font-semibold
    transition-colors focus:outline-none focus:ring-2
    focus:ring-ring focus:ring-offset-2
}

.badge-default {
    border-transparent bg-primary text-primary-foreground
    hover:bg-primary/80
}

.badge-secondary {
    border-transparent bg-secondary
    text-secondary-foreground hover:bg-secondary/80
}

.badge-destructive {
    border-transparent bg-destructive
    text-destructive-foreground hover:bg-destructive/80
}

.badge-outline {
    text-foreground
}

.badge-success {
    border-transparent bg-green-500 text-white
    hover:bg-green-600
}
```

---

## Accessibility

### ARIA Attributes

- `aria-label` - For status badges
- Semantic color usage for meaning

---

## TypeScript API

```typescript
interface BadgeProps {
  variant?: 'default' | 'primary' | 'secondary' | 'destructive' | 'outline' | 'success';
  className?: string;
  children?: React.ReactNode;
}

export const Badge: React.FC<BadgeProps>;
```

---

*Source: [packages/leptos/badge/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/badge/src/default.rs)*
