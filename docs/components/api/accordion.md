# Accordion Component API

A vertically collapsible component for organizing content into expandable sections.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-accordion = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_accordion::Accordion;
```

---

## Import

```rust
use leptos_shadcn_accordion::{
    Accordion,
    AccordionItem,
    AccordionTrigger,
    AccordionContent
};
```

---

## Component API

> Prop tables are verified against `packages/leptos/accordion/src`.

Open items are a `Vec<String>`; set `type=AccordionType::Multiple` to allow several open items (there is no `multiple` prop). The callback is `on_value_change`.

### Accordion

| Prop | Type | Default |
|------|------|---------|
| `type` | `Signal<AccordionType>` | optional |
| `orientation` | `Signal<AccordionOrientation>` | optional |
| `collapsible` | `Signal<bool>` | optional |
| `disabled` | `Signal<bool>` | optional |
| `value` | `RwSignal<Vec<String>>` | optional |
| `default_value` | `Vec<String>` | optional |
| `on_value_change` | `Option<Callback<Vec<String>>>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### AccordionItem

| Prop | Type | Default |
|------|------|---------|
| `value` | `String` | **required** |
| `disabled` | `Signal<bool>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### AccordionTrigger

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `as_child` | `Option<Callback<AccordionTriggerChildProps>>` | optional |
| `children` | `Option<Children>` | optional |

### AccordionContent

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `force_mount` | `Signal<bool>` | optional |
| `children` | `Option<Children>` | optional |

### AccordionType

```rust
pub enum AccordionType {
    Single,
    Multiple,
}
```

### AccordionOrientation

```rust
pub enum AccordionOrientation {
    Vertical,
    Horizontal,
}
```

---

## Usage Examples

### Basic Accordion

```rust
use leptos::prelude::*;
use leptos_shadcn_accordion::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    let value = RwSignal::new(vec!["item-1".to_string()]);

    view! {
        <Accordion
            r#type=AccordionType::Single
            collapsible=true
            value=value
        >
            <AccordionItem value="item-1">
                <AccordionTrigger>"Is it accessible?"</AccordionTrigger>
                <AccordionContent>
                    "Yes. It adheres to the WAI-ARIA design pattern."
                </AccordionContent>
            </AccordionItem>

            <AccordionItem value="item-2">
                <AccordionTrigger>"Is it styled?"</AccordionTrigger>
                <AccordionContent>
                    "Yes. It comes with default styles."
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
```

---

## CSS Classes

```css
.accordion {
    border-b
}

.accordion-item {
    border-b
}

.accordion-trigger {
    flex flex-1 items-center justify-between
    py-4 font-medium transition-all
    hover:underline [&[data-state=open]>svg]:rotate-180
}

.accordion-content {
    overflow-hidden text-sm transition-all
}

.accordion-content[data-state=closed] {
    animate-accordion-up
}

.accordion-content[data-state=open] {
    animate-accordion-down
}
```

---

## Accessibility

### Keyboard Navigation

| Key | Action |
|-----|--------|
| **Tab** | Navigate accordion |
| **Enter/Space** | Toggle section |
| **Arrow Down** | Next section |
| **Arrow Up** | Previous section |
| **Home** | First section |
| **End** | Last section |

---

## TypeScript API

```typescript
interface AccordionProps {
  value: string;
  onValueChange?: (value: string[]) => void;
  type?: "single" | "multiple";
  collapsible?: boolean;
  className?: string;
  children?: React.ReactNode;
}

export const Accordion: React.FC<AccordionProps>;
export const AccordionItem: React.FC<{ value: string; disabled?: boolean }>;
export const AccordionTrigger: React.FC<ComponentProps>;
export const AccordionContent: React.FC<ComponentProps>;
```

---

*Source: [packages/leptos/accordion/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/accordion/src/default.rs)*
