use cv_check::config::DocumentMetadata;
use cv_check::parser::Document;
use cv_check::render::pdf::PdfRenderer;
use cv_check::themes::Theme;

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
fn debug_typst_output() {
    let content = r"# Experience

## Software Engineer
**Tech Corp** | *2020 - Present*

- Achievement 1
- Achievement 2

## Previous Role
**Old Company** | *2018 - 2020*

- Task 1";

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    println!("\n=== GENERATED TYPST SOURCE ===");
    println!("{typst_source}");
    println!("=== END TYPST SOURCE ===\n");

    // Count blocks
    let block_count = typst_source.matches("#block(breakable: false)[").count();
    println!("Found {block_count} non-breakable blocks");

    assert!(
        block_count > 0,
        "Should have at least one non-breakable block"
    );
}
