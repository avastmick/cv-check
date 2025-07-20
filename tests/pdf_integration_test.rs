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
    // Extract text from PDF
    let text = extract_text(pdf_path).expect("Failed to extract text from PDF");

    // Verify all expected texts are present
    for expected in expected_texts {
        // Handle potential spacing issues in PDF text extraction
        let text_normalized = text.replace([' ', '\n'], "").to_lowercase();
        let expected_normalized = expected.replace([' ', '\n'], "").to_lowercase();

        assert!(
            text.contains(expected) || text_normalized.contains(&expected_normalized),
            "PDF missing expected text: '{expected}'\nActual PDF content:\n{text}"
        );
    }
}

#[test]
fn test_cv_single_column_content_matches_input() {
    if !typst_is_available() {
        eprintln!("Skipping test: Typst not available");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("cv.pdf");

    // Load example CV
    let doc =
        Document::from_file(Path::new("examples/cv.md")).expect("Failed to load examples/cv.md");

    // Read the raw markdown to get ALL text content
    let _markdown_content =
        fs::read_to_string("examples/cv.md").expect("Failed to read markdown file");

    // Create renderer and render
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = Theme::new(&doc.metadata.font_theme, &doc.metadata.color_theme)
        .expect("Failed to create theme");

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    // Verify PDF exists
    assert!(output_path.exists());

    // Extract key content that must appear in the PDF
    let expected_content = vec![
        // Personal info
        "Jane Smith",
        "jane.smith@example.com",
        "+1 (555) 123-4567",
        "San Francisco, CA",
        "linkedin.com/in/janesmith",
        "github.com/janesmith",
        "janesmith.dev",
        // Section headers
        "Professional Summary",
        "Experience",
        "Education",
        "Skills",
        "Projects",
        "Certifications",
        "Awards & Recognition",
        // Professional summary content (from markdown input)
        "10+ years of experience",
        "building scalable web applications",
        "cloud architecture",
        "agile methodologies",
        // Job titles and companies
        "Senior Software Engineer",
        "Tech Innovations Inc.",
        "Software Engineer",
        "StartupXYZ",
        "Junior Developer",
        "Digital Solutions Co.",
        // Job descriptions
        "Led development of microservices architecture",
        "5M+ daily active users",
        "Reduced deployment time by 70%",
        "Mentored team of 8 engineers",
        "100K events per second",
        // Education
        "M.S. Computer Science",
        "Stanford University",
        "B.S. Computer Science",
        "UC Berkeley",
        // Skills (from markdown input)
        "JavaScript/TypeScript, Python, Go, Rust, Java",
        "React, Vue.js, Angular, HTML5, CSS3, Webpack",
        "Node.js, Express, Django, FastAPI, GraphQL",
        "PostgreSQL, MongoDB, Redis, Elasticsearch",
        "AWS, GCP, Docker, Kubernetes, Terraform, Jenkins",
        "Git, JIRA, Confluence, DataDog, New Relic",
        // Projects (from markdown input)
        "Open Source Contributions",
        "ReactStateManager",
        "PyDataPipeline",
        "Personal Projects",
        "DevMetrics",
        "CodeMentor",
        "TechBlog",
        // Certifications
        "AWS Certified Solutions Architect",
        "Google Cloud Professional",
        "Certified Kubernetes Administrator",
        // Awards
        "Engineer of the Year",
        "Speaker at ReactConf 2021",
        "Winner of StartupXYZ Hackathon",
    ];

    verify_pdf_contains_text(&output_path, &expected_content);
}

#[test]
fn test_cv_two_column_content_matches_input() {
    if !typst_is_available() {
        eprintln!("Skipping test: Typst not available");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("cv-two-column.pdf");

    // Load example CV with two columns
    let doc = Document::from_file(Path::new("examples/cv-two-column.md"))
        .expect("Failed to load examples/cv-two-column.md");

    // Create renderer and render
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = Theme::new(&doc.metadata.font_theme, &doc.metadata.color_theme)
        .expect("Failed to create theme");

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    // Verify PDF exists
    assert!(output_path.exists());

    // The two-column CV has the same content as single column
    // Verify the same content appears
    let expected_content = vec![
        "Jane Smith",
        "jane.smith@example.com",
        "Professional Summary",
        "10+ years of experience",
        "Senior Software Engineer",
        "Tech Innovations Inc.",
        "Led development of microservices architecture",
        "Stanford University",
        "JavaScript/TypeScript, Python, Go, Rust, Java",
    ];

    verify_pdf_contains_text(&output_path, &expected_content);
}

#[test]
fn test_cover_letter_content_matches_input() {
    if !typst_is_available() {
        eprintln!("Skipping test: Typst not available");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("cover-letter.pdf");

    // Load example cover letter
    let doc = Document::from_file(Path::new("examples/cover-letter.md"))
        .expect("Failed to load examples/cover-letter.md");

    // Create renderer and render
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = Theme::new(&doc.metadata.font_theme, &doc.metadata.color_theme)
        .expect("Failed to create theme");

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    // Verify PDF exists
    assert!(output_path.exists());

    // Verify cover letter specific content
    let expected_content = vec![
        // Sender info
        "Jane Smith",
        "jane.smith@example.com",
        "+1 (555) 123-4567",
        "San Francisco, CA",
        // Recipient info
        "Sarah Johnson",
        "Engineering Manager",
        "Innovation Labs Inc.",
        "456 Tech Boulevard",
        "San Francisco, CA 94105",
        // Date and subject
        "December 15, 2024",
        "Senior Software Engineer Position",
        // Letter content
        "Dear Sarah,",
        "I am writing to express my strong interest",
        "Senior Software Engineer position",
        "Innovation Labs Inc.",
        "With over 10 years of experience",
        "building scalable applications",
        "leading technical teams",
        "especially drawn to Innovation Labs",
        "commitment to open source and knowledge sharing",
        "Thank you for considering my application",
        "discuss how my expertise",
        "Sincerely,",
        "Jane Smith",
    ];

    verify_pdf_contains_text(&output_path, &expected_content);
}

#[test]
fn test_theme_modern_applied_correctly() {
    if !typst_is_available() {
        eprintln!("Skipping test: Typst not available");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Test with CV that uses modern theme
    let doc =
        Document::from_file(Path::new("examples/cv.md")).expect("Failed to load examples/cv.md");

    // Verify it's using modern theme
    assert_eq!(doc.metadata.font_theme, "modern");
    assert_eq!(doc.metadata.color_theme, "modern");

    let output_path = temp_dir.path().join("cv-modern.pdf");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = Theme::new("modern", "modern").expect("Failed to create modern theme");

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    // For theme testing, we mainly verify the PDF renders successfully
    // The actual font/color verification would require PDF structure analysis
    assert!(output_path.exists());
    let pdf_bytes = fs::read(&output_path).expect("Failed to read PDF");
    assert!(pdf_bytes.len() > 5000);
}

#[test]
fn test_all_markdown_elements_preserved() {
    if !typst_is_available() {
        eprintln!("Skipping test: Typst not available");
        return;
    }

    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Test markdown with various formatting
    let test_content = r"---
name: Test User
email: test@example.com
font_theme: modern
color_theme: modern
---

# Main Heading

## Subheading

Regular text with **bold text** and *italic text*.

### List Examples

- First item
- Second item
  - Nested item 2.1
  - Nested item 2.2
- Third item

**Bold Section Title**

More content with *emphasis* and ***bold italic*** text.
";

    let doc = Document::from_string(test_content, Path::new("test.md"))
        .expect("Failed to parse document");

    let output_path = temp_dir.path().join("markdown-test.pdf");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = Theme::new(&doc.metadata.font_theme, &doc.metadata.color_theme)
        .expect("Failed to create theme");

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    // Verify all text content is preserved
    let expected_content = vec![
        "Main Heading",
        "Subheading",
        "Regular text with",
        "bold text",
        "italic text",
        "List Examples",
        "First item",
        "Second item",
        "Nested item 2.1",
        "Nested item 2.2",
        "Third item",
        "Bold Section Title",
        "More content with",
        "emphasis",
        "bold italic",
    ];

    verify_pdf_contains_text(&output_path, &expected_content);
}
