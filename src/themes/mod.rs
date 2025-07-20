pub mod color;
pub mod font;

use anyhow::Result;
use serde::{Deserialize, Serialize};

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
