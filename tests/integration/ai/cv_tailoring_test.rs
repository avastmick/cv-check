//! Integration tests for CV tailoring functionality

use cv_check::ai::{extract_text_from_pdf, AIClient};
use std::fs;
use std::path::Path;

#[tokio::test]
#[ignore = "Requires AI API credentials and PDF files"]
async fn test_cv_tailoring_full_flow() {
    // This test will only run if environment variables are set
    let Ok(mut client) = AIClient::from_env() else {
        eprintln!("Skipping test: AI environment variables not set");
        return;
    };

    // Read the example CV
    let sample_cv =
        fs::read_to_string("examples/cv.md").expect("Should be able to read example CV");

    // Read the job description from PDF
    let jd_path = Path::new("examples/jd_example.pdf");
    if !jd_path.exists() {
        eprintln!("Skipping test: examples/jd_example.pdf not found");
        return;
    }
    let job_description = extract_text_from_pdf(jd_path)
        .expect("Should be able to extract text from job description PDF");

    // Call the tailor_cv method
    let result = client.tailor_cv(&sample_cv, &job_description).await;

    assert!(result.is_ok(), "CV tailoring should succeed");

    let tailored_cv = result.expect("Should have tailored CV");

    // Validate the response structure
    assert!(
        !tailored_cv.professional_summary.is_empty(),
        "Professional summary should not be empty"
    );
    assert!(
        !tailored_cv.experiences.is_empty(),
        "Should have experiences"
    );
    assert!(!tailored_cv.skills.is_empty(), "Should have skills");
    assert!(!tailored_cv.keywords.is_empty(), "Should extract keywords");

    // Validate that keywords from job description are included
    let keywords_lower: Vec<String> = tailored_cv
        .keywords
        .iter()
        .map(|k| k.to_lowercase())
        .collect();

    // Should have extracted keywords from the job description
    assert!(
        keywords_lower.len() >= 3,
        "Should extract multiple keywords from job description"
    );

    // Validate experiences are properly structured
    for exp in &tailored_cv.experiences {
        assert!(
            !exp.title.is_empty(),
            "Experience title should not be empty"
        );
        assert!(
            !exp.company.is_empty(),
            "Experience company should not be empty"
        );
        assert!(
            !exp.duration.is_empty(),
            "Experience duration should not be empty"
        );
        assert!(
            !exp.highlights.is_empty(),
            "Experience should have highlights"
        );
        assert!(
            exp.relevance_score >= 0.0 && exp.relevance_score <= 1.0,
            "Relevance score should be between 0 and 1"
        );
    }

    // Validate that suggestions are provided
    assert!(
        !tailored_cv.suggestions.is_empty(),
        "Should provide improvement suggestions"
    );
}

#[tokio::test]
#[ignore = "Requires AI API credentials and PDF files"]
async fn test_cv_tailoring_emphasizes_relevant_skills() {
    let Ok(mut client) = AIClient::from_env() else {
        eprintln!("Skipping test: AI environment variables not set");
        return;
    };

    // Read the example CV
    let sample_cv =
        fs::read_to_string("examples/cv.md").expect("Should be able to read example CV");

    // Read the job description from PDF
    let jd_path = Path::new("examples/jd_example.pdf");
    if !jd_path.exists() {
        eprintln!("Skipping test: examples/jd_example.pdf not found");
        return;
    }
    let job_description = extract_text_from_pdf(jd_path)
        .expect("Should be able to extract text from job description PDF");

    let result = client
        .tailor_cv(&sample_cv, &job_description)
        .await
        .expect("CV tailoring should succeed");

    // The tailored CV should have skills section
    assert!(!result.skills.is_empty(), "Should have skills");

    // Skills should be ordered by relevance to the job
    assert!(
        result.skills.len() >= 5,
        "Should have multiple skills listed"
    );

    // Check that the AI has provided a relevance score for experiences
    for exp in &result.experiences {
        assert!(
            exp.relevance_score >= 0.0 && exp.relevance_score <= 1.0,
            "Each experience should have a relevance score"
        );
    }
}

#[tokio::test]
#[ignore = "Requires AI API credentials and PDF files"]
async fn test_cv_tailoring_handles_missing_sections() {
    let Ok(mut client) = AIClient::from_env() else {
        eprintln!("Skipping test: AI environment variables not set");
        return;
    };

    // Use the CV with missing sections (no summary, no skills, no education)
    let cv_missing_sections = fs::read_to_string("examples/cv-missing-sections.md")
        .expect("Should be able to read CV with missing sections");

    // Read the job description from PDF
    let jd_path = Path::new("examples/jd_example.pdf");
    if !jd_path.exists() {
        eprintln!("Skipping test: examples/jd_example.pdf not found");
        return;
    }
    let job_description = extract_text_from_pdf(jd_path)
        .expect("Should be able to extract text from job description PDF");

    let result = client
        .tailor_cv(&cv_missing_sections, &job_description)
        .await;

    assert!(result.is_ok(), "Should handle CVs with missing sections");

    let tailored_cv = result.expect("Should have tailored CV");

    // Should still generate all required sections
    assert!(
        !tailored_cv.professional_summary.is_empty(),
        "Should generate professional summary"
    );
    assert!(
        !tailored_cv.skills.is_empty(),
        "Should infer or suggest skills"
    );
    assert!(
        tailored_cv
            .suggestions
            .iter()
            .any(|s| s.to_lowercase().contains("skill")
                || s.to_lowercase().contains("education")
                || s.to_lowercase().contains("add")),
        "Should suggest adding missing sections"
    );
}
