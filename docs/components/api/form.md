# Form Component API

A form container with built-in validation and state management.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-form = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_form::Form;
```

---

## Component API

> Prop tables are verified against `packages/leptos/form/src`.

`on_submit` receives the crate's `FormData` (field name -> value), not a raw DOM event.

### Form

| Prop | Type | Default |
|------|------|---------|
| `on_submit` | `Option<Callback<FormData>>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

### FormField

| Prop | Type | Default |
|------|------|---------|
| `name` | `String` | **required** |
| `class` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `invalid` | `Signal<bool>` | optional |
| `children` | `Option<Children>` | optional |

### FormItem

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

### FormLabel

| Prop | Type | Default |
|------|------|---------|
| `for_field` | `String` | **required** |
| `class` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

### FormControl

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

### FormMessage

| Prop | Type | Default |
|------|------|---------|
| `message` | `MaybeProp<String>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |

### FormDescription

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `children` | `Option<Children>` | optional |

---

## Usage Examples

### Basic Form

```rust
use leptos::prelude::*;
use leptos_shadcn_form::Form;
use leptos_shadcn_input::Input;
use leptos_shadcn_button::Button;

#[component]
pub fn MyForm() -> impl IntoView {
    let handle_submit = Callback::new(move |data: FormData| {
        // `data` holds the submitted field values
        let _ = data;
    });

    view! {
        <Form on_submit=handle_submit>
            <div class="space-y-4">
                <Input name="email" input_type="email" placeholder="Email" />
                <Input name="password" input_type="password" placeholder="Password" />
                <Button>"Submit"</Button>
            </div>
        </Form>
    }
}
```

---

## CSS Classes

```css
.form {
    space-y-6
}
```

---

## Accessibility

- Native HTML form validation
- Proper label association
- Error announcements

---

## TypeScript API

```typescript
interface FormProps {
  onSubmit?: (event: FormEvent) => void;
  className?: string;
  children?: React.ReactNode;
}

export const Form: React.FC<FormProps>;
```

---

*Source: [packages/leptos/form/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/form/src/default.rs)*
