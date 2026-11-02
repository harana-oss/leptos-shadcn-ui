# Select Component API

A dropdown select component for choosing from a list of options.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-select = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_select::Select;
```

---

## Import

```rust
use leptos_shadcn_select::{
    Select,
    SelectTrigger,
    SelectContent,
    SelectItem,
    SelectValue,
    SelectLabel,
    SelectGroup,
    SelectSeparator
};
```

---

## Component API

> Prop tables are verified against `packages/leptos/select/src`.

`Select` is the root (state) component; `SelectTrigger`, `SelectValue`, `SelectContent`, `SelectItem`, `SelectGroup`, `SelectLabel`, `SelectSeparator` and the scroll buttons compose the UI. The change callback is `on_value_change`.

### SelectContent

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### SelectItem

| Prop | Type | Default |
|------|------|---------|
| `value` | `String` | **required** |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### SelectSeparator

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

### SelectLabel

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### SelectGroup

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### Select

| Prop | Type | Default |
|------|------|---------|
| `open` | `Signal<bool>` | optional |
| `on_open_change` | `Option<Callback<bool>>` | optional |
| `value` | `Signal<String>` | optional |
| `on_value_change` | `Option<Callback<String>>` | optional |
| `default_value` | `MaybeProp<String>` | optional |
| `disabled` | `Signal<bool>` | optional |
| `required` | `Signal<bool>` | optional |
| `name` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### SelectTrigger

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### SelectValue

| Prop | Type | Default |
|------|------|---------|
| `placeholder` | `MaybeProp<String>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

### SelectScrollUpButton

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

### SelectScrollDownButton

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

---

## Usage Examples

### Basic Select

```rust
use leptos::prelude::*;
use leptos_shadcn_select::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    let (selected, set_selected) = signal("apple".to_string());

    view! {
        <Select
            value=selected.into()
            on_value_change=Some(Callback::new(move |value| {
                set_selected.set(value);
            }))
        >
            <SelectTrigger>
                <SelectValue placeholder="Choose a fruit" />
            </SelectTrigger>
            <SelectContent>
                <SelectItem value="apple">"Apple"</SelectItem>
                <SelectItem value="banana">"Banana"</SelectItem>
                <SelectItem value="orange">"Orange"</SelectItem>
            </SelectContent>
        </Select>
    }
}
```

---

## CSS Classes

```css
.select-trigger {
    flex h-10 w-full items-center justify-between
    rounded-md border border-input bg-background px-3 py-2
    text-sm ring-offset-background placeholder:text-muted-foreground
    focus:outline-none focus:ring-2 focus:ring-ring
    focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50
}

.select-content {
    relative z-50 min-w-[8rem] overflow-hidden rounded-md
    border bg-popover text-popover-foreground shadow-md
}

.select-item {
    relative flex w-full cursor-pointer select-none
    items-center rounded-sm py-1.5 pl-8 pr-2 text-sm
    outline-none focus:bg-accent focus:text-accent-foreground
}
```

---

## Accessibility

### Keyboard Navigation

| Key | Action |
|-----|--------|
| **Arrow Down** | Next option |
| **Arrow Up** | Previous option |
| **Enter** | Select option |
| **Escape** | Close dropdown |

---

## TypeScript API

```typescript
interface SelectProps {
  value: string;
  onValueChange?: (value: string) => void;
  disabled?: boolean;
  placeholder?: string;
  children?: React.ReactNode;
}

export const Select: React.FC<SelectProps>;
export const SelectTrigger: React.FC<ComponentProps>;
export const SelectContent: React.FC<ComponentProps>;
export const SelectItem: React.FC<{ value: string; disabled?: boolean; children: React.ReactNode }>;
export const SelectValue: React.FC<{ placeholder?: string }>;
```

---

*Source: [packages/leptos/select/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/select/src/default.rs)*
