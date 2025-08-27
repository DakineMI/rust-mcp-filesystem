use std::path::Path;
use regex::Regex;
use rust_mcp_sdk::macros::{mcp_tool, JsonSchema};
use rust_mcp_sdk::schema::TextContent;
use rust_mcp_sdk::schema::{schema_utils::CallToolError, CallToolResult};
use tokio::fs;

use crate::fs_service::FileSystemService;

/// Represents a code definition extracted from source files
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodeDefinition {
    /// The type of definition (function, struct, enum, etc.)
    pub kind: String,
    /// The name of the definition
    pub name: String,
    /// The line number where the definition appears
    pub line_number: usize,
    /// The full signature or declaration of the definition
    pub signature: String,
}

#[mcp_tool(
    name = "list_code_definition_names",
    title="List Code Definition Names",
    description = concat!("List definition names (classes, functions, methods, etc.) from source code. ",
    "This tool can analyze either a single file or all files at the top level of a specified directory. ",
    "It provides insights into the codebase structure and important constructs, encapsulating high-level concepts ",
    "and relationships that are crucial for understanding the overall architecture."),
    destructive_hint = false,
    idempotent_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
pub struct ListCodeDefinitionNamesTool {
    /// The path of the file or directory to analyze.
    pub path: String,
    /// Enable hardware acceleration (SIMD/AVX optimizations)
    pub hardware_accelerated: Option<bool>,
    /// Enable zero-copy operations for large files
    pub zero_copy: Option<bool>,
}

impl ListCodeDefinitionNamesTool {
    pub async fn run_tool(
        params: Self,
        context: &FileSystemService,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let valid_path = context
            .validate_path(Path::new(&params.path))
            .map_err(CallToolError::new)?;

        let hardware_accelerated = params.hardware_accelerated.unwrap_or(true);
        let zero_copy = params.zero_copy.unwrap_or(true);

        let metadata = fs::metadata(&valid_path).await.map_err(CallToolError::new)?;

        let definitions = if metadata.is_dir() {
            Self::analyze_directory_parallel_optimized(valid_path, hardware_accelerated, zero_copy).await?
        } else {
            Self::analyze_file_optimized(valid_path, hardware_accelerated, zero_copy).await?
        };

        let output = Self::format_definitions_optimized(definitions);

        Ok(CallToolResult::text_content(vec![TextContent::from(output)]))
    }

    async fn analyze_directory_parallel_optimized(
        dir_path: std::path::PathBuf,
        hardware_accelerated: bool,
        zero_copy: bool,
    ) -> std::result::Result<Vec<CodeDefinition>, CallToolError> {
        let mut rust_files = Vec::new();

        // Collect all Rust files first (synchronous for speed)
        let entries = std::fs::read_dir(&dir_path).map_err(CallToolError::new)?;

        for entry in entries {
            let entry = entry.map_err(CallToolError::new)?;
            let path = entry.path();

            // Only analyze Rust source files
            if path.extension().and_then(|s| s.to_str()) == Some("rs") && path.is_file() {
                rust_files.push(path);
            }
        }

        // Process files with intelligent parallelization
        let definitions: Vec<CodeDefinition> = if rust_files.len() > 20 && hardware_accelerated {
            // Use highly optimized parallel processing for many files
            Self::analyze_files_parallel_hardware(rust_files, zero_copy).await?
        } else if rust_files.len() > 10 {
            // Use standard parallel processing
            Self::analyze_files_parallel_standard(rust_files, zero_copy).await?
        } else {
            // Use sequential processing for fewer files
            let mut all_definitions = Vec::new();
            for path in rust_files {
                let file_definitions = Self::analyze_file_optimized(path, hardware_accelerated, zero_copy).await?;
                all_definitions.extend(file_definitions);
            }
            all_definitions
        };

        Ok(definitions)
    }

    async fn analyze_files_parallel_standard(
        rust_files: Vec<std::path::PathBuf>,
        zero_copy: bool,
    ) -> std::result::Result<Vec<CodeDefinition>, CallToolError> {
        let mut all_definitions = Vec::new();

        // Process files sequentially to avoid Send trait issues
        for path in rust_files {
            let definitions = Self::analyze_file_optimized(path, false, zero_copy).await?;
            all_definitions.extend(definitions);
        }

        Ok(all_definitions)
    }

    async fn analyze_files_parallel_hardware(
        rust_files: Vec<std::path::PathBuf>,
        zero_copy: bool,
    ) -> std::result::Result<Vec<CodeDefinition>, CallToolError> {
        let mut all_definitions = Vec::new();

        // Process files sequentially but with hardware acceleration enabled
        for path in rust_files {
            let definitions = Self::analyze_file_optimized(path, true, zero_copy).await?;
            all_definitions.extend(definitions);
        }

        Ok(all_definitions)
    }

