use cv_check::config::DocumentMetadata;
use cv_check::constants::icons;
use cv_check::parser::Document;
use cv_check::render::pdf::PdfRenderer;
use cv_check::themes::Theme;

fn create_test_document() -> Document {
    Document {
        metadata: DocumentMetadata {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            phone: Some("+1234567890".to_string()),
            website: Some("https://example.com".to_string()),
            github: Some("testuser".to_string()),
            linkedin: Some("testuser".to_string()),
            location: None,
            font_theme: "modern".to_string(),
            color_theme: "modern".to_string(),
            layout: cv_check::config::LayoutOptions::default(),
            recipient: None,
            date: None,
            subject: None,
            custom: std::collections::HashMap::new(),
        },
        content: "# Test Document".to_string(),
        markdown_ast: vec![],
    }
}

#[test]
fn test_pdf_uses_icon_constants() {
    let doc = create_test_document();
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");

    // Use the testing method to get the generated Typst source
    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Check that the generated source uses our icon constants
    assert!(
        typst_source.contains(icons::PHONE),
        "Should use PHONE icon constant"
    );
    assert!(
        typst_source.contains(icons::EMAIL),
        "Should use EMAIL icon constant"
    );
    assert!(
        typst_source.contains(icons::WEBSITE),
        "Should use WEBSITE icon constant"
    );
    assert!(
        typst_source.contains(icons::GITHUB),
        "Should use GITHUB icon constant"
    );
    assert!(
        typst_source.contains(icons::LINKEDIN),
        "Should use LINKEDIN icon constant"
    );
    assert!(
        typst_source.contains(icons::FONT_NAME),
        "Should use FontAwesome font name"
    );
}
