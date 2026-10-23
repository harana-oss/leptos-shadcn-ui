# Coverage Reporting Quick Reference

Quick reference guide for common coverage reporting tasks.

## Installation

```bash
cargo install cargo-llvm-cov
```

## Common Commands

### Generate Coverage Reports

```bash
# All formats (HTML, JSON, LCov)
just coverage-all

# HTML only (opens in browser)
just coverage-html

# Terminal summary
just coverage-terminal

# JSON for CI/CD
just coverage-json
```

### Component Coverage

```bash
# Specific component
just coverage-component button

# Multiple components
./scripts/generate_coverage_report.sh --package leptos-shadcn-button --package leptos-shadcn-input
```

### Verification

```bash
# Check if coverage meets threshold (95%)
just coverage-verify

# Custom threshold
./scripts/generate_coverage_report.sh --fail-under 90
```

### CI/CD

```bash
# CI mode (optimized for pipelines)
just coverage-ci

# Generate badges
just coverage-badge

# Full report with trends
just coverage-full
```

## File Locations

| File/Directory | Purpose |
|----------------|---------|
| `.coverage-rules.toml` | Coverage thresholds and quality gates |
| `.cargo/config.toml` | Coverage environment variables |
| `scripts/generate_coverage_report.sh` | Shell script for coverage generation |
| `scripts/coverage_reporter.py` | Python script for coverage analysis |
| `coverage-reports/` | Generated coverage reports |
| `coverage-reports/html/` | HTML coverage reports |
| `coverage-reports/badges/` | Coverage badges |
| `docs/testing/coverage-reporting.md` | Full documentation |

## Thresholds

- **Line Coverage**: 95% (minimum)
- **Branch Coverage**: 90% (minimum)
- **Function Coverage**: 100% (minimum)

## Report Formats

| Format | Location | Use Case |
|--------|----------|----------|
| HTML | `coverage-reports/html/index.html` | Interactive viewing |
| JSON | `coverage-reports/coverage.json` | CI/CD integration |
| LCov | `coverage-reports/coverage.lcov` | Codecov, other services |
| Terminal | stdout | Quick checks |

## CI/CD Integration

### GitHub Actions

Automatic on:
- Pull requests
- Pushes to main/develop
- Manual workflow dispatch

Features:
- PR comments with coverage summary
- Fails if coverage below threshold
- Uploads reports as artifacts
- Updates coverage badges

### Local Pre-commit Hook

```bash
# Install
just install-git-hooks

# Manual check
just coverage-verify
```

## Troubleshooting

### Low Coverage

```bash
# Check what's being tested
cargo test --workspace -- --list

# Use llvm-cov for comprehensive coverage
./scripts/generate_coverage_report.sh --tool llvm-cov
```

### Missing Reports

```bash
# Clean and regenerate
just clean
rm -rf coverage-reports/
just coverage-all
```

### CI Failures

1. Check workflow logs
2. Download coverage artifacts
3. Review HTML report
4. Compare with baseline

## Advanced Usage

### Custom Output Directory

```bash
./scripts/generate_coverage_report.sh --output-dir my-reports
```

### Specific Coverage Tool

```bash
./scripts/generate_coverage_report.sh --tool llvm-cov
./scripts/generate_coverage_report.sh --tool tarpaulin
```

### Trend Analysis

```bash
python3 scripts/coverage_reporter.py --trend
```

## Badge in README

```markdown
![Coverage](docs/badges/coverage.svg)
```

## Help

```bash
./scripts/generate_coverage_report.sh --help
python3 scripts/coverage_reporter.py --help
just  # List all recipes
```

## Environment Variables

```bash
export COVERAGE_MINIMUM_LINE_COVERAGE=95
export COVERAGE_MINIMUM_BRANCH_COVERAGE=90
export COVERAGE_MINIMUM_FUNCTION_COVERAGE=100
export CARGO_INCREMENTAL=0
```

## See Also

- [Full Documentation](coverage-reporting.md)
- [Testing Guide](testing-guide.md)
- [CI/CD Configuration](../ci/ci-configuration.md)