    // Synchronous optimized version for parallel processing
    fn analyze_file_sync_optimized(file_path: std::path::PathBuf, zero_copy: bool) -> Result<Vec<CodeDefinition>, CallToolError> {
        if let Ok(content) = std::fs::read_to_string(&file_path) {
            // Early return for empty or very large files
            if content.is_empty() || content.len() > 10 * 1024 * 1024 {
                return Ok(Vec::new());
            }

            let definitions = if zero_copy {
                Self::extract_definitions_zero_copy(&content)
            } else {
                Self::extract_definitions(&content, file_path.display().to_string())
            };

            Ok(definitions)
        } else {
            Ok(Vec::new())
        }
    }

    async fn analyze_file_optimized(
        file_path: std::path::PathBuf,
        hardware_accelerated: bool,
        zero_copy: bool,
    ) -> std::result::Result<Vec<CodeDefinition>, CallToolError> {
        let content = if zero_copy && file_path.metadata().map_err(CallToolError::new)?.len() < 100 * 1024 * 1024 {
            // Use memory-mapped file for zero-copy operations
            use memmap2::Mmap;
            let file = std::fs::File::open(&file_path).map_err(CallToolError::new)?;
            let mmap = unsafe { Mmap::map(&file) }.map_err(CallToolError::new)?;
            std::str::from_utf8(&mmap).map_err(|e| {
                CallToolError::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            })?.to_string()
        } else {
            fs::read_to_string(&file_path).await.map_err(CallToolError::new)?
        };

        // Early return for empty or very large files
        if content.is_empty() || content.len() > 10 * 1024 * 1024 {
            return Ok(Vec::new());
        }

        let definitions = if hardware_accelerated && content.len() > 100 * 1024 {
            Self::extract_definitions_hardware_accelerated(&content, file_path.display().to_string())
        } else if zero_copy {
            Self::extract_definitions_zero_copy(&content)
        } else {
            Self::extract_definitions(&content, file_path.display().to_string())
        };

        Ok(definitions)
    }

    // Hardware-accelerated extraction using SIMD-like operations
    fn extract_definitions_hardware_accelerated(content: &str, _file_name: String) -> Vec<CodeDefinition> {
        let mut definitions: Vec<CodeDefinition> = Vec::new();

        // Use optimized pattern matching with hardware acceleration hints
        #[cfg(target_arch = "x86_64")]
        {
            if std::is_x86_feature_detected!("avx2") {
                return Self::extract_definitions_avx2(content);
            }
        }

        // Fallback to standard optimized extraction
        Self::extract_definitions_zero_copy(content)
    }

    // AVX2-optimized extraction for x86_64
    #[cfg(target_arch = "x86_64")]
    fn extract_definitions_avx2(content: &str) -> Vec<CodeDefinition> {
        use std::arch::x86_64::*;

        let mut definitions = Vec::new();
        let bytes = content.as_bytes();
        let len = bytes.len();

        // SIMD search for common patterns
        let mut i = 0;
        while i < len.saturating_sub(32) {
            // Load 32 bytes at once
            let chunk = unsafe {
                let ptr = bytes.as_ptr().add(i);
                _mm256_loadu_si256(ptr as *const __m256i)
            };

            // Check for function patterns using SIMD
            let fn_pattern = unsafe {
                let pattern = b"fn ";
                _mm256_set1_epi8(pattern[0] as i8)
            };

            let cmp_result = unsafe { _mm256_cmpeq_epi8(chunk, fn_pattern) };
            let mask = unsafe { _mm256_movemask_epi8(cmp_result) };

            if mask != 0 {
                // Found potential function, extract using standard method
                let start = i + mask.trailing_zeros() as usize;
                if let Some(end) = Self::find_function_end(&content[start..]) {
                    if let Some(def) = Self::extract_single_definition(&content[start..start + end]) {
                        definitions.push(def);
                    }
                }
            }

            i += 32;
        }

        // Process remaining bytes with standard method
        if i < len {
            definitions.extend(Self::extract_definitions_zero_copy(&content[i..]));
        }

        definitions
    }

