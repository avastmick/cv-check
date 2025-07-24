# CV Tailoring and Formatting CLI

A modern, Rust-based command-line tool for generating professional CVs and cover letters from Markdown files with YAML frontmatter. Now with AI-powered CV tailoring to optimise your CV for specific job descriptions.

Why? No one actually reads your hand-crafted CV, they are all loaded into a system; format incorrectly, information is lost and you don't get the job! This tool offers an easier means of tailoring HR ATS-compliant CVs to jobs, with AI improvement and focus. Tested on most current job sites to correctly extract roles, experience, skills, etc.

> [!CAUTION]
> Under active development - breaking changes often. Some documentation may be out of date, or not aligned with function. Wait until first release.

## Features

- **Simple Markdown Input**: Write your CV in plain Markdown with YAML frontmatter
- **Themes**: Choose from classic, modern, or sharp font and color themes
- **Multiple Formats**: Generate PDF (via Typst), DOCX, and HTML
- **Flexible Layouts**: Single column, two column, or condensed summary versions
- **Zero Configuration**: Works out of the box with sensible defaults
- **AI-Powered Tailoring**: Automatically optimise your CV for specific job descriptions
- **Keyword Optimisation**: Extract and incorporate relevant keywords from job postings
- **ATS-Friendly**: Ensure your CV passes Applicant Tracking Systems

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

### AI Features (Optional)

For AI-powered CV tailoring, you'll need:

1. An OpenAI API key or compatible API endpoint
2. Set environment variables:

```bash
export AI_ENDPOINT="https://api.openai.com/v1"  # Or your API endpoint
export AI_MODEL="gpt-4o-2024-08-06"            # Model with structured outputs
export AI_API_KEY="your-api-key-here"           # Your API key
```

## Installation

```bash
cargo install cv_check
```

## Quick Start

1. Create a new CV template / Cover letter:

```bash
cv new cv > my-cv.md
cv new coverletter > my-cover-letter.md
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
cv new cv                   # Create CV template
cv new letter              # Create letter template
cv themes                  # List available themes
cv check <input.md>        # Validate markdown structure
cv tailor <cv.md> <job.pdf> # AI-powered CV tailoring (requires API key)
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

## Customisation

Override any theme setting in your YAML frontmatter:

```yaml
---
# Use modern font theme but override header font
font_theme: modern
fonts:
  header:
    family: "Playfair Display"

# Use modern colors but customise primary
color_theme: modern
colors:
  primary: "#1E40AF"
---
```

## AI-Powered CV Tailoring

The `tailor` command uses AI to optimise your CV for specific job descriptions:

```bash
# Tailor your CV to a job description
cv tailor my-cv.md job-description.pdf

# Output: my-cv-tailored.md (optimised markdown)
#         my-cv-tailored.pdf (ready-to-send PDF)
```

### How it works:

1. **Extracts text** from the job description PDF
2. **Analyses requirements** using AI with HR expertise
3. **Optimises your CV** by:
   - Reordering experiences to match job priorities
   - Emphasising relevant skills and keywords
   - Adjusting professional summary
   - Ensuring ATS compatibility
4. **Generates tailored output** maintaining your chosen theme

### Supported AI Providers:

- Any OpenAI-compatible API

The AI uses structured outputs to ensure reliable, consistent results.

## Development

```bash
# Clone and build
git clone https://github.com/yourusername/cv_check
cd cv_check
# Install the pre-commit checks to avoid issues upstream
pre-commit install

# Run tests
cargo test

# Check code quality
cargo clippy --all-targets --all-features

# Check test coverage
cargo tarpaulin
```

## CI/CD Usage

When running `cv_check` in CI environments (GitHub Actions, GitLab CI, Jenkins, etc.), the tool automatically detects the CI environment and disables auto-opening of generated files.

### Environment Variables

- **`CI`**: Standard CI environment variable. When set, auto-open is disabled.
- **`CV_CHECK_NO_OPEN`**: Explicitly disable auto-opening of generated files. Useful for:
  - Running tests locally without opening files
  - Batch processing multiple CVs
  - Server environments

### Example CI Configuration

```yaml
# GitHub Actions example
- name: Generate CV
  run: |
    cv build examples/cv.md --output artifacts/cv.pdf
  env:
    CV_CHECK_NO_OPEN: "1"  # Optional, CI=true is auto-detected

# GitLab CI example
generate_cv:
  script:
    - cv build cv.md --format pdf --quiet
  artifacts:
    paths:
      - "*.pdf"
```

### Running Tests in CI

```bash
# Tests automatically set CV_CHECK_NO_OPEN to prevent file opening
cargo test

# For integration tests that generate files
CV_CHECK_NO_OPEN=1 cargo test --test integration
```

## Contributing

This project follows strict code quality standards:
- Zero clippy warnings with pedantic lints
- No unsafe code
- No unwrap() - proper error handling
- 80% test coverage target

## License

MIT License - see LICENSE file for details
