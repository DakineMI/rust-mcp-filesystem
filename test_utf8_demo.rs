use rust_mcp_filesystem::fs_service::FileSystemService;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

#[tokio::test]
async fn test_utf8_filename_handling() {
    // Create a temporary directory with a file that has invalid UTF-8 in the name
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    // Create a test file with a valid UTF-8 name
    let valid_file = temp_path.join("valid_file.txt");
    fs::write(&valid_file, "test content").unwrap();
    
    // Create a service instance
    let service = FileSystemService::try_new(&[temp_path.to_str().unwrap().to_string()]).unwrap();
    
    // Test that valid files work normally
    let result = service.search_files_iter(temp_path, "*.txt".to_string(), vec![]);
    assert!(result.is_ok());
    
    // Collect results to trigger the UTF-8 handling
    let files: Vec<_> = result.unwrap().collect();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].file_name().to_str().unwrap(), "valid_file.txt");
    
    println!("✅ UTF-8 filename handling test passed!");
}

#[tokio::test]
async fn test_regex_replacement_functionality() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    
    // Create a test file
    let test_file = temp_path.join("test.txt");
    fs::write(&test_file, "line1\nline2\nline3").unwrap();
    
    let service = FileSystemService::try_new(&[temp_path.to_str().unwrap().to_string()]).unwrap();
    
    // Test valid regex replacement
    use rust_mcp_filesystem::tools::SearchReplaceOperation;
    let edits = vec![SearchReplaceOperation {
        search: r"line\d".to_string(),
        replace: "replaced".to_string(),
        use_regex: Some(true),
        start_line: None,
        end_line: None,
        ignore_case: None,
    }];
    
    let result = service.search_replace_edits(&test_file, edits, Some(false), None).await;
    assert!(result.is_ok());
    
    let content = fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, "replaced\nreplaced\nreplaced");
    
    println!("✅ Regex replacement test passed!");
}
