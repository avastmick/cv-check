//! Integration tests for AI functionality
//!
//! These tests directly call the AI API to ensure structured outputs work correctly.
//! They require the following environment variables to be set:
//! - `AI_ENDPOINT`
//! - `AI_API_KEY`
//! - `AI_MODEL`

pub mod cv_tailoring_test;
pub mod full_workflow_test;
pub mod structured_output_test;
