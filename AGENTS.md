# AGENTS.md - Development Guide for leptos-shadcn-ui

## Build/Test Commands
- **Build**: `cargo build --workspace` (or `just build`)
- **Test workspace**: `cargo test --workspace --lib` (or `just test-rust`)
- **Test single component**: `cargo test -p leptos-shadcn-<component>` (e.g., `cargo test -p leptos-shadcn-button`)
- **Test with verbose**: `RUST_LOG=debug cargo test --workspace --lib -- --nocapture`
- **E2E tests**: `just test-e2e` or `pnpm playwright test`
- **Lint**: `cargo clippy --workspace -- -D warnings` (or `just lint`)
- **Format**: `cargo fmt --all` (or `just fmt`)
- **Check**: `cargo check --workspace` (or `just check`)

## Architecture
- **Workspace**: Monorepo; component crates live in `packages/leptos/<name>` (crate `leptos-shadcn-<name>`). Other workspace packages are under `packages/` (e.g. `leptos-shadcn-ui`, `leptos-shadcn-ui-wasm`, `registry`, `signal-management`, `test-utils`), plus `examples/`, `tests/`, `performance-audit/`, `build/scripts/`
- **Components**: 63 crates in `packages/leptos/` for Leptos 0.9.0-rc0: 58 UI components plus support crates (`utils`, `registry`, `analytics`, `error-boundary`, `lazy-loading`). Versions come from `[workspace.package]` (currently 0.9.1)
- **Structure**: Every crate has `src/lib.rs`. Most UI components also have `default.rs` and `new_york.rs` (57), `tests.rs` (54) and `signal_managed.rs` (44). Large components split the default theme into `src/default_components/` (e.g. drawer, select)
- **Testing**: TDD-focused; TDD tests live in `src/tdd_tests/` (directory, ~20 crates) or `src/tdd_tests.rs` (~13 crates)
- **Docs**: Per-component API pages in `docs/components/api/<name>.md`; run `build/scripts/check-docs-api.sh` after editing them
- **Performance**: Includes performance-audit system and benchmarking

## Code Style
- **Imports**: Leptos imports first (`use leptos::prelude::*;`), then utilities, external crates, internal
- **Naming**: kebab-case packages, PascalCase components, snake_case functions, SCREAMING_SNAKE_CASE constants
- **Components**: Use `#[component]` with optional props via `MaybeProp<T>`, `Option<Callback<T>>` for events
- **Props**: Standard props are `class`, `id`, `style`, `children`, with `#[prop(into, optional)]`
- **Errors**: Use validation system with `ValidationRule`, comprehensive error boundaries
- **Derive**: Standard order `#[derive(Debug, Clone, PartialEq)]` for most types
- **CSS**: Define class constants, use Signal::derive for dynamic classes
- **Documentation**: Module docs with `//!`, inline comments for accessibility and TDD enhancements
