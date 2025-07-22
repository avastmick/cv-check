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
fn test_pagebreak_markers_in_typst_source() {
    let content = r"# Experience

## First Job
**Company A** | *2020 - 2021*

Some content.

<!-- pagebreak -->

## Second Job
**Company B** | *2019 - 2020*

More content.

\pagebreak

## Third Job
**Company C** | *2018 - 2019*";

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Count pagebreak commands
    let pagebreak_count = typst_source.matches("#pagebreak()").count();
    assert_eq!(
        pagebreak_count, 2,
        "Should have 2 pagebreak commands (one HTML comment, one LaTeX style)"
    );
}

#[test]
fn test_no_unwanted_pagebreaks() {
    let content = r"# Experience

## Job One
**Company A** | *2020 - 2021*

Description.

## Job Two
**Company B** | *2019 - 2020*

Description.";

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Should not contain any pagebreak commands
    assert!(
        !typst_source.contains("#pagebreak()"),
        "Should not contain automatic pagebreaks between job entries"
    );

    // Should not contain weak pagebreaks
    assert!(
        !typst_source.contains("pagebreak(weak:"),
        "Should not contain weak pagebreaks"
    );
}

#[test]
fn test_h2_wrapping_pattern() {
    let content = r"# Experience

## Software Engineer
**Tech Corp** | *2020 - Present*

- Achievement 1
- Achievement 2

## Previous Role
**Old Company** | *2018 - 2020*

- Task 1
- Task 2";

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Verify the wrapping pattern
    // Should find pattern: block(breakable: false) -> H2 content -> end block
    let lines: Vec<&str> = typst_source.lines().collect();

    let mut found_wrapped_h2 = false;

    for (i, line) in lines.iter().enumerate() {
        if line.contains("#block(breakable: false)[") {
            // Check that within a few lines we have the H2 heading
            for check_line in lines.iter().skip(i + 1).take(9) {
                if check_line.contains("text(size: 14pt, weight: \"bold\"") {
                    found_wrapped_h2 = true;
                    break;
                }
            }
        }
    }

    assert!(
        found_wrapped_h2,
        "Should find H2 headings wrapped in non-breakable blocks"
    );
}

#[test]
fn test_section_transitions() {
    let content = r"# Experience

## Job A
**Company A** | *2020 - Present*

Details about job A.

## Job B
**Company B** | *2019 - 2020*

Details about job B.

# Education

## Degree
**University** | *2015 - 2019*

Details about education.";

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // When transitioning from Experience to Education (H1 sections),
    // the Job B block should be properly closed
    let lines: Vec<&str> = typst_source.lines().collect();

    let mut found_education_h1 = false;
    let mut block_closed_before_education = false;

    for (i, line) in lines.iter().enumerate() {
        if line.contains("text(size: 16pt, weight: \"bold\", fill:") && i > 10 {
            // This is likely the Education H1
            found_education_h1 = true;

            // Check that there's a block end before this
            for j in (0..i).rev() {
                if lines[j].contains("// End of job entry block") {
                    block_closed_before_education = true;
                    break;
                }
                if lines[j].contains("// Start of job entry") {
                    // Found a start without an end
                    break;
                }
            }
        }
    }

    assert!(found_education_h1, "Should find Education H1 section");
    assert!(
        block_closed_before_education,
        "Job entry block should be closed before Education section starts"
    );
}
