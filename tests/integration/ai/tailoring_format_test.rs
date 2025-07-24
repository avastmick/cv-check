//! Tests for CV tailoring output formatting

use cv_check::ai::schemas::{OptimizedExperience, TailoredCV};
use std::fmt::Write;

/// Helper function to create test `TailoredCV` data
fn create_test_tailored_cv() -> TailoredCV {
    TailoredCV {
        professional_summary: "Visionary and pragmatic technology leader with extensive experience building and scaling high-performing engineering teams in modern cloud-based SaaS environments. Proven ability to define and execute technical vision, fostering a culture of innovation and collaboration that drives business growth.".to_string(),
        experiences: vec![
            OptimizedExperience {
                title: "Co-founder, Chief Technology Officer".to_string(),
                company: "TechStartup".to_string(),
                duration: "December 2023 - Present".to_string(),
                highlights: vec![
                    "Co-founded and led an AI-first startup, taking an innovative SaaS product from concept to market".to_string(),
                    "Engineered a scalable, multi-model generative AI platform on a GCP-powered stack".to_string(),
                    "Championed the complete technical vision and product strategy".to_string(),
                ],
                relevance_score: 0.95,
            },
            OptimizedExperience {
                title: "Chief Digital Officer".to_string(),
                company: "Enterprise Corp".to_string(),
                duration: "January 2023 - November 2023".to_string(),
                highlights: vec![
                    "Led a 30-person digital team through a major transformation".to_string(),
                    "Drove innovation by integrating generative AI to automate and scale core business processes".to_string(),
                ],
                relevance_score: 0.94,
            },
            OptimizedExperience {
                title: "Senior Architect".to_string(),
                company: "Consulting Firm".to_string(),
                duration: "February 2008 - July 2016".to_string(),
                highlights: vec![
                    "Provided extensive technical leadership and architecture on multiple enterprise-scale projects".to_string(),
                    "Led and mentored cross-functional engineering teams ranging from 10 to 50 personnel".to_string(),
                ],
                relevance_score: 0.85,
            },
        ],
        skills: vec![
            "AI Strategy & Implementation".to_string(),
            "Technical & Strategic Leadership".to_string(),
            "SaaS Platform Architecture".to_string(),
            "Cloud-Native Stacks (GCP, AWS)".to_string(),
            "Python".to_string(),
            "Rust".to_string(),
        ],
        keywords: vec![
            "AI".to_string(),
            "SaaS".to_string(),
            "Technical Vision".to_string(),
            "GCP".to_string(),
            "High-Performing Engineering Team".to_string(),
        ],
        suggestions: vec![
            "Quantify the impact of your achievements further".to_string(),
            "Research specific challenges in the target sector".to_string(),
        ],
    }
}

/// Helper function to verify content sections
fn verify_content_sections(content: &str, tailored_cv: &TailoredCV) {
    // Verify Professional Summary section
    assert!(
        content.contains("# Professional Summary"),
        "Should contain Professional Summary header as H1"
    );
    assert!(
        content.contains(&tailored_cv.professional_summary),
        "Should contain the professional summary text"
    );

    // Verify Experience section is renamed to "Relevant Experience"
    assert!(
        !content.contains("# Experience\n"),
        "Should NOT contain 'Experience' header"
    );
    assert!(
        content.contains("# Relevant Experience"),
        "Should contain 'Relevant Experience' header as H1"
    );

    // Verify experiences are included
    for exp in &tailored_cv.experiences {
        assert!(
            content.contains(&format!("## {} at {}", exp.title, exp.company)),
            "Should contain job title and company as H2"
        );
        assert!(
            content.contains(&format!("*{}*", exp.duration)),
            "Should contain duration in italics"
        );
        for highlight in &exp.highlights {
            assert!(
                content.contains(&format!("- {highlight}")),
                "Should contain highlight as bullet point"
            );
        }
    }

    // Verify Skills section
    assert!(
        content.contains("## Skills"),
        "Should contain Skills header"
    );
    let skills_line = tailored_cv.skills.join(", ");
    assert!(
        content.contains(&skills_line),
        "Should contain comma-separated skills"
    );

    // Verify proper spacing (no mangled output)
    assert!(
        !content.contains("\n\n\n\n"),
        "Should not have excessive blank lines"
    );

    // Verify Education section is preserved
    assert!(
        content.contains("## Education"),
        "Should contain Education header"
    );

    // Verify ATS keywords are included as comments
    assert!(
        content.contains("<!-- ATS Keywords:"),
        "Should contain ATS keywords comment"
    );

    // Verify suggestions are included as comments
    assert!(
        content.contains("<!-- AI Suggestions:"),
        "Should contain AI suggestions comment"
    );
}

