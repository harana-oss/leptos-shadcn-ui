# Popover Component API

A floating content container positioned relative to a trigger element.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-popover = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_popover::Popover;
```

---

## Import

```rust
use leptos_shadcn_popover::{
    Popover,
    PopoverTrigger,
    PopoverContent
};
```

---

## Component API

> Prop tables are verified against `packages/leptos/popover/src`.

`open` takes an `RwSignal<bool>` (optional; uncontrolled when omitted). `PopoverContent` has no `side`/`align` props.

### Popover

| Prop | Type | Default |
|------|------|---------|
| `variant` | `MaybeProp<String>` | optional |
| `size` | `MaybeProp<String>` | optional |
| `on_click` | `Option<Callback<()>>` | optional |
| `disabled` | `Signal<bool>` | optional |
| `open` | `Option<RwSignal<bool>>` | optional |
| `on_open_change` | `Option<Callback<bool>>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

### PopoverTrigger

| Prop | Type | Default |
|------|------|---------|
| `disabled` | `Signal<bool>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### PopoverContent

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

---

## Usage Examples

### Basic Popover

```rust
use leptos::prelude::*;
use leptos_shadcn_popover::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <Popover
            open=open
            on_open_change=Callback::new(move |v| open.set(v))
        >
            <PopoverTrigger>
                <button>"Open popover"</button>
            </PopoverTrigger>
            <PopoverContent>
                <p>"Popover content"</p>
            </PopoverContent>
        </Popover>
    }
}
```

---

## CSS Classes

```css
.popover-content {
    z-50 w-72 rounded-md border bg-popover
    p-4 text-popover-foreground shadow-md
    outline-none data-[state=open]:animate-in
    data-[state=closed]:animate-out
}
```

---

## Accessibility

### Keyboard Navigation

| Key | Action |
|-----|--------|
| **Escape** | Close popover |
| **Tab** | Focus within popover |

---

## TypeScript API

```typescript
interface PopoverProps {
  open: boolean;
  onOpenChange?: (open: boolean) => void;
  children?: React.ReactNode;
}

export const Popover: React.FC<PopoverProps>;
export const PopoverTrigger: React.FC<ComponentProps>;
export const PopoverContent: React.FC<ComponentProps>;
```

---

*Source: [packages/leptos/popover/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/popover/src/default.rs)*
