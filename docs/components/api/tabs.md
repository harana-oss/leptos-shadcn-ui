# Tabs Component API

A tabbed interface component for organizing content into separate panels.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-tabs = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_tabs::Tabs;
```

---

## Import

```rust
// Default theme
use leptos_shadcn_tabs::{
    Tabs,
    TabsList,
    TabsTrigger,
    TabsContent
};

// New York theme
use leptos_shadcn_tabs::{
    Tabs as TabsNewYork,
    TabsList as TabsListNewYork,
    TabsTrigger as TabsTriggerNewYork,
    TabsContent as TabsContentNewYork
};

// Signal-managed variant
use leptos_shadcn_tabs::{
    SignalManagedTabs,
    SignalManagedTabsState
};
```

---

## Component API

> Prop tables are verified against `packages/leptos/tabs/src`.

`Tabs` has no `id` prop. Use `value` + `on_value_change` for controlled tabs, or `default_value` for uncontrolled.

### Tabs

| Prop | Type | Default |
|------|------|---------|
| `value` | `Signal<String>` | optional |
| `on_value_change` | `Option<Callback<String>>` | optional |
| `default_value` | `MaybeProp<String>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### TabsList

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### TabsTrigger

| Prop | Type | Default |
|------|------|---------|
| `value` | `MaybeProp<String>` | **required** |
| `disabled` | `Signal<bool>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `Signal<Style>` | optional |
| `node_ref` | `AnyNodeRef` | optional |
| `as_child` | `Option<Callback<TabsTriggerChildProps, AnyView>>` | optional |
| `children` | `Option<Children>` | optional |

### TabsContent

| Prop | Type | Default |
|------|------|---------|
| `value` | `MaybeProp<String>` | **required** |
| `class` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

---

## Usage Examples

### Basic Tabs

```rust
use leptos::prelude::*;
use leptos_shadcn_tabs::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    let (active_tab, set_active_tab) = signal("account".to_string());

    view! {
        <Tabs
            value=active_tab.into()
            on_value_change=Some(Callback::new(move |value| {
                set_active_tab.set(value);
            }))
        >
            <TabsList>
                <TabsTrigger value="account">"Account"</TabsTrigger>
                <TabsTrigger value="password">"Password"</TabsTrigger>
                <TabsTrigger value="settings">"Settings"</TabsTrigger>
            </TabsList>

            <TabsContent value="account">
                "Account settings content"
            </TabsContent>

            <TabsContent value="password">
                "Password settings content"
            </TabsContent>

            <TabsContent value="settings">
                "General settings content"
            </TabsContent>
        </Tabs>
    }
}
```

### Vertical Tabs

```rust
view! {
    <div class="flex gap-4">
        <TabsList class="flex-col h-full">
            <TabsTrigger value="tab1">"Tab 1"</TabsTrigger>
            <TabsTrigger value="tab2">"Tab 2"</TabsTrigger>
            <TabsTrigger value="tab3">"Tab 3"</TabsTrigger>
        </TabsList>

        <div class="flex-1">
            <TabsContent value="tab1">"Content 1"</TabsContent>
            <TabsContent value="tab2">"Content 2"</TabsContent>
            <TabsContent value="tab3">"Content 3"</TabsContent>
        </div>
    </div>
}
```

### Disabled Tabs

```rust
view! {
    <Tabs value=active_tab.into()>
        <TabsList>
            <TabsTrigger value="enabled">"Enabled Tab"</TabsTrigger>
            <TabsTrigger
                value="disabled"
                disabled=Signal::derive(|| true)
            >
                "Disabled Tab"
            </TabsTrigger>
        </TabsList>
        <TabsContent value="enabled">"Content"</TabsContent>
    </Tabs>
}
```

### With Icons

```rust
view! {
    <Tabs value=active_tab.into()>
        <TabsList>
            <TabsTrigger value="home">
                <span class="mr-2">"</span>
                "Home"
            </TabsTrigger>
            <TabsTrigger value="profile">
                <span class="mr-2">"</span>
                "Profile"
            </TabsTrigger>
            <TabsTrigger value="settings">
                <span class="mr-2">"</span>
                "Settings"
            </TabsTrigger>
        </TabsList>
        // ... content
    </Tabs>
}
```

---

## Accessibility

### Keyboard Navigation

| Key | Action |
|-----|--------|
| **Arrow Right** | Next tab |
| **Arrow Left** | Previous tab |
| **Home** | First tab |
| **End** | Last tab |
| **Enter/Space** | Activate tab |

### ARIA Attributes

The component automatically manages:

- `role="tablist"` - TabsList container
- `role="tab"` - TabsTrigger elements
- `role="tabpanel"` - TabsContent elements
- `aria-selected` - Current tab state
- `aria-controls` - Panel association
- `aria-labelledby` - Label association

---

## CSS Classes

```css
.tabs-list {
    inline-flex h-10 items-center justify-center
    rounded-md bg-muted p-1 text-muted-foreground
}

.tabs-trigger {
    inline-flex items-center justify-center
    whitespace-nowrap rounded-sm px-3 py-1.5
    text-sm font-medium ring-offset-background
    transition-all focus-visible:outline-none
    focus-visible:ring-2 focus-visible:ring-ring
    focus-visible:ring-offset-2
    disabled:pointer-events-none disabled:opacity-50
}

.tabs-trigger[data-state="active"] {
    bg-background text-foreground shadow-sm
}

.tabs-content {
    mt-2 ring-offset-background
    focus-visible:outline-none focus-visible:ring-2
    focus-visible:ring-ring focus-visible:ring-offset-2
}
```

---

## TypeScript API

```typescript
interface TabsProps {
  value: string;
  onValueChange?: (value: string) => void;
  className?: string;
  children?: React.ReactNode;
}

interface TabsTriggerProps {
  value: string;
  disabled?: boolean;
  className?: string;
  children?: React.ReactNode;
}

interface TabsContentProps {
  value: string;
  className?: string;
  children?: React.ReactNode;
}

export const Tabs: React.FC<TabsProps>;
export const TabsList: React.FC<ComponentProps>;
export const TabsTrigger: React.FC<TabsTriggerProps>;
export const TabsContent: React.FC<TabsContentProps>;
```

---

## Best Practices

1. **Keep tab labels short** - 1-2 words maximum
2. **Use descriptive values** - Match content purpose
3. **Limit tab count** - 3-7 tabs ideal
4. **Maintain content consistency** - Similar structure per tab
5. **Consider mobile** - Horizontal scroll or stack

---

## See Also

- [Accordion](./accordion.md) - Vertical alternative
- [Separator](./separator.md) - Visual dividers

---

*Source: [packages/leptos/tabs/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/tabs/src/default.rs)*
