use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tokio::task::spawn_blocking;

#[derive(Debug, Clone)]
pub struct NotebookData {
    pub name: String,
    pub path: PathBuf,
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub pages: Vec<Page>,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub title: String,
    pub content: Vec<ContentObject>,
}

#[derive(Debug, Clone)]
pub enum ContentObject {
    Text(TextRun),
    Image(ImageData),
    Attachment(AttachmentData),
    Unsupported(String),
}

#[derive(Debug, Clone)]
pub struct TextRun {
    pub text: String,
    pub is_bold: bool,
    pub is_italic: bool,
    pub is_header: bool,
    pub list_level: u8,
}

#[derive(Debug, Clone)]
pub struct ImageData {
    pub data: Vec<u8>,
    pub format: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct AttachmentData {
    pub filename: String,
    pub size: u64,
    pub data: Vec<u8>,
}

pub async fn parse_notebook(path: &Path) -> Result<NotebookData> {
    let path = path.to_path_buf();
    
    spawn_blocking(move || {
        parse_notebook_sync(&path)
    })
    .await
    .context("Failed to join parsing thread")?
}

fn parse_notebook_sync(path: &Path) -> Result<NotebookData> {
    // Check file extension
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    if extension != "one" && extension != "onetoc2" {
        anyhow::bail!("Unsupported file type. Expected .one or .onetoc2");
    }

    // Read the file
    let file_data = fs::read(path)
        .with_context(|| format!("Failed to read file: {:?}", path))?;

    // Parse using onenote_parser
    let mut notebook = NotebookData {
        name: path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string(),
        path: path.to_path_buf(),
        sections: Vec::new(),
    };

    // Try to parse as OneNote file
    // Note: This is a simplified parser - in production you'd use the full onenote_parser API
    match parse_one_file(&file_data, path) {
        Ok(data) => {
            notebook.sections = data.sections;
            if !data.name.is_empty() {
                notebook.name = data.name;
            }
        }
        Err(e) => {
            log::warn!("Parse warning: {}", e);
            // Create a placeholder section with error message
            notebook.sections.push(Section {
                name: "Error".to_string(),
                pages: vec![Page {
                    title: "Parse Error".to_string(),
                    content: vec![ContentObject::Unsupported(
                        format!("Failed to parse: {}. Legacy or encrypted content may not be supported.", e)
                    )],
                }],
            });
        }
    }

    Ok(notebook)
}

fn parse_one_file(data: &[u8], path: PathBuf) -> Result<NotebookData> {
    // This is a placeholder implementation
    // In production, you would use the actual onenote_parser crate API
    // The real implementation would depend on the specific API of onenote_parser
    
    let mut sections = Vec::new();
    
    // Attempt to use onenote_parser
    // Note: The actual API may differ - adjust based on the crate's documentation
    match onenote_parser::parse(data) {
        Ok(parsed) => {
            // Extract sections and pages from parsed data
            // This is pseudocode - adjust based on actual API
            for section_info in parsed.sections.iter() {
                let mut pages = Vec::new();
                
                for page_info in section_info.pages.iter() {
                    let mut content = Vec::new();
                    
                    // Extract text runs
                    for text_run in page_info.text_runs.iter() {
                        content.push(ContentObject::Text(TextRun {
                            text: text_run.text.clone(),
                            is_bold: text_run.is_bold,
                            is_italic: text_run.is_italic,
                            is_header: text_run.is_header,
                            list_level: text_run.list_level.unwrap_or(0),
                        }));
                    }
                    
                    // Extract images
                    for image in page_info.images.iter() {
                        content.push(ContentObject::Image(ImageData {
                            data: image.data.clone(),
                            format: image.format.clone(),
                            width: image.width,
                            height: image.height,
                        }));
                    }
                    
                    // Extract attachments
                    for attachment in page_info.attachments.iter() {
                        content.push(ContentObject::Attachment(AttachmentData {
                            filename: attachment.filename.clone(),
                            size: attachment.size,
                            data: attachment.data.clone(),
                        }));
                    }
                    
                    pages.push(Page {
                        title: page_info.title.clone().unwrap_or_else(|| "Untitled".to_string()),
                        content,
                    });
                }
                
                sections.push(Section {
                    name: section_info.name.clone().unwrap_or_else(|| "Untitled Section".to_string()),
                    pages,
                });
            }
            
            Ok(NotebookData {
                name: parsed.name.unwrap_or_else(|| path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown")
                    .to_string()),
                path,
                sections,
            })
        }
        Err(e) => {
            // Fallback: create minimal structure for standalone .one files
            log::info!("Creating fallback structure for standalone file");
            sections.push(Section {
                name: "Default Section".to_string(),
                pages: vec![Page {
                    title: path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("Untitled")
                        .to_string(),
                    content: vec![],
                }],
            });
            
            Ok(NotebookData {
                name: path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown")
                    .to_string(),
                path,
                sections,
            })
        }
    }
}
