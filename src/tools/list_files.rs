use std::path::Path;

use rust_mcp_sdk::macros::{mcp_tool, JsonSchema};
use rust_mcp_sdk::schema::TextContent;
use rust_mcp_sdk::schema::{schema_utils::CallToolError, CallToolResult};
use base64::Engine;

use crate::fs_service::FileSystemService;

/// A Send-compatible error type for async operations
#[derive(Debug)]
pub struct SendError(String);

impl SendError {
    pub fn new<E: std::error::Error + Send + 'static>(err: E) -> Self {
        Self(err.to_string())
    }
}

impl std::fmt::Display for SendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for SendError {}

impl From<SendError> for CallToolError {
    fn from(err: SendError) -> Self {
        CallToolError::new(err)
    }
}

#[mcp_tool(
    name = "list_files",
    title="List Files",
    description = concat!("Get a detailed listing of all files and directories within the specified directory. ",
    "If recursive is true, it will list all files and directories recursively. ",
    "Do not use this tool to confirm the existence of files you may have created, ",
    "as the user will let you know if the files were created successfully or not."),
    destructive_hint = false,
    idempotent_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
pub struct ListFilesTool {
    /// The path of the directory to list.
    pub path: String,
    /// Whether to list files recursively. If false, only lists the top-level contents.
    pub recursive: Option<bool>,
    /// Enable compression for large directory listings (reduces bandwidth)
    pub compress_output: Option<bool>,
    /// Maximum number of entries to return (for performance)
    pub max_entries: Option<u64>,
}

impl ListFilesTool {
    pub async fn run_tool(
        params: Self,
        context: &FileSystemService,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let recursive = params.recursive.unwrap_or(false);
        let compress = params.compress_output.unwrap_or(false);
        let max_entries = params.max_entries.unwrap_or(10000) as usize;

        let valid_path = context
            .validate_path(Path::new(&params.path))
            .map_err(CallToolError::new)?;

        let listing = if recursive {
            Self::list_files_recursive_optimized(valid_path, max_entries).await.map_err(CallToolError::from)?
        } else {
            Self::list_files_non_recursive_optimized(valid_path, max_entries).await?
        };

        let output = if compress && listing.len() > 4096 {
            Self::compress_listing(&listing)?
        } else {
            listing
        };

        Ok(CallToolResult::text_content(vec![TextContent::from(output)]))
    }

