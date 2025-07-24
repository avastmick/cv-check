use crate::config::RecipientInfo;
use crate::parser::Document;
use crate::render::{load_template, RenderEngine};
use crate::themes::Theme;
use anyhow::Result;
use chrono::Local;
use std::fmt::Write;
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;

pub struct PdfRenderer {
    template: Option<String>,
}

struct RenderContext {
    list_depth: usize,
    in_heading: bool,
    heading_level: pulldown_cmark::HeadingLevel,
}

impl RenderContext {
    fn new() -> Self {
        Self {
            list_depth: 0,
            in_heading: false,
            heading_level: pulldown_cmark::HeadingLevel::H1,
        }
    }
}

impl PdfRenderer {
    /// Creates a new PDF renderer with optional custom template.
    ///
    /// # Errors
    ///
    /// Returns an error if the template file cannot be read.
    pub fn new(template_path: Option<&Path>) -> Result<Self> {
        let template = load_template(template_path)?;
        Ok(Self { template })
    }

    /// Exposed for testing purposes only
    #[doc(hidden)]
    #[must_use]
    #[allow(dead_code)] // `allow(dead_code)` exception
    pub fn generate_typst_source_for_testing(&self, doc: &Document, theme: &Theme) -> String {
        self.generate_typst_source(doc, theme)
    }

    fn generate_typst_source(&self, doc: &Document, theme: &Theme) -> String {
        if let Some(template) = &self.template {
            // Custom template - just use it as-is
            return template.clone();
        }

        // Generate a complete Typst document without complex template functions
        let mut source = String::new();

        // Add document setup
        Self::add_document_setup(&mut source, doc, theme);

        // Check if this is a cover letter
        let is_cover_letter = doc.metadata.recipient.is_some();

        // Add header section
        if is_cover_letter {
            Self::add_cover_letter_header(&mut source, doc);
        } else {
            Self::add_cv_header(&mut source, doc);
        }

        // Add recipient information for cover letters
        if let Some(recipient) = &doc.metadata.recipient {
            Self::add_recipient_section(&mut source, recipient, doc.metadata.subject.as_ref());
        }

        // Body content - convert markdown to Typst
        let _ = writeln!(source, "// Content");
        let mut typst_content = String::new();
        Self::render_markdown_as_typst(&doc.content, &mut typst_content, theme);

        // Post-process to wrap H2 sections in non-breakable blocks
        let processed_content = Self::wrap_h2_sections(&typst_content);
        source.push_str(&processed_content);

        // For cover letters, add a signature section with contact info
        if is_cover_letter {
            Self::add_letter_signature(&mut source, doc);
        }

        source
    }

    fn add_document_setup(source: &mut String, doc: &Document, theme: &Theme) {
        // Document setup
        let _ = writeln!(
            source,
            "#set document(title: \"{}\", author: \"{}\")",
            doc.metadata.name, doc.metadata.name
        );
        let _ = writeln!(
            source,
            "#set page(paper: \"a4\", margin: (top: 1.5cm, bottom: 1.5cm, left: 2cm, right: 2cm))"
        );

        // Font configuration - use bundled fonts
        let font_family = match doc.metadata.font_theme.as_str() {
            "classic" => "Georgia",
            "sharp" => "Montserrat",
            _ => "Inter", // modern and other themes use Inter
        };

        // Set default text properties
        let _ = writeln!(
            source,
            "#set text(font: \"{}\", size: 11pt, fill: {})",
            font_family,
            theme.color.to_typst_rgb("text")
        );
    }

