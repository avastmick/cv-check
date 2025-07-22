# CV Generator - Architecture

## Overview

A Rust-based command-line tool that converts Markdown files with YAML frontmatter into professionally typeset CVs and cover letters. The system generates PDF (via Typst), DOCX, and HTML outputs with configurable themes. The tool includes AI-powered CV tailoring to optimize content for specific job descriptions.

## Core Principles

1. **Simplicity**: Plain Markdown input with YAML frontmatter
2. **Performance**: Sub-second PDF generation with Typst
3. **Quality**: Zero unsafe code, comprehensive error handling
4. **Extensibility**: Theme system for fonts and colors
5. **Portability**: Single binary with embedded assets
6. **Intelligence**: AI-powered optimization using structured outputs
7. **Reliability**: Deterministic AI responses via JSON schemas

## System Architecture

### Components

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   CLI (clap)   │────▶│     Parser      │────▶│     Render      │
│                 │     │  (pulldown-    │     │   (pdf/docx/    │
│  Commands:      │     │   cmark +      │     │     html)       │
│  - build        │     │   serde_yaml)  │     │                 │
│  - watch        │     │                │     │                 │
│  - new          │     └─────────────────┘     └─────────────────┘
│  - check        │              │                       │
│  - themes       │              ▼                       ▼
│  - tailor       │     ┌─────────────────┐     ┌─────────────────┐
└─────────────────┘     │     Config      │     │     Themes      │
         │              │   Management    │     │   (embedded)    │
         │              └─────────────────┘     └─────────────────┘
         ▼
┌─────────────────┐     ┌─────────────────┐
│   File Watch    │     │   AI Module     │
│   (notify)      │     │                 │
└─────────────────┘     │                 │
                        │  - PDF Parser   │
                        │  - LLM Client   │
                        │  - Prompts      │
                        │  - Schemas      │
                        └─────────────────┘
```

### Data Flow

#### Standard Flow (build command)
1. **Input**: Markdown file with YAML frontmatter
2. **Parsing**: Extract metadata and content sections
3. **Validation**: Check required fields and structure
4. **Theme Application**: Apply font and color themes
5. **Rendering**: Generate output in requested format
6. **Output**: PDF/DOCX/HTML files

#### AI-Powered Flow (tailor command)
1. **Inputs**: Base CV markdown + Job description PDF
2. **PDF Extraction**: Extract text from job description
3. **AI Processing**:
   - Send CV + JD to LLM with HR expertise prompt
   - Receive structured JSON response
   - Validate against predefined schema
4. **CV Optimization**: Apply AI suggestions to CV content
5. **Standard Flow**: Continue with steps 2-6 above

## Module Structure

```rust
cv_check/
├── src/
│   ├── main.rs           // Entry point
│   ├── lib.rs            // Library exports
│   ├── cli/
│   │   └── mod.rs        // CLI commands and args
│   ├── parser/
│   │   ├── mod.rs        // Parser interface
│   │   ├── markdown.rs   // Markdown parsing
│   │   └── frontmatter.rs // YAML extraction
│   ├── render/
│   │   ├── mod.rs        // Renderer trait
│   │   ├── pdf.rs        // Typst PDF generation
│   │   ├── docx.rs       // DOCX generation
│   │   └── html.rs       // HTML generation
│   ├── themes/
│   │   ├── mod.rs        // Theme management
│   │   ├── font.rs       // Font themes
│   │   └── color.rs      // Color themes
│   ├── ai/               // AI integration
│   │   ├── mod.rs        // AI module interface
│   │   ├── client.rs     // OpenAI-compatible API client
│   │   ├── pdf_parser.rs // PDF text extraction
│   │   ├── prompts.rs    // AI prompt engineering
│   │   ├── schema_gen.rs // JSON schema generation
│   │   └── schemas.rs    // Structured output schemas
│   ├── templates/        // Markdown templates
│   │   ├── cv_template.md
│   │   └── letter_template.md
│   ├── config.rs         // Configuration types
│   ├── constants.rs      // Shared constants
│   └── error.rs          // Error handling
├── fonts/                // TTF font files
├── examples/             // Example documents
├── cv/                   // Output directory
└── tests/               // Test suite
```

## Theme System

### Font Themes

```rust
pub struct FontTheme {
    pub name: &'static str,
    pub header_font: Font,
    pub body_font: Font,
}

