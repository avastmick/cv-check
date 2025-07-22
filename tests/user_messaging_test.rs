use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_build_command_shows_user_messages() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let input_file = temp_dir.path().join("test.md");

    // Create a minimal valid markdown file
    fs::write(
        &input_file,
        "---\nname: Test User\nemail: test@example.com\n---\n\n# Experience\nTest content\n",
    )
    .expect("Failed to write test file");

    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.arg("build")
        .arg(&input_file)
        .arg("--format")
        .arg("html") // Use HTML to avoid Typst dependency
        .env("CV_CHECK_NO_OPEN", "1"); // Prevent auto-opening in tests

    // Should show user messages (not logging)
    cmd.assert()
        .success()
        .stderr(predicate::str::is_empty()) // No logging in normal mode
        .stdout(predicate::str::contains("Building document"));
}

#[test]
fn test_build_command_quiet_mode() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let input_file = temp_dir.path().join("test.md");

    fs::write(
        &input_file,
        "---\nname: Test User\nemail: test@example.com\n---\n\n# Experience\nTest content\n",
    )
    .expect("Failed to write test file");

    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.arg("build")
        .arg(&input_file)
        .arg("--quiet")
        .arg("--format")
        .arg("html");

    // In quiet mode, should not show any user messages
    cmd.assert()
        .success()
        .stderr(predicate::str::is_empty()) // No logging in quiet mode
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_verbose_mode_shows_logging() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let input_file = temp_dir.path().join("test.md");

    fs::write(
        &input_file,
        "---\nname: Test User\nemail: test@example.com\n---\n\n# Experience\nTest content\n",
    )
    .expect("Failed to write test file");

    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.arg("build")
        .arg(&input_file)
        .arg("--verbose")
        .arg("--format")
        .arg("html")
        .env("RUST_LOG", "info")
        .env("CV_CHECK_NO_OPEN", "1"); // Prevent auto-opening in tests

    // In verbose mode, should show both user messages AND logging
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("INFO")) // Should have logging in stderr
        .stdout(predicate::str::contains("Building document")) // Should have user messages in stdout
        .stdout(predicate::str::contains("Font theme:")); // Should show verbose configuration info
}

#[test]
fn test_new_command_shows_user_feedback() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output_file = temp_dir.path().join("new_cv.md");

    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.arg("new").arg("cv").arg(&output_file);

    // Should show success message to user
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created").or(predicate::str::contains("created")));
}

#[test]
fn test_check_command_shows_validation_result() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let input_file = temp_dir.path().join("valid.md");

    fs::write(
        &input_file,
        "---\nname: Test User\nemail: test@example.com\n---\n\n# Experience\nTest content\n",
    )
    .expect("Failed to write test file");

    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.arg("check").arg(&input_file);

    // Should show validation result to user
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("valid").or(predicate::str::contains("Valid")));
}

#[test]
fn test_themes_command_shows_theme_list() {
    let mut cmd = Command::cargo_bin("cv").expect("Failed to find binary");
    cmd.arg("themes");

    // Should display themes to user
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("classic").or(predicate::str::contains("Classic")))
        .stdout(predicate::str::contains("modern").or(predicate::str::contains("Modern")));
}
