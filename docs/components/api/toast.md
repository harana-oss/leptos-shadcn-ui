# Toast Component API

A temporary notification component for displaying brief messages.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-toast = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_toast::Toast;
```

---

## Import

```rust
use leptos_shadcn_toast::{
    Toast,
    SonnerProvider,
    SonnerViewport,
    ToastBuilder,
    ToastVariant,
    ToastPosition,
    toast,
};
```

---

## Component API

> Prop tables are verified against `packages/leptos/toast/src`.

The default `Toast` is a styled container whose `variant` is a `String` (`"default"`, `"destructive"`). For a full notification system use the Sonner API exported by this crate: wrap the app in `SonnerProvider` and call `toast(...)` / `ToastBuilder`. There is no `ToastProvider`.

### Toast

| Prop | Type | Default |
|------|------|---------|
| `variant` | `MaybeProp<String>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

### SonnerProvider

| Prop | Type | Default |
|------|------|---------|
| `position` | `MaybeProp<ToastPosition>` | optional |
| `theme` | `MaybeProp<ToastTheme>` | optional |
| `max_toasts` | `MaybeProp<usize>` | optional |
| `children` | `Option<Children>` | optional |

### SonnerViewport

| Prop | Type | Default |
|------|------|---------|

### SonnerToast

| Prop | Type | Default |
|------|------|---------|
| `id` | `String` | **required** |
| `data` | `ToastData` | **required** |
| `on_dismiss` | `Callback<()>` | **required** |

### ToastVariant

```rust
pub enum ToastVariant {
    Default,
    Success,
    Error,
    Warning,
    Info,
    Loading,
}
```

### ToastPosition

```rust
pub enum ToastPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    TopCenter,
    BottomCenter,
}
```

---

## Usage Examples

### Basic Toast

```rust
use leptos::prelude::*;
use leptos_shadcn_toast::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <SonnerProvider position=ToastPosition::BottomRight>
            <button on:click=move |_| {
                toast::success("Saved").description("Your changes were saved.".into()).show();
            }>"Show Toast"</button>
            <SonnerViewport />
        </SonnerProvider>
    }
}
```

---

## CSS Classes

```css
.toast {
    group pointer-events-auto relative flex w-full
    items-center justify-between space-x-4 overflow-hidden
    rounded-md border p-6 pr-8 shadow-lg transition-all
}

.toast-default {
    border bg-background text-foreground
}

.toast-destructive {
    border-destructive bg-destructive text-destructive-foreground
}

.toast-success {
    border-success bg-success text-success-foreground
}
```

---

## Accessibility

### ARIA Attributes

- `role="status"` - Status role
- `aria-live="polite"` - Polite announcements
- `aria-atomic="true"` - Complete content reading

---

## TypeScript API

```typescript
interface ToastProps {
  variant?: 'default' | 'destructive' | 'success';
  title?: string;
  description?: string;
  duration?: number;
  onClose?: () => void;
}

export const Toast: React.FC<ToastProps>;
export const ToastAction: React.FC<ButtonProps>;
```

---

*Source: [packages/leptos/toast/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/toast/src/default.rs)*
