use crate::config::GlobalConfig;
use crate::parser::Document;
use crate::render::Renderer;
use crate::themes::Theme;
use anyhow::Result;
use colored::Colorize;
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

        if options.verbose {
            println!("Output: {}", output_path.display());
        }

        // Auto-open if configured
        if self.config.auto_open.unwrap_or(true) && !options.quiet {
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
        println!("{}", "Preview server not yet implemented".yellow());
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