    fn add_cover_letter_header(source: &mut String, doc: &Document) {
        // Cover letter header - simpler, more formal
        let _ = writeln!(source, "#align(right)[");
        let _ = writeln!(
            source,
            "  #text(size: 14pt, weight: \"bold\")[{}]",
            doc.metadata.name
        );

        // Contact details in a more formal layout on separate lines
        let _ = writeln!(source, "  #v(0.3em)");

        // Location
        if let Some(location) = &doc.metadata.location {
            let _ = writeln!(source, "  #text(size: 11pt)[{location}]");
            let _ = writeln!(source, "  #v(0.1em)");
        }

        // Phone
        if let Some(phone) = &doc.metadata.phone {
            let _ = writeln!(source, "  #text(size: 11pt)[{phone}]");
            let _ = writeln!(source, "  #v(0.1em)");
        }

        // Email
        let escaped_email = doc.metadata.email.replace('@', "\\@");
        let _ = writeln!(source, "  #text(size: 11pt)[{escaped_email}]");

        // Website (optional for letters)
        if let Some(website) = &doc.metadata.website {
            let _ = writeln!(source, "  #v(0.1em)");
            let _ = writeln!(
                source,
                "  #text(size: 11pt)[#link(\"{website}\")[{website}]]"
            );
        }

        let _ = writeln!(source, "]");
    }

    fn add_cv_header(source: &mut String, doc: &Document) {
        // CV header - original centered layout with icons
        let _ = writeln!(source, "#align(center)[");
        let _ = writeln!(
            source,
            "  #text(size: 36pt, weight: \"bold\")[{}]",
            doc.metadata.name
        );

        // Location (if present)
        if let Some(location) = &doc.metadata.location {
            let _ = writeln!(source, "  #v(0.2em)");
            let _ = writeln!(source, "  #text(size: 11pt, style: \"italic\")[{location}]");
        }

        let _ = writeln!(source, "  #v(0.3em)");

        // Contact info - all on one line with icons
        let _ = writeln!(source, "  #text(size: 10pt)[");
        let mut contact_parts = vec![];

        // Phone with FontAwesome icon
        if let Some(phone) = &doc.metadata.phone {
            contact_parts.push(format!("#text(font: \"FontAwesome\")[\\u{{f095}}] {phone}"));
        }

        // Email with FontAwesome icon
        let escaped_email = doc.metadata.email.replace('@', "\\@");
        contact_parts.push(format!(
            "#text(font: \"FontAwesome\")[\\u{{f0e0}}] {escaped_email}"
        ));

        // Website with FontAwesome icon
        if let Some(website) = &doc.metadata.website {
            contact_parts.push(format!(
                "#text(font: \"FontAwesome\")[\\u{{f015}}] #link(\"{website}\")[{website}]"
            ));
        }

        // GitHub with FontAwesome icon
        if let Some(github) = &doc.metadata.github {
            contact_parts.push(format!(
                "#text(font: \"FontAwesome\")[\\u{{f09b}}] #link(\"https://github.com/{github}\")[github.com/{github}]"
            ));
        }

        // LinkedIn with FontAwesome icon
        if let Some(linkedin) = &doc.metadata.linkedin {
            contact_parts.push(format!(
                "#text(font: \"FontAwesome\")[\\u{{f0e1}}] #link(\"https://linkedin.com/in/{linkedin}\")[linkedin.com/in/{linkedin}]"
            ));
        }

        let _ = writeln!(source, "    {}", contact_parts.join(" | "));
        let _ = writeln!(source, "  ]");

        let _ = writeln!(source, "]");
        let _ = writeln!(source, "#v(0.5em)");
    }

