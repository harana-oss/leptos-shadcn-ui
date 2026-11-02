# Alert Component API

A component for displaying alert messages with different severity levels.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-alert = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_alert::Alert;
```

---

## Import

```rust
use leptos_shadcn_alert::{
    Alert,
    AlertTitle,
    AlertDescription,
    AlertVariant
};
```

---

## Component API

> Prop tables are verified against `packages/leptos/alert/src`.

### Alert

| Prop | Type | Default |
|------|------|---------|
| `variant` | `MaybeProp<AlertVariant>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

### AlertTitle

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

### AlertDescription

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

### AlertVariant

```rust
pub enum AlertVariant {
    Default,
    Destructive,
    Success,
    Warning,
}
```

---

## Usage Examples

### Basic Alert

```rust
use leptos::prelude::*;
use leptos_shadcn_alert::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <Alert variant=AlertVariant::Default>
            <AlertTitle>"Heads up!"</AlertTitle>
            <AlertDescription>
                "You can add components to your app using the cli."
            </AlertDescription>
        </Alert>
    }
}
```

### Alert Variants

```rust
view! {
    <div class="space-y-4">
        <Alert variant=AlertVariant::Default>
            <AlertDescription>"Default alert"</AlertDescription>
        </Alert>


        <Alert variant=AlertVariant::Success>
            <AlertDescription>"Success! Operation completed."</AlertDescription>
        </Alert>

        <Alert variant=AlertVariant::Warning>
            <AlertDescription>"Warning! Please review."</AlertDescription>
        </Alert>

        <Alert variant=AlertVariant::Destructive>
            <AlertDescription>"Error! Something went wrong."</AlertDescription>
        </Alert>
    </div>
}
```

---

## CSS Classes

```css
.alert {
    relative w-full rounded-lg border px-4 py-3 text-sm
}

.alert-default {
    border-border bg-background text-foreground
}

.alert-destructive {
    border-destructive/50 text-destructive
    bg-destructive/10
}

.alert-warning {
    border-warning/50 text-warning
    bg-warning/10
}

.alert-success {
    border-success/50 text-success
    bg-success/10
}

.alert-info {
    border-info/50 text-info
    bg-info/10
}

.alert-title {
    mb-1 font-medium leading-none tracking-tight
}

.alert-description {
    text-sm opacity-90
}
```

---

## Accessibility

### ARIA Attributes

- `role="alert"` - Alert role for screen readers
- Semantic heading for title
- Descriptive text for context

---

## TypeScript API

```typescript
interface AlertProps {
  variant?: 'default' | 'destructive' | 'warning' | 'info' | 'success';
  className?: string;
  children?: React.ReactNode;
}

export const Alert: React.FC<AlertProps>;
export const AlertTitle: React.FC<ComponentProps>;
export const AlertDescription: React.FC<ComponentProps>;
```

---

*Source: [packages/leptos/alert/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/alert/src/default.rs)*
