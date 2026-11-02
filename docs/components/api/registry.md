# Registry Component API

A component registry for managing and accessing components.

---

## Installation

```toml
# Cargo.toml
[dependencies]
leptos-shadcn-registry = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

> **Note:** the `0.9.0` crates on crates.io are the legacy cloud-shuttle line. The git dependency above (harana-oss, tag `v0.9.1`) is the current, maintained source.

```rust
use leptos_shadcn_registry::Registry;
```

---

## Component API

### Registry

The registry provides component discovery and metadata.

---

## Usage Examples

### Using the Registry

```rust
use leptos_shadcn_registry::Registry;

#[component]
pub fn MyComponent() -> impl IntoView {
    // Get component metadata
    let components = Registry::get_all_components();

    view! {
        <div>
            {components.iter().map(|c| {
                view! {
                    <div>
                        <h3>{c.name.clone()}</h3>
                        <p>{c.description.clone()}</p>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
```

---

## TypeScript API

```typescript
export const Registry: {
  getAllComponents: () => ComponentMetadata[];
  getComponent: (name: string) => ComponentMetadata | null;
};
```

---

*Source: [packages/leptos/registry/src/default.rs](https://github.com/harana-oss/leptos-shadcn-ui/blob/main/packages/leptos/registry/src/default.rs)*