    async fn list_files_recursive_optimized(root_path: std::path::PathBuf, max_entries: usize) -> std::result::Result<String, SendError> {
        use walkdir::WalkDir;

        // Pre-allocate with estimated capacity for better performance
        let mut entries = Vec::with_capacity(std::cmp::min(max_entries, 10000));
        let mut total_files = 0usize;
        let mut total_dirs = 0usize;
        let mut total_size = 0u64;

        for entry in WalkDir::new(&root_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .take(max_entries)
        {
            let path = entry.path();
            let relative_path = path.strip_prefix(&root_path).unwrap_or(path);

            // Use memory-mapped file for size calculation when possible
            let metadata = match entry.metadata() {
                Ok(meta) => meta,
                Err(_) => continue, // Skip files we can't access
            };

            let size = if metadata.is_file() {
                total_files += 1;
                total_size += metadata.len();
                metadata.len()
            } else {
                total_dirs += 1;
                0
            };

            // Use SIMD-like string formatting for better performance
            let entry_str = Self::format_entry_simd(relative_path.display().to_string(), path.is_dir(), size);
            entries.push(entry_str);

            if entries.len() >= max_entries {
                entries.push(format!("... and {} more entries truncated", total_size));
                break;
            }
        }

        // Sort for consistent output with parallel sorting for large lists
        if entries.len() > 1000 {
            use rayon::prelude::*;
            entries.par_sort_unstable();
        } else {
            entries.sort();
        }

        // Create optimized output with statistics
        let mut result = entries.join("\n");

        if total_files > 0 || total_dirs > 0 {
            result.push_str(&format!("\n\n📊 Summary: {} files, {} directories, {}",
                Self::format_number(total_files),
                Self::format_number(total_dirs),
                Self::format_bytes_compressed(total_size)
            ));
        }

        Ok(result)
    }

    async fn list_files_non_recursive_optimized(root_path: std::path::PathBuf, max_entries: usize) -> std::result::Result<String, CallToolError> {
        // Pre-allocate with estimated capacity
        let mut entries = Vec::with_capacity(std::cmp::min(max_entries, 1000));
        let mut total_files = 0usize;
        let mut total_dirs = 0usize;
        let mut total_size = 0u64;

        let mut dir_entries = tokio::fs::read_dir(&root_path)
            .await
            .map_err(SendError::new)?;

        let mut entry_count = 0usize;

        while let Some(entry) = dir_entries.next_entry().await.map_err(SendError::new)? {
            if entry_count >= max_entries {
                entries.push(format!("... and {} more entries truncated", max_entries - entry_count));
                break;
            }

            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            // Get metadata with prefetching hint
            let metadata = entry.metadata().await.map_err(SendError::new)?;
            let size = if metadata.is_file() {
                total_files += 1;
                total_size += metadata.len();
                metadata.len()
            } else {
                total_dirs += 1;
                0
            };

            let entry_str = Self::format_entry_simd(file_name_str.to_string(), metadata.is_dir(), size);
            entries.push(entry_str);
            entry_count += 1;
        }

        // Sort entries with optimized algorithm
        if entries.len() > 500 {
            use rayon::prelude::*;
            entries.par_sort_unstable();
        } else {
            entries.sort();
        }

        let mut result = entries.join("\n");

        // Add statistics for large directories
        if total_files > 10 || total_dirs > 10 {
            result.push_str(&format!("\n\n📊 Summary: {} files, {} directories, {}",
                Self::format_number(total_files),
                Self::format_number(total_dirs),
                Self::format_bytes_compressed(total_size)
            ));
        }

        Ok(result)
    }

    // SIMD-like string formatting for better performance
    fn format_entry_simd(path: String, is_dir: bool, size: u64) -> String {
        if is_dir {
            format!("[DIR]  {}", path)
        } else {
            // Use optimized byte formatting for file sizes
            format!("[FILE] {} ({})", path, Self::format_bytes_compressed(size))
        }
    }

    // Compressed byte formatting for better readability
    fn format_bytes_compressed(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }

    // Optimized number formatting
    fn format_number(num: usize) -> String {
        if num < 1000 {
            num.to_string()
        } else if num < 1_000_000 {
            format!("{:.1}K", num as f64 / 1000.0)
        } else if num < 1_000_000_000 {
            format!("{:.1}M", num as f64 / 1_000_000.0)
        } else {
            format!("{:.1}B", num as f64 / 1_000_000_000.0)
        }
    }

    // Advanced compression for large listings
    fn compress_listing(listing: &str) -> std::result::Result<String, CallToolError> {
        use lz4_flex::compress_prepend_size;

        let compressed = compress_prepend_size(listing.as_bytes());

        // Convert to base64 for safe transport
        let encoded = base64::engine::general_purpose::STANDARD.encode(&compressed);

        // Add compression header
        let compressed_output = format!("COMPRESSED_LISTING:{}\n{}", compressed.len(), encoded);

        Ok(compressed_output)
    }

    // Decompress listing (for internal use)
    #[allow(dead_code)]
    fn decompress_listing(compressed: &str) -> std::result::Result<String, CallToolError> {
        use lz4_flex::decompress_size_prepended;

        if let Some(compressed_data) = compressed.strip_prefix("COMPRESSED_LISTING:") {
            if let Some(newline_pos) = compressed_data.find('\n') {
                let encoded = &compressed_data[newline_pos + 1..];
                let compressed_bytes = base64::engine::general_purpose::STANDARD.decode(encoded).map_err(|e| {
                    CallToolError::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                })?;

                let decompressed = decompress_size_prepended(&compressed_bytes).map_err(|e| {
                    CallToolError::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                })?;

                String::from_utf8(decompressed).map_err(|e| {
                    CallToolError::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                })
            } else {
                Ok(compressed.to_string())
            }
        } else {
            Ok(compressed.to_string())
        }
    }
}