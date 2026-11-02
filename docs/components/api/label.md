# Label Component API

A form label component for associating text with form controls.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-label = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_label::Label;
```

---

## Component API

> Prop tables are verified against `packages/leptos/label/src`.

`Label` has no `for` prop; wrap the control in the label or set the attribute via `attr:for="..."`.

### Label

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

---

## Usage Examples

### Basic Label

```rust
use leptos::prelude::*;
use leptos_shadcn_label::Label;
use leptos_shadcn_input::Input;

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <div class="space-y-2">
            <Label attr:for="email">"Email"</Label>
            <Input id="email" input_type="email" />
        </div>
    }
}
```

---

## CSS Classes

```css
.shadcn-label {
    text-sm font-medium leading-none
    peer-disabled:cursor-not-allowed peer-disabled:opacity-70
}
```

---

## Accessibility

Labels must be associated with form controls using the `for` attribute matching the input's `id`.

---

## TypeScript API

```typescript
interface LabelProps {
  htmlFor?: string;
  className?: string;
  id?: string;
  children?: React.ReactNode;
}

export const Label: React.FC<LabelProps>;
```

---

*Source: [packages/leptos/label/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/label/src/default.rs)*
