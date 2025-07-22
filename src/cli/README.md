# CLI Module

This module handles all command-line interface functionality for the CV Check tool.

## Purpose

Provides a high-level API for building CVs from markdown files, managing themes, and handling various output formats. Acts as the orchestration layer between user commands and the core functionality.

## Key Components

### `BuildOptions`
Configuration struct for the build command containing:
- Input file path
- Theme selections (font and color)
- Output path and format
- Template override option
- Verbosity settings

### `CvGenerator`
Main class that orchestrates the CV generation process:
- Loads global configuration
- Parses input documents
- Applies themes
- Invokes appropriate renderer
- Handles progress feedback

## Public API

```rust
// Create a new generator instance
let generator = CvGenerator::new()?;

// Build a CV with options
let options = BuildOptions {
    input: Path::new("cv.md"),
    font_theme: "modern",
    color_theme: "sharp",
    output: Some(Path::new("output.pdf")),
    format: "pdf",
    template: None,
    verbose: false,
    quiet: false,
};
generator.build(&options)?;

// List available themes
generator.list_themes();

// Create a new template
generator.new_template("cv")?;

// Check/validate a document
generator.check(Path::new("cv.md"))?;
```

## Dependencies

- `config`: For loading global and document configuration
- `parser`: For parsing markdown documents
- `render`: For generating output files
- `themes`: For theme management
- `colored`: For terminal output coloring
- `anyhow`: For error handling

## Error Handling

Uses `anyhow::Result` for all public methods, allowing rich error context. Errors are propagated from underlying modules with additional context added at this layer.

## Available Commands

The CLI module supports these commands:

1. **`build`** - Generate CV/letter from markdown
2. **`new`** - Create new CV or letter from template
3. **`themes`** - List available font and color themes
4. **`check`** - Validate markdown structure
5. **`serve`** - Start preview server (planned)
6. **`tailor`** - AI-powered CV tailoring to job descriptions

### Tailor Command

The `tailor` command uses AI to optimize a CV for specific job descriptions:
- Extracts text from PDF job descriptions
- Sends CV + job description to AI model
- Generates tailored CV with optimized keywords and content
- Outputs in multiple formats (PDF, DOCX, HTML, Markdown)

## Future Enhancements

- [ ] Watch mode for auto-rebuilding
- [ ] Batch processing of multiple files
- [ ] Theme preview/demo generation
- [ ] Interactive theme selection
- [ ] Complete serve command implementation
