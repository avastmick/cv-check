//! Integration test for the full AI-powered CV tailoring workflow

use log::info;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[tokio::test]
#[ignore = "Requires AI API credentials and PDF files"]
async fn test_full_ai_workflow_from_cli() {
    // Check if environment variables are set
    if std::env::var("AI_ENDPOINT").is_err()
        || std::env::var("AI_API_KEY").is_err()
        || std::env::var("AI_MODEL").is_err()
    {
        info!("Skipping test: AI environment variables not set");
        return;
    }

    // Create a temporary directory for output
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("tailored_cv.md");

    // Use the example CV and JD
    let cv_path = Path::new("examples/cv.md");
    let jd_path = Path::new("examples/jd_example.pdf");

    // Check if files exist
    if !cv_path.exists() || !jd_path.exists() {
        info!("Skipping test: Example files not found");
        return;
    }

    // Run the CLI command
    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "--",
            "tailor",
            "--cv",
            cv_path.to_str().expect("CV path should be valid UTF-8"),
            "--job-description",
            jd_path.to_str().expect("JD path should be valid UTF-8"),
            "--output",
            output_path
                .to_str()
                .expect("Output path should be valid UTF-8"),
            "--format",
            "md",
            "--verbose",
        ])
        .output()
        .expect("Failed to execute command");

    // Check the command succeeded
    if !output.status.success() {
        info!(
            "Command failed with stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!("CV tailoring command failed");
    }

    // Verify the output file was created
    assert!(output_path.exists(), "Tailored CV file should be created");

    // Read and verify the content
    let content = fs::read_to_string(&output_path).expect("Should be able to read tailored CV");

    // Basic content checks
    assert!(!content.is_empty(), "Tailored CV should not be empty");
    assert!(
        content.contains("## Professional Summary"),
        "Should have professional summary"
    );
    assert!(
        content.contains("## Experience"),
        "Should have experience section"
    );
    assert!(content.contains("## Skills"), "Should have skills section");

    // Check for AI-specific markers
    assert!(
        content.contains("AI-Tailored CV"),
        "Should indicate it's AI-tailored"
    );
    assert!(
        content.contains("<!-- ATS Keywords:"),
        "Should include ATS keywords"
    );
}

#[tokio::test]
#[ignore = "Requires AI API credentials"]
async fn test_full_ai_workflow_pdf_output() {
    // Check if environment variables are set
    if std::env::var("AI_ENDPOINT").is_err()
        || std::env::var("AI_API_KEY").is_err()
        || std::env::var("AI_MODEL").is_err()
    {
        info!("Skipping test: AI environment variables not set");
        return;
    }

    // Create a temporary directory for output
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_base = temp_dir.path().join("tailored_cv");

    // Use the example CV and JD
    let cv_path = Path::new("examples/cv.md");
    let jd_path = Path::new("examples/jd_example.pdf");

    // Check if files exist
    if !cv_path.exists() || !jd_path.exists() {
        info!("Skipping test: Example files not found");
        return;
    }

    // Run the CLI command for PDF output
    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "--",
            "tailor",
            "--cv",
            cv_path.to_str().expect("CV path should be valid UTF-8"),
            "--job-description",
            jd_path.to_str().expect("JD path should be valid UTF-8"),
            "--output",
            output_base
                .to_str()
                .expect("Output path should be valid UTF-8"),
            "--format",
            "pdf",
        ])
        .output()
        .expect("Failed to execute command");

    // Check the command succeeded
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!("CV tailoring command failed.\nSTDERR: {stderr}\nSTDOUT: {stdout}");
    }

    // Verify both markdown and PDF files were created
    // When format is PDF, the markdown file is saved at the output path
    // and the PDF is saved with .pdf extension
    let md_path = output_base.clone();
    let pdf_path = output_base.with_extension("pdf");

    assert!(
        md_path.exists(),
        "Tailored markdown file should be created at: {}",
        md_path.display()
    );
    assert!(
        pdf_path.exists(),
        "Tailored PDF file should be created at: {}",
        pdf_path.display()
    );

    // Verify PDF has content (basic size check)
    let pdf_metadata = fs::metadata(&pdf_path).expect("Should be able to read PDF metadata");
    assert!(
        pdf_metadata.len() > 1000,
        "PDF should have substantial content"
    );
}

#[tokio::test]
#[ignore = "Requires AI API credentials"]
async fn test_ai_error_handling() {
    // Test with invalid job description file
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("tailored_cv.md");

    let cv_path = Path::new("examples/cv.md");
    let invalid_jd = temp_dir.path().join("not_a_pdf.txt");
    fs::write(&invalid_jd, "This is not a PDF").expect("Failed to write test file");

    // Run the CLI command with invalid PDF
    let output = std::process::Command::new("cargo")
        .args([
            "run",
            "--",
            "tailor",
            "--cv",
            cv_path.to_str().expect("CV path should be valid UTF-8"),
            "--job-description",
            invalid_jd
                .to_str()
                .expect("Invalid JD path should be valid UTF-8"),
            "--output",
            output_path
                .to_str()
                .expect("Output path should be valid UTF-8"),
        ])
        .output()
        .expect("Failed to execute command");

    // Should fail gracefully
    assert!(!output.status.success(), "Should fail with invalid PDF");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Not a PDF file") || stderr.contains("PDF"),
        "Error message should mention PDF issue"
    );
}
