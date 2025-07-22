pub mod display;

use crate::ai::{extract_text_from_pdf, AIClient};
use crate::config::GlobalConfig;
use crate::parser::Document;
use crate::render::Renderer;
use crate::themes::Theme;
use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use std::fmt::Write;
use std::path::{Path, PathBuf};

pub struct BuildOptions<'a> {
    pub input: &'a Path,
    pub font_theme: &'a str,
    pub color_theme: &'a str,
    pub output: Option<&'a Path>,
    pub format: &'a str,
    pub template: Option<&'a Path>,
    pub verbose: bool,
    pub quiet: bool,
}

pub struct TailorOptions<'a> {
    pub cv_path: &'a Path,
    pub job_description_path: &'a Path,
    pub output: Option<&'a Path>,
    pub font_theme: &'a str,
    pub color_theme: &'a str,
    pub format: &'a str,
    pub verbose: bool,
    pub quiet: bool,
}

pub struct CvGenerator {
    config: GlobalConfig,
}

impl CvGenerator {
    /// Creates a new CV generator instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the global configuration cannot be loaded.
    pub fn new() -> Result<Self> {
        let config = GlobalConfig::load()?;
        Ok(Self { config })
    }

    /// Builds a CV or cover letter from markdown input.
    ///
    /// # Errors
    ///
    /// Returns an error if document parsing, theme loading, or rendering fails.
    pub fn build(&self, options: &BuildOptions) -> Result<()> {
        // Parse document
        let doc = Document::from_file(options.input)?;
        doc.validate()?;

        // Load theme
        let theme = Theme::new(options.font_theme, options.color_theme)?;

        // Determine output path
        let output_path = if let Some(path) = options.output {
            path.to_path_buf()
        } else {
            let stem = options
                .input
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            let ext = match options.format {
                "pdf" | "docx" | "html" => options.format,
                _ => "pdf",
            };
            PathBuf::from(format!("{stem}.{ext}"))
        };

        // Create output directory if needed
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Render document
        let renderer = Renderer::new(options.format, options.template)?;
        renderer.render(&doc, &theme, &output_path)?;

        if !options.quiet {
            println!("{} Output: {}", "→".blue(), output_path.display());
        }

        if options.verbose && !options.quiet {
            println!("  Font theme: {}", options.font_theme);
            println!("  Color theme: {}", options.color_theme);
            println!("  Format: {}", options.format);
            println!("  Auto-open: {}", self.config.auto_open.unwrap_or(true));
        }

        info!("Output path: {}", output_path.display());

        // Auto-open if configured
        // Check for CI environment variable to disable auto-open in tests
        let ci_mode = std::env::var("CI").is_ok() || std::env::var("CV_CHECK_NO_OPEN").is_ok();
        if self.config.auto_open.unwrap_or(true) && !options.quiet && !ci_mode {
            Self::open_file(&output_path)?;
        }

        Ok(())
    }

    /// Creates a new CV template file.
    ///
    /// # Errors
    ///
    /// Returns an error if the template file cannot be written.
    pub fn new_cv(output: &Path) -> Result<()> {
        let template = include_str!("../templates/cv_template.md");
        std::fs::write(output, template)?;
        Ok(())
    }

    /// Creates a new cover letter template file.
    ///
    /// # Errors
    ///
    /// Returns an error if the template file cannot be written.
    pub fn new_letter(output: &Path) -> Result<()> {
        let template = include_str!("../templates/letter_template.md");
        std::fs::write(output, template)?;
        Ok(())
    }

    /// Lists available font and/or color themes.
    pub fn list_themes(fonts: bool, colors: bool) {
        let (font_themes, color_themes) = Theme::available_themes();

        if fonts {
            println!("{}", "Font Themes:".bold());
            for theme in font_themes {
                println!(
                    "  • {} - {}",
                    theme.cyan(),
                    match theme {
                        "classic" => "Traditional serif fonts (Georgia/Times)",
                        "modern" => "Clean sans-serif (Inter/Open Sans)",
                        "sharp" => "Bold geometric (Montserrat/Roboto)",
                        _ => "Unknown theme",
                    }
                );
            }
            if colors {
                println!();
            }
        }

        if colors {
            println!("{}", "Color Themes:".bold());
            for theme in color_themes {
                println!(
                    "  • {} - {}",
                    theme.cyan(),
                    match theme {
                        "classic" => "Navy and burgundy (traditional)",
                        "modern" => "Blue and teal (tech)",
                        "sharp" => "Purple and pink (creative)",
                        _ => "Unknown theme",
                    }
                );
            }
        }
    }

