use cv_check::config::DocumentMetadata;
use cv_check::constants::layout;
use cv_check::parser::Document;
use cv_check::render::pdf::PdfRenderer;
use cv_check::themes::Theme;

fn create_test_document() -> Document {
    Document {
        metadata: DocumentMetadata {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            phone: None,
            website: None,
            github: None,
            linkedin: None,
            location: None,
            font_theme: "modern".to_string(),
            color_theme: "modern".to_string(),
            layout: cv_check::config::LayoutOptions::default(),
            recipient: None,
            date: None,
            subject: None,
            custom: std::collections::HashMap::new(),
        },
        content: "# Test Document\n\nThis is a test.".to_string(),
        markdown_ast: vec![],
    }
}

#[test]
fn test_pdf_uses_layout_constants() {
    let doc = create_test_document();
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");

    // Use the testing method to get the generated Typst source
    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Debug: print part of the source to see what we're checking
    println!("Checking for margins in source...");

    // Check that the source uses layout constants
    assert!(
        typst_source.contains(&format!(
            "margin: (top: {}, bottom: {}, left: {}, right: {})",
            layout::margins::TOP,
            layout::margins::BOTTOM,
            layout::margins::LEFT,
            layout::margins::RIGHT
        )),
        "Should use margin constants"
    );

    // Check font sizes
    assert!(
        typst_source.contains(&format!("size: {}", layout::font_sizes::NORMAL)),
        "Should use NORMAL font size constant"
    );

    // Check spacing
    assert!(
        typst_source.contains(&format!("#v({})", layout::spacing::TINY)),
        "Should use TINY spacing constant"
    );
}
