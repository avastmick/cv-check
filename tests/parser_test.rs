use approx::assert_abs_diff_eq;
use cv_gen::parser::{frontmatter, markdown, Document};
use std::path::PathBuf;

#[test]
fn test_parse_valid_cv() {
    let cv_content = r"---
name: John Doe
email: john@example.com
font_theme: modern
color_theme: classic
---
# Experience
Test content";

    let doc = Document::from_string(cv_content, &PathBuf::from("test.md"))
        .expect("Failed to parse document");

    assert_eq!(doc.metadata.name, "John Doe");
    assert_eq!(doc.metadata.email, "john@example.com");
    assert_eq!(doc.metadata.font_theme, "modern");
    assert_eq!(doc.metadata.color_theme, "classic");
    assert!(doc.content.contains("# Experience"));
}

#[test]
fn test_validate_missing_name() {
    let cv_content = r#"---
email: john@example.com
name: ""  # Empty name should fail validation
---
# Content"#;

    let doc = Document::from_string(cv_content, &PathBuf::from("test.md"))
        .expect("Failed to parse document");

    assert!(doc.validate().is_err());
}

#[test]
fn test_parse_cover_letter_with_recipient() {
    let letter_content = r"---
name: Jane Smith
email: jane@example.com
recipient:
  name: Hiring Manager
  company: Tech Corp
  title: Engineering Team
---
Dear Hiring Manager,";

    let doc = Document::from_string(letter_content, &PathBuf::from("letter.md"))
        .expect("Failed to parse document");

    assert_eq!(doc.metadata.name, "Jane Smith");
    assert!(doc.metadata.recipient.is_some());

    let recipient = doc
        .metadata
        .recipient
        .as_ref()
        .expect("Recipient should be present");
    assert_eq!(recipient.name, "Hiring Manager");
    assert_eq!(
        recipient
            .company
            .as_ref()
            .expect("Company should be present"),
        "Tech Corp"
    );
}

#[test]
fn test_frontmatter_parsing() {
    let content = r"---
name: Test User
email: test@example.com
layout:
  columns: 2
  margins:
    top: 2.0
    bottom: 2.0
    left: 2.5
    right: 2.5
---
# Content goes here";

    let (metadata, content) = frontmatter::parse_frontmatter(content, &PathBuf::from("test.md"))
        .expect("Failed to parse frontmatter");

    assert_eq!(metadata.name, "Test User");
    assert_eq!(metadata.layout.columns, 2);
    assert_abs_diff_eq!(metadata.layout.margins.top, 2.0);
    assert!(content.contains("# Content goes here"));
}

#[test]
fn test_markdown_parsing() {
    let content = "# Heading\n\nParagraph with **bold** text.";

    let events = markdown::parse_markdown(content);

    // We now return a "parsed" indicator for successful parsing
    assert!(!events.is_empty());
}

#[test]
fn test_missing_frontmatter() {
    let content = "# Just markdown content";

    let result = frontmatter::parse_frontmatter(content, &PathBuf::from("test.md"));
    assert!(result.is_err());
}

#[test]
fn test_invalid_yaml() {
    let content = r"---
name: Test
email: [invalid yaml
---
Content";

    let result = frontmatter::parse_frontmatter(content, &PathBuf::from("test.md"));
    assert!(result.is_err());
}
