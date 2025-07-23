use chrono::Local;
use cv_check::config::{DocumentMetadata, LayoutOptions, RecipientInfo};
use cv_check::parser::Document;
use cv_check::render::pdf::PdfRenderer;
use cv_check::themes::Theme;
use std::collections::HashMap;

#[test]
fn test_cover_letter_typst_generation() {
    let renderer = PdfRenderer::new(None).expect("Failed to create PDF renderer");

    let doc = Document {
        metadata: DocumentMetadata {
            name: "Jane Smith".to_string(),
            email: "jane.smith@example.com".to_string(),
            phone: Some("+1 (555) 123-4567".to_string()),
            location: Some("San Francisco, CA".to_string()),
            linkedin: None,
            github: None,
            website: None,
            font_theme: "modern".to_string(),
            color_theme: "modern".to_string(),
            recipient: Some(RecipientInfo {
                name: Some("Sarah Johnson".to_string()),
                title: Some("Engineering Manager".to_string()),
                company: Some("Innovation Labs Inc.".to_string()),
                address: Some("456 Tech Boulevard\nSan Francisco, CA 94105".to_string()),
            }),
            date: None,
            subject: Some("Senior Software Engineer Position - Job ID #SE2024".to_string()),
            layout: LayoutOptions::default(),
            custom: HashMap::new(),
        },
        content: "Dear Sarah,\n\nI am writing to express my strong interest in the Senior Software Engineer position at Innovation Labs Inc.\n\nSincerely,\nJane Smith".to_string(),
        markdown_ast: vec![],
    };

    let theme = Theme {
        color: cv_check::themes::color::ColorTheme::load("modern")
            .expect("Failed to load color theme"),
        font: cv_check::themes::font::FontTheme::load("modern").expect("Failed to load font theme"),
    };

    // Access the test method that's exposed for testing
    let source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Verify document setup
    assert!(source.contains("#set document(title: \"Jane Smith\", author: \"Jane Smith\")"));
    assert!(source.contains("#set page(paper: \"a4\""));

    // Verify sender's contact info is present
    assert!(source.contains("Jane Smith"));
    assert!(source.contains("jane.smith\\@example.com"));
    assert!(source.contains("+1 (555) 123-4567"));
    assert!(source.contains("San Francisco, CA"));

    // Verify recipient information formatting
    assert!(source.contains("// Cover Letter Formatting"));
    assert!(source.contains("Sarah Johnson"));
    assert!(source.contains("Engineering Manager"));
    assert!(source.contains("Innovation Labs Inc."));
    assert!(source.contains("456 Tech Boulevard"));
    assert!(source.contains("San Francisco, CA 94105"));

    // Verify date is present (will be today's date)
    // We can't check exact date as it changes daily, but we can verify the date format
    let today = Local::now();
    let month = today.format("%B").to_string();
    let year = today.format("%Y").to_string();
    assert!(source.contains(&month));
    assert!(source.contains(&year));

    // Verify subject line
    assert!(source.contains("Subject: Senior Software Engineer Position - Job ID #SE2024"));

    // Verify letter content
    assert!(source.contains("Dear Sarah,"));
    assert!(source.contains("I am writing to express my strong interest"));
    assert!(source.contains("Sincerely,"));
    assert!(source.contains("Jane Smith"));
}

#[test]
fn test_cover_letter_without_optional_fields() {
    let renderer = PdfRenderer::new(None).expect("Failed to create PDF renderer");

    let doc = Document {
        metadata: DocumentMetadata {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            phone: None,
            location: None,
            linkedin: None,
            github: None,
            website: None,
            font_theme: "classic".to_string(),
            color_theme: "classic".to_string(),
            recipient: Some(RecipientInfo {
                name: Some("Hiring Manager".to_string()),
                title: None,
                company: None,
                address: None,
            }),
            date: None,
            subject: None,
            layout: LayoutOptions::default(),
            custom: HashMap::new(),
        },
        content: "Dear Hiring Manager,\n\nI am interested in the position.\n\nSincerely,\nJohn Doe"
            .to_string(),
        markdown_ast: vec![],
    };

    let theme = Theme {
        color: cv_check::themes::color::ColorTheme::load("classic")
            .expect("Failed to load color theme"),
        font: cv_check::themes::font::FontTheme::load("classic")
            .expect("Failed to load font theme"),
    };

    // Access the test method that's exposed for testing
    let source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Verify minimal recipient info
    assert!(source.contains("Hiring Manager"));

    // Verify no title, company, or address lines when not provided
    assert!(!source.contains("Engineering Manager"));
    assert!(!source.contains("Innovation Labs"));

    // Verify classic font theme
    assert!(source.contains("#set text(font: \"Georgia\""));
}

