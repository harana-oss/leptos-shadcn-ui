# Sidebar Component API

> **Stub page.** Generated from `packages/leptos/sidebar/src`; expand with prose and examples as the component matures.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-sidebar = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_sidebar::*;
```

---

## Exported Components

In addition to the components below, `lib.rs` defines simple slot wrappers via the `slot_div!` macro (`class`/`children` props): `SidebarHeader`, `SidebarFooter`, `SidebarContent`, `SidebarGroup`, `SidebarGroupLabel`, `SidebarGroupContent` (see `src/lib.rs` for the exact list).

### SidebarProvider

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `default_open` | `bool` | `true` |
| `open` | `Option<RwSignal<bool>>` | optional |
| `on_open_change` | `Option<Callback<bool>>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Children` | **required** |

### Sidebar

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `side` | `MaybeProp<SidebarSide>` | optional |
| `variant` | `MaybeProp<SidebarVariant>` | optional |
| `collapsible` | `MaybeProp<SidebarCollapsible>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `ChildrenFn` | **required** |

### SidebarTrigger

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `on_click` | `Option<Callback<ev::MouseEvent>>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

### SidebarRail

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

### SidebarInset

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### SidebarSeparator

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

### SidebarGroupAction

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `on_click` | `Option<Callback<ev::MouseEvent>>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### SidebarMenu

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### SidebarMenuItem

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### SidebarMenuButton

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `variant` | `MaybeProp<SidebarMenuButtonVariant>` | optional |
| `size` | `MaybeProp<SidebarMenuButtonSize>` | optional |
| `is_active` | `MaybeProp<bool>` | optional |
| `tooltip` | `MaybeProp<String>` | optional |
| `on_click` | `Option<Callback<ev::MouseEvent>>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### SidebarMenuAction

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `show_on_hover` | `MaybeProp<bool>` | optional |
| `on_click` | `Option<Callback<ev::MouseEvent>>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### SidebarMenuSkeleton

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `show_icon` | `MaybeProp<bool>` | optional |
| `width` | `MaybeProp<String>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |

### SidebarMenuSub

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### SidebarMenuSubItem

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

### SidebarMenuSubButton

Defined in `src/lib.rs`.

| Prop | Type | Default |
|------|------|---------|
| `href` | `MaybeProp<String>` | optional |
| `size` | `MaybeProp<String>` | optional |
| `is_active` | `MaybeProp<bool>` | optional |
| `class` | `MaybeProp<String>` | optional |
| `id` | `MaybeProp<String>` | optional |
| `style` | `MaybeProp<String>` | optional |
| `children` | `Option<Children>` | optional |

---

## Minimal Example

```rust
use leptos::prelude::*;
use leptos_shadcn_sidebar::*;

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <SidebarProvider>"Content"</SidebarProvider>
    }
}
```

---

## Source

[packages/leptos/sidebar](https://github.com/harana-oss/leptos-shadcn-ui/tree/main/packages/leptos/sidebar)
