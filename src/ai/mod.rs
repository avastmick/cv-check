//! AI-powered CV tailoring module
//!
//! This module provides functionality to optimize CVs for specific job descriptions
//! using OpenAI-compatible APIs with structured outputs.

pub mod client;
pub mod pdf_parser;
pub mod prompts;
pub mod schema_gen;
pub mod schemas;

pub use client::AIClient;
pub use pdf_parser::extract_text_from_pdf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AIError {
    #[error("PDF parsing error: {0}")]
    PdfParse(String),

    #[error("Invalid API response: {0}")]
    InvalidResponse(String),

    #[error("Environment variable not set: {0}")]
    EnvVar(String),

    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AIError>;
