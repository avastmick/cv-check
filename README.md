# CV Generator

A modern, Rust-based command-line tool for generating professional CVs and cover letters from Markdown files with YAML frontmatter.

## Features

- **Simple Markdown Input**: Write your CV in plain Markdown with YAML frontmatter
- **Beautiful Themes**: Choose from classic, modern, or sharp font and color themes  
- **Multiple Formats**: Generate PDF (via Typst), DOCX, and HTML
- **Flexible Layouts**: Single column, two column, or condensed summary versions
- **Live Reload**: Auto-rebuild on file changes with `watch` command
- **Fast**: Sub-second PDF generation with Typst
- **Zero Configuration**: Works out of the box with sensible defaults

## Prerequisites

### Typst (Required for PDF Generation)

This tool requires Typst for generating PDFs. Install it using one of these methods:

```bash
# macOS
brew install typst

# Linux/WSL
# Download from: https://github.com/typst/typst/releases

# Cross-platform via cargo
cargo install typst-cli
```

## Installation

```bash
cargo install cv_gen
```

## Quick Start

1. Create a new CV template:

```bash
cv new cv > my-cv.md
```

2. Edit `my-cv.md` with your information:

```markdown
---
name: John Doe
email: john@example.com
phone: +1 234 567 8900
location: San Francisco, CA

font_theme: modern
color_theme: modern
---

# Professional Summary
Experienced software engineer...

# Experience
## Senior Developer at Tech Corp
*2020 - Present*

Led development of cloud platforms...
```

3. Generate your CV:

```bash
cv build my-cv.md
```

## Commands

```bash
cv build <input.md>         # Generate CV/letter (PDF, DOCX, HTML)
cv watch <input.md>         # Auto-rebuild on changes
cv new cv                   # Create CV template
cv new letter              # Create letter template
cv themes                  # List available themes
cv check <input.md>        # Validate markdown structure
```

## Themes

### Font Themes

- **Classic**: Traditional serif fonts (Georgia/Times New Roman)
- **Modern**: Clean sans-serif (Inter/Open Sans)  
- **Sharp**: Bold geometric fonts (Montserrat/Roboto)

### Color Themes

- **Classic**: Navy (#2C3E50) and burgundy (#8B0000)
- **Modern**: Electric blue (#0066CC) and teal (#00A8A8)
- **Sharp**: Deep purple (#6B46C1) and hot pink (#EC4899)

## YAML Frontmatter Options

```yaml
# Required fields
name: string
email: string

# Optional contact info
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
    top: 1.5    # in cm
    bottom: 1.5
    left: 2.0
    right: 2.0

# Cover letter specific
recipient:
  name: string
  title: string
  company: string
  address: string
date: string
subject: string
```

## Customization

Override any theme setting in your YAML frontmatter:

```yaml
---
# Use modern font theme but override header font
font_theme: modern
fonts:
  header:
    family: "Playfair Display"
    
# Use modern colors but customize primary
color_theme: modern
colors:
  primary: "#1E40AF"
---
```

## Development

```bash
# Clone and build
git clone https://github.com/yourusername/cv_gen
cd cv_gen
cargo build --release

# Run tests
cargo test

# Check code quality
cargo clippy --all-targets --all-features

# Check test coverage
cargo tarpaulin --out Html
```

## Project Structure

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
└── tests/               # Test suite
```

## Contributing

This project follows strict code quality standards:
- Zero clippy warnings with pedantic lints
- No unsafe code
- No unwrap() - proper error handling
- 80% test coverage target
- Test-Driven Development (TDD)

## License

MIT License - see LICENSE file for details