# Textarea Component API

A multi-line text input component for longer content.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-textarea = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_textarea::Textarea;
```

---

## Component API

> Prop tables are verified against `packages/leptos/textarea/src`.

There is no `rows` prop; use `attr:rows=...` or a height class.

### Textarea

| Prop | Type | Default |
|------|------|---------|
| `value` | `MaybeProp<String>` | optional |
| `on_change` | `Option<Callback<String>>` | optional |
| `placeholder` | `MaybeProp<String>` | optional |
| `disabled` | `Signal<bool>` | optional |
| `input_type` | `MaybeProp<String>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |

---

## Usage Examples

### Basic Textarea

```rust
use leptos::prelude::*;
use leptos_shadcn_textarea::Textarea;

#[component]
pub fn MyComponent() -> impl IntoView {
    let (text, set_text) = signal(String::new());

    view! {
        <Textarea
            value=MaybeProp::Derived(text.into())
            on_change=Some(Callback::new(move |value| {
                set_text.set(value);
            }))
            placeholder="Enter your message"
            attr:rows="5"
        />
    }
}
```

---

## CSS Classes

```css
.shadcn-textarea {
    flex min-h-[80px] w-full rounded-md border border-input
    bg-background px-3 py-2 text-sm ring-offset-background
    placeholder:text-muted-foreground focus-visible:outline-none
    focus-visible:ring-2 focus-visible:ring-ring
    focus-visible:ring-offset-2 disabled:cursor-not-allowed
    disabled:opacity-50
}
```

---

## Accessibility

### ARIA Attributes

- `aria-label` - Accessible label
- `aria-describedby` - Description reference
- `aria-invalid` - Validation state

---

## TypeScript API

```typescript
interface TextareaProps {
  value?: string;
  onChange?: (value: string) => void;
  placeholder?: string;
  disabled?: boolean;
  rows?: number;
  className?: string;
  id?: string;
}

export const Textarea: React.FC<TextareaProps>;
```

---

*Source: [packages/leptos/textarea/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/textarea/src/default.rs)*
