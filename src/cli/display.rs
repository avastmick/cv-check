//! Display utilities for CLI output
//!
//! This module provides styled display components for the CLI,
//! including the suggestions box for CV tailoring feedback.

use colored::Colorize;
use log::{debug, info};
use std::fmt::Write as FmtWrite;

/// Maximum width for the suggestions box (including borders)
const MAX_BOX_WIDTH: usize = 60;

/// Padding inside the box
const PADDING: usize = 2;

/// Box drawing characters
const TOP_LEFT: &str = "╭";
const TOP_RIGHT: &str = "╮";
const BOTTOM_LEFT: &str = "╰";
const BOTTOM_RIGHT: &str = "╯";
const HORIZONTAL: &str = "─";
const VERTICAL: &str = "│";

/// A styled box for displaying CV improvement suggestions
pub struct SuggestionsBox<'a> {
    pub suggestions: &'a [String],
}

impl<'a> SuggestionsBox<'a> {
    /// Creates a new suggestions box
    #[must_use]
    pub fn new(suggestions: &'a [String]) -> Self {
        Self { suggestions }
    }

    /// Formats the suggestions into a styled box
    #[must_use]
    pub fn format(&self) -> String {
        let mut output = String::new();

        // Fixed content width to ensure consistent sizing
        // Total box width will be content_width + 2 (for borders)
        // So for a 60-char total width, content_width should be 58
        info!("format() called - MAX_BOX_WIDTH={MAX_BOX_WIDTH}, PADDING={PADDING}");
        let content_width = 58; // Hardcode to 58 to ensure 60 total width

        // Calculate available space for text after padding
        let text_width = content_width - 2 * PADDING;
        info!("content_width={content_width}, text_width={text_width}");

        // Top border
        writeln!(
            &mut output,
            "{}{}{}",
            TOP_LEFT.cyan(),
            HORIZONTAL.repeat(content_width).cyan(),
            TOP_RIGHT.cyan()
        )
        .expect("Failed to write to string");

        // Title line
        let title = "CV Suggestions";
        let title_colored = title.bright_white().bold();
        let title_padding = (content_width - title.len()) / 2;
        writeln!(
            &mut output,
            "{}{}{}{}{}",
            VERTICAL.cyan(),
            " ".repeat(title_padding),
            title_colored,
            " ".repeat(content_width - title_padding - title.len()),
            VERTICAL.cyan(),
        )
        .expect("Failed to write to string");

        // Separator line
        writeln!(
            &mut output,
            "{}{}{}",
            VERTICAL.cyan(),
            HORIZONTAL.repeat(content_width).cyan(),
            VERTICAL.cyan()
        )
        .expect("Failed to write to string");

        // Content
        if self.suggestions.is_empty() {
            let no_suggestions = "No suggestions available".dimmed();
            let msg_padding = (content_width - "No suggestions available".len()) / 2;
            writeln!(
                &mut output,
                "{}{}{}{}{}",
                VERTICAL.cyan(),
                " ".repeat(msg_padding),
                no_suggestions,
                " ".repeat(content_width - msg_padding - "No suggestions available".len()),
                VERTICAL.cyan(),
            )
            .expect("Failed to write to string");
        } else {
            for suggestion in self.suggestions {
                // Wrap long suggestions (account for padding and bullet point)
                // content_width includes padding, so we need: content_width - 2*PADDING - 4 (for bullet)
                let available_width = text_width - 4; // text_width already accounts for padding
                info!("Processing suggestion: available_width={available_width} for wrapping");
                let wrapped_lines = wrap_text(suggestion, available_width);
                for (i, line) in wrapped_lines.iter().enumerate() {
                    if i == 0 {
                        // First line with bullet
                        let bullet_line = format!("  {} {}", "•".bright_green(), line);
                        let padding = content_width.saturating_sub(visual_length(&bullet_line));
                        writeln!(
                            &mut output,
                            "{}{}{}{}",
                            VERTICAL.cyan(),
                            bullet_line,
                            " ".repeat(padding),
                            VERTICAL.cyan(),
                        )
                        .expect("Failed to write to string");
                    } else {
                        // Continuation lines
                        let cont_line = format!("    {line}");
                        let padding = content_width.saturating_sub(cont_line.len());
                        writeln!(
                            &mut output,
                            "{}{}{}{}",
                            VERTICAL.cyan(),
                            cont_line,
                            " ".repeat(padding),
                            VERTICAL.cyan(),
                        )
                        .expect("Failed to write to string");
                    }
                }
            }
        }

        // Bottom border
        write!(
            &mut output,
            "{}{}{}",
            BOTTOM_LEFT.cyan(),
            HORIZONTAL.repeat(content_width).cyan(),
            BOTTOM_RIGHT.cyan()
        )
        .expect("Failed to write to string");

        output
    }

