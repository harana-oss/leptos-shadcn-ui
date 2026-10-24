# leptos-shadcn-ui

shadcn/ui components for [Leptos](https://leptos.dev) 0.9, styled with Tailwind CSS.

<!-- install:start -->
## Install

Components are distributed as a Git dependency. Each component is its own
crate named `leptos-shadcn-<component>`; add the ones you need, pinned to a
release tag (latest: `v0.9.1`).

With `cargo add`:

```sh
cargo add leptos-shadcn-button --git https://github.com/harana-oss/leptos-shadcn-ui --tag v0.9.1
```

Or in `Cargo.toml`:

```toml
[dependencies]
leptos-shadcn-button = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
leptos-shadcn-input  = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
leptos-shadcn-card   = { git = "https://github.com/harana-oss/leptos-shadcn-ui", tag = "v0.9.1" }
```

Cargo resolves each crate by name from the workspace at that tag. Use
`branch = "main"` instead of `tag` to track unreleased changes.
<!-- install:end -->

## Usage

```rust
use leptos::prelude::*;
use leptos_shadcn_button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <Button
            variant=ButtonVariant::Outline
            size=ButtonSize::Sm
            on_click=Callback::new(move |_| set_count.update(|n| *n += 1))
        >
            "Clicked " {count} " times"
        </Button>
    }
}
```

Most crates export a default style and a "New York" style variant
(for example `Button` and `ButtonNewYork`).

## Tailwind setup

Tailwind v4 only generates classes it finds in scanned files, and component
crates live outside your app directory. The `leptos-shadcn-tailwind` build
helper writes a stylesheet of `@source` directives pointing at every
`leptos-shadcn-*` crate in your dependency graph.

```sh
cargo add --build leptos-shadcn-tailwind
```

```rust
// build.rs
fn main() {
    leptos_shadcn_tailwind::write_sources_css("style/shadcn-sources.css")
        .expect("failed to write Tailwind sources for leptos-shadcn-ui");
}
```

```css
/* style/input.css */
@import "tailwindcss";
@import "./shadcn-sources.css";
```

## Components

Crates under `packages/leptos/`, each published as `leptos-shadcn-<name>`:

| | | | |
|---|---|---|---|
| accordion | alert | alert-dialog | aspect-ratio |
| avatar | badge | breadcrumb | button |
| button-group | calendar | card | carousel |
| chart | checkbox | collapsible | combobox |
| command | context-menu | date-picker | dialog |
| drawer | dropdown-menu | empty | field |
| form | hover-card | input | input-group |
| input-otp | item | kbd | label |
| menubar | native-select | navigation-menu | pagination |
| popover | progress | radio-group | resizable |
| scroll-area | select | separator | sheet |
| sidebar | skeleton | slider | sonner |
| spinner | switch | table | tabs |
| textarea | toast | toggle | toggle-group |
| tooltip | typography | | |

Supporting crates: `analytics`, `error-boundary`, `lazy-loading`, `registry`, `utils`.

## Development

```sh
cargo build --workspace                    # or: just build
cargo test --workspace --lib               # or: just test-rust
cargo test -p leptos-shadcn-button         # single component
cargo clippy --workspace -- -D warnings    # or: just lint
cargo fmt --all                            # or: just fmt
cargo check --workspace                    # or: just check
just test-e2e                              # Playwright E2E tests
```
