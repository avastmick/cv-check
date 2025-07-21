//! Tests for PDF text extraction functionality

use cv_check::ai::pdf_parser::extract_text_from_pdf;
use std::path::Path;

#[test]
fn test_extract_text_from_valid_pdf() {
    // Use the existing jd_example.pdf
    let pdf_path = Path::new("examples/jd_example.pdf");

    // Extract text from PDF
    let result = extract_text_from_pdf(pdf_path);

    // Should succeed
    assert!(result.is_ok(), "Failed to extract text: {result:?}");

    let text = result.expect("Should extract text from valid PDF");

    // Should contain expected content
    assert!(!text.is_empty());
    // The PDF should contain job description content
    let text_len = text.len();
    assert!(text_len > 100, "Extracted text too short: {text_len} chars");
}

#[test]
fn test_extract_text_from_nonexistent_file() {
    let pdf_path = Path::new("tests/fixtures/nonexistent.pdf");

    let result = extract_text_from_pdf(pdf_path);

    // Should return an error
    assert!(result.is_err());
}

#[test]
fn test_extract_text_from_invalid_pdf() {
    // Create a path to a non-PDF file
    let pdf_path = Path::new("tests/fixtures/not_a_pdf.txt");

    let result = extract_text_from_pdf(pdf_path);

    // Should return an error
    assert!(result.is_err());
}

#[test]
fn test_extract_text_preserves_structure() {
    // Use the same PDF to test structure preservation
    let pdf_path = Path::new("examples/jd_example.pdf");

    let result = extract_text_from_pdf(pdf_path);
    assert!(result.is_ok());

    let text = result.expect("Should extract text from valid PDF");

    // Should preserve basic structure
    assert!(text.contains('\n'));

    // Should maintain readability
    let lines: Vec<&str> = text.lines().collect();
    assert!(lines.len() > 1);
}

#[test]
#[ignore = "Requires multipage PDF test file"]
fn test_extract_text_handles_multipage_pdf() {
    let pdf_path = Path::new("tests/fixtures/multipage_job_description.pdf");

    // Skip if file doesn't exist
    if !pdf_path.exists() {
        eprintln!("Skipping test: Multipage PDF fixture not found");
        return;
    }

    let result = extract_text_from_pdf(pdf_path);
    assert!(result.is_ok());

    let text = result.expect("Should extract text from valid PDF");

    // Should extract text from all pages
    assert!(text.len() > 1000, "Expected more text from multipage PDF");
}
