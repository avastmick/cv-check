use approx::assert_abs_diff_eq;
use cv_check::parser::{frontmatter, markdown, Document};
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

#[test]
fn test_unclosed_frontmatter() {
    let content = r"---
name: Test User
email: test@example.com
# Missing closing delimiter
Content here";

    let result = frontmatter::parse_frontmatter(content, &PathBuf::from("test.md"));
    assert!(result.is_err());
}

#[test]
fn test_validate_missing_email() {
    let content = r#"---
name: John Doe
email: ""  # Empty email should fail validation
---
# Content"#;

    let doc = Document::from_string(content, &PathBuf::from("test.md"))
        .expect("Failed to parse document");

    assert!(doc.validate().is_err());
}

#[test]
fn test_document_from_file_nonexistent() {
    let result = Document::from_file(&PathBuf::from("/nonexistent/path/file.md"));
    assert!(result.is_err());
}

#[test]
fn test_frontmatter_with_extra_dashes() {
    let content = r"---
name: Test
email: test@test.com
---
# Content
---
More content with dashes";

    let (metadata, content) = frontmatter::parse_frontmatter(content, &PathBuf::from("test.md"))
        .expect("Should parse successfully");

    assert_eq!(metadata.name, "Test");
    assert!(content.contains("More content with dashes"));
}

#[test]
fn test_empty_frontmatter() {
    let content = r"---
---
# Just content";

    let result = frontmatter::parse_frontmatter(content, &PathBuf::from("test.md"));
    // Empty frontmatter should fail because required fields are missing
    assert!(result.is_err());
}

#[test]
fn test_frontmatter_only_whitespace() {
    let content = r"---


---
Content";

    let result = frontmatter::parse_frontmatter(content, &PathBuf::from("test.md"));
    assert!(result.is_err());
}

#[test]
fn test_document_with_all_optional_fields() {
    let content = r"---
name: Full User
email: full@example.com
phone: +1-555-0123
location: San Francisco, CA
linkedin: fulluser
github: fulluser
website: https://fulluser.com
font_theme: sharp
color_theme: sharp
layout:
  columns: 2
  sidebar: right
recipient:
  name: HR Department
  title: Recruitment Team
  company: Big Corp
  address: 123 Main St
date: 2025-07-20
subject: Application
---
# Content";

    let doc = Document::from_string(content, &PathBuf::from("test.md"))
        .expect("Failed to parse document");

    assert_eq!(doc.metadata.name, "Full User");
    assert_eq!(doc.metadata.email, "full@example.com");
    assert!(doc.metadata.phone.is_some());
    assert!(doc.metadata.location.is_some());
    assert!(doc.metadata.linkedin.is_some());
    assert!(doc.metadata.github.is_some());
    assert!(doc.metadata.website.is_some());
    assert!(doc.metadata.recipient.is_some());
    assert!(doc.metadata.date.is_some());
    assert!(doc.metadata.subject.is_some());
    assert_eq!(doc.metadata.layout.columns, 2);

    // Should validate successfully with all fields
    assert!(doc.validate().is_ok());
}

#[test]
fn test_markdown_with_complex_structures() {
    let content = r#"# Title

## Subtitle

### Subsubtitle

Regular paragraph.

1. First item
   - Nested bullet
   - Another nested
2. Second item

| Table | Headers |
|-------|---------|
| Data  | More    |

```rust
fn code_block() {
    println!("Hello");
}
```

> Block quote
> Multiple lines

***

[Link](https://example.com) and **bold** and *italic* and ~~strikethrough~~.
"#;

    let events = markdown::parse_markdown(content);

    // Should parse all these structures
    assert!(!events.is_empty());

    // Check that we have various event types
    let has_heading = events.iter().any(|e| {
        matches!(
            e,
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading { .. })
        )
    });
    let has_list = events.iter().any(|e| {
        matches!(
            e,
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::List(_))
        )
    });
    let has_table = events.iter().any(|e| {
        matches!(
            e,
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Table(_))
        )
    });
    let has_code_block = events.iter().any(|e| {
        matches!(
            e,
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(_))
        )
    });

    assert!(has_heading);
    assert!(has_list);
    assert!(has_table);
    assert!(has_code_block);
}
