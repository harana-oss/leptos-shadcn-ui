# Scroll Area Component API

A container with custom styled scrollbars.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-scroll-area = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_scroll_area::ScrollArea;
```

---

## Component API

### ScrollArea

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `MaybeProp<String>` | `None` | Additional CSS classes |
| `children` | `Option<Children>` | `None` | Scrollable content |

---

## Usage Examples

### Basic Scroll Area

```rust
use leptos::prelude::*;
use leptos_shadcn_scroll_area::ScrollArea;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <ScrollArea class="h-72 w-48 rounded-md border">
            <div class="p-4">
                <p>"Long content that scrolls..."</p>
            </div>
        </ScrollArea>
    }
}
```

---

## CSS Classes

```css
.scroll-area {
    overflow-auto
}

.scroll-area::-webkit-scrollbar {
    width: 8px
}

.scroll-area::-webkit-scrollbar-track {
    background: transparent
}

.scroll-area::-webkit-scrollbar-thumb {
    background-color: hsl(var(--muted))
    border-radius: 4px
}
```

---

## Accessibility

- Standard scrolling behavior maintained
- Keyboard navigation supported

---

## TypeScript API

```typescript
interface ScrollAreaProps {
  className?: string;
  children?: React.ReactNode;
}

export const ScrollArea: React.FC<ScrollAreaProps>;
```

---

*Source: [packages/leptos/scroll-area/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/scroll-area/src/default.rs)*
