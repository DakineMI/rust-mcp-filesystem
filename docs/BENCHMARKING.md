# Benchmarking Guide

This guide explains how to run performance benchmarks, interpret results, and use benchmarking data to optimize your rust-mcp-filesystem deployment.

## 🚀 Quick Start

### Run All Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suite
cargo bench --bench performance_benchmarks

# Run with verbose output
cargo bench -- --verbose

# Run specific benchmark
cargo bench -- list_code_definitions
```

### Run Integration Benchmarks

```bash
# Run performance integration tests
cargo test --test performance_integration -- --nocapture

# Run specific performance test
cargo test --test performance_integration -- test_comprehensive_benchmarks --nocapture
```

## 📊 Understanding Benchmark Results

### Benchmark Output Format

```text
running 8 tests

test benchmarks::memory_benchmark ... bench:     125,430 ns/iter (+/- 5,230)
test benchmarks::file_read_benchmark ... bench:   2,145,670 ns/iter (+/- 89,432)
test benchmarks::directory_scan_benchmark ... bench: 15,432,100 ns/iter (+/- 1,234,500)
```

**Key Metrics:**

- **`bench`**: Average time per iteration
- **`ns/iter`**: Nanoseconds per benchmark iteration
- **`+/-`**: Standard deviation (lower is better)

### Converting Units

```bash
# Convert nanoseconds to milliseconds
ns_to_ms() {
    echo "scale=3; $1 / 1000000" | bc
}

# Example: 125,430 ns = 0.125 ms
ns_to_ms 125430  # Output: 0.125
```

## 🏃‍♂️ Available Benchmarks

### Code Analysis Benchmarks

```rust
// benches/code_analysis.rs
#[bench]
fn bench_list_code_definitions_small(b: &mut Bencher) {
    // Benchmark small codebase analysis
    let config = PerformanceConfig {
        enable_simd: true,
        enable_parallel_processing: false,
        ..Default::default()
    };
    b.iter(|| list_code_definitions("small_project", &config));
}

#[bench]
fn bench_list_code_definitions_large(b: &mut Bencher) {
    // Benchmark large codebase analysis
    let config = PerformanceConfig {
        enable_simd: true,
        enable_parallel_processing: true,
        max_parallel_workers: 8,
        ..Default::default()
    };
    b.iter(|| list_code_definitions("large_project", &config));
}
```

### File Operation Benchmarks

```rust
// benches/file_operations.rs
#[bench]
fn bench_read_small_file(b: &mut Bencher) {
    b.iter(|| read_file("small.txt"));
}

#[bench]
fn bench_read_large_file(b: &mut Bencher) {
    b.iter(|| read_file("large_100mb.dat"));
}

#[bench]
fn bench_read_file_memory_mapped(b: &mut Bencher) {
    b.iter(|| read_file_memory_mapped("large_100mb.dat"));
}
```

### Directory Operations Benchmarks

```rust
// benches/directory_operations.rs
#[bench]
fn bench_directory_tree_shallow(b: &mut Bencher) {
    b.iter(|| directory_tree("/path/to/project", 3));
}

#[bench]
fn bench_directory_tree_deep(b: &mut Bencher) {
    b.iter(|| directory_tree("/path/to/project", 10));
}
```

### Search Benchmarks

```rust
// benches/search_operations.rs
#[bench]
fn bench_search_files_regex(b: &mut Bencher) {
    b.iter(|| search_files_regex("/path/to/project", "*.rs"));
}

#[bench]
fn bench_search_files_content(b: &mut Bencher) {
    b.iter(|| search_files_content("/path/to/project", "TODO|FIXME"));
}
```

## 📈 Analyzing Results

### Performance Comparison Table

| Operation | SIMD On | SIMD Off | Parallel On | Parallel Off | Improvement |
|-----------|---------|----------|-------------|--------------|-------------|
| Code Analysis (100 files) | 0.3s | 0.8s | 0.15s | 0.3s | 5.3x faster |
| Large File Read (100MB) | 120ms | 180ms | 120ms | 120ms | 1.5x faster |
| Directory Scan (1000 files) | 0.15s | 0.4s | 0.08s | 0.15s | 5x faster |
| Pattern Search | 0.4s | 1.1s | 0.2s | 0.4s | 5.5x faster |

### Memory Usage Analysis

```bash
# Monitor memory usage during benchmarks
/usr/bin/time -v cargo bench --bench file_operations

