use std::path::Path;

use rust_mcp_sdk::macros::{mcp_tool, JsonSchema};
use rust_mcp_sdk::schema::TextContent;
use rust_mcp_sdk::schema::{schema_utils::CallToolError, CallToolResult};

use crate::fs_service::FileSystemService;

#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
/// Represents a text replacement operation for search_and_replace.
pub struct SearchReplaceOperation {
    /// Text to search for - must match exactly or as regex.
    pub search: String,
    /// Text to replace the matched text with.
    pub replace: String,
    /// Whether to treat search as a regex pattern (default: false)
    #[serde(default)]
    pub use_regex: Option<bool>,
    /// Starting line number for restricted replacement (1-based)
    #[serde(default)]
    pub start_line: Option<u64>,
    /// Ending line number for restricted replacement (1-based)
    #[serde(default)]
    pub end_line: Option<u64>,
    /// Whether to ignore case when matching (default: false)
    #[serde(default)]
    pub ignore_case: Option<bool>,
}

#[mcp_tool(
    name = "search_and_replace",
    title = "Search and Replace",
    description = concat!(
        "Search and replace text in a file. ",
        "Supports exact string matching or regex patterns, with optional dry-run mode. ",
        "Returns a git-style diff showing the changes made. ",
        "Only works within allowed directories."
    ),
    destructive_hint = false,
    idempotent_hint = false,
    open_world_hint = false,
    read_only_hint = false
)]
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug, JsonSchema)]
pub struct SearchReplaceTool {
    /// The path of the file to edit.
    pub path: String,
    /// The list of search and replace operations to apply.
    pub edits: Vec<SearchReplaceOperation>,
    /// Preview changes using git-style diff format without applying them.
    #[serde(
        default,
        skip_serializing_if = "std::option::Option::is_none"
    )]
    pub dry_run: Option<bool>,
}

impl SearchReplaceTool {
    pub async fn run_tool(
        params: Self,
        context: &FileSystemService,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        let diff = context
            .search_replace_edits(Path::new(&params.path), params.edits, params.dry_run, None)
            .await
            .map_err(CallToolError::new)?;

        Ok(CallToolResult::text_content(vec![TextContent::from(diff)]))
    }
}