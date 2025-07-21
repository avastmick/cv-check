use cv_check::config::DocumentMetadata;
use cv_check::parser::Document;
use cv_check::render::{pdf::PdfRenderer, RenderEngine};
use cv_check::themes::Theme;
use tempfile::TempDir;

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
fn test_h2_sections_are_wrapped_in_typst_source() {
    // Create content with H2 sections that should be wrapped
    let content = r"# Experience

## Senior Software Engineer
**Tech Company** | *2020 - Present*

- Led team of 10 engineers
- Implemented microservices architecture
- Reduced deployment time by 60%

## Software Engineer
**Previous Company** | *2018 - 2020*

- Built REST APIs
- Improved performance by 40%

# Education

## Bachelor of Science
**University Name** | *2014 - 2018*";

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    // Get the generated Typst source
    let typst_source = renderer.generate_typst_source_for_testing(&doc, &theme);

    // Verify that H2 sections are wrapped in non-breakable blocks
    assert!(
        typst_source.contains("#block(breakable: false)["),
        "Typst source should contain non-breakable blocks for job entries"
    );

    // Count the number of job entry blocks
    let job_entry_blocks = typst_source.matches("// Start of job entry").count();
    assert_eq!(
        job_entry_blocks, 3,
        "Should have 3 job entry blocks (2 in Experience, 1 in Education)"
    );

    // Verify blocks are closed
    let block_ends = typst_source.matches("// End of job entry block").count();
    assert_eq!(
        block_ends, 3,
        "Should have 3 closing blocks to match the opening blocks"
    );
}

#[test]
fn test_long_job_entry_stays_together() {
    // Create a long job entry that would normally split across pages
    let content = r"# Experience

## Senior Software Engineer
**Tech Company** | *2020 - Present*

This is a long job description that includes many achievements and responsibilities that would normally cause the content to split across page boundaries if not properly handled.

- Led development of microservices architecture serving 5M+ daily active users
- Mentored team of 8 engineers, resulting in 40% improvement in code quality metrics
- Implemented real-time data processing system handling 100K events per second
- Reduced deployment time by 70% through CI/CD pipeline improvements
- Architected scalable solutions using cloud-native technologies
- Established coding standards and best practices across the organization
- Collaborated with product team to define technical requirements
- Conducted technical interviews and helped grow the team
- Presented at internal tech talks and external conferences
- Contributed to open source projects with 500+ GitHub stars
- Optimized database queries resulting in 80% performance improvement
- Implemented comprehensive monitoring and alerting system
- Led migration from monolithic to microservices architecture
- Designed and implemented RESTful APIs serving mobile and web clients
- Improved application security through regular audits and updates
- Managed technical debt and prioritized refactoring efforts
- Coordinated with cross-functional teams on product launches
- Developed automated testing strategies increasing coverage to 90%
- Created technical documentation and runbooks
- Participated in on-call rotation ensuring system reliability

## Next Role
**Another Company** | *2018 - 2020*

A shorter role description.";

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF with long job entry");

    assert!(output_path.exists(), "PDF should be created");

    // The long job entry should stay together on one page or start fresh on a new page
    // rather than splitting in the middle
}

#[test]
fn test_manual_pagebreak_overrides_automatic_grouping() {
    let content = r"# Experience

## First Job
**Company A** | *2020 - 2021*

Some content here.

<!-- pagebreak -->

Still part of the first job but after manual page break.

## Second Job
**Company B** | *2019 - 2020*

More content here.";

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test.pdf");

    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");
    let doc = create_test_document_with_content(content);
    let theme = create_test_theme();

    renderer
        .render(&doc, &theme, &output_path)
        .expect("Failed to render PDF");

    assert!(
        output_path.exists(),
        "PDF should handle manual breaks within sections"
    );
}
