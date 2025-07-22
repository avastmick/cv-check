use cv_check::config::DocumentMetadata;
use cv_check::parser::Document;
use cv_check::render::pdf::PdfRenderer;
use cv_check::themes::Theme;

fn create_test_document_modern_theme() -> Document {
    let content = r"# Professional Summary

Experienced software engineer with expertise in building scalable systems.

# Experience

## Senior Software Engineer
**Tech Corp** | *2020 - Present*

- Led development of cloud infrastructure
- Implemented CI/CD pipelines

### Key Achievements
- Reduced deployment time by 70%
- Improved system reliability

# Education

## B.S. Computer Science
**Stanford University** | *2010 - 2014*";

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
fn test_modern_theme_h1_spacing() {
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_modern_theme();
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // H1 should have increased spacing above (2.5em for modern theme)
    assert!(
        typst_source.contains("#v(2.5em)") && typst_source.contains("text(size: 16pt"),
        "H1 headings should have 2.5em vertical spacing above in modern theme"
    );
}

#[test]
fn test_modern_theme_h2_color() {
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_modern_theme();
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // H2 headings should use blue-grey color instead of primary
    assert!(
        typst_source.contains("text(size: 14pt, weight: \"bold\", fill: rgb(\"#607D8B\"))"),
        "H2 headings should use blue-grey color #607D8B"
    );
}

#[test]
fn test_modern_theme_h3_styling() {
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_modern_theme();
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // H3 headings should have dark grey color
    assert!(
        typst_source.contains("text(size: 12pt, weight: \"semibold\", fill: rgb(\"#424242\"))"),
        "H3 headings should have size 12pt, semibold weight, and dark grey color #424242"
    );

    // H3 should have increased vertical spacing (Typst outputs 1em not 1.0em)
    assert!(
        typst_source.contains("#v(1em)") && typst_source.contains("text(size: 12pt"),
        "H3 headings should have 1em vertical spacing above"
    );
}

#[test]
fn test_modern_theme_separator_line() {
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_modern_theme();
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Check separator line after H1 - should be thinner (1.0pt)
    assert!(
        typst_source.contains("#line(length: 100%, stroke: 1pt + rgb(\"#FF6B35\"))"),
        "Separator line should use modern theme accent color #FF6B35 with reduced 1pt thickness"
    );
}

#[test]
fn test_modern_theme_text_color() {
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_modern_theme();
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Default text should use text color from theme
    assert!(
        typst_source.contains("fill: rgb(\"#333333\")"),
        "Default text should use modern theme text color #333333"
    );
}

#[test]
fn test_h3_spacing_pattern() {
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_modern_theme();
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Check that H3 has proper spacing block
    let lines: Vec<&str> = typst_source.lines().collect();

    let mut found_h3_with_spacing = false;
    for (i, line) in lines.iter().enumerate() {
        if line.contains("#v(1em)")
            && i + 1 < lines.len()
            && lines[i + 1].contains("#block(above: 0em, below: 0.8em)")
            && i + 2 < lines.len()
            && lines[i + 2].contains("text(size: 12pt")
        {
            found_h3_with_spacing = true;
            break;
        }
    }

    assert!(
        found_h3_with_spacing,
        "H3 should have proper vertical spacing pattern with 1em above and 0.8em below"
    );
}

#[test]
fn test_different_theme_colors() {
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");

    // Test classic theme
    let mut doc = create_test_document_modern_theme();
    doc.metadata.color_theme = "classic".to_string();
    let theme = Theme::new("modern", "classic").expect("Failed to create theme");

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Classic theme should use navy color for H2
    assert!(
        typst_source.contains("rgb(\"#2C3E50\")"),
        "Classic theme should use navy color #2C3E50"
    );

    // Classic theme should use burgundy accent
    assert!(
        typst_source.contains("rgb(\"#8B0000\")"),
        "Classic theme should use burgundy accent #8B0000"
    );
}
