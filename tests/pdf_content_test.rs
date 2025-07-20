use cv_gen::config::DocumentMetadata;
use cv_gen::parser::Document;
use cv_gen::render::{pdf::PdfRenderer, RenderEngine};
use cv_gen::themes::Theme;
use pdf_extract::extract_text;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn typst_is_available() -> bool {
    std::process::Command::new("typst")
        .arg("--version")
        .output()
        .is_ok()
}

fn verify_pdf_contains_text(pdf_path: &Path, expected_texts: &[&str]) {
    let text = extract_text(pdf_path).expect("Failed to extract text from PDF");

    for expected in expected_texts {
        let text_normalized = text.replace([' ', '\n'], "").to_lowercase();
        let expected_normalized = expected.replace([' ', '\n'], "").to_lowercase();

        assert!(
            text.contains(expected) || text_normalized.contains(&expected_normalized),
            "PDF missing expected text: '{expected}'\nActual PDF content:\n{text}"
        );
    }
}

fn create_test_document() -> Document {
    let content = "# Professional Summary\n\nExperienced software engineer.";

    Document {
        metadata: DocumentMetadata {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            phone: Some("+1 234 567 8900".to_string()),
            location: Some("Test City, TC".to_string()),
            linkedin: None,
            github: None,
            website: None,
            font_theme: "modern".to_string(),
            color_theme: "modern".to_string(),
            layout: cv_gen::config::LayoutOptions::default(),
            recipient: None,
            date: None,
            subject: None,
            custom: std::collections::HashMap::new(),
        },
        content: content.to_string(),
        markdown_ast: cv_gen::parser::markdown::parse_markdown(content),
    }
}

#[test]
fn test_pdf_requires_typst() {
    if typst_is_available() {
        println!("Typst is available, skipping requirement test");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document();
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");

    let result = renderer.render(&doc, &theme, &output_path);
    assert!(result.is_err(), "Should fail without Typst");

    let error_msg = result.expect_err("Should fail without Typst").to_string();
    assert!(
        error_msg.contains("Typst is required"),
        "Error should mention Typst requirement"
    );
    assert!(
        error_msg.contains("brew install typst"),
        "Error should include installation instructions"
    );
}

#[test]
fn test_pdf_contains_actual_content() {
    if !typst_is_available() {
        eprintln!("Skipping test: Typst not available");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document();
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    // Verify PDF structure
    let pdf_bytes = fs::read(&output_path).expect("Failed to read PDF");
    assert!(
        pdf_bytes.len() > 1000,
        "PDF should have substantial content"
    );
    assert!(
        &pdf_bytes[0..8] == b"%PDF-1.7" || &pdf_bytes[0..5] == b"%PDF-",
        "Should be a valid PDF"
    );

    // Verify actual content appears in PDF
    let expected_content = vec![
        "Test User",
        "test@example.com",
        "+1 234 567 8900",
        "Test City, TC",
        "Professional Summary",
        "Experienced software engineer",
    ];
    verify_pdf_contains_text(&output_path, &expected_content);
}

#[test]
fn test_pdf_has_valid_structure() {
    if !typst_is_available() {
        eprintln!("Skipping test: Typst not available");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document();
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    let pdf_bytes = fs::read(&output_path).expect("Failed to read PDF");

    // Check PDF header
    assert!(
        &pdf_bytes[0..8] == b"%PDF-1.7" || &pdf_bytes[0..5] == b"%PDF-",
        "Should have valid PDF header"
    );

    // Check for EOF marker at the end
    let end = std::str::from_utf8(&pdf_bytes[pdf_bytes.len().saturating_sub(10)..]).unwrap_or("");
    assert!(end.contains("%%EOF"), "PDF should end with EOF marker");
}

#[test]
fn test_pdf_with_minimal_content() {
    if !typst_is_available() {
        eprintln!("Skipping test: Typst not available");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let mut doc = create_test_document();
    doc.metadata.phone = None;
    doc.metadata.location = None;
    doc.content = "Short content".to_string();
    doc.markdown_ast = cv_gen::parser::markdown::parse_markdown(&doc.content);

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    let pdf_bytes = fs::read(&output_path).expect("Failed to read PDF");
    assert!(
        pdf_bytes.len() > 500,
        "PDF should have content even with minimal input"
    );
    assert!(&pdf_bytes[0..5] == b"%PDF-", "Should be a valid PDF");

    // Verify minimal content still appears
    let expected_content = vec!["Test User", "test@example.com", "Short content"];
    verify_pdf_contains_text(&output_path, &expected_content);
}

#[test]
fn test_pdf_with_unicode_content() {
    if !typst_is_available() {
        eprintln!("Skipping test: Typst not available");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let mut doc = create_test_document();
    doc.metadata.name = "José García".to_string();
    doc.content = "Experiência em programação • Développement • 开发经验".to_string();
    doc.markdown_ast = cv_gen::parser::markdown::parse_markdown(&doc.content);

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");

    // Should handle Unicode properly through Typst
    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF with Unicode");

    let pdf_bytes = fs::read(&output_path).expect("Failed to read PDF");
    assert!(pdf_bytes.len() > 500, "PDF should handle Unicode content");

    // Verify Unicode content appears correctly
    let expected_content = vec![
        "José García",
        "test@example.com",
        "Experiência",
        "programação",
        "Développement", // Note: Chinese characters might not extract well from PDF, so testing western characters
    ];
    verify_pdf_contains_text(&output_path, &expected_content);
}
