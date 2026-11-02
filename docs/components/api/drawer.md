# Drawer Component API

A slide-out panel from any edge of the screen, similar to Sheet but with different styling.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-drawer = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_drawer::Drawer;
```

---

## Import

```rust
use leptos_shadcn_drawer::{
    Drawer,
    DrawerTrigger,
    DrawerContent,
    DrawerHeader,
    DrawerTitle,
    DrawerDescription,
    DrawerFooter,
    DrawerClose
};
```

---

## Component API

> Prop tables are verified against `packages/leptos/drawer/src`.

`open` is an `RwSignal<bool>`; `direction` is a `DrawerDirection` (default `Bottom`).

### DrawerClose

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### DrawerContent

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<ChildrenFn>` | optional |

### Drawer

| Prop | Type | Default |
|------|------|---------|
| `open` | `RwSignal<bool>` | **required** |
| `on_open_change` | `Option<Callback<bool>>` | optional |
| `direction` | `Signal<DrawerDirection>` | optional |
| `should_scale_background` | `Signal<bool>` | optional |
| `children` | `Option<ChildrenFn>` | optional |

### DrawerHeader

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### DrawerFooter

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### DrawerNestedRoot

| Prop | Type | Default |
|------|------|---------|
| `open` | `RwSignal<bool>` | **required** |
| `on_open_change` | `Option<Callback<bool>>` | optional |
| `direction` | `Signal<DrawerDirection>` | optional |
| `should_scale_background` | `Signal<bool>` | optional |
| `children` | `Option<Children>` | optional |

### DrawerPortal

| Prop | Type | Default |
|------|------|---------|
| `children` | `Option<Children>` | optional |

### DrawerOverlay

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<ChildrenFn>` | optional |

### DrawerTitle

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### DrawerDescription

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### DrawerTrigger

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### DrawerDirection

```rust
pub enum DrawerDirection {
    Top,
    Bottom,
    Left,
    Right,
}
```

---

## Usage Examples

### Basic Drawer

```rust
use leptos::prelude::*;
use leptos_shadcn_drawer::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <Drawer
            open=open
            on_open_change=Callback::new(move |v| open.set(v))
        >
            <DrawerTrigger>
                <button>"Open Drawer"</button>
            </DrawerTrigger>
            <DrawerContent>
                <DrawerHeader>
                    <DrawerTitle>"Drawer Title"</DrawerTitle>
                </DrawerHeader>
                <div class="p-4">
                    "Drawer content"
                </div>
            </DrawerContent>
        </Drawer>
    }
}
```

---

## CSS Classes

```css
.drawer-content {
    fixed z-50 gap-4 bg-background p-6 shadow-lg
    transition ease-in-out
}

.drawer-content[data-direction=right] {
    inset-y-0 right-0 h-full w-3/4 border-l
}
```

---

## Accessibility

### Keyboard Navigation

| Key | Action |
|-----|--------|
| **Escape** | Close drawer |

---

## TypeScript API

```typescript
interface DrawerProps {
  open: boolean;
  onOpenChange?: (open: boolean) => void;
  direction?: 'top' | 'right' | 'bottom' | 'left';
}

export const Drawer: React.FC<DrawerProps>;
export const DrawerTrigger: React.FC<ComponentProps>;
export const DrawerContent: React.FC<ComponentProps>;
```

---

*Source: [packages/leptos/drawer/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/drawer/src/default.rs)*