    /// Validates the structure and content of a markdown document.
    ///
    /// # Errors
    ///
    /// Returns an error if the document cannot be parsed or is invalid.
    pub fn check(input: &Path) -> Result<()> {
        let doc = Document::from_file(input)?;
        doc.validate()?;
        Ok(())
    }

    /// Starts a preview server for the document (not yet implemented).
    pub fn serve(_input: &Path, _port: u16) {
        // TODO: Implement preview server
        warn!("{}", "Preview server not yet implemented".yellow());
    }

    /// Generates the frontmatter for a tailored CV.
    fn generate_frontmatter(original_doc: &Document, options: &TailorOptions) -> Result<String> {
        let mut frontmatter = String::from("---\n");

        writeln!(&mut frontmatter, "name: {}", original_doc.metadata.name)?;
        writeln!(&mut frontmatter, "email: {}", original_doc.metadata.email)?;

        if let Some(phone) = &original_doc.metadata.phone {
            writeln!(&mut frontmatter, "phone: {phone}")?;
        }
        if let Some(location) = &original_doc.metadata.location {
            writeln!(&mut frontmatter, "location: {location}")?;
        }
        if let Some(linkedin) = &original_doc.metadata.linkedin {
            writeln!(&mut frontmatter, "linkedin: {linkedin}")?;
        }
        if let Some(github) = &original_doc.metadata.github {
            writeln!(&mut frontmatter, "github: {github}")?;
        }
        if let Some(website) = &original_doc.metadata.website {
            writeln!(&mut frontmatter, "website: {website}")?;
        }

        writeln!(&mut frontmatter, "\n# AI-Tailored CV")?;
        writeln!(
            &mut frontmatter,
            "# Original: {}",
            options.cv_path.display()
        )?;
        writeln!(
            &mut frontmatter,
            "# Job: {}",
            options.job_description_path.display()
        )?;
        frontmatter.push_str("---\n\n");

        Ok(frontmatter)
    }

    /// Generates the content sections for a tailored CV.
    fn generate_tailored_content(tailored_cv: &crate::ai::schemas::TailoredCV) -> Result<String> {
        let mut content = String::new();

        // Add professional summary
        content.push_str("## Professional Summary\n\n");
        content.push_str(&tailored_cv.professional_summary);
        content.push_str("\n\n");

        // Add experiences
        content.push_str("## Experience\n\n");
        for exp in &tailored_cv.experiences {
            writeln!(&mut content, "### {} at {}", exp.title, exp.company)?;
            writeln!(&mut content, "*{}*\n", exp.duration)?;
            for highlight in &exp.highlights {
                writeln!(&mut content, "- {highlight}")?;
            }
            // Always include relevance score as a comment
            writeln!(
                &mut content,
                "\n<!-- Relevance Score: {:.2} -->",
                exp.relevance_score
            )?;
            content.push('\n');
        }

        // Add skills
        content.push_str("## Skills\n\n");
        content.push_str(&tailored_cv.skills.join(", "));
        content.push_str("\n\n");

        // Add keywords for ATS
        content.push_str("<!-- ATS Keywords: ");
        content.push_str(&tailored_cv.keywords.join(", "));
        content.push_str(" -->\n\n");

        // Add suggestions as comments
        if !tailored_cv.suggestions.is_empty() {
            content.push_str("<!-- AI Suggestions:\n");
            for suggestion in &tailored_cv.suggestions {
                writeln!(&mut content, "- {suggestion}")?;
            }
            content.push_str("-->\n");
        }

        Ok(content)
    }