pub struct Font {
    pub family: &'static str,
    pub weight: FontWeight,
    pub size: f32,
}
```

Available themes:
- **Classic**: Georgia (headers) + Times New Roman (body)
- **Modern**: Inter (headers) + Open Sans (body)
- **Sharp**: Montserrat (headers) + Roboto (body)

### Color Themes

```rust
pub struct ColorTheme {
    pub name: &'static str,
    pub primary: &'static str,    // Headers, links
    pub secondary: &'static str,  // Accents
    pub text: &'static str,       // Body text
    pub background: &'static str, // Page background
}
```

Available themes:
- **Classic**: Navy + Burgundy
- **Modern**: Electric Blue + Teal
- **Sharp**: Deep Purple + Hot Pink

## Configuration Schema

```rust
#[derive(Deserialize)]
pub struct Config {
    // Required
    pub name: String,
    pub email: String,

    // Optional contact
    pub phone: Option<String>,
    pub location: Option<String>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub website: Option<String>,

    // Themes
    pub font_theme: FontThemeType,
    pub color_theme: ColorThemeType,

    // Layout
    pub layout: LayoutConfig,

    // Cover letter specific
    pub recipient: Option<Recipient>,
    pub date: Option<String>,
    pub subject: Option<String>,
}
```

## AI Module Architecture

### Components

```rust
pub mod ai {
    pub mod client;      // OpenAI API client wrapper
    pub mod pdf_parser;  // PDF text extraction
    pub mod prompts;     // Prompt templates
    pub mod schemas;     // JSON schema definitions
}
```

### AI Client Design

```rust
pub struct AIClient {
    endpoint: String,
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl AIClient {
    pub async fn tailor_cv(
        &self,
        cv_content: &str,
        job_description: &str
    ) -> Result<TailoredCV> {
        // Send structured request
        // Receive and validate response
    }
}
```

### Structured Output Schemas

```rust
#[derive(Serialize, Deserialize)]
pub struct TailoredCV {
    pub professional_summary: String,
    pub experiences: Vec<OptimizedExperience>,
    pub skills: Vec<String>,
    pub keywords: Vec<String>,
    pub suggestions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OptimizedExperience {
    pub title: String,
    pub company: String,
    pub duration: String,
    pub highlights: Vec<String>,
    pub relevance_score: f32,
}
```

### Prompt Engineering

The system uses a carefully crafted prompt that:
1. Establishes the AI as an expert HR professional
2. Provides clear instructions for CV optimization
3. Specifies the exact JSON schema for responses
4. Includes examples of good tailoring practices

## Rendering Pipeline

### PDF Generation (Typst)

1. Generate Typst source code programmatically
2. Apply theme variables and document content
3. Compile to PDF using Typst library
4. Write to output directory

### DOCX Generation

1. Create document structure with docx-rs
2. Apply theme styles to paragraphs
3. Build document sections
4. Save as DOCX file

### HTML Generation

1. Generate semantic HTML structure
2. Embed CSS with theme variables
3. Include print-friendly styles
4. Output self-contained HTML

## Quality Assurance

### Code Standards
- Zero `unsafe` code blocks
- No `unwrap()` - all errors handled
- Zero clippy warnings (pedantic mode)
- 80% test coverage minimum

### Testing Strategy
- Unit tests for each module
- Integration tests for CLI commands
- Snapshot tests for theme rendering
- Property tests for parser edge cases

### Performance Targets
- < 100ms PDF generation
- < 50ms HTML generation
- < 10ms file watching latency
- < 50MB binary size

## Security Considerations

1. **Input Validation**: Sanitize all YAML/Markdown input
2. **Path Traversal**: Validate all file paths
3. **Template Injection**: Escape all user content
4. **Dependencies**: Minimal, audited dependencies
5. **Binary Distribution**: Signed releases
6. **API Keys**: Never log or expose API credentials
7. **PDF Parsing**: Validate PDF content before processing
8. **AI Responses**: Validate all AI output against schemas

## Future Enhancements

1. **Web Preview Server**: Live preview with hot reload
2. **Enhanced AI Features**:
   - Cover letter generation
   - Interview preparation based on CV/JD match
   - Multiple CV versions for different roles
3. **Template Library**: Additional document types
4. **Cloud Export**: Direct upload to job sites
5. **GUI Wrapper**: Native desktop application
6. **Analytics**: Track application success rates
