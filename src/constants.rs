/// Shared constants used throughout the application
pub mod icons;
pub mod layout;

/// Available theme names
pub const AVAILABLE_THEMES: &[&str] = &["classic", "modern", "sharp"];

/// Default theme name
pub const DEFAULT_THEME: &str = "modern";

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
