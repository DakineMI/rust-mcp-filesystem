use std::path::Path;

use rust_mcp_sdk::macros::{mcp_tool, JsonSchema};
use rust_mcp_sdk::schema::TextContent;
use rust_mcp_sdk::schema::{schema_utils::CallToolError, CallToolResult};

use crate::fs_service::FileSystemService;

#[mcp_tool(
    name = "insert_content",
    title="Insert Content",
    description = concat!("Add new lines of content into a file without modifying existing content. ",
    "Specify the line number to insert before, or use line 0 to append to the end. ",
    "Ideal for adding imports, functions, configuration blocks, log entries, or any multi-line text block."),
    destructive_hint = false,
    idempotent_hint = false,
    open_world_hint = false,
    read_only_hint = false
)]
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
pub struct InsertContentTool {
    /// The path of the file to modify.
    pub path: String,
    /// The line number where content will be inserted (1-based). Use 0 to append at end of file. Use any positive number to insert before that line.
    pub line: u64,
    /// The content to insert at the specified line.
    pub content: String,
}

impl InsertContentTool {
    pub async fn run_tool(
        params: Self,
        context: &FileSystemService,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let valid_path = context
            .validate_path(Path::new(&params.path))
            .map_err(CallToolError::new)?;

        // Check file size for large file handling
        let metadata = tokio::fs::metadata(&valid_path).await.map_err(CallToolError::new)?;
        let file_size = metadata.len();

        // For very large files (>50MB), use streaming approach
        if file_size > 50 * 1024 * 1024 {
            return Self::insert_content_streaming(valid_path, params).await;
        }

        // Read the current file content
        let content = tokio::fs::read_to_string(&valid_path)
            .await
            .map_err(CallToolError::new)?;

        // Insert the content at the specified line
        let modified_content = Self::insert_content_at_line(&content, params.line, &params.content)?;

        // Write the modified content back to the file
        tokio::fs::write(&valid_path, &modified_content)
            .await
            .map_err(CallToolError::new)?;

        // Generate a diff output
        let diff_output = Self::create_diff_output(&content, &modified_content, &params.path);

        Ok(CallToolResult::text_content(vec![TextContent::from(diff_output)]))
    }

    async fn insert_content_streaming(
        valid_path: std::path::PathBuf,
        params: Self,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        // For very large files, we'd need to implement streaming
        // For now, return an error indicating the file is too large
        Err(CallToolError::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "File too large for streaming content insertion (>50MB). Please use smaller files or implement streaming support.",
        )))
    }

    fn insert_content_at_line(
        content: &str,
        line_number: u64,
        content_to_insert: &str,
    ) -> std::result::Result<String, CallToolError> {
        // Pre-allocate result with estimated capacity
        let estimated_capacity = content.len() + content_to_insert.len() + 10;
        let mut result = String::with_capacity(estimated_capacity);

        if line_number == 0 {
            // Append to end - most efficient case
            result.push_str(content);
            result.push_str(content_to_insert);
            return Ok(result);
        }

        // Handle insertion at specific line
        let target_line = (line_number as usize).saturating_sub(1); // Convert to 0-based
        let mut current_line = 0;
        let mut chars = content.char_indices().peekable();

        // Copy content up to target line
        while current_line < target_line {
            if let Some((_, ch)) = chars.next() {
                result.push(ch);
                if ch == '\n' {
                    current_line += 1;
                }
            } else {
                break;
            }
        }

        // Insert the new content
        result.push_str(content_to_insert);

        // Copy the rest of the original content
        while let Some((_, ch)) = chars.next() {
            result.push(ch);
        }

        // Validate that we didn't exceed reasonable bounds
        if result.len() > content.len() * 2 {
            return Err(CallToolError::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Resulting content is excessively large. Operation may have failed.",
            )));
        }

        Ok(result)
    }

    fn create_diff_output(original: &str, modified: &str, file_path: &str) -> String {
        use similar::TextDiff;

        let diff = TextDiff::from_lines(original, modified);
        let mut unified_diff = diff.unified_diff();

        format!(
            "Index: {}\n{}\n{}",
            file_path,
            "=".repeat(68),
            unified_diff
                .header(
                    format!("a/{}", file_path).as_str(),
                    format!("b/{}", file_path).as_str(),
                )
                .to_string()
        )
    }
}