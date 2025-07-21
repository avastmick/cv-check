//! PDF text extraction module

use crate::ai::{AIError, Result};
use std::path::Path;

/// Extract text content from a PDF file
///
/// # Errors
///
/// Returns an error if the file doesn't exist, isn't a PDF, or extraction fails
pub fn extract_text_from_pdf(path: &Path) -> Result<String> {
    // Check if file exists
    if !path.exists() {
        return Err(AIError::PdfParse(format!(
            "PDF file not found: {}",
            path.display()
        )));
    }

    // Check if it's a PDF file
    if path.extension().and_then(|s| s.to_str()) != Some("pdf") {
        return Err(AIError::PdfParse(format!(
            "Not a PDF file: {}",
            path.display()
        )));
    }

    // Use pdf_extract to get the text directly as a string
    let text = pdf_extract::extract_text(path)
        .map_err(|e| AIError::PdfParse(format!("Failed to extract text from PDF: {e}")))?;

    // Clean up the text - remove excessive whitespace while preserving structure
    let cleaned_text = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    if cleaned_text.is_empty() {
        return Err(AIError::PdfParse(
            "No text content found in PDF".to_string(),
        ));
    }

    Ok(cleaned_text)
}
