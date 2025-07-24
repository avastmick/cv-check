#[cfg(test)]
mod tests {
    use cv_check::test_utils::{create_minimal_document, create_test_document, create_test_theme};

    #[test]
    fn test_create_test_document() {
        let doc = create_test_document();
        assert_eq!(doc.metadata.name, "Test User");
        assert_eq!(doc.metadata.email, "test@example.com");
        assert_eq!(doc.metadata.font_theme, "modern");
        assert_eq!(doc.metadata.color_theme, "modern");
        assert!(doc.metadata.phone.is_some());
        assert!(doc.metadata.location.is_some());
        assert!(!doc.content.is_empty());
    }

    #[test]
    fn test_create_minimal_document() {
        let doc = create_minimal_document("John Doe", "john@example.com");
        assert_eq!(doc.metadata.name, "John Doe");
        assert_eq!(doc.metadata.email, "john@example.com");
        assert_eq!(doc.metadata.font_theme, "modern");
        assert_eq!(doc.metadata.color_theme, "modern");
        assert!(doc.metadata.phone.is_none());
        assert!(doc.metadata.location.is_none());
    }

    #[test]
    fn test_create_test_theme() {
        let theme = create_test_theme();
        assert_eq!(theme.font.header.family, "Inter");
        assert_eq!(theme.color.primary, "#0066CC");
    }

    #[test]
    fn test_create_document_with_content() {
        use cv_check::test_utils::create_document_with_content;

        let content = "# Test\n\nThis is test content.";
        let doc = create_document_with_content(content);
        assert_eq!(doc.content, content);
        assert!(!doc.markdown_ast.is_empty());
    }

    #[test]
    fn test_create_theme_by_name() {
        use cv_check::test_utils::create_theme_by_name;

        let classic_theme = create_theme_by_name("classic", "classic");
        assert_eq!(classic_theme.font.header.family, "Georgia");

        let modern_theme = create_theme_by_name("modern", "modern");
        assert_eq!(modern_theme.font.header.family, "Inter");

        let sharp_theme = create_theme_by_name("sharp", "sharp");
        assert_eq!(sharp_theme.font.header.family, "Montserrat");
    }
}