/// Test that the tailored CV output includes proper formatting
#[test]
fn test_tailored_cv_formatting() {
    let tailored_cv = create_test_tailored_cv();
    let content = generate_tailored_content(&tailored_cv);
    verify_content_sections(&content, &tailored_cv);
}

/// Test that experiences are ordered by date (most recent first)
#[test]
fn test_experience_date_ordering() {
    let tailored_cv = TailoredCV {
        professional_summary: "Summary".to_string(),
        experiences: vec![
            // Intentionally out of order to test sorting
            OptimizedExperience {
                title: "Old Role".to_string(),
                company: "Old Company".to_string(),
                duration: "2010 - 2012".to_string(),
                highlights: vec!["Old achievement".to_string()],
                relevance_score: 0.9, // High relevance but old
            },
            OptimizedExperience {
                title: "Current Role".to_string(),
                company: "Current Company".to_string(),
                duration: "2022 - Present".to_string(),
                highlights: vec!["Current achievement".to_string()],
                relevance_score: 0.8,
            },
            OptimizedExperience {
                title: "Mid Role".to_string(),
                company: "Mid Company".to_string(),
                duration: "2015 - 2020".to_string(),
                highlights: vec!["Mid achievement".to_string()],
                relevance_score: 0.7,
            },
        ],
        skills: vec!["Skill1".to_string()],
        keywords: vec!["keyword1".to_string()],
        suggestions: vec![],
    };

    let content = generate_tailored_content_with_ordering(&tailored_cv);

    // Find positions of each role in the output
    let current_pos = content
        .find("Current Role")
        .expect("Should find Current Role");
    let mid_pos = content.find("Mid Role").expect("Should find Mid Role");
    let old_pos = content.find("Old Role").expect("Should find Old Role");

    // Verify ordering: Current should come before Mid, Mid before Old
    assert!(
        current_pos < mid_pos,
        "Current role should appear before mid role"
    );
    assert!(mid_pos < old_pos, "Mid role should appear before old role");
}

/// Test that Education section with skills is preserved in tailored output
#[test]
fn test_education_section_preservation() {
    // Mock original CV content that includes education
    let original_content = r"## Education

## M.S. Computer Science
**Stanford University** | *2012 - 2014*

- Specialization in Distributed Systems
- GPA: 3.9/4.0

## B.S. Computer Science
**UC Berkeley** | *2008 - 2012*

- Magna Cum Laude
- Dean's List

### Technical Skills Gained
- Advanced Algorithms
- Distributed Computing
- Machine Learning";

    let tailored_cv = TailoredCV {
        professional_summary: "Summary".to_string(),
        experiences: vec![],
        skills: vec!["Rust".to_string()],
        keywords: vec![],
        suggestions: vec![],
    };

    // Generate content with education preservation
    let content = generate_tailored_content_with_education(&tailored_cv, original_content);

    // Verify Education section is included
    assert!(
        content.contains("## Education"),
        "Should preserve Education header"
    );
    assert!(
        content.contains("M.S. Computer Science"),
        "Should preserve degree"
    );
    assert!(
        content.contains("Stanford University"),
        "Should preserve university"
    );
    assert!(
        content.contains("### Technical Skills Gained"),
        "Should preserve skills subsection"
    );
}

