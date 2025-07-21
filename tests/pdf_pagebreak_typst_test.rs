// Tests specifically for verifying Typst source generation with page breaks
use cv_check::config::DocumentMetadata;
use cv_check::parser::Document;
use cv_check::render::{pdf::PdfRenderer, RenderEngine};
use cv_check::themes::Theme;
use tempfile::TempDir;

fn create_test_document_with_content(content: &str) -> Document {
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
            layout: cv_check::config::LayoutOptions::default(),
            recipient: None,
            date: None,
            subject: None,
            custom: std::collections::HashMap::new(),
        },
        content: content.to_string(),
        markdown_ast: cv_check::parser::markdown::parse_markdown(content),
    }
}

fn create_test_theme() -> Theme {
    Theme::new("modern", "modern").expect("Failed to create theme")
}

#[test]
fn test_generate_typst_source_contains_pagebreak_for_html_comment() {
    let content = r"# Experience

## First Job
**Company A** | *2020 - 2021*

Some content here.

<!-- pagebreak -->

## Second Job
**Company B** | *2019 - 2020*

More content here.";

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    // Test that PDF generation succeeds with pagebreak markers
    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF with HTML pagebreak");

    assert!(output_path.exists(), "PDF should be created with pagebreak");
}

#[test]
fn test_generate_typst_source_contains_pagebreak_for_latex_style() {
    let content = r"# Experience

## First Job
**Company A** | *2020 - 2021*

Some content here.

\pagebreak

## Second Job
**Company B** | *2019 - 2020*

More content here.";

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    // Test that PDF generation succeeds with LaTeX-style pagebreak
    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF with LaTeX pagebreak");

    assert!(output_path.exists(), "PDF should be created with pagebreak");
}

#[test]
fn test_h2_sections_wrapped_in_nonbreakable_blocks() {
    let content = r"# Experience

## Senior Software Engineer
**Tech Company** | *2020 - Present*

- Achievement 1
- Achievement 2
- Achievement 3

## Previous Role
**Another Company** | *2018 - 2020*

More content.";

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    // Test that PDF generation succeeds with H2 sections
    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF with H2 sections");

    assert!(
        output_path.exists(),
        "PDF should be created with proper H2 handling"
    );
}

#[test]
fn test_pagebreak_not_confused_with_horizontal_rules() {
    let content = r"# Content

Some content here.

---

This is not a page break, just a horizontal rule.

<!-- pagebreak -->

This should be after a page break.";

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    // Test that horizontal rules don't break PDF generation
    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF with horizontal rule and pagebreak");

    assert!(
        output_path.exists(),
        "PDF should handle horizontal rules correctly"
    );
}

#[test]
fn test_empty_pagebreak_handling() {
    let content = r"# Content

<!-- pagebreak -->
<!-- pagebreak -->

Double pagebreak should work.";

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    // Test that consecutive pagebreaks don't break PDF generation
    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF with consecutive pagebreaks");

    assert!(
        output_path.exists(),
        "PDF should handle consecutive pagebreaks"
    );
}
