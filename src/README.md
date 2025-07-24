# CV Check Source Code Overview

This directory contains the core Rust implementation of the CV Check CLI tool for generating professional CVs and cover letters from Markdown files.

## Module Structure

### Core Modules

- **`main.rs`** - CLI entry point and command routing
  - Handles argument parsing with Clap
  - Routes commands to appropriate handlers
  - Manages terminal output and error display

- **`lib.rs`** - Library public API exports
  - Re-exports all public modules for external usage
  - Minimal file that just exposes the internal modules

- **`config.rs`** - Configuration and metadata structures
  - `DocumentMetadata` - YAML frontmatter structure
  - `LayoutOptions` - Document layout configuration
  - `RecipientInfo` - Cover letter recipient details
  - Handles global configuration loading

- **`constants.rs`** - Shared constants across the application
  - Available theme names
  - Default theme configuration
  - Standard font sizes
  - Markdown parser options

- **`constants/icons.rs`** - FontAwesome icon constants
  - Icon unicode values for common symbols
  - Font name constant
  - Used throughout PDF rendering for consistent icons

- **`error.rs`** - Error types and handling
  - `CvError` enum - All possible application errors
  - Uses `thiserror` for ergonomic error definitions
  - Provides user-friendly error messages

### Feature Modules

- **`cli/`** - Command-line interface implementation
  - Command handling and options parsing
  - CV generator orchestration
  - User interaction and feedback

- **`parser/`** - Input file parsing
  - YAML frontmatter extraction
  - Markdown content parsing
  - Document validation

- **`render/`** - Output generation engines
  - PDF generation via Typst
  - DOCX document creation
  - HTML output with styling

- **`themes/`** - Visual customization system
  - Font theme definitions
  - Color palette management
  - Theme loading and validation

- **`templates/`** - Built-in document templates
  - CV starter template
  - Cover letter template

- **`ai/`** - AI-powered CV tailoring
  - OpenAI-compatible API client
  - PDF text extraction from job descriptions
  - Structured JSON output schemas
  - Prompt engineering for HR expertise

## Module Dependencies

```
main.rs
  └─> cli/ (command handling)
      └─> parser/ (document parsing)
      └─> render/ (output generation)
      └─> themes/ (styling)
      └─> ai/ (CV tailoring)
      └─> config.rs (metadata structures)
      └─> constants.rs (shared constants)
      └─> error.rs (error types)

lib.rs (exports all public modules)
```

## Key Design Patterns

1. **Trait-Based Rendering**: The `render/` module uses a `RenderEngine` trait to allow multiple output formats
2. **Error Propagation**: Uses `anyhow` for application errors and `thiserror` for library errors
3. **Builder Pattern**: Document construction through metadata and content separation
4. **Strategy Pattern**: Theme system allows swapping visual styles at runtime

## Public API Surface

The library exposes these main types through `lib.rs`:
- Document parsing via `parser::Document`
- Theme management via `themes::Theme`
- Rendering via `render::Renderer`
- Configuration via `config::DocumentMetadata`
- Error handling via `error::CvError`

## Adding New Features

- **New Output Format**: Add a new module in `render/` implementing `RenderEngine`
- **New Theme**: Add theme definitions in `themes/font.rs` or `themes/color.rs`
- **New Command**: Update `cli/mod.rs` and `main.rs` command routing
- **New Metadata Field**: Update `config::DocumentMetadata` structure
