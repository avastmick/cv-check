pub mod color;
pub mod font;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Information about a theme including descriptions
#[derive(Debug, Clone)]
pub struct ThemeInfo {
    /// Theme name (e.g., "classic", "modern", "sharp")
    pub name: &'static str,
    /// Description of the font theme
    pub font_description: &'static str,
    /// Description of the color theme
    pub color_description: &'static str,
}

/// Registry of all available themes with their descriptions
pub const THEME_REGISTRY: &[ThemeInfo] = &[
    ThemeInfo {
        name: "classic",
        font_description: "Traditional serif fonts (Georgia/Times)",
        color_description: "Navy and burgundy (traditional)",
    },
    ThemeInfo {
        name: "modern",
        font_description: "Clean sans-serif (Inter/Open Sans)",
        color_description: "Blue and teal (tech)",
    },
    ThemeInfo {
        name: "sharp",
        font_description: "Bold geometric (Montserrat/Roboto)",
        color_description: "Purple and pink (creative)",
    },
];

/// Get theme info by name
#[must_use]
pub fn get_theme_info(name: &str) -> Option<&'static ThemeInfo> {
    THEME_REGISTRY.iter().find(|t| t.name == name)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub font: font::FontTheme,
    pub color: color::ColorTheme,
}

impl Theme {
    /// Creates a new theme with the specified font and color themes.
    ///
    /// # Errors
    ///
    /// Returns an error if either theme name is not recognized.
    pub fn new(font_theme_name: &str, color_theme_name: &str) -> Result<Self> {
        let font = font::FontTheme::load(font_theme_name)?;
        let color = color::ColorTheme::load(color_theme_name)?;

        Ok(Self { font, color })
    }

    /// Returns lists of available font and color theme names.
    #[must_use]
    pub fn available_themes() -> (Vec<&'static str>, Vec<&'static str>) {
        use crate::constants::AVAILABLE_THEMES;
        (AVAILABLE_THEMES.to_vec(), AVAILABLE_THEMES.to_vec())
    }
}
