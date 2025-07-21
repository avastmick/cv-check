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
fn test_manual_pagebreak_html_comment() {
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

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    // Read the generated Typst source (we'll need to expose this for testing)
    // For now, we'll check if the PDF was created
    assert!(output_path.exists(), "PDF should be created");

    // TODO: Once implemented, verify that the Typst source contains #pagebreak()
    // where the <!-- pagebreak --> marker was placed
}

#[test]
fn test_manual_pagebreak_latex_style() {
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

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    assert!(output_path.exists(), "PDF should be created");

    // TODO: Verify \pagebreak is converted to #pagebreak() in Typst
}

#[test]
fn test_automatic_block_preservation() {
    // Create content with a section that should not be split
    let content = r"# Experience

## Senior Software Engineer
**Tech Company** | *2020 - Present*

- Achievement 1 that is quite long and detailed
- Achievement 2 with substantial content
- Achievement 3 that provides significant value
- Achievement 4 demonstrating key skills
- Achievement 5 showing leadership qualities
- Achievement 6 with measurable impact
- Achievement 7 highlighting technical expertise
- Achievement 8 showing collaboration skills
- Achievement 9 with business impact
- Achievement 10 demonstrating innovation

## Previous Role
**Another Company** | *2018 - 2020*

More content that should start on a new page if the previous section is too long.";

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    assert!(output_path.exists(), "PDF should be created");

    // TODO: Verify that H2 sections are wrapped in block(breakable: false)
}

#[test]
fn test_multiple_pagebreaks() {
    let content = r"# Section 1

Content for section 1.

<!-- pagebreak -->

# Section 2

Content for section 2.

\pagebreak

# Section 3

Content for section 3.";

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    assert!(output_path.exists(), "PDF should be created");

    // TODO: Verify multiple pagebreaks are handled correctly
}

#[test]
fn test_pagebreak_in_list() {
    let content = r"# Skills

- Skill 1
- Skill 2

<!-- pagebreak -->

- Skill 3
- Skill 4";

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    assert!(output_path.exists(), "PDF should be created");

    // TODO: Verify pagebreak works correctly within lists
}

#[test]
fn test_typst_source_generation_with_pagebreaks() {
    // This test will need access to the generated Typst source
    // We may need to refactor PdfRenderer to expose the Typst generation
    // for testing purposes

    let content = r"# Content

Some content.

<!-- pagebreak -->

More content.";

    let _renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let _doc = create_test_document_with_content(content);
    let _theme = create_test_theme();

    // TODO: Once we can access the Typst source, verify it contains:
    // - #pagebreak() where markers are placed
    // - block(breakable: false) around H2 sections
}
