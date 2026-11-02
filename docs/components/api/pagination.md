# Pagination Component API

A navigation component for browsing through pages of content.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-pagination = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_pagination::Pagination;
```

---

## Import

```rust
use leptos_shadcn_pagination::{
    Pagination,
    PaginationContent,
    PaginationItem,
    PaginationPrevious,
    PaginationNext,
    PaginationLink,
    PaginationEllipsis
};
```

---

## Component API

### Pagination

Root container for pagination controls.

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `current_page` | `Signal<usize>` | **Required** | Current page number |
| `total_pages` | `Signal<usize>` | **Required** | Total pages |
| `on_change` | `Option<Callback<usize>>` | `None` | Page change handler |

---

## Usage Examples

### Basic Pagination

```rust
use leptos::prelude::*;
use leptos_shadcn_pagination::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    let (current_page, set_current_page) = signal(1);
    let total_pages = Signal::derive(|| 10);

    view! {
        <Pagination
            current_page=current_page.into()
            total_pages=total_pages
            on_change=Some(Callback::new(move |page| set_current_page.set(page)))
        >
            <PaginationContent>
                <PaginationItem>
                    <PaginationPrevious />
                </PaginationItem>
                <PaginationItem>
                    <PaginationLink page=1>"1"</PaginationLink>
                </PaginationItem>
                <PaginationItem>
                    <PaginationNext />
                </PaginationItem>
            </PaginationContent>
        </Pagination>
    }
}
```

---

## CSS Classes

```css
.pagination {
    flex items-center justify-between
}

.pagination-link {
    inline-flex items-center justify-center
    rounded-md text-sm font-medium transition-colors
    hover:bg-accent hover:text-accent-foreground
}
```

---

## Accessibility

### ARIA Attributes

- `aria-label` - Pagination labels
- `aria-current="page"` - Current page indicator

---

## TypeScript API

```typescript
interface PaginationProps {
  currentPage: number;
  totalPages: number;
  onChange?: (page: number) => void;
}

export const Pagination: React.FC<PaginationProps>;
export const PaginationContent: React.FC<ComponentProps>;
export const PaginationLink: React.FC<{ page: number }>;
export const PaginationPrevious: React.FC<ComponentProps>;
export const PaginationNext: React.FC<ComponentProps>;
```

---

*Source: [packages/leptos/pagination/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/pagination/src/default.rs)*
