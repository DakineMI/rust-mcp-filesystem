use serde::{Deserialize, Serialize};
use std::env;

/// Configuration for MCP filesystem performance optimizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable streaming for large files
    pub enable_streaming: bool,
    /// Enable compression for large directory listings
    pub enable_compression: bool,
    /// Enable parallel processing for code analysis
    pub enable_parallel_processing: bool,
    /// Maximum file size for memory mapping (in bytes)
    pub max_file_size_for_memory_map: u64,
    /// Minimum size threshold for compression (in bytes)
    pub compression_threshold: usize,
    /// Minimum file size for parallel processing (in bytes)
    pub parallel_threshold: usize,
    /// Number of parallel workers (0 = auto-detect CPU cores)
    pub max_parallel_workers: usize,
    /// Enable SIMD optimizations where available
    pub enable_simd: bool,
    /// Memory usage limit (in bytes, 0 = no limit)
    pub memory_limit: u64,
    /// Enable detailed performance logging
    pub enable_performance_logging: bool,
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
            max_parallel_workers: 0, // Auto-detect
            enable_simd: true,
            memory_limit: 0, // No limit
            enable_performance_logging: false,
        }
    }
}

impl PerformanceConfig {
    /// Load configuration from environment variables with validation
    pub fn from_env() -> Self {
        let mut config = Self::default();

        // Boolean configurations
        config.enable_streaming = Self::parse_bool_env("MCP_ENABLE_STREAMING", true);
        config.enable_compression = Self::parse_bool_env("MCP_ENABLE_COMPRESSION", true);
        config.enable_parallel_processing = Self::parse_bool_env("MCP_ENABLE_PARALLEL", true);
        config.enable_simd = Self::parse_bool_env("MCP_ENABLE_SIMD", true);
        config.enable_performance_logging = Self::parse_bool_env("MCP_PERFORMANCE_LOGGING", false);

        // Size configurations with validation
        config.max_file_size_for_memory_map = Self::parse_size_env("MCP_MAX_MEMORY_MAP_SIZE", 500 * 1024 * 1024);
        config.compression_threshold = Self::parse_size_env("MCP_COMPRESSION_THRESHOLD", 4096) as usize;
        config.parallel_threshold = Self::parse_size_env("MCP_PARALLEL_THRESHOLD", 10000) as usize;
        config.max_parallel_workers = Self::parse_workers_env("MCP_MAX_WORKERS", 0);
        config.memory_limit = Self::parse_size_env("MCP_MEMORY_LIMIT", 0);

        // Validate final configuration
        config.validate();

        config
    }

    /// Parse boolean environment variable with fallback
    fn parse_bool_env(var_name: &str, default: bool) -> bool {
        match env::var(var_name) {
            Ok(val) => val.parse().unwrap_or_else(|_| {
                eprintln!("Warning: Invalid boolean value for {}: '{}', using default: {}", var_name, val, default);
                default
            }),
            Err(_) => default,
        }
    }

    /// Parse size environment variable with validation (bytes)
    fn parse_size_env(var_name: &str, default: u64) -> u64 {
        match env::var(var_name) {
            Ok(val) => match val.parse::<u64>() {
                Ok(size) if size <= 10 * 1024 * 1024 * 1024 => size, // Max 10GB
                Ok(size) => {
                    eprintln!("Warning: {} value too large: {} bytes, capping at 10GB", var_name, size);
                    10 * 1024 * 1024 * 1024
                }
                Err(_) => {
                    eprintln!("Warning: Invalid size value for {}: '{}', using default: {} bytes", var_name, val, default);
                    default
                }
            },
            Err(_) => default,
        }
    }

    /// Parse worker count with CPU core validation
    fn parse_workers_env(var_name: &str, default: usize) -> usize {
        match env::var(var_name) {
            Ok(val) => match val.parse::<usize>() {
                Ok(workers) => {
                    let max_workers = num_cpus::get() * 2; // Allow up to 2x CPU cores
                    if workers > max_workers {
                        eprintln!("Warning: {} value too high: {}, capping at {}", var_name, workers, max_workers);
                        max_workers
                    } else {
                        workers
                    }
                }
                Err(_) => {
                    eprintln!("Warning: Invalid worker count for {}: '{}', using default: {}", var_name, val, default);
                    default
                }
            },
            Err(_) => default,
        }
    }

