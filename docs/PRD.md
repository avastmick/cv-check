# Product Requirements Document: CV Generator (Rust Implementation)

## Overview

A modern, Rust-based command-line tool that converts Markdown files with YAML frontmatter into professionally typeset CVs and cover letters. The system generates PDF (via Typst), DOCX, and HTML outputs with configurable themes. In Phase 2, the tool leverages generative AI to automatically tailor CVs to specific job descriptions, optimizing content, keywords, and layout for maximum job application success.

### Goal

The application will iterate on additional features until the following features exist:
- [x] Markdown CV to professionally typeset PDF output
- [ ] Additional MS Word (`.docx`) output
- [x] Simple, multiple, impactfull, themeing
- [🚧] Align output CV keywords and experience to input **Job Description** with Generative AI assistance
- [ ] Fully aligned generated Cover Letter to input **Job Description** with Generative AI assistance
- [ ] Static site generation for CV including themeing and deployment

## Core Features

### Input Format
- **Markdown files** with YAML frontmatter for metadata
- **Validation** via `check` command ensures proper structure
- **Live reload** with file watching for rapid iteration
- **PDF Job Descriptions** (Phase 2) for AI-powered CV tailoring

### Output Formats
- **PDF**: Professional typesetting via Typst
- **DOCX**: Microsoft Word compatible documents
- **HTML**: Web-ready output with embedded styling

### Theme System
**Font Themes:**
- `classic`: Georgia (headers) + Times New Roman (body) - Traditional serif
- `modern`: Inter (headers) + Open Sans (body) - Clean sans-serif
- `sharp`: Montserrat (headers) + Roboto (body) - Bold geometric