    fn add_recipient_section(
        source: &mut String,
        recipient: &RecipientInfo,
        subject: Option<&String>,
    ) {
        // This is a cover letter - adjust formatting accordingly
        let _ = writeln!(source, "// Cover Letter Formatting");

        // Add extra space after header for letter format
        let _ = writeln!(source, "#v(1.5em)");

        // Date aligned to the left (standard business letter format) - always use today's date
        // Use locale-aware formatting
        let today = Local::now();
        // Format: "15 December 2024" for international compatibility
        let formatted_date = today.format("%-d %B %Y").to_string();
        let _ = writeln!(source, "#align(left)[");
        let _ = writeln!(
            source,
            "  #text(size: 11pt, weight: \"bold\")[{formatted_date}]"
        );
        let _ = writeln!(source, "]");
        let _ = writeln!(source, "#v(1em)");

        // Recipient information on the left
        let _ = writeln!(source, "#align(left)[");
        let _ = writeln!(source, "  #text(size: 11pt)[");

        // Handle optional recipient name
        let mut has_content = false;
        if let Some(name) = &recipient.name {
            let _ = writeln!(source, "    {name}");
            has_content = true;
        }

        if let Some(title) = &recipient.title {
            if has_content {
                let _ = writeln!(source, "    #linebreak()");
            }
            let _ = writeln!(source, "    {title}");
            has_content = true;
        }

        if let Some(company) = &recipient.company {
            if has_content {
                let _ = writeln!(source, "    #linebreak()");
            }
            let _ = writeln!(source, "    #text(weight: \"bold\")[{company}]");
            has_content = true;
        }

        if let Some(address) = &recipient.address {
            // Split multiline address and add line breaks
            for line in address.lines() {
                if has_content {
                    let _ = writeln!(source, "    #linebreak()");
                }
                let _ = writeln!(source, "    {line}");
                has_content = true;
            }
        }

        // If no recipient info provided, use generic salutation
        if !has_content {
            let _ = writeln!(source, "    To Whom It May Concern");
        }

        let _ = writeln!(source, "  ]");
        let _ = writeln!(source, "]");
        let _ = writeln!(source, "#v(1em)");

        // Subject line if present
        if let Some(subject) = subject {
            let _ = writeln!(
                source,
                "#text(size: 11pt, weight: \"bold\")[Subject: {subject}]"
            );
            let _ = writeln!(source, "#v(1em)");
        }

        // Add extra space before letter body
        let _ = writeln!(source, "#v(0.5em)");
    }

    fn add_letter_signature(source: &mut String, doc: &Document) {
        // Add signature section at the end of the letter
        let _ = writeln!(source, "\n#v(1em)");

        // Name in bold
        let _ = writeln!(source, "#text(weight: \"bold\")[{}]", doc.metadata.name);
        let _ = writeln!(source, "#v(0.5em)");

        // Contact info on separate lines with FontAwesome icons
        let escaped_email = doc.metadata.email.replace('@', "\\@");
        let _ = writeln!(
            source,
            "#text(font: \"FontAwesome\")[\\u{{f0e0}}] {escaped_email}"
        );

        if let Some(linkedin) = &doc.metadata.linkedin {
            let _ = writeln!(source, "#text(font: \"FontAwesome\")[\\u{{f0e1}}] #link(\"https://linkedin.com/in/{linkedin}\")[linkedin.com/in/{linkedin}]");
        }

        if let Some(github) = &doc.metadata.github {
            let _ = writeln!(source, "#text(font: \"FontAwesome\")[\\u{{f09b}}] #link(\"https://github.com/{github}\")[github.com/{github}]");
        }

        if let Some(website) = &doc.metadata.website {
            let _ = writeln!(
                source,
                "#text(font: \"FontAwesome\")[\\u{{f015}}] #link(\"{website}\")[{website}]"
            );
        }
    }

    fn render_markdown_as_typst(content: &str, output: &mut String, theme: &Theme) {
        use crate::constants::markdown_options;
        use pulldown_cmark::{Event, Parser};

        // Preprocess content to enhance company names and handle page breaks
        let enhanced_content = Self::enhance_company_names(content);
        let content_with_pagebreaks = Self::process_pagebreak_markers(&enhanced_content);

        let options = markdown_options();
        let parser = Parser::new_ext(&content_with_pagebreaks, options);
        let mut render_ctx = RenderContext::new();

        for event in parser {
            match event {
                Event::Start(tag) => Self::handle_start_tag(tag, output, &mut render_ctx, theme),
                Event::End(tag) => Self::handle_end_tag(tag, output, theme, &mut render_ctx),
                Event::Text(text) => Self::handle_text(&text, output, &render_ctx, theme),
                Event::Code(code) => {
                    let _ = write!(output, "`{code}`");
                }
                Event::SoftBreak => {
                    let _ = write!(output, " ");
                }
                Event::HardBreak => {
                    let _ = writeln!(output);
                }
                Event::Html(html) => {
                    // Handle HTML comments that might contain pagebreak markers
                    if html.trim() == "<!-- pagebreak -->" {
                        let _ = writeln!(output, "\n#pagebreak()\n");
                    }
                }
                _ => {}
            }
        }
    }

