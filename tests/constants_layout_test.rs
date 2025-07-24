#[cfg(test)]
mod tests {
    use cv_check::constants::layout;

    #[test]
    fn test_margin_constants_are_defined() {
        // Test that all required margin constants are defined
        assert_eq!(layout::margins::TOP, "1.5cm");
        assert_eq!(layout::margins::BOTTOM, "1.5cm");
        assert_eq!(layout::margins::LEFT, "2cm");
        assert_eq!(layout::margins::RIGHT, "2cm");
    }

    #[test]
    fn test_spacing_constants_are_defined() {
        // Test vertical spacing constants
        assert_eq!(layout::spacing::EXTRA_TINY, "0.1em");
        assert_eq!(layout::spacing::VERY_TINY, "0.2em");
        assert_eq!(layout::spacing::TINY, "0.3em");
        assert_eq!(layout::spacing::SMALL, "0.5em");
        assert_eq!(layout::spacing::MEDIUM, "1em");
        assert_eq!(layout::spacing::LARGE, "1.5em");
    }

    #[test]
    fn test_font_size_constants_are_defined() {
        // Test font size constants
        assert_eq!(layout::font_sizes::SMALL, "10pt");
        assert_eq!(layout::font_sizes::NORMAL, "11pt");
        assert_eq!(layout::font_sizes::MEDIUM, "12pt");
        assert_eq!(layout::font_sizes::SUBSECTION, "14pt");
        assert_eq!(layout::font_sizes::SECTION, "16pt");
        assert_eq!(layout::font_sizes::TITLE, "36pt");
        assert_eq!(layout::font_sizes::NAME, "28pt");
    }
}
