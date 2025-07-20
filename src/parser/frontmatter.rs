use crate::config::DocumentMetadata;
use crate::error::CvError;
use anyhow::Result;
use std::path::Path;

/// Parses YAML frontmatter and markdown content from a document.
///
/// # Errors
///
/// Returns an error if the frontmatter is missing, malformed, or cannot be parsed.
pub fn parse_frontmatter(input: &str, _source_path: &Path) -> Result<(DocumentMetadata, String)> {
    let lines: Vec<&str> = input.lines().collect();

    // Check if document starts with frontmatter delimiter
    if lines.is_empty() || lines[0].trim() != "---" {
        return Err(CvError::InvalidMarkdown {
            reason: "Document must start with YAML frontmatter (---)".to_string(),
        }
        .into());
    }

    // Find the closing delimiter
    let mut frontmatter_end = None;
    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim() == "---" {
            frontmatter_end = Some(i);
            break;
        }
    }

    let frontmatter_end = frontmatter_end.ok_or_else(|| CvError::InvalidMarkdown {
        reason: "Frontmatter must be closed with ---".to_string(),
    })?;

    // Extract frontmatter
    let frontmatter = lines[1..frontmatter_end].join("\n");

    // Parse YAML
    let metadata: DocumentMetadata =
        serde_yaml::from_str(&frontmatter).map_err(|e| CvError::InvalidMarkdown {
            reason: format!("Invalid YAML in frontmatter: {e}"),
        })?;

    // Extract content
    let content = lines[(frontmatter_end + 1)..].join("\n");

    Ok((metadata, content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_valid_frontmatter() {
        let input = r"---
name: John Doe
email: john@example.com
font_theme: modern
color_theme: classic
---
# My CV
Content here";

        let (metadata, content) = parse_frontmatter(input, &PathBuf::from("test.md"))
            .expect("Failed to parse frontmatter");

        assert_eq!(metadata.name, "John Doe");
        assert_eq!(metadata.email, "john@example.com");
        assert_eq!(metadata.font_theme, "modern");
        assert_eq!(metadata.color_theme, "classic");
        assert!(content.contains("# My CV"));
    }

    #[test]
    fn test_missing_frontmatter() {
        let input = "# My CV\nContent here";

        let result = parse_frontmatter(input, &PathBuf::from("test.md"));
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_frontmatter() {
        let input = r"---
name: John Doe
email: john@example.com
# My CV";

        let result = parse_frontmatter(input, &PathBuf::from("test.md"));
        assert!(result.is_err());
    }
}