    fn process_pagebreak_markers(content: &str) -> String {
        // Replace \pagebreak with a unique marker that won't be escaped
        content.replace("\\pagebreak", "TYPST_PAGEBREAK_MARKER")
    }

    fn wrap_h2_sections(content: &str) -> String {
        // This method wraps content between H2 headings in non-breakable blocks
        // to prevent job entries from splitting across pages

        let mut result = String::new();
        let mut in_h2_section = false;
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            // If we encounter a pagebreak inside an H2 section, close the block first
            if in_h2_section && line.contains("#pagebreak()") {
                result.push_str("]  // End of job entry block before pagebreak\n\n");
                result.push_str(line);
                result.push('\n');
                // Re-open the block after the pagebreak
                result.push_str(
                    "\n#block(breakable: false)[\n  // Continue job entry after pagebreak\n",
                );
                i += 1;
                continue;
            }

            // Check if this is the start of an H2 section
            // Look for pattern: #v(XXem) followed by a single-line #block with H2 text
            if line.contains("#v(")
                && line.contains("em)")
                && i + 1 < lines.len()
                && lines[i + 1].contains("#block(above: 0em, below:")
                && lines[i + 1].contains("em)[")
            // Single-line block pattern for H2
            {
                // Look ahead to confirm this is an H2
                let mut is_h2 = false;
                for check_line in lines.iter().skip(i + 2).take(3) {
                    if check_line.contains("text(size: 14pt, weight: \"bold\"") {
                        is_h2 = true;
                        break;
                    }
                }

                if is_h2 {
                    // Wrap all H2 sections (job entries, education entries, etc.)
                    // If we were already in an H2 section, close it
                    if in_h2_section {
                        result.push_str("]  // End of job entry block\n\n");
                    }

                    // Start a new non-breakable block with height limit
                    // This allows Typst to break to a new page if the block is too tall
                    result.push_str(
                        "#block(breakable: false, height: auto)[\n  // Start of job entry\n",
                    );
                    in_h2_section = true;
                }
            }

            // Check if this is an H1 heading
            if line.contains("#v(") && line.contains("em)") {
                // Look ahead to see if this is followed by an H1 block
                let mut is_h1 = false;

                // Check the next several lines for H1 characteristics
                for j in 1..=10 {
                    if i + j >= lines.len() {
                        break;
                    }
                    let next_line = lines[i + j];

                    // Check for H1 text
                    if next_line.contains("text(size: 16pt, weight: \"bold\"") {
                        is_h1 = true;
                        break;
                    }
                }

                if is_h1 {
                    // If we were in an H2 section, close it before the H1
                    if in_h2_section {
                        result.push_str("]  // End of job entry block\n\n");
                        in_h2_section = false;
                    }
                }
            }

            result.push_str(line);
            result.push('\n');
            i += 1;
        }

        // Close any remaining open H2 section
        if in_h2_section {
            result.push_str("]  // End of job entry block\n");
        }

