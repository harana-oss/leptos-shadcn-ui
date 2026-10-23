# 🧪 Enhanced WASM Browser Testing Guide

> **Comprehensive WASM Testing Infrastructure for leptos-shadcn-ui**

## 📋 Overview

Our enhanced WASM browser testing infrastructure provides comprehensive validation of WebAssembly functionality across all supported browsers, ensuring reliable performance and compatibility for production-ready components.

## 🎯 Key Features

- **Cross-Browser Testing**: Chrome, Firefox, Safari, Mobile Chrome, Mobile Safari
- **Performance Monitoring**: Initialization time, memory usage, interaction latency
- **Memory Management**: Leak detection and memory pressure testing
- **Error Handling**: Graceful error recovery and failure scenarios
- **Bundle Analysis**: WASM bundle size and loading optimization
- **Automated Reporting**: HTML, JSON, and Markdown test reports

## 🚀 Quick Start

### Basic Usage

```bash
# Run all WASM tests with default settings
just test-wasm-browser

# Run tests on specific browsers
just test-wasm-browsers chromium,firefox

# Run tests in headed mode (see browser windows)
just test-wasm-headed

# Run tests in parallel for faster execution
just test-wasm-parallel

# Run tests with verbose output
just test-wasm-verbose
```

### Advanced Usage

```bash
# Run specific test scenarios
./scripts/run-wasm-tests.sh -s basic-initialization,memory-management

# Run on specific browsers with custom settings
./scripts/run-wasm-tests.sh -b chromium,firefox -H -v

# Run in parallel without generating reports
./scripts/run-wasm-tests.sh -p -r
```

## 🏗️ Test Architecture

### Test Structure

```
tests/e2e/
├── wasm-browser-testing.spec.ts    # Main WASM test suite
├── wasm-performance-monitor.ts     # Performance monitoring utility
├── wasm-test-config.ts            # Configuration management
└── wasm-test-results/             # Test results and reports
```

### Test Categories

#### 1. **WASM Initialization & Loading**
- ✅ Successful initialization across browsers
- ✅ Error handling for failed loads
- ✅ Initialization time measurement
- ✅ Loading state management

#### 2. **Memory Management**
- ✅ Memory leak detection
- ✅ Memory pressure handling
- ✅ Memory usage monitoring
- ✅ Garbage collection validation

#### 3. **Cross-Browser Compatibility**
- ✅ Consistent behavior across browsers
- ✅ Browser-specific feature detection
- ✅ WASM capability validation
- ✅ Fallback mechanism testing

#### 4. **Performance Monitoring**
- ✅ Performance benchmark validation
- ✅ Load time measurement
- ✅ Interaction latency testing
- ✅ Performance regression detection

#### 5. **Error Handling & Recovery**
- ✅ Runtime error handling
- ✅ Failure recovery testing
- ✅ Error boundary validation
- ✅ Graceful degradation

#### 6. **Bundle Analysis**
- ✅ WASM bundle size validation
- ✅ Loading efficiency testing
- ✅ Network request monitoring
- ✅ Resource optimization

## 📊 Performance Benchmarks

### Default Thresholds

| Metric | Threshold | Description |
|--------|-----------|-------------|
| **Initialization Time** | 5 seconds | Maximum time for WASM to initialize |
| **First Paint** | 3 seconds | Maximum time to first visual content |
| **First Contentful Paint** | 4 seconds | Maximum time to meaningful content |
| **Interaction Latency** | 100ms | Maximum average interaction response time |
| **Memory Increase** | 50% | Maximum memory increase during operations |

### Browser-Specific Thresholds

| Browser | Init Time | First Paint | Notes |
|---------|-----------|-------------|-------|
| **Chrome** | 5s | 3s | Standard thresholds |
| **Firefox** | 6s | 3s | Slightly more time for initialization |
| **Safari** | 7s | 3.5s | More conservative thresholds |
| **Mobile Chrome** | 8s | 4s | Mobile-optimized thresholds |
| **Mobile Safari** | 10s | 5s | Most conservative for mobile Safari |

## 🔧 Configuration

### Environment Variables

```bash
# Performance thresholds
export WASM_MAX_INIT_TIME=5000
export WASM_MAX_FIRST_PAINT=3000
export WASM_MAX_FCP=4000
export WASM_MAX_INTERACTION_LATENCY=100
export WASM_MAX_MEMORY_INCREASE=50

# Browser selection
export WASM_ENABLED_BROWSERS="chromium,firefox,webkit"

# Scenario selection
export WASM_ENABLED_SCENARIOS="basic-initialization,memory-management"

# Reporting
export WASM_OUTPUT_DIR="test-results/wasm-tests"
export WASM_GENERATE_HTML_REPORT=true
export WASM_GENERATE_JSON_REPORT=true
export WASM_GENERATE_MARKDOWN_REPORT=true
```

### Custom Configuration

Create a custom configuration file:

