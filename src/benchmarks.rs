use std::time::{Duration, Instant};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Performance benchmark suite for MCP filesystem optimizations
pub struct PerformanceBenchmarks {
    temp_dir: TempDir,
}

impl PerformanceBenchmarks {
    pub fn new() -> Self {
        Self {
            temp_dir: TempDir::new().expect("Failed to create temp directory"),
        }
    }

    /// Benchmark file reading performance
    pub fn benchmark_file_reading(&self) -> BenchmarkResult {
        let sizes = vec![1024, 10 * 1024, 100 * 1024, 1024 * 1024]; // 1KB, 10KB, 100KB, 1MB
        let mut results = Vec::new();

        for &size in &sizes {
            // Create test file
            let test_file = self.temp_dir.path().join(format!("test_{}.txt", size));
            let content = "x".repeat(size);
            fs::write(&test_file, &content).expect("Failed to write test file");

            // Benchmark reading
            let start = Instant::now();
            let _content = fs::read_to_string(&test_file).expect("Failed to read file");
            let duration = start.elapsed();

            results.push(BenchmarkMeasurement {
                operation: format!("read_{}_bytes", size),
                duration,
                throughput: size as f64 / duration.as_secs_f64(),
                memory_usage: size,
            });
        }

        BenchmarkResult {
            test_name: "File Reading Performance".to_string(),
            measurements: results,
        }
    }

    /// Benchmark directory listing performance
    pub fn benchmark_directory_listing(&self) -> BenchmarkResult {
        let entry_counts = vec![10, 100, 1000];
        let mut results = Vec::new();

        for &count in &entry_counts {
            // Create test directory with files
            let test_dir = self.temp_dir.path().join(format!("test_dir_{}", count));
            fs::create_dir(&test_dir).expect("Failed to create test directory");

            for i in 0..count {
                let file_path = test_dir.join(format!("file_{}.txt", i));
                fs::write(&file_path, format!("Content of file {}", i)).expect("Failed to write file");
            }

            // Benchmark directory listing
            let start = Instant::now();
            let _entries = fs::read_dir(&test_dir).expect("Failed to read directory");
            let duration = start.elapsed();

            results.push(BenchmarkMeasurement {
                operation: format!("list_{}_entries", count),
                duration,
                throughput: count as f64 / duration.as_secs_f64(),
                memory_usage: count * 256, // Estimate memory usage
            });
        }

        BenchmarkResult {
            test_name: "Directory Listing Performance".to_string(),
            measurements: results,
        }
    }

    /// Benchmark string operations
    pub fn benchmark_string_operations(&self) -> BenchmarkResult {
        let sizes = vec![1000, 10000, 100000];
        let mut results = Vec::new();

        for &size in &sizes {
            let test_string = "x".repeat(size);

            // Benchmark string find
            let start = Instant::now();
            let _pos = test_string.find("test");
            let find_duration = start.elapsed();

            // Benchmark string split
            let start = Instant::now();
            let _lines: Vec<&str> = test_string.lines().collect();
            let split_duration = start.elapsed();

            results.push(BenchmarkMeasurement {
                operation: format!("string_find_{}_chars", size),
                duration: find_duration,
                throughput: size as f64 / find_duration.as_secs_f64(),
                memory_usage: size * 4, // UTF-8 estimate
            });

            results.push(BenchmarkMeasurement {
                operation: format!("string_split_{}_chars", size),
                duration: split_duration,
                throughput: size as f64 / split_duration.as_secs_f64(),
                memory_usage: size * 4,
            });
        }

        BenchmarkResult {
            test_name: "String Operations Performance".to_string(),
            measurements: results,
        }
    }

    /// Run all benchmarks
    pub fn run_all_benchmarks(&self) -> Vec<BenchmarkResult> {
        vec![
            self.benchmark_file_reading(),
            self.benchmark_directory_listing(),
            self.benchmark_string_operations(),
        ]
    }

