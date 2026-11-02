# Checkbox Component API

A checkbox component for binary selection states.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-checkbox = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_checkbox::Checkbox;
```

---

## Component API

### Checkbox

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `checked` | `Signal<bool>` | **Required** | Checkbox state |
| `on_change` | `Option<Callback<bool>>` | `None` | Change handler |
| `disabled` | `Signal<bool>` | `false` | Disable checkbox |
| `class` | `MaybeProp<String>` | `None` | Additional CSS classes |
| `id` | `MaybeProp<String>` | `None` | Unique identifier |
| `style` | `Signal<Style>` | `None` | Inline styles |

---

## Usage Examples

### Basic Checkbox

```rust
use leptos::prelude::*;
use leptos_shadcn_checkbox::Checkbox;

#[component]
pub fn MyComponent() -> impl IntoView {
    let (checked, set_checked) = signal(false);

    view! {
        <div class="flex items-center space-x-2">
            <Checkbox
                checked=checked.into()
                on_change=Some(Callback::new(move |is_checked| {
                    set_checked.set(is_checked);
                }))
                id="terms"
            />
            <label for="terms">"Accept terms and conditions"</label>
        </div>
    }
}
```

### Controlled Checkbox

```rust
view! {
    <Checkbox
        checked=Signal::derive(|| true)
        disabled=Signal::derive(|| true)
    />
}
```

---

## CSS Classes

```css
.shadcn-checkbox {
    h-4 w-4 shrink-0 rounded-sm border border-primary
    ring-offset-background focus-visible:outline-none
    focus-visible:ring-2 focus-visible:ring-ring
    focus-visible:ring-offset-2 disabled:cursor-not-allowed
    disabled:opacity-50
}

.shadcn-checkbox[data-state="checked"] {
    bg-primary text-primary-foreground
}
```

---

## Accessibility

### Keyboard Navigation

| Key | Action |
|-----|--------|
| **Space** | Toggle checkbox |

---

## TypeScript API

```typescript
interface CheckboxProps {
  checked: boolean;
  onChange?: (checked: boolean) => void;
  disabled?: boolean;
  className?: string;
  id?: string;
}

export const Checkbox: React.FC<CheckboxProps>;
```

---

*Source: [packages/leptos/checkbox/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/checkbox/src/default.rs)*
