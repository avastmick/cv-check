#[cfg(test)]
mod tests {
    use cv_check::constants::icons;

    #[test]
    fn test_icon_constants_are_defined() {
        // Test that all required icon constants are defined
        assert_eq!(icons::PHONE, "\u{f095}");
        assert_eq!(icons::EMAIL, "\u{f0e0}");
        assert_eq!(icons::WEBSITE, "\u{f015}");
        assert_eq!(icons::GITHUB, "\u{f09b}");
        assert_eq!(icons::LINKEDIN, "\u{f0e1}");
    }

    #[test]
    fn test_icon_font_name() {
        assert_eq!(icons::FONT_NAME, "FontAwesome");
    }
}
