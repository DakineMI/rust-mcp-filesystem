mod search_and_replace;
mod create_directory;
mod directory_tree;
mod edit_file;
mod get_file_info;
mod list_allowed_directories;
mod list_directory;
mod list_directory_with_sizes;
mod move_file;
mod read_files;
mod read_multiple_files;
mod search_file;
mod search_files_content;
mod write_file;
mod zip_unzip;
mod list_files;
mod apply_diff;
mod insert_content;
mod list_code_definition_names;

pub use search_and_replace::{SearchReplaceOperation, SearchReplaceTool};
pub use create_directory::CreateDirectoryTool;
pub use directory_tree::DirectoryTreeTool;
pub use edit_file::{EditFileTool, EditOperation};
pub use get_file_info::GetFileInfoTool;
pub use list_allowed_directories::ListAllowedDirectoriesTool;
pub use list_directory::ListDirectoryTool;
pub use list_directory_with_sizes::ListDirectoryWithSizesTool;
pub use move_file::MoveFileTool;
pub use read_files::ReadFileTool;
pub use read_multiple_files::ReadMultipleFilesTool;
pub use rust_mcp_sdk::tool_box;
pub use search_file::SearchFilesTool;
pub use search_files_content::SearchFilesContentTool;
pub use write_file::WriteFileTool;
pub use zip_unzip::{UnzipFileTool, ZipDirectoryTool, ZipFilesTool};
pub use list_files::ListFilesTool;
pub use apply_diff::ApplyDiffTool;
pub use insert_content::InsertContentTool;
pub use list_code_definition_names::ListCodeDefinitionNamesTool;

//Generate FileSystemTools enum , tools() function, and TryFrom<CallToolRequestParams> trait implementation
tool_box!(
    FileSystemTools,
    [
        ReadFileTool,
        CreateDirectoryTool,
        DirectoryTreeTool,
        EditFileTool,
        SearchReplaceTool,
        GetFileInfoTool,
        ListAllowedDirectoriesTool,
        ListDirectoryTool,
        MoveFileTool,
        ReadMultipleFilesTool,
        SearchFilesTool,
        WriteFileTool,
        ZipFilesTool,
        UnzipFileTool,
        ZipDirectoryTool,
        SearchFilesContentTool,
        ListDirectoryWithSizesTool,
        ListFilesTool,
        ApplyDiffTool,
        InsertContentTool,
        ListCodeDefinitionNamesTool
    ]
);

impl FileSystemTools {
    // Determines whether the filesystem tool requires write access to the filesystem.
    // Returns `true` for tools that modify files or directories, and `false` otherwise.
    pub fn require_write_access(&self) -> bool {
        match self {
            FileSystemTools::CreateDirectoryTool(_)
            | FileSystemTools::MoveFileTool(_)
            | FileSystemTools::WriteFileTool(_)
            | FileSystemTools::EditFileTool(_)
            | FileSystemTools::SearchReplaceTool(_)
            | FileSystemTools::ZipFilesTool(_)
            | FileSystemTools::UnzipFileTool(_)
            | FileSystemTools::ZipDirectoryTool(_)
            | FileSystemTools::ApplyDiffTool(_)
            | FileSystemTools::InsertContentTool(_) => true,
            FileSystemTools::ReadFileTool(_)
            | FileSystemTools::DirectoryTreeTool(_)
            | FileSystemTools::GetFileInfoTool(_)
            | FileSystemTools::ListAllowedDirectoriesTool(_)
            | FileSystemTools::ListDirectoryTool(_)
            | FileSystemTools::ReadMultipleFilesTool(_)
            | FileSystemTools::SearchFilesContentTool(_)
            | FileSystemTools::ListDirectoryWithSizesTool(_)
            | FileSystemTools::SearchFilesTool(_)
            | FileSystemTools::ListFilesTool(_)
            | FileSystemTools::ListCodeDefinitionNamesTool(_) => false,
        }
    }
}