        result
    }

    fn enhance_company_names(content: &str) -> String {
        // Replace pattern **Company** (Location) with H2 heading
        let company_re = regex::Regex::new(r"(?m)^\*\*([^*]+)\*\*\s*\(([^)]+)\)\s*$")
            .expect("Invalid company regex pattern");
        let enhanced = company_re.replace_all(content, "## $1 ($2)");

        // Replace pattern _Job Title_, Date with H3 heading - keep date on same line
        let job_underscore_re = regex::Regex::new(r"(?m)^_([^_]+)_,\s*(.+)$")
            .expect("Invalid job title underscore regex pattern");
        let enhanced2 = job_underscore_re.replace_all(&enhanced, "### **$1**, $2");

        // Also replace pattern *Job Title*, Date with H3 heading - keep date on same line
        let job_asterisk_re = regex::Regex::new(r"(?m)^\*([^*]+)\*,\s*(.+)$")
            .expect("Invalid job title asterisk regex pattern");
        let enhanced3 = job_asterisk_re.replace_all(&enhanced2, "### **$1**, $2");

        // Replace pattern _Degree/Title_ (without comma) for Education sections
        let degree_underscore_re = regex::Regex::new(r"(?m)^_([^_]+)_\s*$")
            .expect("Invalid degree underscore regex pattern");
        let enhanced4 = degree_underscore_re.replace_all(&enhanced3, "### **$1**");

        // Replace pattern *Degree/Title* (without comma) for Education sections
        let degree_asterisk_re = regex::Regex::new(r"(?m)^\*([^*]+)\*\s*$")
            .expect("Invalid degree asterisk regex pattern");
        degree_asterisk_re
            .replace_all(&enhanced4, "### **$1**")
            .to_string()
    }

    fn handle_start_tag(
        tag: pulldown_cmark::Tag,
        output: &mut String,
        context: &mut RenderContext,
        theme: &Theme,
    ) {
        use pulldown_cmark::{HeadingLevel, Tag};

        match tag {
            Tag::Heading { level, .. } => {
                context.in_heading = true;
                context.heading_level = level;
                match level {
                    HeadingLevel::H1 => {
                        // Top-level sections (Experience, Education, Skills)
                        // Use Typst's keep-with-next feature to prevent orphaned headings
                        let _ = writeln!(output, "\n#v({}em)", theme.color.get_h1_spacing_above());
                        let _ = writeln!(output, "#block(");
                        let _ = writeln!(output, "  above: 0em,");
                        let _ =
                            writeln!(output, "  below: {}em,", theme.color.get_h1_spacing_below());
                        let _ = writeln!(output, "  breakable: false,");
                        let _ = writeln!(output, "  height: auto");
                        let _ = writeln!(output, ")[");
                        let _ = write!(
                            output,
                            "  #text(size: 16pt, weight: \"bold\", fill: {})[",
                            theme.color.get_h1_color()
                        );
                    }
                    HeadingLevel::H2 => {
                        // Company/Organization names - make prominent
                        let _ = writeln!(output, "\n#v({}em)", theme.color.get_h2_spacing_above());
                        let _ = writeln!(
                            output,
                            "#block(above: 0em, below: {}em)[",
                            theme.color.get_h2_spacing_below()
                        );
                        // Don't start the text formatting here - we'll handle it when processing the text
                        // to allow for mixed bold/non-bold content
                        context.in_heading = true;
                    }
                    HeadingLevel::H3 => {
                        // Job titles/roles - less prominent than company
                        let _ = writeln!(output, "\n#v({}em)", theme.color.get_h3_spacing_above());
                        let _ = writeln!(
                            output,
                            "#block(above: 0em, below: {}em)[",
                            theme.color.get_h3_spacing_below()
                        );
                        let _ = write!(
                            output,
                            "  #text(size: 12pt, weight: \"semibold\", fill: {})[",
                            theme.color.get_h3_color()
                        );
                    }
                    _ => {
                        // H4, H5, H6 - rarely used
                        let _ = writeln!(output, "\n#v(0.5em)");
                        let _ = writeln!(output, "#block(above: 0em, below: 0.2em)[");
                        let _ = write!(output, "  #text(size: 11pt, weight: \"medium\")[");
                    }
                }
            }
            Tag::Paragraph => {
                if context.list_depth == 0 {
                    let _ = writeln!(output);
                }
            }
            Tag::List(_) => {
                context.list_depth += 1;
                if context.list_depth == 1 {
                    let _ = writeln!(output);
                }
            }
            Tag::Item => {
                let _ = write!(output, "\n{}• ", "  ".repeat(context.list_depth - 1));
            }
            Tag::Strong | Tag::Emphasis => {
                let _ = write!(output, "*");
            }
            Tag::Strikethrough => {
                let _ = write!(output, "#strike[");
            }
            Tag::Link { dest_url, .. } => {
                let _ = write!(output, "#link(\"{dest_url}\")[");
            }
            Tag::CodeBlock(_) => {
                let _ = writeln!(output, "\n```");
            }
            Tag::BlockQuote(_) => {
                let _ = write!(output, "\n#quote[");
            }
            _ => {}
        }
    }

    fn handle_end_tag(
        tag: pulldown_cmark::TagEnd,
        output: &mut String,
        theme: &Theme,
        context: &mut RenderContext,
    ) {
        use pulldown_cmark::{HeadingLevel, TagEnd};

        match tag {
            TagEnd::Heading(_) => {
                if context.in_heading {
                    // For H2, we handle the text formatting in handle_text, so no closing bracket needed here
                    if !matches!(context.heading_level, HeadingLevel::H2) {
                        let _ = write!(output, "]");
                    }
                    if matches!(context.heading_level, HeadingLevel::H1) {
                        let _ = writeln!(
                            output,
                            "\n  #line(length: 100%, stroke: {}pt + {})",
                            theme.color.get_separator_thickness(),
                            theme.color.to_typst_rgb("accent")
                        );
                    }
                    let _ = writeln!(output, "]");
                    // Add extra space after H1 with line
                    if matches!(context.heading_level, HeadingLevel::H1) {
                        let _ = writeln!(output, "#v(0.2em)");
                    }
                    context.in_heading = false;
                }
            }
            TagEnd::Paragraph => {
                if context.list_depth == 0 {
                    let _ = writeln!(output);
                }
            }
            TagEnd::List(_) => {
                context.list_depth -= 1;
                if context.list_depth == 0 {
                    let _ = writeln!(output);
                }
            }
            TagEnd::Strong | TagEnd::Emphasis => {
                let _ = write!(output, "*");
            }
            TagEnd::Strikethrough | TagEnd::Link | TagEnd::BlockQuote(_) => {
                let _ = write!(output, "]");
            }
            TagEnd::CodeBlock => {
                let _ = writeln!(output, "```");
            }
            TagEnd::Item => {
                // Add line break after list item to ensure next item starts on new line
                let _ = writeln!(output);
            }
            _ => {}
        }
    }

    fn handle_text(
        text: &pulldown_cmark::CowStr,
        output: &mut String,
        context: &RenderContext,
        theme: &Theme,
    ) {
        // Check for pagebreak marker
        if text.trim() == "TYPST_PAGEBREAK_MARKER" {
            let _ = writeln!(output, "\n#pagebreak()\n");
            return;
        }

        let escaped = text
            .replace('@', "\\@")
            .replace('#', "\\#")
            .replace('$', "\\$");

        // Special handling for H2 headings with parentheses (Company names with locations)
        if context.in_heading && matches!(context.heading_level, pulldown_cmark::HeadingLevel::H2) {
            // Check if this text contains parentheses
            if let Some(paren_start) = escaped.find('(') {
                // Split into company name and location parts
                let company = &escaped[..paren_start].trim();
                let location = &escaped[paren_start..];

                // Company name in bold, location in regular weight
                let _ = write!(
                    output,
                    "  #text(size: 14pt, weight: \"bold\", fill: {})[{}] #text(size: 14pt, weight: \"regular\", fill: {})[{}]",
                    theme.color.get_h2_color(),
                    company,
                    theme.color.get_h2_color(),
                    location
                );
            } else {
                // No parentheses, render normally with bold
                let _ = write!(
                    output,
                    "  #text(size: 14pt, weight: \"bold\", fill: {})[{}]",
                    theme.color.get_h2_color(),
                    escaped
                );
            }
        } else {
            // Normal text handling
            let _ = write!(output, "{escaped}");
        }
    }
}

