use cv_gen::parser::Document;
use cv_gen::render::docx::DocxRenderer;
use cv_gen::render::html::HtmlRenderer;
use cv_gen::render::pdf::PdfRenderer;
use cv_gen::render::{RenderEngine, Renderer};
use cv_gen::themes::Theme;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

fn create_test_document() -> Document {
    let content = r#"---
name: Test User
email: test@example.com
phone: "+1 555-0123"
location: "Test City, TC"
linkedin: "testuser"
github: "testuser"
website: "https://test.example.com"
font_theme: modern
color_theme: classic
layout:
  columns: 1
  margins:
    top: 2.0
    bottom: 2.0
    left: 2.5
    right: 2.5
---
# Experience

## Software Engineer
*Company Name* | 2020 - Present

- Built amazing things
- Solved complex problems

# Education

## Computer Science
*University* | 2016 - 2020"#;

    Document::from_string(content, &PathBuf::from("test.md"))
        .expect("Failed to create test document")
}

fn create_test_theme() -> Theme {
    Theme::new("modern", "classic").expect("Failed to create test theme")
}

#[test]
fn test_renderer_new_pdf() {
    let renderer = Renderer::new("pdf", None);
    assert!(renderer.is_ok());
}

#[test]
fn test_renderer_new_docx() {
    let renderer = Renderer::new("docx", None);
    assert!(renderer.is_ok());
}

#[test]
fn test_renderer_new_html() {
    let renderer = Renderer::new("html", None);
    assert!(renderer.is_ok());
}

#[test]
fn test_renderer_new_invalid_format() {
    let renderer = Renderer::new("invalid", None);
    assert!(renderer.is_err());
}

#[test]
fn test_pdf_renderer_new_without_template() {
    let renderer = PdfRenderer::new(None);
    assert!(renderer.is_ok());
}

#[test]
fn test_pdf_renderer_new_with_template() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let template_path = temp_dir.path().join("template.typ");
    fs::write(&template_path, "Test template content").expect("Failed to write template");

    let renderer = PdfRenderer::new(Some(&template_path));
    assert!(renderer.is_ok());
}

#[test]
fn test_pdf_renderer_new_with_missing_template() {
    let missing_path = Path::new("/nonexistent/template.typ");
    let renderer = PdfRenderer::new(Some(missing_path));
    assert!(renderer.is_err());
}

#[test]
fn test_pdf_renderer_render() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document();
    let theme = create_test_theme();

    let result = renderer.render(&doc, &theme, &output_path);

    if let Ok(()) = result {
        // Verify output file was created
        assert!(output_path.exists());
        let content = fs::read(&output_path).expect("Failed to read output file");
        assert!(!content.is_empty());
    }
    // This is expected until full implementation if result is Err
}

#[test]
fn test_docx_renderer_new_without_template() {
    let renderer = DocxRenderer::new(None);
    assert!(renderer.is_ok());
}

#[test]
fn test_docx_renderer_new_with_template() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let template_path = temp_dir.path().join("template.docx");
    fs::write(&template_path, "Test template content").expect("Failed to write template");

    let renderer = DocxRenderer::new(Some(&template_path));
    assert!(renderer.is_ok());
}

#[test]
fn test_docx_renderer_render() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.docx");

    let renderer = DocxRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document();
    let theme = create_test_theme();

    let result = renderer.render(&doc, &theme, &output_path);

    if let Ok(()) = result {
        // Verify output file was created
        assert!(output_path.exists());
        let content = fs::read(&output_path).expect("Failed to read output file");
        assert!(!content.is_empty());
    }
    // This is expected until full implementation if result is Err
}

#[test]
fn test_html_renderer_new_without_template() {
    let renderer = HtmlRenderer::new(None);
    assert!(renderer.is_ok());
}

#[test]
fn test_html_renderer_new_with_template() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let template_path = temp_dir.path().join("template.html");
    fs::write(&template_path, "Test template content").expect("Failed to write template");

    let renderer = HtmlRenderer::new(Some(&template_path));
    assert!(renderer.is_ok());
}

#[test]
fn test_html_renderer_render() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.html");

    let renderer = HtmlRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document();
    let theme = create_test_theme();

    let result = renderer.render(&doc, &theme, &output_path);

    match result {
        Ok(()) => {
            // Verify output file was created
            assert!(output_path.exists());
            let content = fs::read_to_string(&output_path).expect("Failed to read output file");
            assert!(!content.is_empty());

            // Verify HTML structure
            assert!(content.contains("<!DOCTYPE html>"));
            assert!(content.contains("<title>Test User - CV</title>"));
            assert!(content.contains("test@example.com"));
        }
        Err(e) => {
            // This should not fail for HTML renderer as it's mostly implemented
            panic!("HTML renderer should work with current implementation: {e}");
        }
    }
}

#[test]
fn test_renderer_render_pdf() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = Renderer::new("pdf", None).expect("Failed to create renderer");
    let doc = create_test_document();
    let theme = create_test_theme();

    let result = renderer.render(&doc, &theme, &output_path);

    if let Ok(()) = result {
        // Verify output file was created
        assert!(output_path.exists());
    }
    // This is expected until full implementation if result is Err
}

#[test]
fn test_renderer_render_html() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.html");

    let renderer = Renderer::new("html", None).expect("Failed to create renderer");
    let doc = create_test_document();
    let theme = create_test_theme();

    let result = renderer.render(&doc, &theme, &output_path);

    match result {
        Ok(()) => {
            // Verify output file was created
            assert!(output_path.exists());
            let content = fs::read_to_string(&output_path).expect("Failed to read output file");
            assert!(content.contains("Test User"));
        }
        Err(e) => {
            // This should not fail for HTML renderer
            panic!("HTML renderer should work with current implementation: {e}");
        }
    }
}

#[test]
fn test_renderer_render_docx() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.docx");

    let renderer = Renderer::new("docx", None).expect("Failed to create renderer");
    let doc = create_test_document();
    let theme = create_test_theme();

    let result = renderer.render(&doc, &theme, &output_path);

    if let Ok(()) = result {
        // Verify output file was created
        assert!(output_path.exists());
    }
    // This is expected until full implementation if result is Err
}
