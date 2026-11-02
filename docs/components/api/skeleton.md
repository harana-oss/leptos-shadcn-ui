# Skeleton Component API

A loading placeholder component for content that is still loading.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-skeleton = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_skeleton::Skeleton;
```

---

## Component API

### Skeleton

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `MaybeProp<String>` | `None` | Additional CSS classes |
| `width` | `MaybeProp<String>` | `None` | Custom width |
| `height` | `MaybeProp<String>` | `None` | Custom height |

---

## Usage Examples

### Basic Skeleton

```rust
use leptos::prelude::*;
use leptos_shadcn_skeleton::Skeleton;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <Skeleton />
    }
}
```

### Card Skeleton

```rust
view! {
    <div class="space-y-4">
        <Skeleton class="h-12 w-12 rounded-full" />
        <Skeleton class="h-4 w-[250px]" />
        <Skeleton class="h-4 w-[200px]" />
    </div>
}
```

### Custom Size

```rust
view! {
    <Skeleton width="100px" height="100px" />
}
```

---

## CSS Classes

```css
.skeleton {
    animate-pulse rounded-md bg-muted
}
```

---

## Accessibility

### ARIA Attributes

- `role="status"` - Status for screen readers
- `aria-live="polite"` - Announces to screen readers
- Hidden from assistive technology (loading placeholder)

---

## TypeScript API

```typescript
interface SkeletonProps {
  className?: string;
  width?: string;
  height?: string;
}

export const Skeleton: React.FC<SkeletonProps>;
```

---

*Source: [packages/leptos/skeleton/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/skeleton/src/default.rs)*
