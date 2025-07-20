# Product Requirements Document: CV Generator (Rust Implementation)

## Overview

A modern, Rust-based command-line tool that converts Markdown files with YAML frontmatter into professionally typeset CVs and cover letters. The system generates PDF (via Typst), DOCX, and HTML outputs with configurable themes.

### Goal

The application will iterate on additional features until the following features exist:
- [-] Markdown CV to professionally typeset PDF output
- [ ] Additional MS Word (`.docx`) output
- [ ] Simple, multiple, impactfull, themeing
- [ ] Align output CV keywords and experience to input **Job Description** with Generative AI assistance
- [ ] Fully aligned generated Cover Letter to input **Job Description** with Generative AI assistance
- [ ] Static site generation for CV including themeing and deployment

## Core Features

### Input Format
- **Markdown files** with YAML frontmatter for metadata
- **Validation** via `check` command ensures proper structure
- **Live reload** with file watching for rapid iteration

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

## Technical Architecture

### CLI Commands
```bash
cv build <input.md>         # Generate CV/letter
cv watch <input.md>         # Auto-rebuild on changes
cv new cv                   # Create CV template
cv new letter              # Create letter template
cv themes                  # List available themes
cv check <input.md>        # Validate markdown structure
cv serve <input.md>        # Preview server (planned)
```

### Project Structure
```
cv_gen/
├── src/
│   ├── cli/              # Command interface
│   ├── parser/           # Markdown/YAML parsing
│   ├── render/           # Output generation
│   ├── themes/           # Theme definitions
│   └── watch.rs          # File watching
├── templates/            # Typst templates
├── examples/             # Example documents
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
cargo install cv_gen
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

### 📋 Planned
- DOCX export implementation
- HTML preview server
- Additional themes
- GUI wrapper (future)

## Dependencies

- **typst**: Modern typesetting engine
- **clap**: CLI argument parsing
- **serde**: YAML/JSON serialization
- **pulldown-cmark**: Markdown parsing
- **notify**: File system watching
- **colored**: Terminal output styling

## Non-Goals

- Not a general-purpose document processor
- Not a WYSIWYG editor
- Not backwards compatible with LaTeX templates
- Not supporting arbitrary custom themes (use predefined set)

## Constraints

- Must maintain professional typesetting quality
- Must work offline (no external API dependencies)
- Must be installable as single binary
- Must respect user privacy (no telemetry)
