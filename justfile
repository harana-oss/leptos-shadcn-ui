# leptos-shadcn-ui task runner. Run `just` to list recipes.

# List available recipes
default:
    @just --list

# Run all tests (native target – fast, default)
test:
    cargo nextest run --workspace --target aarch64-apple-darwin

# Run only native tests (explicitly)
test-native: test

# Run WASM tests only for client‑side component libraries

# (no --workspace, avoiding native‑only crates that pull in mio)
test-wasm:
    #!/usr/bin/env bash
    set -euo pipefail
    pkgs=""
    for dir in packages/leptos/*/; do
        if [ -f "${dir}Cargo.toml" ]; then
            pkgid=$(cargo pkgid --manifest-path "${dir}Cargo.toml" 2>/dev/null || true)
            if [ -n "$pkgid" ]; then
                pkgs="$pkgs --package $pkgid"
            fi
        fi
    done
    # Add the WASM entry point
    pkgid=$(cargo pkgid --manifest-path packages/leptos-shadcn-ui-wasm/Cargo.toml)
    pkgs="$pkgs --package $pkgid"
    cargo nextest run --target wasm32-unknown-unknown $pkgs

# Run all tests (native + WASM)
test-all: test-native test-wasm

# Run tests with coverage (native only)
coverage:
    cargo llvm-cov nextest --workspace --target aarch64-apple-darwin --html

# Quick check native compilation
check-native:
    cargo check --workspace --target aarch64-apple-darwin

# Quick check WASM compilation (for relevant crates)
check-wasm:
    cargo check --workspace --target wasm32-unknown-unknown -p leptos-shadcn-ui-wasm

# Format code
fmt:
    cargo fmt --all

# Run clippy lints
clippy:
    cargo clippy --workspace --target aarch64-apple-darwin -- -D warnings

# File watching (your original recipe)
wr:
    watchexec -w ./wr.sh --clear -r "sh ./wr.sh"

# ---------------------------------------------------------------------------
# Development
# ---------------------------------------------------------------------------

# Start development environment with file watching
dev:
    cargo watch -x "check --workspace" -x "test --workspace --lib"

# Run example applications
dev-examples:
    cd book-examples/leptos && trunk serve --open

# ---------------------------------------------------------------------------
# Building
# ---------------------------------------------------------------------------

# Build all packages and examples
build:
    cargo build --workspace
    cargo build --workspace --target wasm32-unknown-unknown

# Build optimized release versions
build-release:
    cargo build --workspace --release
    cargo build --workspace --release --target wasm32-unknown-unknown

# Build example applications
build-examples:
    cd book-examples/leptos && trunk build --release
    cd book-examples/yew && trunk build --release

# ---------------------------------------------------------------------------
# Testing
# ---------------------------------------------------------------------------

# Run all tests (Rust + E2E)
test-full: test-rust test-e2e

# Run Rust unit and integration tests
test-rust:
    cargo test --workspace --lib
    cargo test --workspace --target wasm32-unknown-unknown

# Run Rust tests with verbose output
test-rust-verbose:
    RUST_LOG=debug cargo test --workspace --lib -- --nocapture

# Run tests for a specific component (e.g. just test-single button)
test-single component:
    cargo test -p leptos-shadcn-{{ component }}

# Run Playwright E2E tests
test-e2e: install-playwright
    pnpm playwright test

# Run Playwright tests in headed mode
test-e2e-headed:
    pnpm playwright test --headed

# Run Playwright E2E tests with UI
test-e2e-ui:
    pnpm playwright test --ui

# Run E2E tests in debug mode
test-e2e-debug:
    DEBUG=true HEADLESS=false pnpm playwright test --config=playwright.config.ts

# Run specific E2E test file (e.g. just test-e2e-specific tests/e2e/foo.spec.ts)
test-e2e-specific file:
    pnpm playwright test {{ file }}

# Run E2E tests in specific browser (e.g. just test-e2e-browser chromium)
test-e2e-browser browser:
    pnpm playwright test --project={{ browser }}

# Run E2E tests in parallel
test-e2e-parallel:
    pnpm playwright test --workers=4

# Show E2E test report
test-e2e-report:
    pnpm playwright show-report

# Install Playwright browsers
test-e2e-install:
    pnpm playwright install

# Generate E2E test code
test-e2e-codegen:
    pnpm playwright codegen http://127.0.0.1:8080

# Run enhanced E2E tests with comprehensive reporting
test-e2e-enhanced:
    pnpm playwright test --config=playwright.config.ts

# Run E2E tests in CI mode
test-e2e-ci:
    CI=true pnpm playwright test --config=playwright.config.ts

# Run E2E performance tests only
test-e2e-performance:
    pnpm playwright test --project=performance-tests

# Run E2E accessibility tests only
test-e2e-accessibility:
    pnpm playwright test --project=accessibility-tests

# Run E2E WASM tests only
test-e2e-wasm:
    pnpm playwright test --project=wasm-tests

# ---------------------------------------------------------------------------
# WASM browser testing
# ---------------------------------------------------------------------------

# Run comprehensive WASM browser tests
test-wasm-browser:
    ./build/scripts/run-wasm-tests.sh

# Run WASM tests on specific browsers (e.g. just test-wasm-browsers chromium,firefox)
test-wasm-browsers browsers:
    ./build/scripts/run-wasm-tests.sh -b "{{ browsers }}"

# Run WASM tests in headed mode
test-wasm-headed:
    ./build/scripts/run-wasm-tests.sh -H

# Run WASM tests in parallel
test-wasm-parallel:
    ./build/scripts/run-wasm-tests.sh -p

# Run WASM tests with verbose output
test-wasm-verbose:
    ./build/scripts/run-wasm-tests.sh -v

# ---------------------------------------------------------------------------
# Performance benchmarking
# ---------------------------------------------------------------------------

# Run performance benchmarks
benchmark:
    ./build/scripts/run-performance-benchmarks.sh benchmark

# Run benchmarks for specific components (e.g. just benchmark-components button,input)
benchmark-components components:
    ./build/scripts/run-performance-benchmarks.sh benchmark -c "{{ components }}"

# Run benchmarks and generate HTML report
benchmark-html:
    ./build/scripts/run-performance-benchmarks.sh benchmark -f html -o test-results/performance/benchmark-report.html

# Run performance regression tests
regression-test:
    ./build/scripts/run-performance-benchmarks.sh regression

# Run regression tests and update baseline
regression-update:
    ./build/scripts/run-performance-benchmarks.sh regression -u

# Start automated performance monitoring
performance-monitor:
    ./build/scripts/run-performance-benchmarks.sh monitor

# Start monitoring with alerts enabled
performance-monitor-alerts:
    ./build/scripts/run-performance-benchmarks.sh monitor -a

# Setup performance baseline
setup-baseline:
    ./build/scripts/run-performance-benchmarks.sh setup

# Generate performance report
performance-report:
    ./build/scripts/run-performance-benchmarks.sh report

# ---------------------------------------------------------------------------
# Accessibility
# ---------------------------------------------------------------------------

# Run comprehensive accessibility audit
accessibility-audit:
    ./build/scripts/run-accessibility-audit.sh

# Run accessibility audit with specific WCAG level (e.g. just accessibility-audit-wcag AAA)
accessibility-audit-wcag level:
    ./build/scripts/run-accessibility-audit.sh -l "{{ level }}"

# Run accessibility audit for specific components (e.g. just accessibility-audit-components button,input)
accessibility-audit-components components:
    ./build/scripts/run-accessibility-audit.sh -c "{{ components }}"

# Run accessibility audit and generate HTML report
accessibility-audit-html:
    ./build/scripts/run-accessibility-audit.sh -f html -o test-results/accessibility/accessibility-report.html

# Run accessibility audit with verbose output
accessibility-audit-verbose:
    ./build/scripts/run-accessibility-audit.sh -v

# Run accessibility audit focusing on focus management
accessibility-audit-focus:
    ./build/scripts/run-accessibility-audit.sh --no-color-contrast --no-screen-reader

# ---------------------------------------------------------------------------
# Coverage reporting
# ---------------------------------------------------------------------------

# Generate comprehensive coverage reports (HTML, JSON, LCov)
coverage-all:
    ./build/scripts/generate_coverage_report.sh --format all

# Generate HTML coverage report and open it
coverage-html:
    ./build/scripts/generate_coverage_report.sh --format html --open

# Generate JSON coverage report
coverage-json:
    ./build/scripts/generate_coverage_report.sh --format json

# Generate LCov coverage report
coverage-lcov:
    ./build/scripts/generate_coverage_report.sh --format lcov

# Generate terminal coverage report
coverage-terminal:
    ./build/scripts/generate_coverage_report.sh --format terminal

# Generate coverage for specific component (e.g. just coverage-component button)
coverage-component component:
    ./build/scripts/generate_coverage_report.sh --package leptos-shadcn-{{ component }}

# Open HTML coverage report in browser
coverage-open: coverage-html

# Generate coverage reports in CI mode
coverage-ci:
    ./build/scripts/generate_coverage_report.sh --ci --fail-under 95

# Verify coverage meets minimum thresholds
coverage-verify:
    ./build/scripts/generate_coverage_report.sh --fail-under 95 --ci

# Generate coverage badge
coverage-badge:
    python3 build/scripts/coverage_reporter.py --format json --output-dir coverage-reports/badges

# Analyze coverage trends over time
coverage-trend:
    python3 build/scripts/coverage_reporter.py --trend --format terminal

# Generate all coverage reports and badges
coverage-full: coverage-all coverage-badge coverage-trend

# ---------------------------------------------------------------------------
# Production readiness and WASM size
# ---------------------------------------------------------------------------

# Analyze bundle sizes and optimization opportunities
analyze-bundle:
    ./build/scripts/analyze_bundle.sh

# Build production-optimized version
build-production:
    ./build/scripts/build_production.sh

# Build WASM with size optimizations (<2MB target)
build-wasm-optimized:
    ./build/scripts/build-wasm-optimized.sh

# Build WASM release with maximum size optimizations
build-wasm-release:
    cargo build --profile wasm-release --target wasm32-unknown-unknown -p leptos-shadcn-ui-wasm

# List the 10 largest WASM files without building
check-wasm-size:
    @find . -name "*.wasm" -type f -exec sh -c 'echo "$(stat -c%s "$1" 2>/dev/null || stat -f%z "$1" 2>/dev/null) bytes - $1"' _ {} \; 2>/dev/null | sort -n | tail -10

# Measure and report WASM bundle size against the 2MB target
measure-wasm:
    #!/usr/bin/env bash
    set -euo pipefail
    f=target/wasm32-unknown-unknown/release/leptos_shadcn_ui_wasm.wasm
    if [ ! -f "$f" ]; then
        echo "No WASM file found. Run 'just build-wasm-optimized' first."
        exit 0
    fi
    size=$(stat -c%s "$f" 2>/dev/null || stat -f%z "$f")
    echo "Size: $size bytes ($(awk "BEGIN {printf \"%.2f\", $size/1048576}") MB)"
    if [ "$size" -le 2097152 ]; then echo "SUCCESS: Under 2MB target"; else echo "FAILED: Exceeds 2MB target"; fi

# Build optimized WASM and check size
optimize-wasm: build-wasm-optimized check-wasm-size

# Complete production readiness check
production-check: analyze-bundle build-production

# ---------------------------------------------------------------------------
# Quality and linting
# ---------------------------------------------------------------------------

# Run cargo check on all packages (native + WASM)
check:
    cargo check --workspace
    cargo check --workspace --target wasm32-unknown-unknown

# Run clippy linting (native + WASM)
lint:
    cargo clippy --workspace -- -D warnings
    cargo clippy --workspace --target wasm32-unknown-unknown -- -D warnings

# Check if code is formatted
fmt-check:
    cargo fmt --all -- --check

# Run security audit
audit:
    cargo audit

# ---------------------------------------------------------------------------
# Dependencies
# ---------------------------------------------------------------------------

# Install all dependencies
install-deps:
    pnpm install

# Install Playwright and browsers
install-playwright:
    pnpm create playwright@latest --yes
    pnpm playwright install

# Complete Playwright setup
setup-playwright: install-deps install-playwright

# ---------------------------------------------------------------------------
# Documentation
# ---------------------------------------------------------------------------

# Generate and open documentation
docs:
    cargo doc --workspace --open

# Build and serve the documentation book
docs-book:
    cd book && mdbook serve --open

# ---------------------------------------------------------------------------
# Maintenance
# ---------------------------------------------------------------------------

# Clean build artifacts
clean:
    cargo clean
    rm -rf book-examples/*/dist
    rm -rf node_modules
    rm -rf .playwright-browsers