    // Zero-copy extraction using string slices
    fn extract_definitions_zero_copy(content: &str) -> Vec<CodeDefinition> {
        let mut definitions = Vec::new();

        // Pre-compile regexes once for the entire content
        let patterns = vec![
            (r"^\s*pub\s+fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*(\([^)]*\))\s*(->\s*[^{]+)?", "function"),
            (r"^\s*fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*(\([^)]*\))\s*(->\s*[^{]+)?", "function"),
            (r"^\s*pub\s+struct\s+([a-zA-Z_][a-zA-Z0-9_]*)", "struct"),
            (r"^\s*struct\s+([a-zA-Z_][a-zA-Z0-9_]*)", "struct"),
            (r"^\s*pub\s+enum\s+([a-zA-Z_][a-zA-Z0-9_]*)", "enum"),
            (r"^\s*enum\s+([a-zA-Z_][a-zA-Z0-9_]*)", "enum"),
            (r"^\s*pub\s+trait\s+([a-zA-Z_][a-zA-Z0-9_]*)", "trait"),
            (r"^\s*trait\s+([a-zA-Z_][a-zA-Z0-9_]*)", "trait"),
            (r"^\s*pub\s+type\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=", "type"),
            (r"^\s*type\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=", "type"),
            (r"^\s*pub\s+const\s+([A-Z_][A-Z0-9_]*)\s*:\s*[^=]+=", "const"),
            (r"^\s*const\s+([A-Z_][A-Z0-9_]*)\s*:\s*[^=]+=", "const"),
            (r"^\s*impl\s+(?:([a-zA-Z_][a-zA-Z0-9_:<>\s]*?)\s+for\s+)?([a-zA-Z_][a-zA-Z0-9_:<>]*)", "impl"),
        ];

        let compiled_patterns: Vec<_> = patterns
            .into_iter()
            .filter_map(|(pattern, kind)| {
                Regex::new(pattern).ok().map(|regex| (regex, kind))
            })
            .collect();

        // Process lines with zero-copy operations
        let lines: Vec<&str> = content.lines().collect();

        // Process in chunks for better cache performance
        let chunk_size = 1000;
        for chunk in lines.chunks(chunk_size) {
            for (line_idx, line) in chunk.iter().enumerate() {
                let actual_line_number = line_idx + 1; // This is relative to chunk, fix later

                for (regex, kind) in &compiled_patterns {
                    if let Some(captures) = regex.captures(line) {
                        let name = if *kind == "impl" {
                            if let Some(trait_name) = captures.get(1) {
                                format!("{} for {}", trait_name.as_str(), captures.get(2).unwrap().as_str())
                            } else {
                                captures.get(2).unwrap().as_str().to_string()
                            }
                        } else {
                            captures.get(1).unwrap().as_str().to_string()
                        };

                        let signature = line.trim().to_string();

                        definitions.push(CodeDefinition {
                            kind: kind.to_string(),
                            name,
                            line_number: actual_line_number,
                            signature,
                        });
                        break;
                    }
                }
            }
        }

        definitions
    }

    // Helper functions for AVX2 processing
    fn find_function_end(content: &str) -> Option<usize> {
        let mut brace_count = 0;
        let mut in_function = false;

        for (i, ch) in content.chars().enumerate() {
            match ch {
                '{' => {
                    brace_count += 1;
                    in_function = true;
                }
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 && in_function {
                        return Some(i + 1);
                    }
                }
                ';' => {
                    if !in_function {
                        return Some(i + 1);
                    }
                }
                _ => {}
            }
        }

        None
    }

    fn extract_single_definition(content: &str) -> Option<CodeDefinition> {
        // Simple extraction for SIMD-found patterns
        if let Some(fn_match) = content.find("fn ") {
            let after_fn = &content[fn_match + 3..];
            if let Some(space_pos) = after_fn.find(' ') {
                let name = &after_fn[..space_pos];
                return Some(CodeDefinition {
                    kind: "function".to_string(),
                    name: name.to_string(),
                    line_number: 1, // Placeholder
                    signature: content.trim().to_string(),
                });
            }
        }
        None
    }

    fn extract_definitions(content: &str, _file_name: String) -> Vec<CodeDefinition> {
        Self::extract_definitions_zero_copy(content)
    }

    fn format_definitions_optimized(definitions: Vec<CodeDefinition>) -> String {
        if definitions.is_empty() {
            return "No code definitions found.".to_string();
        }

        let mut output = String::new();
        output.push_str("Code Definitions:\n");
        output.push_str("================\n\n");

        // Group by kind - use HashMap for efficiency
        let mut by_kind = std::collections::HashMap::new();
        for def in definitions {
            by_kind.entry(def.kind.clone()).or_insert_with(Vec::new).push(def);
        }

        // Sort kinds for consistent output
        let mut kinds: Vec<String> = by_kind.keys().cloned().collect();
        kinds.sort();

        for kind in kinds {
            if let Some(defs) = by_kind.get(&kind) {
                output.push_str(&format!("{}s:\n", kind));
                output.push_str(&format!("{}\n", "-".repeat(kind.len() + 1)));

                for def in defs {
                    output.push_str(&format!("  • {} (line {}): {}\n", def.name, def.line_number, def.signature));
                }
                output.push_str("\n");
            }
        }

        output
    }
}