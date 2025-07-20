use crate::constants::{font_sizes, AVAILABLE_THEMES};
use crate::error::CvError;
use anyhow::Result;
use serde::{Deserialize, Serialize};

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

impl FontSpec {
    fn new(
        family: &str,
        weight_regular: u16,
        weight_bold: u16,
        letter_spacing: Option<&str>,
    ) -> Self {
        Self {
            family: family.to_string(),
            weight_regular,
            weight_bold,
            size_name: font_sizes::NAME.to_string(),
            size_section: font_sizes::SECTION.to_string(),
            size_subsection: font_sizes::SUBSECTION.to_string(),
            size_normal: font_sizes::NORMAL.to_string(),
            size_small: font_sizes::SMALL.to_string(),
            line_height: 1.5,
            letter_spacing: letter_spacing.map(std::string::ToString::to_string),
        }
    }
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
            header: FontSpec::new("Georgia", 400, 700, None),
            body: FontSpec::new("Times New Roman", 400, 700, None),
        }
    }

    fn modern() -> Self {
        Self {
            header: FontSpec::new("Inter", 400, 700, Some("-0.02em")),
            body: FontSpec::new("Open Sans", 400, 600, None),
        }
    }

    fn sharp() -> Self {
        Self {
            header: FontSpec::new("Montserrat", 400, 700, Some("-0.03em")),
            body: FontSpec::new("Roboto", 400, 700, None),
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
