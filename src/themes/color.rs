use crate::constants::AVAILABLE_THEMES;
use crate::error::CvError;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorTheme {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub text: String,
    pub muted: String,
    pub background: String,
    pub surface: String,
    pub border: String,
}

impl ColorTheme {
    /// Loads a color theme by name.
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
            primary: "#2C3E50".to_string(),    // Navy
            secondary: "#34495E".to_string(),  // Dark Gray
            accent: "#8B0000".to_string(),     // Burgundy
            text: "#2C2C2C".to_string(),       // Charcoal
            muted: "#7F7F7F".to_string(),      // Gray
            background: "#FAFAFA".to_string(), // Warm White
            surface: "#F0F0F0".to_string(),    // Light Gray
            border: "#D0D0D0".to_string(),     // Medium Gray
        }
    }

    fn modern() -> Self {
        Self {
            primary: "#0066CC".to_string(),    // Electric Blue
            secondary: "#00A8A8".to_string(),  // Teal
            accent: "#FF6B35".to_string(),     // Orange
            text: "#333333".to_string(),       // Dark Gray
            muted: "#666666".to_string(),      // Medium Gray
            background: "#FFFFFF".to_string(), // Pure White
            surface: "#F3F4F6".to_string(),    // Cool Gray
            border: "#E5E7EB".to_string(),     // Light Gray
        }
    }

    fn sharp() -> Self {
        Self {
            primary: "#6B46C1".to_string(),    // Deep Purple
            secondary: "#EC4899".to_string(),  // Hot Pink
            accent: "#84CC16".to_string(),     // Lime
            text: "#1A1A1A".to_string(),       // Near Black
            muted: "#6B7280".to_string(),      // Cool Gray
            background: "#F8FAFC".to_string(), // Cool White
            surface: "#F1F5F9".to_string(),    // Slate
            border: "#E2E8F0".to_string(),     // Light Slate
        }
    }

    /// Convert hex color to Typst RGB format
    #[must_use]
    pub fn to_typst_rgb(&self, color_field: &str) -> String {
        let hex = match color_field {
            "primary" => &self.primary,
            "secondary" => &self.secondary,
            "accent" => &self.accent,
            "text" => &self.text,
            "muted" => &self.muted,
            "background" => &self.background,
            "surface" => &self.surface,
            "border" => &self.border,
            _ => "#000000",
        };

        format!("rgb(\"{hex}\")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_modern_color_theme() {
        let theme = ColorTheme::load("modern").expect("Failed to load modern theme");
        assert_eq!(theme.primary, "#0066CC");
        assert_eq!(theme.secondary, "#00A8A8");
        assert_eq!(theme.accent, "#FF6B35");
    }

    #[test]
    fn test_load_classic_color_theme() {
        let theme = ColorTheme::load("classic").expect("Failed to load classic theme");
        assert_eq!(theme.primary, "#2C3E50");
        assert_eq!(theme.secondary, "#34495E");
        assert_eq!(theme.accent, "#8B0000");
    }

    #[test]
    fn test_load_sharp_color_theme() {
        let theme = ColorTheme::load("sharp").expect("Failed to load sharp theme");
        assert_eq!(theme.primary, "#6B46C1");
        assert_eq!(theme.secondary, "#EC4899");
        assert_eq!(theme.accent, "#84CC16");
    }

    #[test]
    fn test_load_unknown_color_theme() {
        let result = ColorTheme::load("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_to_typst_rgb() {
        let theme = ColorTheme::load("modern").expect("Failed to load theme");
        assert_eq!(theme.to_typst_rgb("primary"), "rgb(\"#0066CC\")");
        assert_eq!(theme.to_typst_rgb("accent"), "rgb(\"#FF6B35\")");
        assert_eq!(theme.to_typst_rgb("unknown"), "rgb(\"#000000\")");
    }
}
