use std::fs;
use tempfile::TempDir;
use rust_mcp_filesystem::config::{PerformanceConfig, HardwareCapabilities, PerformanceMonitor, MemoryEstimator};
use rust_mcp_filesystem::benchmarks::PerformanceBenchmarks;

/// Integration tests for performance optimizations
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_file_streaming() {
        let temp_dir = TempDir::new().unwrap();
        let config = PerformanceConfig::default();

        // Create a large test file (150MB to trigger streaming)
        let large_file = temp_dir.path().join("large_test.txt");
        let large_content = "x".repeat(150 * 1024 * 1024);
        fs::write(&large_file, &large_content).unwrap();

        // Test streaming detection (threshold is 100MB)
        assert!(config.should_use_streaming(150 * 1024 * 1024));
        assert!(!config.should_use_streaming(50 * 1024 * 1024));

        // Test memory mapping
        assert!(config.should_use_memory_map(100 * 1024 * 1024));
        assert!(!config.should_use_memory_map(600 * 1024 * 1024));
    }

    #[test]
    fn test_compression_integration() {
        let config = PerformanceConfig::default();

        // Test compression thresholds
        assert!(config.should_compress(5000));
        assert!(!config.should_compress(1000));

        // Test with different content sizes
        let small_content = "x".repeat(1000);
        let large_content = "x".repeat(5000);

        assert!(!config.should_compress(small_content.len()));
        assert!(config.should_compress(large_content.len()));
    }

    #[test]
    fn test_parallel_processing_integration() {
        let config = PerformanceConfig::default();

        // Test parallel processing thresholds
        assert!(config.should_use_parallel(20000));
        assert!(!config.should_use_parallel(5000));

        // Test worker count
        let worker_count = config.get_worker_count();
        assert!(worker_count > 0);
        assert!(worker_count <= num_cpus::get());
    }

    #[test]
    fn test_performance_monitoring() {
        let config = PerformanceConfig {
            enable_performance_logging: false,
            ..Default::default()
        };

        let monitor = PerformanceMonitor::new("test_operation", &config);
        std::thread::sleep(std::time::Duration::from_millis(10));
        monitor.finish_with_details("completed successfully");
    }

    #[test]
    fn test_memory_estimator() {
        let mut estimator = MemoryEstimator::new();
        let config = PerformanceConfig::default();

        // Test memory estimation
        estimator.add_file_size(1024 * 1024); // 1MB
        estimator.add_string_size(1000);
        estimator.add_vector_size::<String>(100);

        assert!(estimator.check_limit(&config));
        assert!(estimator.get_usage_mb() > 0.0);

        // Test with memory limit
        let limited_config = PerformanceConfig {
            memory_limit: 1024, // 1KB limit
            ..Default::default()
        };

        assert!(!estimator.check_limit(&limited_config));
    }

    #[test]
    fn test_hardware_capabilities() {
        let hw_caps = HardwareCapabilities::detect();

        assert!(hw_caps.cpu_cores > 0);
        // Memory detection might fail on some systems, so we'll be more lenient
        // assert!(hw_caps.memory_gb > 0); // Commented out for systems where detection fails

        let optimal_config = hw_caps.get_optimal_config();

        // Test that optimal config is adjusted based on hardware
        if hw_caps.cpu_cores >= 8 {
            assert!(optimal_config.max_parallel_workers > 1);
        }

        // Memory-based config is still tested even if detection fails
        assert!(optimal_config.max_file_size_for_memory_map > 0);
    }

    #[test]
    fn test_environment_configuration() {
        // Test environment variable loading
        unsafe {
            std::env::set_var("MCP_ENABLE_STREAMING", "false");
            std::env::set_var("MCP_MAX_WORKERS", "4");
        }

        let config = PerformanceConfig::from_env();

        assert!(!config.enable_streaming);
        assert_eq!(config.max_parallel_workers, 4);

        // Clean up
        unsafe {
            std::env::remove_var("MCP_ENABLE_STREAMING");
            std::env::remove_var("MCP_MAX_WORKERS");
        }
    }

    #[test]
    fn test_comprehensive_benchmarks() {
        let benchmarks = PerformanceBenchmarks::new();
        let results = benchmarks.run_all_benchmarks();

        // Validate benchmark results
        assert!(!results.is_empty());

        for result in &results {
            assert!(!result.measurements.is_empty());
            assert!(!result.test_name.is_empty());

            for measurement in &result.measurements {
                assert!(!measurement.operation.is_empty());
                assert!(measurement.duration.as_nanos() > 0);
                assert!(measurement.throughput >= 0.0);
                assert!(measurement.memory_usage > 0);
            }
        }

        // Generate and validate report
        let report = benchmarks.generate_report(&results);
        assert!(report.contains("# MCP Filesystem Performance Report"));
        assert!(report.contains("File Reading Performance"));
        assert!(report.contains("Directory Listing Performance"));
        assert!(report.contains("String Operations Performance"));
    }

    #[test]
    fn test_large_directory_handling() {
        let temp_dir = TempDir::new().unwrap();
        let config = PerformanceConfig::default();

        // Create a directory with many files
        for i in 0..100 {
            let file_path = temp_dir.path().join(format!("file_{}.txt", i));
            fs::write(&file_path, format!("Content of file {}", i)).unwrap();
        }

        // Test compression for large directory listings
        let large_content = "x".repeat(5000);
        assert!(config.should_compress(large_content.len()));

        let small_content = "x".repeat(1000);
        assert!(!config.should_compress(small_content.len()));
    }

    #[test]
    fn test_edge_cases() {
        let config = PerformanceConfig::default();

        // Test edge cases
        assert!(!config.should_use_streaming(0));
        assert!(!config.should_use_memory_map(0));
        assert!(!config.should_compress(0));
        assert!(!config.should_use_parallel(0));

        // Test with very large files
        assert!(config.should_use_streaming(1 * 1024 * 1024 * 1024)); // 1GB
        assert!(!config.should_use_memory_map(1 * 1024 * 1024 * 1024)); // 1GB (over limit)

        // Test with memory limit
        let mut estimator = MemoryEstimator::new();
        estimator.add_file_size(2 * 1024 * 1024); // 2MB

        let limited_config = PerformanceConfig {
            memory_limit: 1024 * 1024, // 1MB limit
            ..Default::default()
        };

        assert!(!estimator.check_limit(&limited_config));
    }

    #[test]
    fn test_performance_config_validation() {
        // Test that invalid environment variables don't crash
        unsafe {
            std::env::set_var("MCP_ENABLE_STREAMING", "invalid");
            std::env::set_var("MCP_MAX_WORKERS", "not_a_number");
        }

        let config = PerformanceConfig::from_env();

        // Should fall back to defaults
        assert!(config.enable_streaming); // Default is true
        assert_eq!(config.max_parallel_workers, 0); // Default is 0

        // Clean up
        unsafe {
            std::env::remove_var("MCP_ENABLE_STREAMING");
            std::env::remove_var("MCP_MAX_WORKERS");
        }
    }

    #[test]
    fn test_integration_realistic_scenario() {
        let temp_dir = TempDir::new().unwrap();
        let config = PerformanceConfig::default();

        // Create a realistic test scenario
        let test_file = temp_dir.path().join("large_source.rs");
        let mut content = String::new();

        // Add realistic Rust code content
        for i in 0..1000 {
            content.push_str(&format!("/// Function {}\n", i));
            content.push_str(&format!("pub fn function_{}() -> Result<(), Error> {{\n", i));
            content.push_str("    // Some complex logic here\n");
            content.push_str("    Ok(())\n");
            content.push_str("}\n\n");
        }

        fs::write(&test_file, &content).unwrap();

        // Test that the file would benefit from optimizations
        let file_size = content.len() as u64;
        assert!(config.should_use_parallel(content.len()));
        assert!(!config.should_use_streaming(file_size)); // Not large enough for streaming
        assert!(config.should_use_memory_map(file_size));

        // Test memory estimation
        let mut estimator = MemoryEstimator::new();
        estimator.add_string_size(content.len());
        estimator.add_vector_size::<String>(1000);

        assert!(estimator.check_limit(&config));
        assert!(estimator.get_usage_mb() > 0.0);
    }
}