    /// Generate performance report
    pub fn generate_report(&self, results: &[BenchmarkResult]) -> String {
        let mut report = String::new();
        report.push_str("# MCP Filesystem Performance Report\n\n");

        for result in results {
            report.push_str(&format!("## {}\n\n", result.test_name));

            for measurement in &result.measurements {
                report.push_str(&format!(
                    "### {}\n",
                    measurement.operation
                ));
                report.push_str(&format!("- Duration: {:.4} ms\n", measurement.duration.as_millis()));
                report.push_str(&format!("- Throughput: {:.2} ops/sec\n", measurement.throughput));
                report.push_str(&format!("- Memory Usage: {} bytes\n\n", measurement.memory_usage));
            }
        }

        report
    }
}

/// Individual benchmark measurement
#[derive(Debug)]
pub struct BenchmarkMeasurement {
    pub operation: String,
    pub duration: Duration,
    pub throughput: f64,
    pub memory_usage: usize,
}

/// Benchmark result for a test suite
#[derive(Debug)]
pub struct BenchmarkResult {
    pub test_name: String,
    pub measurements: Vec<BenchmarkMeasurement>,
}

/// Performance configuration
#[derive(Debug)]
pub struct PerformanceConfig {
    pub enable_streaming: bool,
    pub enable_compression: bool,
    pub enable_parallel_processing: bool,
    pub max_file_size_for_memory_map: u64,
    pub compression_threshold: usize,
    pub parallel_threshold: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_streaming: true,
            enable_compression: true,
            enable_parallel_processing: true,
            max_file_size_for_memory_map: 500 * 1024 * 1024, // 500MB
            compression_threshold: 4096, // 4KB
            parallel_threshold: 10000, // 10KB
        }
    }
}

/// Performance metrics collector
pub struct PerformanceMetrics {
    pub operation_count: u64,
    pub total_time: Duration,
    pub peak_memory_usage: usize,
    pub error_count: u64,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            operation_count: 0,
            total_time: Duration::default(),
            peak_memory_usage: 0,
            error_count: 0,
        }
    }

    pub fn record_operation(&mut self, duration: Duration, memory_usage: usize) {
        self.operation_count += 1;
        self.total_time += duration;
        self.peak_memory_usage = self.peak_memory_usage.max(memory_usage);
    }

    pub fn record_error(&mut self) {
        self.error_count += 1;
    }

    pub fn average_time(&self) -> Duration {
        if self.operation_count == 0 {
            Duration::default()
        } else {
            self.total_time / self.operation_count as u32
        }
    }

    pub fn operations_per_second(&self) -> f64 {
        if self.total_time.as_secs_f64() == 0.0 {
            0.0
        } else {
            self.operation_count as f64 / self.total_time.as_secs_f64()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmarks_run() {
        let benchmarks = PerformanceBenchmarks::new();
        let results = benchmarks.run_all_benchmarks();

        assert!(!results.is_empty());
        assert!(results.iter().all(|r| !r.measurements.is_empty()));

        let report = benchmarks.generate_report(&results);
        assert!(!report.is_empty());
        println!("{}", report);
    }

    #[test]
    fn test_performance_config() {
        let config = PerformanceConfig::default();
        assert!(config.enable_streaming);
        assert!(config.enable_compression);
        assert!(config.enable_parallel_processing);
        assert_eq!(config.max_file_size_for_memory_map, 500 * 1024 * 1024);
    }

    #[test]
    fn test_performance_metrics() {
        let mut metrics = PerformanceMetrics::new();

        metrics.record_operation(Duration::from_millis(100), 1024);
        metrics.record_operation(Duration::from_millis(200), 2048);
        metrics.record_error();

        assert_eq!(metrics.operation_count, 2);
        assert_eq!(metrics.average_time(), Duration::from_millis(150));
        assert_eq!(metrics.peak_memory_usage, 2048);
        assert_eq!(metrics.error_count, 1);
    }
}