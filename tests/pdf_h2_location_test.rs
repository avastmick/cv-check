use cv_check::config::DocumentMetadata;
use cv_check::parser::Document;
use cv_check::render::pdf::PdfRenderer;
use cv_check::themes::Theme;

fn create_test_document_with_h2_locations() -> Document {
    let content = r"# Experience

## Tech Corp (San Francisco, CA)
_Senior Software Engineer_, 2020 - Present

- Led development of cloud infrastructure
- Implemented CI/CD pipelines

## Previous Company (New York, NY)
_Software Engineer_, 2018 - 2020

- Built microservices architecture
- Improved system performance";

    Document {
        metadata: DocumentMetadata {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            phone: Some("+1 234 567 8900".to_string()),
            location: Some("San Francisco, CA".to_string()),
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

#[test]
fn test_h2_location_not_bold() {
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_h2_locations();
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Check that H2 headings have separate formatting for company name and location
    // Company name should be bold, location should be regular weight
    assert!(
        typst_source.contains("text(size: 14pt, weight: \"bold\", fill: rgb(\"#607D8B\"))[Tech Corp] #text(size: 14pt, weight: \"regular\", fill: rgb(\"#607D8B\"))[(San Francisco, CA)]"),
        "H2 heading should have company name in bold and location in regular weight"
    );

    assert!(
        typst_source.contains("text(size: 14pt, weight: \"bold\", fill: rgb(\"#607D8B\"))[Previous Company] #text(size: 14pt, weight: \"regular\", fill: rgb(\"#607D8B\"))[(New York, NY)]"),
        "H2 heading should have company name in bold and location in regular weight"
    );
}

#[test]
fn test_h2_without_parentheses_still_bold() {
    let content = r"# Education

## Stanford University
B.S. Computer Science, 2010 - 2014";

    let doc = Document {
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
            layout: cv_check::config::LayoutOptions::default(),
            recipient: None,
            date: None,
            subject: None,
            custom: std::collections::HashMap::new(),
        },
        content: content.to_string(),
        markdown_ast: cv_check::parser::markdown::parse_markdown(content),
    };

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // H2 without parentheses should still be entirely bold
    assert!(
        typst_source.contains(
            "text(size: 14pt, weight: \"bold\", fill: rgb(\"#607D8B\"))[Stanford University]"
        ),
        "H2 heading without parentheses should be entirely bold"
    );
}
