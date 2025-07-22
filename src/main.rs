use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use log::{error, info};
use std::fmt::Display;
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

/// Initialize the logger based on the command's verbose flag
fn init_logger(command: &Commands) {
    let default_filter = match command {
        Commands::Build { verbose: true, .. } | Commands::Tailor { verbose: true, .. } => "info",
        _ => "warn",
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(default_filter))
        .init();
    info!("Logger set up.");
}

/// Output a message to the user (respecting quiet mode)
fn output_user_message<T: Display>(message: T, quiet: bool) {
    if !quiet {
        println!("{message}");
    }
}

/// Handle the build command
fn handle_build(generator: &CvGenerator, options: &BuildOptions<'_>) -> Result<()> {
    output_user_message(
        format!("{} Building document...", "→".blue()),
        options.quiet,
    );

    if !options.quiet {
        output_user_message(
            format!("  Input: {}", options.input.display().to_string().dimmed()),
            options.quiet,
        );
    }

    info!(
        "Building {} from {}",
        options.format,
        options.input.display()
    );

    generator.build(options)?;

    output_user_message(
        format!("{} Document generated successfully!", "✓".green()),
        options.quiet,
    );

    Ok(())
}

/// Handle the tailor command
async fn handle_tailor(generator: &CvGenerator, options: &TailorOptions<'_>) -> Result<()> {
    output_user_message(
        format!("{} Tailoring CV to job description using AI...", "→".blue()),
        options.quiet,
    );

    info!(
        "Using AI endpoint: {}",
        std::env::var("AI_ENDPOINT").unwrap_or_else(|_| "not set".to_string())
    );
    info!(
        "Using AI model: {}",
        std::env::var("AI_MODEL").unwrap_or_else(|_| "not set".to_string())
    );

    if let Err(e) = generator.tailor(options).await {
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

    output_user_message(
        format!("{} CV successfully tailored!", "✓".green()),
        options.quiet,
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logger based on verbose flag
    init_logger(&cli.command);

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
        } => handle_build(
            &generator,
            &BuildOptions {
                input: &input,
                font_theme: &font_theme,
                color_theme: &color_theme,
                output: output.as_deref(),
                format: &format,
                template: template.as_deref(),
                verbose,
                quiet,
            },
        )?,

        Commands::New { doc_type, output } => {
            match doc_type {
                NewDocType::Cv => CvGenerator::new_cv(&output)?,
                NewDocType::Letter => CvGenerator::new_letter(&output)?,
            }
            // Show user message
            println!("{} Created {} template", "✓".green(), output.display());
            // Log separately
            info!("Created {} template", output.display());
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
            println!("{} Checking document structure...", "→".blue());
            println!("  Input: {}", input.display().to_string().dimmed());

            CvGenerator::check(&input)?;

            // Show user message
            println!("{} {} is valid!", "✓".green(), input.display());
            // Log separately
            info!("{} is valid!", input.display());
        }

        Commands::Serve { input, port } => {
            // Show user message
            println!("{} Starting preview server...", "→".blue());
            println!("  Input: {}", input.display().to_string().dimmed());
            println!("  Server: http://localhost:{}", port.to_string().dimmed());
            // Log separately
            info!("Preview server at http://localhost:{port}");
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
            handle_tailor(
                &generator,
                &TailorOptions {
                    cv_path: &cv,
                    job_description_path: &job_description,
                    output: output.as_deref(),
                    font_theme: &font_theme,
                    color_theme: &color_theme,
                    format: &format,
                    verbose,
                    quiet,
                },
            )
            .await?;
        }
    }

    Ok(())
}
