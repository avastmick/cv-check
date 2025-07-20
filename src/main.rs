use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;

mod cli;
mod config;
mod error;
mod parser;
mod render;
mod themes;

use crate::cli::{BuildOptions, CvGenerator};

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
        #[arg(short, long, default_value = "modern")]
        font_theme: String,

        /// Color theme (classic, modern, sharp)
        #[arg(short, long, default_value = "modern")]
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
}

#[derive(clap::ValueEnum, Clone)]
enum NewDocType {
    Cv,
    Letter,
}

fn main() -> Result<()> {
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
                println!("{}", "Building document...".blue());
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
                println!("{}", "✓ Document generated successfully!".green());
            }
        }

        Commands::New { doc_type, output } => {
            match doc_type {
                NewDocType::Cv => CvGenerator::new_cv(&output)?,
                NewDocType::Letter => CvGenerator::new_letter(&output)?,
            }
            println!("{} Created {} template", "✓".green(), output.display());
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
            println!("{} {} is valid!", "✓".green(), input.display());
        }

        Commands::Serve { input, port } => {
            println!("{} Preview server at http://localhost:{}", "→".blue(), port);
            CvGenerator::serve(&input, port);
        }
    }

    Ok(())
}