#[test]
fn test_cover_letter_multiline_address() {
    let renderer = PdfRenderer::new(None).expect("Failed to create PDF renderer");

    let doc = Document {
        metadata: DocumentMetadata {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            phone: None,
            location: None,
            linkedin: None,
            github: None,
            website: None,
            font_theme: "sharp".to_string(),
            color_theme: "sharp".to_string(),
            recipient: Some(RecipientInfo {
                name: Some("HR Department".to_string()),
                title: None,
                company: Some("Big Corp".to_string()),
                address: Some("123 Main St\nSuite 500\nNew York, NY 10001\nUSA".to_string()),
            }),
            date: None,
            subject: None,
            layout: LayoutOptions::default(),
            custom: HashMap::new(),
        },
        content:
            "To whom it may concern,\n\nPlease consider my application.\n\nBest regards,\nTest User"
                .to_string(),
        markdown_ast: vec![],
    };

    let theme = Theme {
        color: cv_check::themes::color::ColorTheme::load("sharp")
            .expect("Failed to load color theme"),
        font: cv_check::themes::font::FontTheme::load("sharp").expect("Failed to load font theme"),
    };

    // Access the test method that's exposed for testing
    let source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Verify multiline address handling
    assert!(source.contains("123 Main St"));
    assert!(source.contains("Suite 500"));
    assert!(source.contains("New York, NY 10001"));
    assert!(source.contains("USA"));

    // Verify each line has proper linebreak
    assert!(source.contains("#linebreak()"));

    // Verify sharp font theme
    assert!(source.contains("#set text(font: \"Montserrat\""));
}

#[test]
fn test_cover_letter_professional_formatting() {
    let renderer = PdfRenderer::new(None).expect("Failed to create PDF renderer");

    let doc = Document {
        metadata: DocumentMetadata {
            name: "Professional Writer".to_string(),
            email: "writer@example.com".to_string(),
            phone: Some("+1-555-0123".to_string()),
            location: Some("Boston, MA".to_string()),
            linkedin: None,
            github: None,
            website: Some("https://writer.example.com".to_string()),
            font_theme: "modern".to_string(),
            color_theme: "modern".to_string(),
            recipient: Some(RecipientInfo {
                name: Some("Dr. Emily Chen".to_string()),
                title: Some("Director of Engineering".to_string()),
                company: Some("TechStart Inc.".to_string()),
                address: Some("789 Innovation Way\nCambridge, MA 02139".to_string()),
            }),
            date: None,
            subject: Some("Application for Technical Writer Position".to_string()),
            layout: LayoutOptions::default(),
            custom: HashMap::new(),
        },
        content: r"Dear Dr. Chen,

I am writing to express my interest in the Technical Writer position at TechStart Inc. With over five years of experience creating comprehensive documentation for complex software systems, I am excited about the opportunity to contribute to your team.

In my current role at Documentation Experts LLC, I have:
- Created user guides, API documentation, and technical specifications for enterprise software
- Collaborated with engineering teams to ensure accuracy and completeness
- Reduced support tickets by 40% through improved documentation

I look forward to discussing how my skills can benefit TechStart Inc.

Sincerely,
Professional Writer".to_string(),
        markdown_ast: vec![],
    };

    let theme = Theme {
        color: cv_check::themes::color::ColorTheme::load("modern")
            .expect("Failed to load color theme"),
        font: cv_check::themes::font::FontTheme::load("modern").expect("Failed to load font theme"),
    };

    // Access the test method that's exposed for testing
    let source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Verify proper spacing elements
    assert!(source.contains("#v(")); // Vertical spacing

    // Verify all sections are present in correct order
    let sender_pos = source
        .find("Professional Writer")
        .expect("Sender name not found");
    // Find today's date pattern - we know it will contain the current year
    let year = Local::now().format("%Y").to_string();
    let date_pos = source.find(&year).expect("Date (year) not found");
    let recipient_pos = source
        .find("Dr. Emily Chen")
        .expect("Recipient name not found");
    let subject_pos = source
        .find("Subject: Application for Technical Writer Position")
        .expect("Subject not found");
    let content_pos = source.find("Dear Dr. Chen,").expect("Salutation not found");

    // Verify order: sender -> date -> recipient -> subject -> content
    assert!(sender_pos < date_pos);
    assert!(date_pos < recipient_pos);
    assert!(recipient_pos < subject_pos);
    assert!(subject_pos < content_pos);
}

#[test]
fn test_cover_letter_without_recipient_info() {
    let renderer = PdfRenderer::new(None).expect("Failed to create PDF renderer");

    let doc = Document {
        metadata: DocumentMetadata {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            phone: None,
            location: None,
            linkedin: None,
            github: None,
            website: None,
            font_theme: "classic".to_string(),
            color_theme: "classic".to_string(),
            recipient: Some(RecipientInfo {
                name: None,
                title: None,
                company: None,
                address: None,
            }),
            date: None,
            subject: Some("Application for Software Developer Position".to_string()),
            layout: LayoutOptions::default(),
            custom: HashMap::new(),
        },
        content: "To Whom It May Concern,\n\nI am interested in the Software Developer position.\n\nSincerely,\nJohn Doe".to_string(),
        markdown_ast: vec![],
    };

    let theme = Theme {
        color: cv_check::themes::color::ColorTheme::load("classic")
            .expect("Failed to load color theme"),
        font: cv_check::themes::font::FontTheme::load("classic")
            .expect("Failed to load font theme"),
    };

    // Access the test method that's exposed for testing
    let source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Verify "To Whom It May Concern" is used when no recipient info is provided
    assert!(source.contains("To Whom It May Concern"));

    // Verify subject line is still present
    assert!(source.contains("Subject: Application for Software Developer Position"));

    // Verify classic font theme
    assert!(source.contains("#set text(font: \"Georgia\""));
}