    /// Tailors a CV for a specific job description using AI.
    ///
    /// # Errors
    ///
    /// Returns an error if CV parsing, PDF extraction, AI processing, or rendering fails.
    ///
    /// # Panics
    ///
    /// Panics if the progress bar style template is invalid.
    pub async fn tailor(&self, options: &TailorOptions<'_>) -> Result<()> {
        if !options.quiet {
            println!("{} Reading CV and job description...", "→".blue());
            println!("  CV: {}", options.cv_path.display().to_string().dimmed());
            println!(
                "  JD: {}",
                options.job_description_path.display().to_string().dimmed()
            );
        }

        info!("CV path: {}", options.cv_path.display());
        info!(
            "Job description path: {}",
            options.job_description_path.display()
        );

        // Read the CV markdown file
        let cv_content = std::fs::read_to_string(options.cv_path)?;

        // Parse the original CV to extract metadata
        let original_doc = Document::from_file(options.cv_path)?;

        // Extract text from the job description PDF
        let job_description = extract_text_from_pdf(options.job_description_path)?;

        if !options.quiet {
            println!("{} Connecting to AI service...", "→".blue());
        }

        // Create AI client from environment
        let mut ai_client = AIClient::from_env()
            .map_err(|e| anyhow::anyhow!("Failed to create AI client: {e}. Make sure AI_ENDPOINT, AI_API_KEY, and AI_MODEL are set."))?;

        if !options.quiet {
            let endpoint = std::env::var("AI_ENDPOINT").unwrap_or_else(|_| "not set".to_string());
            let model = &ai_client.model;
            println!("  Provider: {}", endpoint.dimmed());
            println!("  Model: {}", model.dimmed());
        }

        // Create a progress spinner for AI processing
        let spinner = if options.quiet {
            None
        } else {
            println!(); // Add blank line for spacing
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.blue} {msg}")
                    .expect("Failed to set progress style")
                    .tick_chars("⣷⣯⣟⡿⢿⣻⣽⣾"),
            );
            pb.set_message(
                "Analyzing job requirements and tailoring CV (this may take a moment)...",
            );
            pb.enable_steady_tick(std::time::Duration::from_millis(80));
            Some(pb)
        };

        // Get tailored CV content
        let tailored_cv = ai_client.tailor_cv(&cv_content, &job_description).await?;

        // Stop the spinner
        if let Some(pb) = spinner {
            pb.finish_and_clear();
        }

        // Display suggestions to the user
        if !options.quiet && !tailored_cv.suggestions.is_empty() {
            display::show_suggestions(&tailored_cv.suggestions);
        }

        // Generate the tailored markdown
        let frontmatter = Self::generate_frontmatter(&original_doc, options)?;
        let content = Self::generate_tailored_content(&tailored_cv)?;
        let tailored_markdown = frontmatter + &content;

        // Determine output path
        let output_path = if let Some(path) = options.output {
            path.to_path_buf()
        } else {
            let stem = options
                .cv_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("cv");
            PathBuf::from(format!("{stem}_tailored.md"))
        };

        // Save the tailored markdown
        std::fs::write(&output_path, &tailored_markdown)?;

        if !options.quiet {
            println!(
                "{} Tailored CV saved to: {}",
                "✓".green(),
                output_path.display()
            );
        }

        // If format is not markdown, generate the final document
        if options.format != "md" {
            if !options.quiet {
                println!("{} Generating {} output...", "→".blue(), options.format);
            }

            // Create the output path for the final format
            let final_output_path = output_path.with_extension(options.format);

            let build_options = BuildOptions {
                input: &output_path,
                font_theme: options.font_theme,
                color_theme: options.color_theme,
                output: Some(&final_output_path),
                format: options.format,
                template: None,
                verbose: options.verbose,
                quiet: options.quiet,
            };

            self.build(&build_options)?;
        }

        Ok(())
    }

    /// Opens a file using the system default application.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened.
    fn open_file(path: &Path) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open").arg(path).spawn()?;
        }

        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open").arg(path).spawn()?;
        }

        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/C", "start", "", path.to_str().unwrap_or("")])
                .spawn()?;
        }

        Ok(())
    }
}