**Color Themes:**
- `classic`: Navy (#2C3E50) + Burgundy (#8B0000) - Professional traditional
- `modern`: Electric Blue (#0066CC) + Teal (#00A8A8) - Tech-focused
- `sharp`: Deep Purple (#6B46C1) + Hot Pink (#EC4899) - Creative/bold

### Layout Options
- **Single column**: Traditional layout (default)
- **Two column**: Compact layout for longer CVs
- **Summary mode**: One-page condensed version

### AI-Powered Features (Phase 2)
- **CV Tailoring**: Automatically optimize CV content for specific job descriptions
- **Keyword Optimization**: Extract and incorporate relevant keywords from job descriptions
- **Experience Alignment**: Reorder and emphasize experiences matching job requirements
- **Skills Matching**: Highlight skills that align with job requirements
- **ATS Optimization**: Ensure CV passes Applicant Tracking Systems
- **Structured Output**: Use OpenAI-compatible API with JSON schema validation

## Technical Architecture

### CLI Commands
```bash
cv build <input.md>         # Generate CV/letter
cv watch <input.md>         # Auto-rebuild on changes
cv new cv                   # Create CV template
cv new letter              # Create letter template
cv themes                  # List available themes
cv check <input.md>        # Validate markdown structure
cv tailor <cv.md> <job.pdf> # AI-powered CV tailoring (Phase 2)
cv serve <input.md>        # Preview server (planned)
```

### Project Structure
```
cv_check/
├── src/
│   ├── cli/              # Command interface
│   ├── parser/           # Markdown/YAML parsing
│   ├── render/           # Output generation
│   ├── themes/           # Theme definitions
│   ├── templates/        # Markdown templates
│   ├── ai/               # AI integration (Phase 2)
│   │   ├── client.rs     # OpenAI-compatible API client
│   │   ├── pdf_parser.rs # PDF text extraction
│   │   ├── prompts.rs    # AI prompt engineering
│   │   └── schemas.rs    # Structured output schemas
│   ├── config.rs         # Configuration types
│   └── error.rs          # Error handling
├── fonts/                # TTF font files
├── examples/             # Example documents
├── cv/                   # Output directory
└── tests/               # Comprehensive test suite
```

### YAML Frontmatter Schema
```yaml
# Required fields
name: string
email: string

# Optional fields
phone: string
location: string
linkedin: string
github: string
website: string

# Theme configuration
font_theme: classic|modern|sharp
color_theme: classic|modern|sharp

# Layout options
layout:
  columns: 1|2
  margins:
    top: float
    bottom: float
    left: float
    right: float

# Cover letter specific
recipient:
  name: string
  title: string
  company: string
  address: string
date: string
subject: string
```

### AI Configuration (Phase 2)
The AI integration uses environment variables for configuration:

```bash
# Required environment variables
AI_ENDPOINT=https://api.openai.com/v1  # OpenAI-compatible API endpoint
AI_MODEL=gpt-4o-2024-08-06            # Model supporting structured outputs
AI_API_KEY=your-api-key-here          # API authentication key
```

**Supported AI Providers:**
- OpenAI (GPT-4o models with structured outputs)
- Any OpenAI-compatible API (Azure OpenAI, local LLMs, etc.)

**AI Processing Pipeline:**
1. Extract text from PDF job description
2. Parse base CV markdown and frontmatter
3. Send to LLM with expert HR prompt
4. Receive structured JSON response with tailored content
5. Generate optimized CV output

## Quality Standards

### Code Quality
- **Zero clippy warnings** with pedantic lints
- **No unsafe code** - memory safety guaranteed
- **No unwrap()** - proper error handling throughout
- **80% test coverage** target with Tarpaulin

### Testing Strategy
- **Test-Driven Development** (TDD) strictly enforced
- **68 tests** currently across all modules
- Unit tests for all components
- Integration tests for CLI commands

### Performance
- Fast compilation via Typst (not LaTeX)
- Efficient file watching with debouncing
- Minimal dependencies for quick installation

## User Experience

### Installation
```bash
cargo install cv_check
```

### Basic Usage
1. Write CV in Markdown with YAML frontmatter
2. Run `cv build my-cv.md`
3. Get professional PDF/DOCX/HTML output

### Customization
- All styling via YAML frontmatter
- No knowledge of Typst/LaTeX required
- Themes are composable (mix and match)

## Success Criteria

1. **Professional Quality**: Output matches or exceeds LaTeX quality
2. **User Friendly**: Zero configuration required to start
3. **Fast**: Sub-second PDF generation
4. **Maintainable**: Clean, tested, documented code
5. **Cross-platform**: Works on Windows, macOS, Linux
6. **Extensible**: Easy to add new themes and layouts

## Current Status

### ✅ Completed
- Rust CLI structure with all commands
- Markdown parser with YAML frontmatter
- Theme system (fonts and colors)
- File watching for auto-rebuild
- Comprehensive test suite
- Strict code quality enforcement

### 🚧 In Progress
- Typst integration for PDF generation
- Template embedding in binary
- AI-powered CV tailoring (Phase 2)
  - PDF text extraction from job descriptions
  - OpenAI API integration with structured outputs
  - Prompt engineering for HR expertise
  - JSON schema definitions for CV optimization

### 📋 Planned
- DOCX export implementation
- HTML preview server
- Additional themes
- Cover letter AI generation
- GUI wrapper (future)

## Dependencies

### Core Dependencies
- **typst**: Modern typesetting engine
- **clap**: CLI argument parsing
- **serde**: YAML/JSON serialization
- **pulldown-cmark**: Markdown parsing
- **notify**: File system watching
- **colored**: Terminal output styling

### Phase 2 Dependencies
- **openai-api-rs**: OpenAI-compatible API client
- **pdf-extract** or **lopdf**: PDF text extraction
- **serde_json**: Structured JSON handling
- **tokio**: Async runtime for API calls

## Non-Goals

- Not a general-purpose document processor
- Not a WYSIWYG editor
- Not backwards compatible with LaTeX templates
- Not supporting arbitrary custom themes (use predefined set)

## Constraints

- Must maintain professional typesetting quality
- Phase 1 must work offline (no external API dependencies)
- Phase 2 AI features require internet connection and API key
- Must be installable as single binary
- Must respect user privacy (no telemetry)
- AI processing must use structured outputs for reliability
- PDF parsing must handle various job description formats
