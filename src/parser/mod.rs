pub mod frontmatter;
pub mod markdown;

use crate::config::DocumentMetadata;
use crate::error::CvError;
use anyhow::Result;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Document {
    pub metadata: DocumentMetadata,
    pub content: String,
    pub markdown_ast: Vec<pulldown_cmark::Event<'static>>,
}

impl Document {
    /// Creates a document from a file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn from_file(path: &Path) -> Result<Self> {
        let content =
            std::fs::read_to_string(path).map_err(|_| CvError::FileNotFound(path.to_path_buf()))?;

        Self::from_string(&content, path)
    }

    /// Creates a document from a string containing markdown with frontmatter.
    ///
    /// # Errors
    ///
    /// Returns an error if the frontmatter or markdown cannot be parsed.
    pub fn from_string(input: &str, source_path: &Path) -> Result<Self> {
        let (metadata, content) = frontmatter::parse_frontmatter(input, source_path)?;
        let markdown_ast = markdown::parse_markdown(&content);

        Ok(Self {
            metadata,
            content,
            markdown_ast,
        })
    }

    /// Validates that the document contains required fields.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields (name, email) are missing.
    pub fn validate(&self) -> Result<()> {
        // Validate required fields
        if self.metadata.name.is_empty() {
            return Err(CvError::MissingField {
                field: "name".to_string(),
                file: std::path::PathBuf::from("document"),
            }
            .into());
        }

        if self.metadata.email.is_empty() {
            return Err(CvError::MissingField {
                field: "email".to_string(),
                file: std::path::PathBuf::from("document"),
            }
            .into());
        }

        Ok(())
    }
}
