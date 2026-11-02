# Navigation Menu Component API

A navigation menu component with support for dropdowns and keyboard navigation.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-navigation-menu = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_navigation_menu::NavigationMenu;
```

---

## Import

```rust
use leptos_shadcn_navigation_menu::{
    NavigationMenu,
    NavigationMenuList,
    NavigationMenuItem,
    NavigationMenuTrigger,
    NavigationMenuContent,
    NavigationMenuLink
};
```

---

## Component API

### NavigationMenu

Root navigation container.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `Signal<String>` | **Required** | Current active item |
| `on_change` | `Option<Callback<String>>` | `None` | Change handler |

### NavigationMenuTrigger

Clickable menu item trigger.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `MaybeProp<String>` | `None` | Additional CSS classes |

### NavigationMenuContent

Dropdown content container.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `class` | `MaybeProp<String>` | `None` | Additional CSS classes |

---

## Usage Examples

### Basic Navigation Menu

```rust
use leptos::prelude::*;
use leptos_shadcn_navigation_menu::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    let (value, set_value) = signal("home".to_string());

    view! {
        <NavigationMenu
            value=value.into()
            on_change=Some(Callback::new(move |v| set_value.set(v)))
        >
            <NavigationMenuList>
                <NavigationMenuItem>
                    <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
                    <NavigationMenuContent>
                        <div>"Product links..."</div>
                    </NavigationMenuContent>
                </NavigationMenuItem>
            </NavigationMenuList>
        </NavigationMenu>
    }
}
```

---

## CSS Classes

```css
.navigation-menu {
    relative flex w-full items-center justify-center
}

.navigation-menu-list {
    flex flex-1 items-center justify-center space-x-1
}

.navigation-menu-trigger {
    inline-flex items-center justify-center rounded-md
    px-4 py-2 text-sm font-medium transition-colors
    hover:bg-accent hover:text-accent-foreground
    focus:bg-accent focus:text-accent-foreground
}
```

---

## Accessibility

### Keyboard Navigation

| Key | Action |
|-----|--------|
| **Arrow Keys** | Navigate items |
| **Enter/Space** | Select item |
| **Escape** | Close menu |

---

## TypeScript API

```typescript
interface NavigationMenuProps {
  value: string;
  onChange?: (value: string) => void;
  children?: React.ReactNode;
}

export const NavigationMenu: React.FC<NavigationMenuProps>;
export const NavigationMenuList: React.FC<ComponentProps>;
export const NavigationMenuItem: React.FC<ComponentProps>;
export const NavigationMenuTrigger: React.FC<ComponentProps>;
export const NavigationMenuContent: React.FC<ComponentProps>;
```

---

*Source: [packages/leptos/navigation-menu/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/navigation-menu/src/default.rs)*