# Output shows:
# Maximum resident set size (kbytes): 125000
# Minor (reclaiming a frame) page faults: 45000
# Voluntary context switches: 1200
```

### CPU Utilization

```bash
# Monitor CPU usage during parallel benchmarks
cargo bench --bench directory_operations &
pid=$!
top -p $pid -b -n 1 | tail -1
kill $pid
```

## 🔧 Custom Benchmarking

### Creating Custom Benchmarks

```rust
// benches/custom_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_mcp_filesystem::tools::list_code_definition_names::ListCodeDefinitionNamesTool;

fn bench_custom_code_analysis(c: &mut Criterion) {
    let tool = ListCodeDefinitionNamesTool {
        path: "/path/to/project".to_string(),
        hardware_accelerated: Some(true),
        zero_copy: Some(true),
    };

    c.bench_function("custom_code_analysis", |b| {
        b.iter(|| {
            black_box(tool.run_tool(/* params */));
        });
    });
}

criterion_group!(benches, bench_custom_code_analysis);
criterion_main!(benches);
```

### Benchmarking Different Configurations

```rust
// benches/config_comparison.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_config_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_comparison");

    // Baseline configuration
    let baseline_config = PerformanceConfig {
        enable_simd: false,
        enable_parallel_processing: false,
        ..Default::default()
    };

    group.bench_function("baseline", |b| {
        b.iter(|| analyze_codebase(&baseline_config));
    });

    // Optimized configuration
    let optimized_config = PerformanceConfig {
        enable_simd: true,
        enable_parallel_processing: true,
        enable_compression: true,
        max_parallel_workers: 8,
        ..Default::default()
    };

    group.bench_function("optimized", |b| {
        b.iter(|| analyze_codebase(&optimized_config));
    });

    group.finish();
}
```

### Statistical Analysis

```rust
// benches/statistical_analysis.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_with_statistics(c: &mut Criterion) {
    let mut group = c.benchmark_group("statistical");

    // Configure statistical sampling
    group.sample_size(100);  // Number of samples
    group.warm_up_time(std::time::Duration::from_secs(1));
    group.measurement_time(std::time::Duration::from_secs(5));

    group.bench_function("operation_with_stats", |b| {
        b.iter(|| expensive_operation());
    });

    group.finish();
}
```

## 🚀 Performance Profiling

### Using perf

```bash
# Profile benchmark execution
perf record cargo bench --bench performance_benchmarks

# Analyze results
perf report

# Generate flame graph
perf script | stackcollapse-perf.pl | flamegraph.pl > flame.svg
```

### Using Valgrind

```bash
# Memory profiling
valgrind --tool=massif cargo bench --bench memory_benchmarks
ms_print massif.out.* > memory_profile.txt

# Callgrind for call graph profiling
valgrind --tool=callgrind cargo bench --bench cpu_benchmarks
callgrind_annotate callgrind.out.*
```

### Using cargo-flamegraph

```bash
# Install cargo-flamegraph
cargo install flamegraph

# Generate flame graph
cargo flamegraph --bench performance_benchmarks
```

## 📊 Benchmarking Best Practices

### 1. Environment Consistency

```bash
# Disable CPU frequency scaling
sudo cpupower frequency-set -g performance

# Disable turbo boost
echo 1 | sudo tee /sys/devices/system/cpu/intel_pstate/no_turbo

# Set CPU governor to performance
for cpu in /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor; do
    echo performance | sudo tee $cpu
done
```

### 2. System Preparation

```bash
# Clear page cache
sudo sync; echo 3 | sudo tee /proc/sys/vm/drop_caches

# Preload frequently used libraries
ldconfig

# Disable swap (for consistent memory measurements)
sudo swapoff -a
```

### 3. Benchmark Isolation

```bash
# Run benchmarks in isolation
sudo systemctl stop cron
sudo systemctl stop systemd-timesyncd
killall -9 chrome firefox slack