    /// Validate configuration values
    fn validate(&mut self) {
        // Ensure reasonable minimums
        self.compression_threshold = self.compression_threshold.max(1024); // Min 1KB
        self.parallel_threshold = self.parallel_threshold.max(1000); // Min 1KB
        self.max_file_size_for_memory_map = self.max_file_size_for_memory_map.max(1024 * 1024); // Min 1MB

        // Ensure memory map size is reasonable relative to memory limit
        if self.memory_limit > 0 && self.max_file_size_for_memory_map > self.memory_limit / 4 {
            self.max_file_size_for_memory_map = self.memory_limit / 4;
            eprintln!("Warning: Adjusted memory map size to fit within memory limit");
        }

        // Ensure worker count is reasonable
        if self.max_parallel_workers > 0 && self.max_parallel_workers < 1 {
            self.max_parallel_workers = 1;
        }
    }

    /// Get the number of parallel workers to use
    pub fn get_worker_count(&self) -> usize {
        if self.max_parallel_workers == 0 {
            num_cpus::get()
        } else {
            self.max_parallel_workers
        }
    }

    /// Check if a file should use streaming
    pub fn should_use_streaming(&self, file_size: u64) -> bool {
        self.enable_streaming && file_size > 100 * 1024 * 1024 // 100MB
    }

    /// Check if content should be compressed
    pub fn should_compress(&self, content_size: usize) -> bool {
        self.enable_compression && content_size > self.compression_threshold
    }

    /// Check if parallel processing should be used
    pub fn should_use_parallel(&self, content_size: usize) -> bool {
        self.enable_parallel_processing && content_size > self.parallel_threshold
    }

    /// Check if memory mapping should be used
    pub fn should_use_memory_map(&self, file_size: u64) -> bool {
        file_size <= self.max_file_size_for_memory_map && file_size > 0
    }

    /// Log performance information if enabled
    pub fn log_performance(&self, operation: &str, duration: std::time::Duration, details: &str) {
        if self.enable_performance_logging {
            println!(
                "[PERF] {} took {:.4}ms - {}",
                operation,
                duration.as_millis(),
                details
            );
        }
    }
}

/// Hardware capability detection
pub struct HardwareCapabilities {
    pub has_avx2: bool,
    pub has_sse4_2: bool,
    pub cpu_cores: usize,
    pub memory_gb: usize,
}

impl HardwareCapabilities {
    pub fn detect() -> Self {
        let cpu_cores = num_cpus::get();

        // Estimate memory (this is a rough approximation)
        let memory_gb = (sys_info::mem_info().map(|info| info.total).unwrap_or(8 * 1024 * 1024 * 1024) / 1024 / 1024 / 1024) as usize;

        // Check for SIMD support (basic detection)
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        let has_avx2 = std::is_x86_feature_detected!("avx2");
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        let has_sse4_2 = std::is_x86_feature_detected!("sse4.2");

        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        let has_avx2 = false;
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        let has_sse4_2 = false;

        Self {
            has_avx2,
            has_sse4_2,
            cpu_cores,
            memory_gb,
        }
    }

    pub fn get_optimal_config(&self) -> PerformanceConfig {
        let mut config = PerformanceConfig::default();

        // Adjust configuration based on hardware capabilities
        if self.cpu_cores >= 8 {
            config.max_parallel_workers = self.cpu_cores / 2; // Use half cores for I/O
        }

        if self.memory_gb >= 16 {
            config.max_file_size_for_memory_map = 1024 * 1024 * 1024; // 1GB for high-memory systems
        } else if self.memory_gb >= 8 {
            config.max_file_size_for_memory_map = 500 * 1024 * 1024; // 500MB for medium-memory systems
        } else {
            config.max_file_size_for_memory_map = 100 * 1024 * 1024; // 100MB for low-memory systems
        }

        // Enable SIMD if available
        config.enable_simd = self.has_avx2 || self.has_sse4_2;

        config
    }
}

