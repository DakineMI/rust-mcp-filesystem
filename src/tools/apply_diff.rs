use std::path::Path;

use rust_mcp_sdk::macros::{mcp_tool, JsonSchema};
use rust_mcp_sdk::schema::TextContent;
use rust_mcp_sdk::schema::{schema_utils::CallToolError, CallToolResult};

use crate::fs_service::FileSystemService;

#[mcp_tool(
    name = "apply_diff",
    title="Apply Diff",
    description = concat!("Apply PRECISE, TARGETED modifications to one or more files by searching for ",
    "specific sections of content and replacing them. This tool is for SURGICAL EDITS ONLY - ",
    "specific changes to existing code."),
    destructive_hint = false,
    idempotent_hint = false,
    open_world_hint = false,
    read_only_hint = false
)]
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
pub struct ApplyDiffTool {
    /// The path of the file to modify.
    pub path: String,
    /// The diff to apply to the file.
    pub diff: DiffContent,
}

impl ApplyDiffTool {
    pub async fn run_tool(
        params: Self,
        context: &FileSystemService,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let valid_path = context
            .validate_path(Path::new(&params.path))
            .map_err(CallToolError::new)?;

        // Read the current file content
        let content = tokio::fs::read_to_string(&valid_path)
            .await
            .map_err(CallToolError::new)?;

        // Parse the diff content
        let (old_content, new_content) = Self::parse_diff_content(&params.diff.content)?;

        // Apply the diff
        let modified_content = Self::apply_diff_to_content(&content, &old_content, &new_content, params.diff.start_line)?;

        // Write the modified content back to the file
        tokio::fs::write(&valid_path, &modified_content)
            .await
            .map_err(CallToolError::new)?;

        // Generate a diff output
        let diff_output = Self::create_diff_output(&content, &modified_content, &params.path);

        Ok(CallToolResult::text_content(vec![TextContent::from(diff_output)]))
    }

    fn parse_diff_content(diff_content: &str) -> std::result::Result<(String, String), CallToolError> {
        let lines: Vec<&str> = diff_content.lines().collect();

        if lines.len() < 6 {
            return Err(CallToolError::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Diff content is too short or malformed",
            )));
        }

        // Expected format:
        // <<<<<<< SEARCH
        // :start_line: (required) The line number of original content where the search block starts.
        // -------
        // [exact content to find including whitespace]
        // =======
        // [new content to replace with]
        // >>>>>>> REPLACE

        if lines[0].trim() != "<<<<<<< SEARCH" {
            return Err(CallToolError::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Diff content must start with '<<<<<<< SEARCH'",
            )));
        }

        if lines.last().unwrap().trim() != ">>>>>>> REPLACE" {
            return Err(CallToolError::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Diff content must end with '>>>>>>> REPLACE'",
            )));
        }

        // Find the separator lines
        let mut old_start = None;
        let mut new_start = None;

        for (i, line) in lines.iter().enumerate() {
            if line.trim() == "-------" && old_start.is_none() {
                old_start = Some(i + 1);
            } else if line.trim() == "=======" && old_start.is_some() && new_start.is_none() {
                new_start = Some(i + 1);
                break;
            }
        }

        let old_start = old_start.ok_or_else(|| {
            CallToolError::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Could not find old content separator '-------'",
            ))
        })?;

        let new_start = new_start.ok_or_else(|| {
            CallToolError::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Could not find new content separator '======='",
            ))
        })?;

        // Extract old content (from old_start to new_start - 1)
        let old_content_lines = &lines[old_start..new_start - 1];
        let old_content = old_content_lines.join("\n");

        // Extract new content (from new_start to end - 1)
        let new_content_lines = &lines[new_start..lines.len() - 1];
        let new_content = new_content_lines.join("\n");

        Ok((old_content, new_content))
    }

    fn apply_diff_to_content(
        content: &str,
        old_content: &str,
        new_content: &str,
        _start_line: u64,
    ) -> std::result::Result<String, CallToolError> {
        // Find the exact match
        if let Some(pos) = content.find(old_content) {
            let before = &content[..pos];
            let after = &content[pos + old_content.len()..];

            // Construct the new content
            let mut result = String::with_capacity(before.len() + new_content.len() + after.len());
            result.push_str(before);
            result.push_str(new_content);
            result.push_str(after);

            Ok(result)
        } else {
            Err(CallToolError::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find the specified content to replace",
            )))
        }
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

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
pub struct DiffContent {
    /// The search/replace block defining the changes.
    pub content: String,
    /// The line number of original content where the search block starts.
    pub start_line: u64,
}