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

fn load_example_document(filename: &str) -> Document {
    let example_path = Path::new("examples").join(filename);
    Document::from_file(&example_path)
        .unwrap_or_else(|_| panic!("Failed to load example file: {}", example_path.display()))
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

#[test]
fn test_cv_example_renders_successfully() {
    if !typst_is_available() {
        eprintln!("Skipping test: Typst not available");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("cv.pdf");

    let doc = load_example_document("cv.md");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = Theme::new(&doc.metadata.font_theme, &doc.metadata.color_theme)
        .expect("Failed to create theme");

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    let pdf_bytes = fs::read(&output_path).expect("Failed to read PDF");
    assert!(
        pdf_bytes.len() > 5000,
        "PDF should have substantial content"
    );
    assert!(&pdf_bytes[0..5] == b"%PDF-", "Should be a valid PDF");

    // Verify actual content from examples/cv.md appears
    let expected_content = vec![
        "Jane Smith",
        "jane.smith@example.com",
        "Professional Summary",
        "Senior Software Engineer",
        "Tech Innovations Inc.",
        "microservices architecture",
        "Stanford University",
        "JavaScript/TypeScript",
    ];
    verify_pdf_contains_text(&output_path, &expected_content);
}

#[test]
fn test_cv_two_column_example_renders_with_layout() {
    if !typst_is_available() {
        eprintln!("Skipping test: Typst not available");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("cv-two-column.pdf");

    let doc = load_example_document("cv-two-column.md");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = Theme::new(&doc.metadata.font_theme, &doc.metadata.color_theme)
        .expect("Failed to create theme");

    // Verify layout settings are parsed correctly
    assert_eq!(
        doc.metadata.layout.columns, 2,
        "Document should have 2-column layout"
    );

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    let pdf_bytes = fs::read(&output_path).expect("Failed to read PDF");
    assert!(
        pdf_bytes.len() > 5000,
        "PDF should have substantial content"
    );
    assert!(&pdf_bytes[0..5] == b"%PDF-", "Should be a valid PDF");

    // Verify two-column layout content appears
    let expected_content = vec![
        "Jane Smith",
        "jane.smith@example.com",
        "Professional Summary",
        "Senior Software Engineer",
    ];
    verify_pdf_contains_text(&output_path, &expected_content);
}

#[test]
fn test_pdf_renders_all_markdown_elements() {
    if !typst_is_available() {
        eprintln!("Skipping test: Typst not available");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("markdown-test.pdf");

    let content = r"---
name: Test User
email: test@example.com
phone: +1 234 567 8900
location: Test City, TC
font_theme: modern
color_theme: modern
---

# Heading 1

## Heading 2

### Heading 3

Regular paragraph text with **bold text** and *italic text*.

- Bullet point one
- Bullet point two
- Bullet point three

Another paragraph with more content to test line wrapping. This is a long line that should wrap when rendered in the PDF to ensure our line wrapping logic works correctly.

**Bold Section Title**

Content under a bold title.

*Italic emphasis* for important notes.";

    let doc =
        Document::from_string(content, Path::new("test.md")).expect("Failed to parse document");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = Theme::new(&doc.metadata.font_theme, &doc.metadata.color_theme)
        .expect("Failed to create theme");

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    let pdf_bytes = fs::read(&output_path).expect("Failed to read PDF");
    assert!(pdf_bytes.len() > 1000, "PDF should have content");
    assert!(&pdf_bytes[0..5] == b"%PDF-", "Should be a valid PDF");

    // Verify all markdown elements are rendered
    let expected_content = vec![
        "Test User",
        "test@example.com",
        "Heading 1",
        "Heading 2",
        "Heading 3",
        "bold text",
        "italic text",
        "Bullet point one",
        "Bullet point two",
        "Bold Section Title",
        "Italic emphasis",
    ];
    verify_pdf_contains_text(&output_path, &expected_content);
}

#[test]
fn test_pdf_multipage_rendering() {
    if !typst_is_available() {
        eprintln!("Skipping test: Typst not available");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("multipage.pdf");

    // Create a document with lots of content to force multiple pages
    let mut content = r"---
name: Test User
email: test@example.com
phone: +1 234 567 8900
location: Test City, TC
font_theme: modern
color_theme: modern
---

"
    .to_string();

    // Add 100 sections to ensure multiple pages
    for i in 1..=100 {
        use std::fmt::Write;
        let _ = write!(content, "# Section {i}\n\n");
        let _ = write!(content, "This is content for section {i}. ");
        content.push_str("It contains some text to fill up space on the page. ");
        content.push_str(
            "We want to ensure that our PDF renderer can handle multiple pages correctly.\n\n",
        );
    }

    let doc =
        Document::from_string(&content, Path::new("test.md")).expect("Failed to parse document");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = Theme::new(&doc.metadata.font_theme, &doc.metadata.color_theme)
        .expect("Failed to create theme");

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    let pdf_bytes = fs::read(&output_path).expect("Failed to read PDF");
    assert!(
        pdf_bytes.len() > 10000,
        "PDF should be large for multiple pages"
    );
    assert!(&pdf_bytes[0..5] == b"%PDF-", "Should be a valid PDF");

    // Verify some sections from the generated content
    let expected_content = vec![
        "Test User",
        "test@example.com",
        "Section 1",
        "Section 50",
        "Section 100",
        "content for section",
    ];
    verify_pdf_contains_text(&output_path, &expected_content);
}
