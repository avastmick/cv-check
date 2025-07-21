use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use log::{error, info};
use std::path::PathBuf;

mod ai;
mod cli;
mod config;
mod constants;
mod error;
mod parser;
mod render;
mod themes;

use crate::cli::{BuildOptions, CvGenerator, TailorOptions};

/// Modern CV and cover letter generator with themeable output
#[derive(Parser)]
#[command(name = "cv")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate PDF/DOCX from markdown
    Build {
        /// Input markdown file
        input: PathBuf,

        /// Font theme (classic, modern, sharp)
        #[arg(short, long, default_value = crate::constants::DEFAULT_THEME)]
        font_theme: String,

        /// Color theme (classic, modern, sharp)
        #[arg(short, long, default_value = crate::constants::DEFAULT_THEME)]
        color_theme: String,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output format (pdf, docx, html)
        #[arg(short = 'F', long, default_value = "pdf")]
        format: String,

        /// Custom template path
        #[arg(short, long)]
        template: Option<PathBuf>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Suppress output
        #[arg(short, long)]
        quiet: bool,
    },

    /// Create from template
    New {
        /// Type (cv or letter)
        #[arg(value_enum)]
        doc_type: NewDocType,

        /// Output file path
        #[arg(default_value = "cv.md")]
        output: PathBuf,
    },

    /// List/preview themes
    Themes {
        /// Show font themes
        #[arg(long)]
        fonts: bool,

        /// Show color themes
        #[arg(long)]
        colors: bool,
    },

    /// Validate markdown structure
    Check {
        /// Input markdown file
        input: PathBuf,
    },

    /// Start preview server
    Serve {
        /// Input markdown file
        input: PathBuf,

        /// Port to serve on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },

    /// Tailor CV for a specific job description using AI
    Tailor {
        /// Input CV markdown file (.md)
        #[arg(short, long)]
        cv: PathBuf,

        /// Job description PDF file (.pdf)
        #[arg(short, long)]
        job_description: PathBuf,

        /// Output markdown file for tailored CV
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Font theme (classic, modern, sharp)
        #[arg(short, long, default_value = crate::constants::DEFAULT_THEME)]
        font_theme: String,

        /// Color theme (classic, modern, sharp)
        #[arg(short = 'C', long, default_value = crate::constants::DEFAULT_THEME)]
        color_theme: String,

        /// Output format (pdf, docx, html, md)
        #[arg(short = 'F', long, default_value = "pdf")]
        format: String,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Suppress output
        #[arg(short, long)]
        quiet: bool,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum NewDocType {
    Cv,
    Letter,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger - will use RUST_LOG environment variable
    env_logger::init();

    let cli = Cli::parse();

    let generator = CvGenerator::new()?;

    match cli.command {
        Commands::Build {
            input,
            font_theme,
            color_theme,
            output,
            format,
            template,
            verbose,
            quiet,
        } => {
            if verbose {
                info!("{}", "Building document...".blue());
            }

            generator.build(&BuildOptions {
                input: &input,
                font_theme: &font_theme,
                color_theme: &color_theme,
                output: output.as_deref(),
                format: &format,
                template: template.as_deref(),
                verbose,
                quiet,
            })?;

            if !quiet {
                info!("{}", "✓ Document generated successfully!".green());
            }
        }

        Commands::New { doc_type, output } => {
            match doc_type {
                NewDocType::Cv => CvGenerator::new_cv(&output)?,
                NewDocType::Letter => CvGenerator::new_letter(&output)?,
            }
            info!("{} Created {} template", "✓".green(), output.display());
        }

        Commands::Themes { fonts, colors } => {
            if !fonts && !colors {
                // Show both if neither specified
                CvGenerator::list_themes(true, true);
            } else {
                CvGenerator::list_themes(fonts, colors);
            }
        }

        Commands::Check { input } => {
            CvGenerator::check(&input)?;
            info!("{} {} is valid!", "✓".green(), input.display());
        }

        Commands::Serve { input, port } => {
            info!("{} Preview server at http://localhost:{}", "→".blue(), port);
            CvGenerator::serve(&input, port);
        }

        Commands::Tailor {
            cv,
            job_description,
            output,
            font_theme,
            color_theme,
            format,
            verbose,
            quiet,
        } => {
            if !quiet {
                info!("{} Tailoring CV to job description using AI...", "→".blue());
            }

            if let Err(e) = generator
                .tailor(&TailorOptions {
                    cv_path: &cv,
                    job_description_path: &job_description,
                    output: output.as_deref(),
                    font_theme: &font_theme,
                    color_theme: &color_theme,
                    format: &format,
                    verbose,
                    quiet,
                })
                .await
            {
                error!("Error during CV tailoring: {e}");

                // Check if it's a JSON parsing error
                if let Some(source) = e.source() {
                    error!("Caused by: {source}");
                    if let Some(inner) = source.source() {
                        error!("Inner error: {inner}");
                    }
                }

                return Err(e);
            }

            if !quiet {
                info!("{} CV successfully tailored!", "✓".green());
            }
        }
    }

    Ok(())
}
