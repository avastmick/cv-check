use crate::error::CvError;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const AVAILABLE_THEMES: &[&str] = &["classic", "modern", "sharp"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontTheme {
    pub header: FontSpec,
    pub body: FontSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSpec {
    pub family: String,
    pub weight_regular: u16,
    pub weight_bold: u16,
    pub size_name: String,
    pub size_section: String,
    pub size_subsection: String,
    pub size_normal: String,
    pub size_small: String,
    pub line_height: f32,
    pub letter_spacing: Option<String>,
}

impl FontTheme {
    /// Loads a font theme by name.
    ///
    /// # Errors
    ///
    /// Returns an error if the theme name is not recognized.
    pub fn load(theme_name: &str) -> Result<Self> {
        match theme_name {
            "classic" => Ok(Self::classic()),
            "modern" => Ok(Self::modern()),
            "sharp" => Ok(Self::sharp()),
            _ => Err(CvError::UnknownTheme {
                theme: theme_name.to_string(),
                available: AVAILABLE_THEMES.join(", "),
            }
            .into()),
        }
    }

    fn classic() -> Self {
        Self {
            header: FontSpec {
                family: "Georgia".to_string(),
                weight_regular: 400,
                weight_bold: 700,
                size_name: "28pt".to_string(),
                size_section: "16pt".to_string(),
                size_subsection: "14pt".to_string(),
                size_normal: "11pt".to_string(),
                size_small: "10pt".to_string(),
                line_height: 1.5,
                letter_spacing: None,
            },
            body: FontSpec {
                family: "Times New Roman".to_string(),
                weight_regular: 400,
                weight_bold: 700,
                size_name: "28pt".to_string(),
                size_section: "16pt".to_string(),
                size_subsection: "14pt".to_string(),
                size_normal: "11pt".to_string(),
                size_small: "10pt".to_string(),
                line_height: 1.5,
                letter_spacing: None,
            },
        }
    }

    fn modern() -> Self {
        Self {
            header: FontSpec {
                family: "Inter".to_string(),
                weight_regular: 400,
                weight_bold: 700,
                size_name: "28pt".to_string(),
                size_section: "16pt".to_string(),
                size_subsection: "14pt".to_string(),
                size_normal: "11pt".to_string(),
                size_small: "10pt".to_string(),
                line_height: 1.5,
                letter_spacing: Some("-0.02em".to_string()),
            },
            body: FontSpec {
                family: "Open Sans".to_string(),
                weight_regular: 400,
                weight_bold: 600,
                size_name: "28pt".to_string(),
                size_section: "16pt".to_string(),
                size_subsection: "14pt".to_string(),
                size_normal: "11pt".to_string(),
                size_small: "10pt".to_string(),
                line_height: 1.5,
                letter_spacing: None,
            },
        }
    }

    fn sharp() -> Self {
        Self {
            header: FontSpec {
                family: "Montserrat".to_string(),
                weight_regular: 400,
                weight_bold: 700,
                size_name: "28pt".to_string(),
                size_section: "16pt".to_string(),
                size_subsection: "14pt".to_string(),
                size_normal: "11pt".to_string(),
                size_small: "10pt".to_string(),
                line_height: 1.5,
                letter_spacing: Some("-0.03em".to_string()),
            },
            body: FontSpec {
                family: "Roboto".to_string(),
                weight_regular: 400,
                weight_bold: 700,
                size_name: "28pt".to_string(),
                size_section: "16pt".to_string(),
                size_subsection: "14pt".to_string(),
                size_normal: "11pt".to_string(),
                size_small: "10pt".to_string(),
                line_height: 1.5,
                letter_spacing: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_modern_theme() {
        let theme = FontTheme::load("modern").expect("Failed to load modern theme");
        assert_eq!(theme.header.family, "Inter");
        assert_eq!(theme.body.family, "Open Sans");
    }

    #[test]
    fn test_load_classic_theme() {
        let theme = FontTheme::load("classic").expect("Failed to load classic theme");
        assert_eq!(theme.header.family, "Georgia");
        assert_eq!(theme.body.family, "Times New Roman");
    }

    #[test]
    fn test_load_sharp_theme() {
        let theme = FontTheme::load("sharp").expect("Failed to load sharp theme");
        assert_eq!(theme.header.family, "Montserrat");
        assert_eq!(theme.body.family, "Roboto");
    }

    #[test]
    fn test_load_unknown_theme() {
        let result = FontTheme::load("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_available_themes_contains_expected() {
        assert!(AVAILABLE_THEMES.contains(&"classic"));
        assert!(AVAILABLE_THEMES.contains(&"modern"));
        assert!(AVAILABLE_THEMES.contains(&"sharp"));
    }
}
