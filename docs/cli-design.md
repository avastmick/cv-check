# Rust CLI Design

## Overview

A modern, cross-platform command-line tool for generating professional CVs and cover letters from Markdown files with theme support.

## Design Principles

1. **Zero Configuration**: Works out of the box with sensible defaults
2. **Fast**: Instant generation and preview
3. **Cross-Platform**: Single binary for Windows, macOS, Linux
4. **User-Friendly**: Clear commands and helpful error messages
5. **Extensible**: Plugin system for custom themes and formats

## CLI Interface

### Basic Commands

```bash
# Generate CV
cv build cv.md

# Generate cover letter
cv build cover-letter.md

# Generate both with themes
cv build cv.md --font-theme modern --color-theme modern

# Watch mode for development
cv watch cv.md

# Create new CV from template
cv new cv
cv new letter

# List available themes
cv themes

# Validate markdown file
cv check cv.md
```

### Command Structure

```
cv <command> [arguments] [options]

Commands:
  build     Generate PDF/DOCX from markdown
  watch     Auto-rebuild on file changes
  new       Create from template
  themes    List/preview themes
  check     Validate markdown structure
  serve     Start preview server
  help      Show help

Options:
  -f, --font-theme <name>      Font theme (classic|modern|sharp)
  -c, --color-theme <name>     Color theme (classic|modern|sharp)
  -o, --output <path>          Output file path
  -F, --format <type>          Output format (pdf|docx|html)
  -t, --template <path>        Custom template
  -v, --verbose                Verbose output
  -q, --quiet                  Suppress output
      --version                Show version
```

## Architecture

### Module Structure

```rust
cv_check/
├── src/
│   ├── main.rs              // CLI entry point
│   ├── lib.rs               // Library exports
│   ├── cli/                 // CLI module
│   │   └── mod.rs           // Command definitions
│   ├── config.rs            // Configuration handling
│   ├── error.rs             // Error handling
│   ├── parser/              // Markdown parsing
│   │   ├── mod.rs
│   │   ├── frontmatter.rs   // YAML extraction
│   │   └── markdown.rs      // Markdown parsing
│   ├── themes/              // Theme engine
│   │   ├── mod.rs
│   │   ├── font.rs          // Font themes
│   │   └── color.rs         // Color themes
│   ├── render/              // Document generation
│   │   ├── mod.rs
│   │   ├── pdf.rs           // PDF via Typst
│   │   ├── docx.rs          // DOCX generation
│   │   └── html.rs          // HTML output
│   └── templates/           // Markdown templates
│       ├── cv_template.md
│       └── letter_template.md
├── templates/               // Typst templates
├── fonts/                   // TTF font files
├── examples/                // Example documents
├── cv/                      // Output directory
└── tests/                   // Test suite
```

### Dependencies

```toml
[dependencies]
clap = "4.0"              # CLI framework
serde = "1.0"             # Serialization
serde_yaml = "0.9"        # YAML parsing
pulldown-cmark = "0.9"    # Markdown parsing
typst = "0.11"            # PDF generation
notify = "6.0"            # File watching
tokio = "1.0"             # Async runtime
anyhow = "1.0"            # Error handling
colored = "2.0"           # Terminal colors
indicatif = "0.17"        # Progress bars

[dev-dependencies]
tempfile = "3.0"          # Testing
assert_cmd = "2.0"        # CLI testing
```

## Core Components

### 1. Markdown Parser

```rust
pub struct Document {
    pub metadata: Metadata,
    pub content: Content,
}

pub struct Metadata {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub font_theme: String,
    pub color_theme: String,
    pub custom: HashMap<String, Value>,
}

impl Document {
    pub fn from_markdown(input: &str) -> Result<Self>
}
```

### 2. Theme Engine

```rust
pub trait Theme {
    fn name(&self) -> &str;
    fn apply(&self, doc: &mut Document) -> Result<()>;
}

pub struct FontTheme {
    pub header_font: FontSpec,
    pub body_font: FontSpec,
}

pub struct ColorTheme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub text: Color,
}
```

### 3. Renderer System

```rust
pub trait Renderer {
    fn render(&self, doc: &Document, output: &Path) -> Result<()>;
}

pub struct PdfRenderer {
    typst_engine: TypstEngine,
}

pub struct DocxRenderer {
    template: DocxTemplate,
}
```

### 4. File Watcher

```rust
pub async fn watch(paths: Vec<PathBuf>, callback: impl Fn()) -> Result<()> {
    // Hot reload implementation
}
```

## Error Handling

### User-Friendly Messages

```rust
#[derive(Debug, thiserror::Error)]
pub enum CvError {
    #[error("Missing required field '{field}' in {file}")]
    MissingField { field: String, file: PathBuf },

    #[error("Unknown theme '{theme}'. Available: {available}")]
    UnknownTheme { theme: String, available: String },

    #[error("Invalid markdown structure: {reason}")]
    InvalidMarkdown { reason: String },
}
```

### Error Display

```
Error: Missing required field 'name' in cv.md

The markdown file must include a 'name' field in the frontmatter:

---
name: Your Name
email: your.email@example.com
---

Run 'cv new cv' to create a valid template.
```

## Configuration

### Global Config

```yaml
# ~/.config/cv/config.yaml
default_font_theme: modern
default_color_theme: modern
pdf_engine: typst
custom_themes_dir: ~/.config/cv/themes/
```

### Project Config

```yaml
# .cv.yaml
output_dir: ./output
watch_paths:
  - "*.md"
  - "themes/*.yaml"
auto_open: true
```

## Performance Considerations

1. **Lazy Loading**: Load themes only when needed
2. **Caching**: Cache compiled Typst templates
3. **Parallelization**: Build multiple documents concurrently
4. **Incremental Builds**: Only rebuild changed sections

## Testing Strategy

### Unit Tests
```rust
#[test]
fn test_parse_frontmatter() {
    let input = "---\nname: Test\n---\n# Content";
    let doc = Document::from_markdown(input).unwrap();
    assert_eq!(doc.metadata.name, "Test");
}
```

### Integration Tests
```rust
#[test]
fn test_pdf_generation() {
    let temp = tempdir().unwrap();
    let output = temp.path().join("test.pdf");

    cmd!("cv", "build", "fixtures/test.md", "-o", &output)
        .assert()
        .success();

    assert!(output.exists());
}
```

## Distribution

### Release Artifacts
- Single binary per platform
- No runtime dependencies
- Embedded templates and themes
- Auto-updater support

### Installation Methods
```bash
# Cargo
cargo install cv-cli

# Homebrew
brew install cv-cli

# Binary download
curl -L https://github.com/user/cv-cli/releases/latest/download/cv-linux-x64 -o cv
chmod +x cv
```

## Future Enhancements

1. **Plugin System**: Custom themes and renderers
2. **Cloud Sync**: Theme sharing and backups
3. **GUI Preview**: Native preview window
4. **AI Assistant**: Content suggestions
5. **Version Control**: Track CV versions
6. **Export Formats**: LinkedIn, ATS-friendly text