# Clean cargo cache and lock files
clean-cache:
    cargo clean
    rm -f Cargo.lock
    rm -f package-lock.json

# Update all dependencies
update-deps:
    cargo update
    pnpm update

# Generate a new component (e.g. just generate-component my-component leptos)
generate-component name framework:
    cargo run --bin component-generator -- --name {{ name }} --framework {{ framework }}

# Enter Nix development shell
nix-develop:
    nix develop

# Build using Nix
nix-build:
    nix build

# Fix file permissions
fix-permissions:
    find . -name "*.rs" -type f -exec chmod 644 {} \;
    find . -name "*.toml" -type f -exec chmod 644 {} \;
    find scripts -name "*.sh" -type f -exec chmod +x {} \;

# Install git pre-commit hook
install-git-hooks:
    printf '#!/bin/sh\njust fmt-check && just check && just test-rust\n' > .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit

# Show environment information
env-info:
    #!/usr/bin/env bash
    echo "Rust version: $(rustc --version 2>/dev/null || echo 'Not installed')"
    echo "Cargo version: $(cargo --version 2>/dev/null || echo 'Not installed')"
    echo "Node.js version: $(node --version 2>/dev/null || echo 'Not installed')"
    echo "pnpm version: $(pnpm --version 2>/dev/null || echo 'Not installed')"
    echo "just version: $(just --version 2>/dev/null || echo 'Not installed')"
    echo "Git version: $(git --version 2>/dev/null || echo 'Not installed')"
    echo "Nix version: $(nix --version 2>/dev/null || echo 'Not installed')"