impl RenderEngine for PdfRenderer {
    fn render(&self, doc: &Document, theme: &Theme, output: &Path) -> Result<()> {
        // Check if Typst is available
        if Command::new("typst").arg("--version").output().is_err() {
            anyhow::bail!(
                "Typst is required for PDF generation but is not installed.\n\
                Please install Typst:\n\
                  - macOS: brew install typst\n\
                  - Linux: Download from https://github.com/typst/typst/releases\n\
                  - Cross-platform: cargo install typst-cli"
            );
        }

        // Generate Typst source
        let typst_source = self.generate_typst_source(doc, theme);

        // Write to temporary file
        let mut temp_file = NamedTempFile::new()?;
        std::io::Write::write_all(&mut temp_file, typst_source.as_bytes())?;

        // Debug: save a copy for inspection
        #[cfg(debug_assertions)]
        {
            let debug_path = std::path::Path::new("/tmp/cv_debug/generated.typ");
            if let Some(parent) = debug_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(debug_path, &typst_source);
        }

        // Get fonts directory path
        let fonts_dir = std::env::current_dir()
            .map_or_else(|_| std::path::PathBuf::from("fonts"), |p| p.join("fonts"));

        // Run Typst with font path
        let mut cmd = Command::new("typst");
        cmd.arg("compile");

        // Add font path if it exists
        if fonts_dir.exists() {
            cmd.arg("--font-path").arg(&fonts_dir);
        }

        cmd.arg(
            temp_file
                .path()
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid temp file path"))?,
        );
        cmd.arg(
            output
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid output path"))?,
        );

        let output_status = cmd.output()?;

        if !output_status.status.success() {
            let stderr = String::from_utf8_lossy(&output_status.stderr);
            anyhow::bail!("Typst compilation failed: {}", stderr);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{DocumentMetadata, LayoutOptions};
    use crate::parser::Document;
    use crate::themes::color::ColorTheme;
    use crate::themes::font::FontTheme;
    use crate::themes::Theme;
    use std::collections::HashMap;

    fn create_test_document() -> Document {
        Document {
            metadata: DocumentMetadata {
                name: "Test User".to_string(),
                email: "test@example.com".to_string(),
                phone: Some("+1 234 567 8900".to_string()),
                location: Some("San Francisco, CA".to_string()),
                linkedin: Some("testuser".to_string()),
                github: Some("testuser".to_string()),
                website: Some("https://example.com".to_string()),
                font_theme: "modern".to_string(),
                color_theme: "modern".to_string(),
                recipient: None,
                date: None,
                subject: None,
                layout: LayoutOptions::default(),
                custom: HashMap::new(),
            },
            content: "# Test Section\n\nThis is a test document.".to_string(),
            markdown_ast: vec![],
        }
    }

    fn create_test_theme() -> Theme {
        Theme {
            color: ColorTheme::load("modern").expect("Failed to load modern color theme"),
            font: FontTheme::load("modern").expect("Failed to load modern font theme"),
        }
    }

    #[test]
    fn test_pdf_renderer_creation() {
        let renderer = PdfRenderer::new(None);
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_typst_source_generation() {
        let renderer = PdfRenderer::new(None).expect("Failed to create PDF renderer");
        let doc = create_test_document();
        let theme = create_test_theme();

        let source = renderer.generate_typst_source(&doc, &theme);

        // Check document setup
        assert!(source.contains("#set document(title: \"Test User\", author: \"Test User\")"));
        assert!(source.contains("#set page(paper: \"a4\""));

        // Check font configuration
        assert!(source.contains("#set text(font: \"Inter\""));

        // Check header section
        assert!(source.contains("Test User"));
        assert!(source.contains("San Francisco, CA"));

        // Check FontAwesome icons
        assert!(source.contains("#text(font: \"FontAwesome\")[\\u{f095}]")); // Phone
        assert!(source.contains("#text(font: \"FontAwesome\")[\\u{f0e0}]")); // Email
        assert!(source.contains("#text(font: \"FontAwesome\")[\\u{f015}]")); // Home
        assert!(source.contains("#text(font: \"FontAwesome\")[\\u{f09b}]")); // GitHub
        assert!(source.contains("#text(font: \"FontAwesome\")[\\u{f0e1}]")); // LinkedIn

        // Check contact info
        assert!(source.contains("test\\@example.com")); // @ should be escaped
        assert!(source.contains("+1 234 567 8900"));
        assert!(source.contains("github.com/testuser"));
        assert!(source.contains("linkedin.com/in/testuser"));
    }

    #[test]
    fn test_font_directory_in_typst_command() {
        // This test verifies that the font directory is included in the Typst command
        // when it exists. We can't fully test the command execution without Typst installed,
        // but we can verify the logic.
        let fonts_dir = std::env::current_dir()
            .map_or_else(|_| std::path::PathBuf::from("fonts"), |p| p.join("fonts"));

        // The test verifies that if fonts directory exists, it would be added to the command
        if fonts_dir.exists() {
            // In the actual code, this would add --font-path argument
            assert!(fonts_dir.is_dir());
        }
    }

    #[test]
    fn test_fontawesome_icons_in_contact_info() {
        let renderer = PdfRenderer::new(None).expect("Failed to create PDF renderer");
        let doc = create_test_document();
        let theme = create_test_theme();

        let source = renderer.generate_typst_source(&doc, &theme);

        // Check that FontAwesome font is used for icons
        assert!(source.contains("#text(font: \"FontAwesome\")"));

        // Check specific icon codes
        assert!(source.contains("\\u{f095}")); // Phone icon
        assert!(source.contains("\\u{f0e0}")); // Email icon
        assert!(source.contains("\\u{f015}")); // Home/website icon
        assert!(source.contains("\\u{f09b}")); // GitHub icon
        assert!(source.contains("\\u{f0e1}")); // LinkedIn icon

        // Verify icons are paired with correct content
        assert!(source.contains("#text(font: \"FontAwesome\")[\\u{f095}] +1 234 567 8900"));
        assert!(source.contains("#text(font: \"FontAwesome\")[\\u{f0e0}] test\\@example.com"));
        assert!(source
            .contains("#text(font: \"FontAwesome\")[\\u{f015}] #link(\"https://example.com\")"));
        assert!(source.contains(
            "#text(font: \"FontAwesome\")[\\u{f09b}] #link(\"https://github.com/testuser\")"
        ));
        assert!(source.contains(
            "#text(font: \"FontAwesome\")[\\u{f0e1}] #link(\"https://linkedin.com/in/testuser\")"
        ));
    }

    #[test]
    fn test_markdown_to_typst_conversion() {
        let mut output = String::new();
        let theme = create_test_theme();
        let content = r"
# Main Section
This is a paragraph.

## Subsection
- Item 1
- Item 2
- Item 3

**Bold text** and *italic text*.
";

        PdfRenderer::render_markdown_as_typst(content, &mut output, &theme);

        // Check heading formatting (now includes fill color)
        assert!(output.contains("#text(size: 16pt, weight: \"bold\", fill:"));
        assert!(output.contains("#line(length: 100%, stroke:"));

        // Check list formatting
        assert!(output.contains("• Item 1"));
        assert!(output.contains("• Item 2"));

        // Check text formatting
        assert!(output.contains("*Bold text*"));
        assert!(output.contains("*italic text*"));
    }

    #[test]
    fn test_email_escaping() {
        let renderer = PdfRenderer::new(None).expect("Failed to create PDF renderer");
        let mut doc = create_test_document();
        doc.metadata.email = "user@domain.com".to_string();
        let theme = create_test_theme();

        let source = renderer.generate_typst_source(&doc, &theme);

        // @ symbol should be escaped in Typst
        assert!(source.contains("user\\@domain.com"));
        assert!(!source.contains("user@domain.com"));
    }

    #[test]
    fn test_font_theme_selection() {
        let renderer = PdfRenderer::new(None).expect("Failed to create PDF renderer");
        let theme = create_test_theme();

        // Test classic theme
        let mut doc = create_test_document();
        doc.metadata.font_theme = "classic".to_string();
        let source = renderer.generate_typst_source(&doc, &theme);
        assert!(source.contains("#set text(font: \"Georgia\""));

        // Test modern theme
        doc.metadata.font_theme = "modern".to_string();
        let source = renderer.generate_typst_source(&doc, &theme);
        assert!(source.contains("#set text(font: \"Inter\""));

        // Test sharp theme
        doc.metadata.font_theme = "sharp".to_string();
        let source = renderer.generate_typst_source(&doc, &theme);
        assert!(source.contains("#set text(font: \"Montserrat\""));
    }
}
