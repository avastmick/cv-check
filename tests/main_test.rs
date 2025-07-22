use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::process::Command as StdCommand;
use tempfile::TempDir;

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "A modern CV and cover letter generator",
        ));
}

#[test]
fn test_build_command_help() {
    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args(["build", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Generate PDF/DOCX from markdown"));
}

#[test]
fn test_build_missing_input() {
    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.arg("build")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "required arguments were not provided",
        ));
}

#[test]
fn test_build_nonexistent_file() {
    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args(["build", "nonexistent.md"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn test_new_cv_command() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test-cv.md");

    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args(["new", "cv", output_path.to_str().expect("Invalid path")])
        .env("RUST_LOG", "info")
        .assert()
        .success()
        .stderr(predicate::str::contains("Created"));

    assert!(output_path.exists());
    let content = fs::read_to_string(&output_path).expect("Failed to read created file");
    assert!(content.contains("name:"));
    assert!(content.contains("email:"));
}

#[test]
fn test_new_letter_command() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test-letter.md");

    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args(["new", "letter", output_path.to_str().expect("Invalid path")])
        .env("RUST_LOG", "info")
        .assert()
        .success()
        .stderr(predicate::str::contains("Created"));

    assert!(output_path.exists());
    let content = fs::read_to_string(&output_path).expect("Failed to read created file");
    assert!(content.contains("recipient:"));
}

#[test]
fn test_themes_command() {
    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.arg("themes")
        .assert()
        .success()
        .stdout(predicate::str::contains("Font Themes"))
        .stdout(predicate::str::contains("Color Themes"));
}

#[test]
fn test_themes_fonts_only() {
    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args(["themes", "--fonts"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Font Themes"))
        .stdout(predicate::str::contains("Color Themes").not());
}

#[test]
fn test_themes_colors_only() {
    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args(["themes", "--colors"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Color Themes"))
        .stdout(predicate::str::contains("Font Themes").not());
}

#[test]
fn test_check_valid_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let test_file = temp_dir.path().join("valid.md");

    fs::write(
        &test_file,
        r"---
name: John Doe
email: john@example.com
---

# Test Document
",
    )
    .expect("Failed to write test file");

    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args(["check", test_file.to_str().expect("Invalid path")])
        .env("RUST_LOG", "info")
        .assert()
        .success()
        .stderr(predicate::str::contains("is valid!"));
}

#[test]
fn test_check_invalid_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let test_file = temp_dir.path().join("invalid.md");

    fs::write(
        &test_file,
        r"---
email: john@example.com
---

# Missing name field
",
    )
    .expect("Failed to write test file");

    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args(["check", test_file.to_str().expect("Invalid path")])
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing field `name`"));
}

#[test]
fn test_build_with_output_path() {
    // Use the actual example file
    let example_path = std::path::Path::new("examples/cv.md");
    assert!(example_path.exists(), "Example file should exist");

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_file = temp_dir.path().join("custom-output.pdf");

    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args([
        "build",
        example_path.to_str().expect("Invalid path"),
        "--output",
        output_file.to_str().expect("Invalid path"),
        "--quiet",
    ])
    .assert()
    .success();

    // Verify PDF was created at the specified path
    assert!(output_file.exists(), "PDF should be created at custom path");
    let pdf_size = fs::metadata(&output_file)
        .expect("Failed to get PDF metadata")
        .len();
    assert!(
        pdf_size > 5000,
        "PDF should have substantial content, but was only {pdf_size} bytes"
    );
}

#[test]
fn test_pdf_generation_with_actual_content() {
    // Skip test if Typst is not available
    if StdCommand::new("typst").arg("--version").output().is_err() {
        eprintln!("Skipping test: Typst not installed");
        return;
    }

    // Use the actual example file
    let example_path = std::path::Path::new("examples/cv.md");
    assert!(example_path.exists(), "Example file should exist");

    // Verify the markdown file can be read
    let _ = fs::read_to_string(example_path).expect("Failed to read example file");

    // Extract key content that must appear in the PDF
    let expected_content = vec![
        "Jane Smith",
        "jane.smith\\@example.com",
        "+1 (555) 123-4567",
        "San Francisco, CA",
        "Professional Summary",
        "Innovative software engineer with 10+ years",
        "Experience",
        "Senior Software Engineer",
        "Tech Innovations Inc.",
        "Led development of microservices architecture",
        "5M+ daily active users",
        "Software Engineer",
        "StartupXYZ",
        "Education",
        "M.S. Computer Science",
        "Stanford University",
        "Skills",
        "Languages",
        "JavaScript/TypeScript, Python, Go, Rust, Java",
        "Cloud/DevOps",
        "AWS, GCP, Docker, Kubernetes",
    ];

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_file = temp_dir.path().join("test-output.pdf");

    // Build the PDF
    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args([
        "build",
        example_path.to_str().expect("Invalid path"),
        "--output",
        output_file.to_str().expect("Invalid path"),
        "--quiet",
    ])
    .assert()
    .success();

    // Verify PDF exists
    assert!(output_file.exists(), "PDF should be created");

    // Since we can't easily extract text from PDF in tests, verify the generated Typst source
    // which should have been saved in debug mode
    let typst_debug_path = std::path::Path::new("/tmp/cv_debug/generated.typ");
    if typst_debug_path.exists() {
        let typst_content =
            fs::read_to_string(typst_debug_path).expect("Failed to read debug Typst file");

        // Verify all expected content is in the generated Typst file
        for expected in &expected_content {
            assert!(
                typst_content.contains(expected),
                "Generated Typst should contain '{expected}' from the markdown input"
            );
        }
    } else {
        // If no debug file, at least verify PDF is substantial
        let pdf_size = fs::metadata(&output_file)
            .expect("Failed to get PDF metadata")
            .len();
        assert!(
            pdf_size > 15000,
            "PDF with full CV content should be at least 15KB, but was only {pdf_size} bytes"
        );
    }
}

#[test]
fn test_build_with_verbose() {
    // Use the actual example file
    let example_path = std::path::Path::new("examples/cv.md");
    assert!(example_path.exists(), "Example file should exist");

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("output.pdf");

    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args([
        "build",
        example_path.to_str().expect("Invalid path"),
        "--output",
        output_path.to_str().expect("Invalid path"),
        "--verbose",
        "--quiet",
    ])
    .env("RUST_LOG", "info")
    .assert()
    .success()
    .stderr(predicate::str::contains("INFO"));

    // Verify PDF was created and has substantial content
    assert!(output_path.exists(), "PDF should be created");
    let pdf_size = fs::metadata(&output_path)
        .expect("Failed to get PDF metadata")
        .len();
    assert!(
        pdf_size > 5000,
        "PDF should have substantial content, but was only {pdf_size} bytes"
    );
}

#[test]
fn test_build_with_quiet() {
    // Use the actual example file
    let example_path = std::path::Path::new("examples/cv.md");
    assert!(example_path.exists(), "Example file should exist");

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("output.pdf");

    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args([
        "build",
        example_path.to_str().expect("Invalid path"),
        "--output",
        output_path.to_str().expect("Invalid path"),
        "--quiet",
    ])
    .assert()
    .success()
    .stdout(predicate::str::is_empty());

    // Verify PDF was created even in quiet mode
    assert!(output_path.exists(), "PDF should be created");
    let pdf_size = fs::metadata(&output_path)
        .expect("Failed to get PDF metadata")
        .len();
    assert!(
        pdf_size > 5000,
        "PDF should have substantial content, but was only {pdf_size} bytes"
    );
}

#[test]
fn test_serve_command() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_file = temp_dir.path().join("test.md");

    fs::write(
        &input_file,
        r"---
name: Test User
email: test@example.com
---

# Test CV
",
    )
    .expect("Failed to write test file");

    // Note: We can't actually test the server running, but we can test the command starts
    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args([
        "serve",
        input_file.to_str().expect("Invalid path"),
        "--port",
        "9999",
    ])
    .env("RUST_LOG", "info")
    .assert()
    .success()
    .stderr(predicate::str::contains(
        "Preview server at http://localhost:9999",
    ));
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.arg("invalid-command")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_build_invalid_format() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let input_file = temp_dir.path().join("test.md");

    fs::write(
        &input_file,
        r"---
name: Test User
email: test@example.com
---

# Test CV
",
    )
    .expect("Failed to write test file");

    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.args([
        "build",
        input_file.to_str().expect("Invalid path"),
        "--format",
        "invalid",
    ])
    .assert()
    .failure()
    .stderr(predicate::str::contains("Invalid output format"));
}
