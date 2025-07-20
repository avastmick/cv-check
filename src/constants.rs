/// Shared constants used throughout the application
/// Available theme names
pub const AVAILABLE_THEMES: &[&str] = &["classic", "modern", "sharp"];

/// Default theme name
pub const DEFAULT_THEME: &str = "modern";

/// Standard font sizes used across all themes
pub mod font_sizes {
    pub const NAME: &str = "28pt";
    pub const SECTION: &str = "16pt";
    pub const SUBSECTION: &str = "14pt";
    pub const NORMAL: &str = "11pt";
    pub const SMALL: &str = "10pt";
}

/// Markdown parser options
#[must_use]
pub fn markdown_options() -> pulldown_cmark::Options {
    use pulldown_cmark::Options;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options
}
