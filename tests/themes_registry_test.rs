#[cfg(test)]
mod tests {
    use cv_check::themes::THEME_REGISTRY;

    #[test]
    fn test_theme_registry_contains_all_themes() {
        // Test that all expected themes are in the registry
        let theme_names: Vec<&str> = THEME_REGISTRY.iter().map(|t| t.name).collect();
        assert!(theme_names.contains(&"classic"));
        assert!(theme_names.contains(&"modern"));
        assert!(theme_names.contains(&"sharp"));
        assert_eq!(theme_names.len(), 3);
    }

    #[test]
    fn test_theme_registry_has_descriptions() {
        // Test that all themes have both font and color descriptions
        for theme in THEME_REGISTRY {
            assert!(!theme.font_description.is_empty());
            assert!(!theme.color_description.is_empty());
        }
    }

    #[test]
    fn test_classic_theme_info() {
        let classic = THEME_REGISTRY
            .iter()
            .find(|t| t.name == "classic")
            .expect("Classic theme should exist in registry");
        assert_eq!(
            classic.font_description,
            "Traditional serif fonts (Georgia/Times)"
        );
        assert_eq!(classic.color_description, "Navy and burgundy (traditional)");
    }

    #[test]
    fn test_modern_theme_info() {
        let modern = THEME_REGISTRY
            .iter()
            .find(|t| t.name == "modern")
            .expect("Modern theme should exist in registry");
        assert_eq!(
            modern.font_description,
            "Clean sans-serif (Inter/Open Sans)"
        );
        assert_eq!(modern.color_description, "Blue and teal (tech)");
    }

    #[test]
    fn test_sharp_theme_info() {
        let sharp = THEME_REGISTRY
            .iter()
            .find(|t| t.name == "sharp")
            .expect("Sharp theme should exist in registry");
        assert_eq!(sharp.font_description, "Bold geometric (Montserrat/Roboto)");
        assert_eq!(sharp.color_description, "Purple and pink (creative)");
    }

    #[test]
    fn test_get_theme_info() {
        // Test helper function to get theme info by name
        use cv_check::themes::get_theme_info;

        assert!(get_theme_info("classic").is_some());
        assert!(get_theme_info("modern").is_some());
        assert!(get_theme_info("sharp").is_some());
        assert!(get_theme_info("nonexistent").is_none());
    }
}
