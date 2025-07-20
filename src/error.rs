use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CvError {
    #[error("Missing required field '{field}' in {file}")]
    MissingField { field: String, file: PathBuf },

    #[error("Unknown theme '{theme}'. Available themes: {available}")]
    UnknownTheme { theme: String, available: String },

    #[error("Invalid markdown structure: {reason}")]
    InvalidMarkdown { reason: String },

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Invalid output format: {format}. Supported formats: pdf, docx, html")]
    InvalidFormat { format: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
}