    /// Display the suggestions box to stdout
    pub fn display(&self) {
        println!("\n{}\n", self.format());
    }

    /// Formats the suggestions into a styled box WITHOUT colors (for testing)
    ///
    /// This method is used by integration tests to verify box width without ANSI codes
    #[must_use]
    #[allow(dead_code)] // `allow(dead_code)` exception
    pub fn format_plain(&self) -> String {
        let mut output = String::new();

        // Fixed content width
        let content_width = 58; // For 60 total width
        let text_width = content_width - 2 * PADDING;

        // Top border
        writeln!(
            &mut output,
            "{}{}{}",
            TOP_LEFT,
            HORIZONTAL.repeat(content_width),
            TOP_RIGHT
        )
        .expect("Failed to write to string");

        // Title line
        let title = "CV Suggestions";
        writeln!(&mut output, "{VERTICAL}{title:^content_width$}{VERTICAL}",)
            .expect("Failed to write to string");

        // Separator
        writeln!(
            &mut output,
            "{}{}{}",
            VERTICAL,
            HORIZONTAL.repeat(content_width),
            VERTICAL
        )
        .expect("Failed to write to string");

        // Content
        if self.suggestions.is_empty() {
            writeln!(
                &mut output,
                "{}{:^content_width$}{}",
                VERTICAL, "No suggestions available", VERTICAL,
            )
            .expect("Failed to write to string");
        } else {
            for suggestion in self.suggestions {
                let available_width = text_width - 4;
                let wrapped_lines = wrap_text(suggestion, available_width);
                for (i, line) in wrapped_lines.iter().enumerate() {
                    if i == 0 {
                        writeln!(
                            &mut output,
                            "{}{:<content_width$}{}",
                            VERTICAL,
                            format!("  • {}", line),
                            VERTICAL,
                        )
                        .expect("Failed to write to string");
                    } else {
                        writeln!(
                            &mut output,
                            "{}{:<content_width$}{}",
                            VERTICAL,
                            format!("    {}", line),
                            VERTICAL,
                        )
                        .expect("Failed to write to string");
                    }
                }
            }
        }

        // Bottom border
        write!(
            &mut output,
            "{}{}{}",
            BOTTOM_LEFT,
            HORIZONTAL.repeat(content_width),
            BOTTOM_RIGHT
        )
        .expect("Failed to write to string");

        output
    }
}

/// Calculate the visual length of a string, accounting for ANSI codes
fn visual_length(s: &str) -> usize {
    // Simple approach: count only printable characters
    // ANSI escape sequences start with ESC (0x1b) and end with 'm'
    let mut len = 0;
    let mut in_ansi = false;

    for ch in s.chars() {
        if ch == '\x1b' {
            in_ansi = true;
        } else if in_ansi && ch == 'm' {
            in_ansi = false;
        } else if !in_ansi {
            len += 1;
        }
    }

    len
}

/// Wraps text to fit within the specified width
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    info!(
        "wrap_text called with max_width={max_width}, text_len={}",
        text.len()
    );
    debug!("wrap_text input text: '{text}'");

    let mut lines = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty() {
        return vec![String::new()];
    }

    let mut current_line = String::new();

    for word in words {
        if current_line.is_empty() {
            current_line = word.to_string();
            debug!("Starting new line with word: '{word}'");
        } else if current_line.len() + 1 + word.len() <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
            debug!(
                "Adding word '{word}' to current line, new length: {}",
                current_line.len()
            );
        } else {
            debug!(
                "Line full, pushing: '{current_line}' (len={})",
                current_line.len()
            );
            lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        debug!(
            "Pushing final line: '{current_line}' (len={})",
            current_line.len()
        );
        lines.push(current_line);
    }

    info!("wrap_text returning {} lines", lines.len());
    for (i, line) in lines.iter().enumerate() {
        debug!("  Line {i}: '{line}' (len={})", line.len());
    }

    lines
}

/// Display suggestions in a styled box
pub fn show_suggestions(suggestions: &[String]) {
    let display_box = SuggestionsBox::new(suggestions);
    display_box.display();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text_short() {
        let text = "This is a short line";
        let wrapped = wrap_text(text, 50);
        assert_eq!(wrapped.len(), 1);
        assert_eq!(wrapped[0], "This is a short line");
    }

    #[test]
    fn test_wrap_text_long() {
        let text = "This is a very long line that needs to be wrapped because it exceeds the maximum width";
        let wrapped = wrap_text(text, 30);
        assert!(wrapped.len() > 1);
        for line in &wrapped {
            assert!(line.len() <= 30);
        }
    }

    #[test]
    fn test_wrap_text_empty() {
        let text = "";
        let wrapped = wrap_text(text, 50);
        assert_eq!(wrapped.len(), 1);
        assert_eq!(wrapped[0], "");
    }
}
