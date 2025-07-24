//! Test utilities for creating common test objects
//!
//! This module provides helper functions to create test documents, themes,
//! and other common test data. It's available as a feature for tests.

use crate::config::{DocumentMetadata, LayoutOptions};
use crate::parser::{markdown::parse_markdown, Document};
use crate::themes::{color::ColorTheme, font::FontTheme, Theme};
use std::collections::HashMap;

/// Creates a standard test document with all fields populated
#[must_use]
pub fn create_test_document() -> Document {
    Document {
        metadata: DocumentMetadata {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            phone: Some("+1 234 567 8900".to_string()),
            location: Some("San Francisco, CA".to_string()),
            linkedin: Some("testuser".to_string()),
            github: Some("testuser".to_string()),
            website: Some("https://example.com".to_string()),
            font_theme: "modern".to_string(),
            color_theme: "modern".to_string(),
            recipient: None,
            date: None,
            subject: None,
            layout: LayoutOptions::default(),
            custom: HashMap::new(),
        },
        content: "# Test Section\n\nThis is a test document.".to_string(),
        markdown_ast: vec![],
    }
}

/// Creates a minimal test document with only required fields
#[must_use]
pub fn create_minimal_document(name: &str, email: &str) -> Document {
    Document {
        metadata: DocumentMetadata {
            name: name.to_string(),
            email: email.to_string(),
            phone: None,
            location: None,
            linkedin: None,
            github: None,
            website: None,
            font_theme: "modern".to_string(),
            color_theme: "modern".to_string(),
            recipient: None,
            date: None,
            subject: None,
            layout: LayoutOptions::default(),
            custom: HashMap::new(),
        },
        content: String::new(),
        markdown_ast: vec![],
    }
}

/// Creates a test document with custom content
#[must_use]
pub fn create_document_with_content(content: &str) -> Document {
    Document {
        metadata: DocumentMetadata {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            phone: None,
            location: None,
            linkedin: None,
            github: None,
            website: None,
            font_theme: "modern".to_string(),
            color_theme: "modern".to_string(),
            recipient: None,
            date: None,
            subject: None,
            layout: LayoutOptions::default(),
            custom: HashMap::new(),
        },
        content: content.to_string(),
        markdown_ast: parse_markdown(content),
    }
}

/// Creates a standard modern theme for testing
#[must_use]
pub fn create_test_theme() -> Theme {
    Theme {
        color: ColorTheme::load("modern").expect("Failed to load modern color theme"),
        font: FontTheme::load("modern").expect("Failed to load modern font theme"),
    }
}

/// Creates a theme with specific font and color theme names
#[must_use]
pub fn create_theme_by_name(font_theme: &str, color_theme: &str) -> Theme {
    Theme {
        color: ColorTheme::load(color_theme)
            .unwrap_or_else(|_| panic!("Failed to load color theme: {color_theme}")),
        font: FontTheme::load(font_theme)
            .unwrap_or_else(|_| panic!("Failed to load font theme: {font_theme}")),
    }
}

/// Creates a realistic CV test document with full content
#[must_use]
pub fn create_full_cv_document() -> Document {
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
            layout: LayoutOptions::default(),
            recipient: None,
            date: None,
            subject: None,
            custom: HashMap::new(),
        },
        content: content.to_string(),
        markdown_ast: parse_markdown(content),
    }
}

/// Creates a cover letter test document
#[must_use]
pub fn create_cover_letter_document() -> Document {
    use crate::config::RecipientInfo;

    let content = r"Dear Hiring Manager,

I am writing to express my interest in the Software Engineer position at your company.

With over 10 years of experience in software development, I believe I would be a valuable addition to your team.

I look forward to discussing this opportunity with you.

Sincerely,
John Doe";

    Document {
        metadata: DocumentMetadata {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            phone: Some("+1 234 567 8900".to_string()),
            location: Some("San Francisco, CA".to_string()),
            linkedin: None,
            github: None,
            website: None,
            font_theme: "modern".to_string(),
            color_theme: "modern".to_string(),
            recipient: Some(RecipientInfo {
                name: Some("Jane Smith".to_string()),
                title: Some("Hiring Manager".to_string()),
                company: Some("Tech Corp".to_string()),
                address: Some("123 Main St\nSan Francisco, CA 94105".to_string()),
            }),
            date: None,
            subject: Some("Software Engineer Position".to_string()),
            layout: LayoutOptions::default(),
            custom: HashMap::new(),
        },
        content: content.to_string(),
        markdown_ast: parse_markdown(content),
    }
}
