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
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(val) = env::var("MCP_ENABLE_STREAMING") {
            config.enable_streaming = val.parse().unwrap_or(true);
        }
        if let Ok(val) = env::var("MCP_ENABLE_COMPRESSION") {
            config.enable_compression = val.parse().unwrap_or(true);
        }
        if let Ok(val) = env::var("MCP_ENABLE_PARALLEL") {
            config.enable_parallel_processing = val.parse().unwrap_or(true);
        }
        if let Ok(val) = env::var("MCP_MAX_MEMORY_MAP_SIZE") {
            config.max_file_size_for_memory_map = val.parse().unwrap_or(500 * 1024 * 1024);
        }
        if let Ok(val) = env::var("MCP_COMPRESSION_THRESHOLD") {
            config.compression_threshold = val.parse().unwrap_or(4096);
        }
        if let Ok(val) = env::var("MCP_PARALLEL_THRESHOLD") {
            config.parallel_threshold = val.parse().unwrap_or(10000);
        }
        if let Ok(val) = env::var("MCP_MAX_WORKERS") {
            config.max_parallel_workers = val.parse().unwrap_or(0);
        }
        if let Ok(val) = env::var("MCP_ENABLE_SIMD") {
            config.enable_simd = val.parse().unwrap_or(true);
        }
        if let Ok(val) = env::var("MCP_MEMORY_LIMIT") {
            config.memory_limit = val.parse().unwrap_or(0);
        }
        if let Ok(val) = env::var("MCP_PERFORMANCE_LOGGING") {
            config.enable_performance_logging = val.parse().unwrap_or(false);
        }

        config
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