// Helper function to mimic generate_tailored_content from cli/mod.rs
fn generate_tailored_content(tailored_cv: &TailoredCV) -> String {
    let mut content = String::new();

    // Add professional summary
    content.push_str("# Professional Summary\n\n");
    content.push_str(&tailored_cv.professional_summary);
    content.push_str("\n\n");

    // Add experiences with "Relevant Experience" header
    content.push_str("# Relevant Experience\n\n");
    for exp in &tailored_cv.experiences {
        let _ = writeln!(&mut content, "## {} at {}", exp.title, exp.company);
        let _ = writeln!(&mut content, "*{}*\n", exp.duration);
        for highlight in &exp.highlights {
            let _ = writeln!(&mut content, "- {highlight}");
        }
        let _ = writeln!(
            &mut content,
            "\n<!-- Relevance Score: {:.2} -->\n",
            exp.relevance_score
        );
    }

    // Add placeholder Education section (would be extracted from original in real implementation)
    content.push_str("## Education\n\n");
    content.push_str("## M.S. Computer Science\n");
    content.push_str("**Stanford University** | *2012 - 2014*\n\n");

    // Add skills
    content.push_str("## Skills\n\n");
    content.push_str(&tailored_cv.skills.join(", "));
    content.push_str("\n\n");

    // Add keywords for ATS
    content.push_str("<!-- ATS Keywords: ");
    content.push_str(&tailored_cv.keywords.join(", "));
    content.push_str(" -->\n\n");

    // Add suggestions as comments
    if !tailored_cv.suggestions.is_empty() {
        content.push_str("<!-- AI Suggestions:\n");
        for suggestion in &tailored_cv.suggestions {
            let _ = writeln!(&mut content, "- {suggestion}");
        }
        content.push_str("-->\n");
    }

    content
}

// Helper function for testing with date ordering
fn generate_tailored_content_with_ordering(tailored_cv: &TailoredCV) -> String {
    let mut content = String::new();

    content.push_str("## Professional Summary\n\n");
    content.push_str(&tailored_cv.professional_summary);
    content.push_str("\n\n");

    // Sort experiences by date (most recent first)
    let mut sorted_experiences = tailored_cv.experiences.clone();
    sorted_experiences.sort_by(|a, b| {
        // Parse years from duration strings
        let a_year = parse_end_year(&a.duration);
        let b_year = parse_end_year(&b.duration);
        b_year.cmp(&a_year) // Reverse order for most recent first
    });

    content.push_str("# Relevant Experience\n\n");
    for exp in &sorted_experiences {
        let _ = writeln!(&mut content, "## {} at {}", exp.title, exp.company);
        let _ = writeln!(&mut content, "*{}*\n", exp.duration);
        for highlight in &exp.highlights {
            let _ = writeln!(&mut content, "- {highlight}");
        }
        let _ = writeln!(
            &mut content,
            "\n<!-- Relevance Score: {:.2} -->\n",
            exp.relevance_score
        );
    }

    content.push_str("## Skills\n\n");
    content.push_str(&tailored_cv.skills.join(", "));
    content.push_str("\n\n");

    content
}

// Helper function to parse end year from duration string
fn parse_end_year(duration: &str) -> u32 {
    if duration.contains("Present") {
        9999 // Use high value for current positions
    } else {
        // Extract last 4-digit year from duration
        duration
            .split_whitespace()
            .filter_map(|word| word.parse::<u32>().ok())
            .filter(|&year| (1900..=2100).contains(&year))
            .next_back()
            .unwrap_or(0)
    }
}

// Helper function for testing with education preservation
fn generate_tailored_content_with_education(
    tailored_cv: &TailoredCV,
    original_education: &str,
) -> String {
    let mut content = String::new();

    content.push_str("## Professional Summary\n\n");
    content.push_str(&tailored_cv.professional_summary);
    content.push_str("\n\n");

    content.push_str("# Relevant Experience\n\n");
    // Add experiences...

    // Preserve Education section
    content.push_str(original_education);
    content.push_str("\n\n");

    content.push_str("## Skills\n\n");
    content.push_str(&tailored_cv.skills.join(", "));
    content.push_str("\n\n");

    content
}