/// Performance monitoring
pub struct PerformanceMonitor {
    start_time: std::time::Instant,
    operation_name: String,
    config: PerformanceConfig,
}

impl PerformanceMonitor {
    pub fn new(operation_name: &str, config: &PerformanceConfig) -> Self {
        Self {
            start_time: std::time::Instant::now(),
            operation_name: operation_name.to_string(),
            config: config.clone(),
        }
    }

    pub fn finish_with_details(&self, details: &str) {
        let duration = self.start_time.elapsed();
        self.config.log_performance(&self.operation_name, duration, details);
    }

    pub fn finish(&self) {
        self.finish_with_details("");
    }
}

/// Memory usage estimator
pub struct MemoryEstimator {
    pub estimated_usage: usize,
}

impl MemoryEstimator {
    pub fn new() -> Self {
        Self {
            estimated_usage: 0,
        }
    }

    pub fn add_file_size(&mut self, size: u64) {
        self.estimated_usage += size as usize;
    }

    pub fn add_string_size(&mut self, size: usize) {
        self.estimated_usage += size * 2; // UTF-8 estimate
    }

    pub fn add_vector_size<T>(&mut self, capacity: usize) {
        self.estimated_usage += capacity * std::mem::size_of::<T>();
    }

    pub fn check_limit(&self, config: &PerformanceConfig) -> bool {
        config.memory_limit == 0 || self.estimated_usage < config.memory_limit as usize
    }

    pub fn get_usage_mb(&self) -> f64 {
        self.estimated_usage as f64 / 1024.0 / 1024.0
    }
}

/// Error handling for performance-related failures
#[derive(Debug)]
pub enum PerformanceError {
    MemoryLimitExceeded(String),
    OptimizationNotAvailable(String),
    HardwareNotSupported(String),
}

impl std::fmt::Display for PerformanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerformanceError::MemoryLimitExceeded(msg) => write!(f, "Memory limit exceeded: {}", msg),
            PerformanceError::OptimizationNotAvailable(msg) => write!(f, "Optimization not available: {}", msg),
            PerformanceError::HardwareNotSupported(msg) => write!(f, "Hardware not supported: {}", msg),
        }
    }
}

impl std::error::Error for PerformanceError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PerformanceConfig::default();
        assert!(config.enable_streaming);
        assert!(config.enable_compression);
        assert!(config.enable_parallel_processing);
        assert_eq!(config.max_file_size_for_memory_map, 500 * 1024 * 1024);
    }

    #[test]
    fn test_config_should_methods() {
        let config = PerformanceConfig::default();

        assert!(config.should_use_streaming(200 * 1024 * 1024));
        assert!(!config.should_use_streaming(50 * 1024 * 1024));

        assert!(config.should_compress(5000));
        assert!(!config.should_compress(1000));

        assert!(config.should_use_parallel(20000));
        assert!(!config.should_use_parallel(5000));

        assert!(config.should_use_memory_map(100 * 1024 * 1024));
        assert!(!config.should_use_memory_map(600 * 1024 * 1024));
    }

    #[test]
    fn test_memory_estimator() {
        let mut estimator = MemoryEstimator::new();
        estimator.add_file_size(1024 * 1024); // 1MB
        estimator.add_string_size(1000);
        estimator.add_vector_size::<String>(100);

        let config = PerformanceConfig::default();
        assert!(estimator.check_limit(&config));
        assert!(estimator.get_usage_mb() > 0.0);
    }

    #[test]
    fn test_performance_monitor() {
        let config = PerformanceConfig {
            enable_performance_logging: false,
            ..Default::default()
        };

        let monitor = PerformanceMonitor::new("test_operation", &config);
        std::thread::sleep(std::time::Duration::from_millis(10));
        monitor.finish();
    }
}