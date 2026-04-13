
use std::path::Path;

/// Check if a file path has a supported extension
pub fn is_supported_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|ext| {
            let ext_lower = ext.to_lowercase();
            ext_lower == "one" || ext_lower == "onetoc2"
        })
        .unwrap_or(false)
}

/// Get a human-readable description of file type
pub fn get_file_type_description(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).map(|s| s.to_lowercase()).as_deref() {
        Some("onetoc2") => "OneNote Notebook (Table of Contents)",
        Some("one") => "OneNote Section/Page",
        _ => "Unknown file type",
    }
}

/// Validate that a path exists and is readable
pub fn validate_path(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("File does not exist: {:?}", path));
    }
    
    if !path.is_file() {
        return Err(format!("Not a file: {:?}", path));
    }
    
    if !is_supported_file(path) {
        return Err(format!("Unsupported file type: {:?}", path));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_supported_extensions() {
        assert!(is_supported_file(Path::new("notebook.onetoc2")));
        assert!(is_supported_file(Path::new("section.one")));
        assert!(is_supported_file(Path::new("FILE.ONETOC2")));
        assert!(!is_supported_file(Path::new("document.pdf")));
        assert!(!is_supported_file(Path::new("image.png")));
    }

    #[test]
    fn test_file_type_description() {
        assert_eq!(
            get_file_type_description(Path::new("notebook.onetoc2")),
            "OneNote Notebook (Table of Contents)"
        );
        assert_eq!(
            get_file_type_description(Path::new("section.one")),
            "OneNote Section/Page"
        );
    }
}