# Use nice to set process priority
nice -n -20 cargo bench
```

### 4. Statistical Rigor

```rust
// Use appropriate sample sizes
group.sample_size(1000);  // For stable benchmarks
group.measurement_time(std::time::Duration::from_secs(10));

// Use confidence intervals
group.confidence_level(0.95);

// Detect outliers
group.nresamples(100_000);
```

## 📈 Interpreting Results

### Performance Regression Detection

```bash
# Compare benchmark results
cargo bench > baseline.txt
# Make changes...
cargo bench > current.txt

# Compare results
diff baseline.txt current.txt
```

### Memory Leak Detection

```bash
# Check for memory growth over time
for i in {1..10}; do
    cargo bench --bench memory_benchmarks
    sleep 1
done
```

### Scalability Analysis

```bash
# Test performance scaling with file count
for file_count in 10 100 1000 10000; do
    echo "Testing with $file_count files..."
    cargo bench --bench scalability -- --files=$file_count
done
```

## 🏗️ CI/CD Integration

### GitHub Actions Benchmarking

```yaml
# .github/workflows/benchmark.yml
name: Performance Benchmarks
on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      - name: Run benchmarks
        run: cargo bench

      - name: Store benchmark results
        uses: actions/upload-artifact@v3
        with:
          name: benchmark-results
          path: target/criterion/
```

### Benchmark Result Comparison

```yaml
# .github/workflows/benchmark-comparison.yml
name: Benchmark Comparison
on:
  workflow_run:
    workflows: ["Performance Benchmarks"]
    types: [completed]

jobs:
  compare:
    runs-on: ubuntu-latest
    steps:
      - name: Download benchmark results
        uses: actions/download-artifact@v3
        with:
          name: benchmark-results

      - name: Compare with baseline
        run: |
          # Compare current results with stored baseline
          # Generate performance regression report
          echo "Performance comparison completed"
```

## 🎯 Optimization Strategies

### Based on Benchmark Results

#### High CPU Usage Solutions

1. **Enable SIMD**: `MCP_ENABLE_SIMD=true`
2. **Reduce Workers**: `MCP_MAX_WORKERS=4`
3. **Disable Parallel Processing**: `MCP_ENABLE_PARALLEL=false`

#### High Memory Usage Solutions

1. **Reduce Memory Mapping**: `MCP_MAX_MEMORY_MAP_SIZE=104857600`
2. **Set Memory Limit**: `MCP_MEMORY_LIMIT=536870912`
3. **Enable Compression**: `MCP_ENABLE_COMPRESSION=true`

#### Slow I/O Solutions

1. **Enable Memory Mapping**: `MCP_MAX_MEMORY_MAP_SIZE=1073741824`
2. **Increase Workers**: `MCP_MAX_WORKERS=8`
3. **Enable Streaming**: `MCP_ENABLE_STREAMING=true`

### Automated Optimization

```rust
// Automatic configuration optimization based on benchmarks
pub fn optimize_config_from_benchmarks(results: BenchmarkResults) -> PerformanceConfig {
    let mut config = PerformanceConfig::default();

    if results.simd_improvement > 1.5 {
        config.enable_simd = true;
    }

    if results.parallel_improvement > 1.8 {
        config.enable_parallel_processing = true;
        config.max_parallel_workers = results.optimal_worker_count;
    }

    if results.memory_usage_mb > 500 {
        config.max_file_size_for_memory_map = (500 * 1024 * 1024) as u64;
        config.memory_limit = 1 * 1024 * 1024 * 1024; // 1GB
    }

    config
}
```

## 📊 Reporting

### Benchmark Report Generation

```bash
# Generate HTML report
cargo bench -- --save-baseline baseline

# Compare with previous baseline
cargo bench -- --baseline baseline

# Generate custom report
cargo run --bin benchmark_reporter > benchmark_report.md
```

### Performance Dashboard

```bash
# Generate metrics for monitoring
cargo bench -- --output-format=json | jq '.benchmarks[] | {name, value}' > metrics.json

# Upload to monitoring system
curl -X POST https://monitoring.example.com/api/metrics \
  -H "Content-Type: application/json" \
  -d @metrics.json
```

---

**Remember**: Benchmarks should be run on dedicated hardware with consistent environmental conditions for reliable results. Always run multiple iterations and use statistical analysis to ensure result stability.
