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
    // Heading-specific colors
    pub h1_color: Option<String>,
    pub h2_color: Option<String>,
    pub h3_color: Option<String>,
    // Styling properties
    pub separator_thickness: Option<f32>,
    pub h1_spacing_above: Option<f32>,
    pub h1_spacing_below: Option<f32>,
    pub h2_spacing_above: Option<f32>,
    pub h2_spacing_below: Option<f32>,
    pub h3_spacing_above: Option<f32>,
    pub h3_spacing_below: Option<f32>,
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
            h1_color: None,                    // Use default
            h2_color: None,                    // Use primary
            h3_color: None,                    // Use text
            separator_thickness: None,         // Use default 2pt
            h1_spacing_above: None,            // Use default
            h1_spacing_below: None,            // Use default
            h2_spacing_above: None,            // Use default
            h2_spacing_below: None,            // Use default
            h3_spacing_above: None,            // Use default
            h3_spacing_below: None,            // Use default
        }
    }

    fn modern() -> Self {
        Self {
            primary: "#0066CC".to_string(),        // Electric Blue
            secondary: "#00A8A8".to_string(),      // Teal
            accent: "#FF6B35".to_string(),         // Orange
            text: "#333333".to_string(),           // Dark Gray
            muted: "#666666".to_string(),          // Medium Gray
            background: "#FFFFFF".to_string(),     // Pure White
            surface: "#F3F4F6".to_string(),        // Cool Gray
            border: "#E5E7EB".to_string(),         // Light Gray
            h1_color: None,                        // Use default (text color)
            h2_color: Some("#607D8B".to_string()), // Blue-grey
            h3_color: Some("#424242".to_string()), // Dark grey
            separator_thickness: Some(1.0),        // Thinner line
            h1_spacing_above: Some(2.5),           // More space above H1
            h1_spacing_below: Some(0.8),           // Standard below H1
            h2_spacing_above: Some(1.2),           // Default H2 spacing
            h2_spacing_below: Some(0.8),           // Default H2 spacing
            h3_spacing_above: Some(1.0),           // Increased spacing
            h3_spacing_below: Some(0.8),           // Increased spacing
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
            h1_color: None,                    // Use default
            h2_color: None,                    // Use primary
            h3_color: None,                    // Use text
            separator_thickness: None,         // Use default
            h1_spacing_above: None,            // Use default
            h1_spacing_below: None,            // Use default
            h2_spacing_above: None,            // Use default
            h2_spacing_below: None,            // Use default
            h3_spacing_above: None,            // Use default
            h3_spacing_below: None,            // Use default
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

    /// Get H1 color with fallback to text color
    #[must_use]
    pub fn get_h1_color(&self) -> String {
        self.h1_color
            .as_ref()
            .map_or_else(|| self.to_typst_rgb("text"), |c| format!("rgb(\"{c}\")"))
    }

    /// Get H2 color with fallback to primary color
    #[must_use]
    pub fn get_h2_color(&self) -> String {
        self.h2_color
            .as_ref()
            .map_or_else(|| self.to_typst_rgb("primary"), |c| format!("rgb(\"{c}\")"))
    }

    /// Get H3 color with fallback to text color
    #[must_use]
    pub fn get_h3_color(&self) -> String {
        self.h3_color
            .as_ref()
            .map_or_else(|| self.to_typst_rgb("text"), |c| format!("rgb(\"{c}\")"))
    }

    /// Get separator thickness with fallback to 2pt
    #[must_use]
    pub fn get_separator_thickness(&self) -> f32 {
        self.separator_thickness.unwrap_or(2.0)
    }

    /// Get H1 spacing above with fallback to 1.5em
    #[must_use]
    pub fn get_h1_spacing_above(&self) -> f32 {
        self.h1_spacing_above.unwrap_or(1.5)
    }

    /// Get H1 spacing below with fallback to 0.8em
    #[must_use]
    pub fn get_h1_spacing_below(&self) -> f32 {
        self.h1_spacing_below.unwrap_or(0.8)
    }

    /// Get H2 spacing above with fallback to 1.2em
    #[must_use]
    pub fn get_h2_spacing_above(&self) -> f32 {
        self.h2_spacing_above.unwrap_or(1.2)
    }

    /// Get H2 spacing below with fallback to 0.8em
    #[must_use]
    pub fn get_h2_spacing_below(&self) -> f32 {
        self.h2_spacing_below.unwrap_or(0.8)
    }

    /// Get H3 spacing above with fallback to 0.8em
    #[must_use]
    pub fn get_h3_spacing_above(&self) -> f32 {
        self.h3_spacing_above.unwrap_or(0.8)
    }

    /// Get H3 spacing below with fallback to 0.6em
    #[must_use]
    pub fn get_h3_spacing_below(&self) -> f32 {
        self.h3_spacing_below.unwrap_or(0.6)
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
