use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    // Personal Information
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub website: Option<String>,

    // Theme Configuration
    #[serde(default = "default_font_theme")]
    pub font_theme: String,
    #[serde(default = "default_color_theme")]
    pub color_theme: String,

    // Layout Options
    #[serde(default)]
    pub layout: LayoutOptions,

    // Cover Letter specific
    pub recipient: Option<RecipientInfo>,
    pub date: Option<String>,
    pub subject: Option<String>,

    // Custom fields
    #[serde(flatten)]
    pub custom: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientInfo {
    pub name: String,
    pub title: Option<String>,
    pub company: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LayoutOptions {
    pub columns: u8,
    pub margins: Margins,
    pub sidebar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Margins {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            columns: 1,
            margins: Margins::default(),
            sidebar: None,
        }
    }
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            top: 1.5,
            bottom: 1.5,
            left: 2.0,
            right: 2.0,
        }
    }
}

use crate::constants::DEFAULT_THEME;

fn default_font_theme() -> String {
    DEFAULT_THEME.to_string()
}

fn default_color_theme() -> String {
    DEFAULT_THEME.to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub default_font_theme: Option<String>,
    pub default_color_theme: Option<String>,
    pub pdf_engine: Option<String>,
    pub custom_themes_dir: Option<String>,
    pub output_dir: Option<String>,
    pub auto_open: Option<bool>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            default_font_theme: Some(DEFAULT_THEME.to_string()),
            default_color_theme: Some(DEFAULT_THEME.to_string()),
            pdf_engine: Some("typst".to_string()),
            custom_themes_dir: None,
            output_dir: Some("./output".to_string()),
            auto_open: Some(true),
        }
    }
}

impl GlobalConfig {
    /// Loads the global configuration from the user's config directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the config file exists but cannot be read or parsed.
    pub fn load() -> anyhow::Result<Self> {
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("cv_gen").join("config.yaml");
            if config_path.exists() {
                let content = std::fs::read_to_string(config_path)?;
                let config: GlobalConfig = serde_yaml::from_str(&content)?;
                return Ok(config);
            }
        }
        Ok(Self::default())
    }
}