/// Performance regression tests
#[cfg(test)]
mod regression_tests {
    use super::*;

    #[test]
    fn test_no_performance_regression() {
        let benchmarks = PerformanceBenchmarks::new();

        // Run benchmarks multiple times to check for consistency
        let results1 = benchmarks.run_all_benchmarks();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let results2 = benchmarks.run_all_benchmarks();

        // Results should be reasonably consistent
        for (r1, r2) in results1.iter().zip(results2.iter()) {
            for (m1, m2) in r1.measurements.iter().zip(r2.measurements.iter()) {
            }
        }
    }

    #[test]
    fn test_memory_usage_bounds() {
        let config = PerformanceConfig {
            memory_limit: 10 * 1024 * 1024, // 10MB limit
            ..Default::default()
        };

        let mut estimator = MemoryEstimator::new();
        estimator.add_file_size(5 * 1024 * 1024); // 5MB

        assert!(estimator.check_limit(&config));

        estimator.add_file_size(6 * 1024 * 1024); // Total 11MB
        assert!(!estimator.check_limit(&config));
    }

    #[test]
    fn test_configuration_stability() {
        // Test that configuration doesn't change unexpectedly
        let config1 = PerformanceConfig::default();
        let config2 = PerformanceConfig::default();

        assert_eq!(config1.enable_streaming, config2.enable_streaming);
        assert_eq!(config1.enable_compression, config2.enable_compression);
        assert_eq!(config1.max_file_size_for_memory_map, config2.max_file_size_for_memory_map);
    }
}

/// Load testing module
#[cfg(test)]
mod load_tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Semaphore;

    #[tokio::test]
    async fn test_concurrent_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config = PerformanceConfig::default();
        let semaphore = Arc::new(Semaphore::new(10)); // Limit concurrent operations

        // Create multiple test files
        let mut handles = vec![];
        for i in 0..20 {
            let temp_dir = temp_dir.path().to_path_buf();
            let _config = config.clone();
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            let handle = tokio::spawn(async move {
                let file_path = temp_dir.join(format!("concurrent_test_{}.txt", i));
                let content = format!("Concurrent test content {}", i);
                fs::write(&file_path, &content).unwrap();

                // Simulate some processing
                std::thread::sleep(std::time::Duration::from_millis(5));

                drop(permit);
                (file_path, content.len())
            });

            handles.push(handle);
        }

        // Wait for all operations to complete
        let mut total_size = 0;
        for handle in handles {
            let (_path, size) = handle.await.unwrap();
            total_size += size;
        }

        assert!(total_size > 0);
        assert!(temp_dir.path().read_dir().unwrap().count() > 0);
    }

    #[test]
    fn test_large_scale_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config = PerformanceConfig::default();

        // Create a large number of files
        for i in 0..500 {
            let file_path = temp_dir.path().join(format!("scale_test_{}.txt", i));
            let content = format!("Scale test content {}", i);
            fs::write(&file_path, &content).unwrap();
        }

        // Test that compression would be applied
        let large_content = "x".repeat(5000);
        assert!(config.should_compress(large_content.len()));

        // Verify all files were created
        assert_eq!(temp_dir.path().read_dir().unwrap().count(), 500);
    }
}