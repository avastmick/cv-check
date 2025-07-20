use cv_gen::config::DocumentMetadata;
use cv_gen::parser::Document;
use cv_gen::render::{pdf::PdfRenderer, RenderEngine};
use cv_gen::themes::Theme;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn create_test_document() -> Document {
    let content = r"# Professional Summary

Experienced software engineer with 10+ years building scalable applications.

# Experience

## Senior Software Engineer
**Tech Corp** | *2020 - Present*

- Led development of microservices architecture
- Mentored team of 8 engineers
- Reduced deployment time by 70%

# Education

## B.S. Computer Science
**Stanford University** | *2010 - 2014*

# Skills

**Languages**: Rust, Python, JavaScript
**Tools**: Docker, Kubernetes, AWS";

    Document {
        metadata: DocumentMetadata {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            phone: Some("+1 234 567 8900".to_string()),
            location: Some("San Francisco, CA".to_string()),
            linkedin: Some("johndoe".to_string()),
            github: Some("johndoe".to_string()),
            website: Some("https://johndoe.com".to_string()),
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

fn create_test_theme() -> Theme {
    Theme::new("modern", "modern").expect("Failed to create theme")
}

#[test]
fn test_pdf_renderer_creates_output_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document();
    let theme = create_test_theme();

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    assert!(output_path.exists(), "PDF file should be created");

    // Check that it's a valid PDF (starts with %PDF)
    let content = fs::read(&output_path).expect("Failed to read PDF");
    assert!(content.len() > 4, "PDF should have content");
    assert_eq!(&content[0..4], b"%PDF", "File should be a valid PDF");
}

#[test]
fn test_pdf_renderer_with_custom_template() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let template_path = temp_dir.path().join("custom.typ");
    let output_path = temp_dir.path().join("test.pdf");

    // Create a custom template
    fs::write(
        &template_path,
        r#"
#let cv(name: "", email: "", body) = {
  set document(title: name)
  set text(font: "Arial", size: 12pt)
  
  align(center)[
    #text(size: 24pt, weight: "bold")[#name]
    #email
  ]
  
  body
}
"#,
    )
    .expect("Failed to write template");

    let renderer = PdfRenderer::new(Some(&template_path))
        .expect("Failed to create renderer with custom template");
    let doc = create_test_document();
    let theme = create_test_theme();

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF with custom template");

    assert!(output_path.exists(), "PDF file should be created");
}

#[test]
fn test_pdf_renderer_handles_special_characters() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let mut doc = create_test_document();
    doc.metadata.name = "Jean-François O'Malley".to_string();
    let new_content = "# Summary\n\nExperience with \"special\" characters & symbols like <>&{}.";
    doc.content = new_content.to_string();
    doc.markdown_ast = cv_gen::parser::markdown::parse_markdown(new_content);

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = create_test_theme();

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF with special characters");

    assert!(output_path.exists(), "PDF file should be created");
}

#[test]
fn test_pdf_renderer_two_column_layout() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let mut doc = create_test_document();
    doc.metadata.layout.columns = 2;

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = create_test_theme();

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF with two-column layout");

    assert!(output_path.exists(), "PDF file should be created");
}

#[test]
fn test_pdf_renderer_missing_optional_fields() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let mut doc = create_test_document();
    doc.metadata.phone = None;
    doc.metadata.location = None;
    doc.metadata.linkedin = None;
    doc.metadata.github = None;
    doc.metadata.website = None;

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = create_test_theme();

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF with minimal fields");

    assert!(output_path.exists(), "PDF file should be created");
}

#[test]
fn test_pdf_renderer_nonexistent_template() {
    let result = PdfRenderer::new(Some(Path::new("/nonexistent/template.typ")));
    assert!(result.is_err(), "Should fail with nonexistent template");
}

#[test]
fn test_pdf_renderer_with_embedded_template() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    // Ensure we can render with the embedded template
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document();
    let theme = create_test_theme();

    // This should use the embedded template from include_str!
    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF with embedded template");

    assert!(output_path.exists(), "PDF file should be created");
}