```typescript
// wasm-test-config.custom.ts
import { WASMTestConfig } from './wasm-test-config';

export const customConfig: WASMTestConfig = {
  performance: {
    maxInitializationTime: 3000, // Stricter for CI
    maxFirstPaint: 2000,
    maxFirstContentfulPaint: 3000,
    maxInteractionLatency: 50,
    maxMemoryIncrease: 25,
  },
  browsers: {
    chromium: { enabled: true, timeout: 20000, retries: 1 },
    firefox: { enabled: false }, // Skip Firefox in CI
    webkit: { enabled: true, timeout: 25000, retries: 2 },
  },
  // ... other settings
};
```

## 📈 Test Reports

### Report Types

1. **HTML Reports**: Interactive test results with screenshots and videos
2. **JSON Reports**: Machine-readable data for CI/CD integration
3. **Markdown Reports**: Human-readable summaries with performance metrics

### Report Structure

```
test-results/wasm-tests/
├── wasm-test-report-20241213_143022.md    # Main summary report
├── chromium/                              # Browser-specific results
│   ├── results.json
│   ├── results.html
│   └── screenshots/
├── firefox/
│   ├── results.json
│   ├── results.html
│   └── screenshots/
└── webkit/
    ├── results.json
    ├── results.html
    └── screenshots/
```

### Sample Report

```markdown
# WASM Browser Testing Report

**Generated**: 2024-12-13T14:30:22Z
**Test Configuration**:
- Browsers: chromium,firefox,webkit
- Scenarios: basic-initialization,memory-management,performance-monitoring
- Headless Mode: true
- Parallel Execution: false

## Test Results Summary

### chromium
- **Passed**: 15
- **Failed**: 0
- **Skipped**: 0

### firefox
- **Passed**: 14
- **Failed**: 1
- **Skipped**: 0

### webkit
- **Passed**: 13
- **Failed**: 2
- **Skipped**: 0
```

## 🐛 Debugging

### Common Issues

#### 1. **WASM Initialization Failures**

```bash
# Check WASM target installation
rustup target list --installed | grep wasm32-unknown-unknown

# Reinstall WASM target if missing
rustup target add wasm32-unknown-unknown

# Check browser console for errors
just test-wasm-headed
```

#### 2. **Performance Threshold Failures**

```bash
# Run with verbose output to see detailed metrics
just test-wasm-verbose

# Check specific browser performance
./scripts/run-wasm-tests.sh -b chromium -v
```

#### 3. **Memory Issues**

```bash
# Monitor memory usage during tests
./scripts/run-wasm-tests.sh -s memory-management -v

# Check for memory leaks
./scripts/run-wasm-tests.sh -s memory-management -H
```

### Debug Mode

```bash
# Run tests in debug mode with browser inspector
pnpm playwright test tests/e2e/wasm-browser-testing.spec.ts --debug

# Run specific test in debug mode
pnpm playwright test tests/e2e/wasm-browser-testing.spec.ts --debug --grep "should initialize WASM successfully"
```

## 🔄 CI/CD Integration

### GitHub Actions

```yaml
name: WASM Browser Testing

on: [push, pull_request]

jobs:
  wasm-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
          
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
          
      - name: Install dependencies
        run: |
          pnpm install
          pnpm playwright install
          
      - name: Run WASM tests
        run: just test-wasm-browser
        env:
          WASM_MAX_INIT_TIME: 8000  # More lenient in CI
          WASM_ENABLED_BROWSERS: "chromium,firefox"
          
      - name: Upload test results
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: wasm-test-results
          path: test-results/wasm-tests/
```

### Local Development

```bash
# Pre-commit hook for WASM testing
echo '#!/bin/sh
just test-wasm-browsers chromium
' > .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

## 📚 Best Practices

### 1. **Test Strategy**
- Run WASM tests on every component change
- Use parallel execution for faster feedback
- Monitor performance trends over time
- Test on multiple browsers before releases

### 2. **Performance Optimization**
- Set realistic performance thresholds
- Monitor memory usage patterns
- Optimize WASM bundle sizes
- Use browser-specific optimizations

### 3. **Error Handling**
- Test error scenarios thoroughly
- Implement graceful fallbacks
- Monitor error rates in production
- Document known limitations

### 4. **Maintenance**
- Update browser versions regularly
- Review and adjust performance thresholds
- Monitor test execution times
- Keep dependencies up to date

## 🎯 Future Enhancements

### Planned Features

- **Visual Regression Testing**: Automated screenshot comparison
- **Performance Budgets**: Enforce performance thresholds in CI
- **Real Device Testing**: Test on actual mobile devices
- **WASM Profiling**: Detailed performance profiling integration
- **Automated Optimization**: AI-powered performance recommendations

### Integration Opportunities

- **Storybook Integration**: Component story testing
- **Design System Testing**: Visual consistency validation
- **API Testing**: Backend integration testing
- **Load Testing**: High-traffic scenario testing

## 📞 Support

For issues or questions about WASM testing:

1. Check the [troubleshooting section](#-debugging)
2. Review test reports for specific failures
3. Run tests in debug mode for detailed analysis
4. Check browser console for error messages
5. Verify WASM target installation and browser compatibility

---

**Last Updated**: December 2024  
**Version**: 1.0.0  
**Maintainer**: leptos-shadcn-ui Team
