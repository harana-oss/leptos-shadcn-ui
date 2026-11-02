# Aspect Ratio Component API

A container that maintains a consistent aspect ratio for its content.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-aspect-ratio = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_aspect_ratio::AspectRatio;
```

---

## Component API

### AspectRatio

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `ratio` | `f64` | **Required** | Aspect ratio (width/height) |
| `class` | `MaybeProp<String>` | `None` | Additional CSS classes |

---

## Usage Examples

### Basic Aspect Ratio

```rust
use leptos::prelude::*;
use leptos_shadcn_aspect_ratio::AspectRatio;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <AspectRatio ratio=16.0 / 9.0>
            <img src="/image.jpg" alt="16:9 image" />
        </AspectRatio>
    }
}
```

### Square Container

```rust
view! {
    <AspectRatio ratio=1.0>
        <div class="bg-muted">"Square content"</div>
    </AspectRatio>
}
```

---

## CSS Classes

```css
.aspect-ratio {
    relative w-full
}

.aspect-ratio::before {
    content: ""
    display: block
    padding-bottom: calc(var(--ratio) * 100%)
}
```

---

## Common Ratios

| Ratio | Value | Usage |
|-------|-------|-------|
| Square | 1:1 | `1.0` |
| Standard | 4:3 | `1.33` |
| Widescreen | 16:9 | `1.78` |
| Cinematic | 21:9 | `2.33` |

---

## TypeScript API

```typescript
interface AspectRatioProps {
  ratio: number;
  className?: string;
  children?: React.ReactNode;
}

export const AspectRatio: React.FC<AspectRatioProps>;
```

---

*Source: [packages/leptos/aspect-ratio/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/aspect-ratio/src/default.rs)*
