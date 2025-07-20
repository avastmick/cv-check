# Render Module

This module handles output generation in multiple formats (PDF, DOCX, HTML) from parsed documents.

## Purpose

Provides a unified interface for rendering documents to different output formats while maintaining consistent styling through the theme system.

## Architecture

### Core Trait: `RenderEngine`
```rust
pub trait RenderEngine {
    fn render(&self, doc: &Document, theme: &Theme, output: &Path) -> Result<()>;
}
```

All format-specific renderers implement this trait, allowing for easy extension.

### Main Components

#### `mod.rs` - Renderer Factory
- `Renderer` struct - Factory and facade for format-specific engines
- `Renderer::new()` - Creates appropriate engine based on format string
- `Renderer::render()` - Delegates to engine implementation

#### `pdf.rs` - PDF Generation
- Uses Typst for high-quality typesetting
- Loads Typst templates from `templates/` directory
- Supports custom template overrides
- Handles font embedding and styling

#### `docx.rs` - Word Document Generation
- Creates Microsoft Word compatible documents
- Applies theme styles to paragraphs and sections
- Maintains document structure and formatting

#### `html.rs` - HTML Generation
- Generates self-contained HTML with embedded CSS
- Includes print-friendly styles
- Supports responsive design
- Theme colors and fonts applied via CSS variables

## Usage Example

```rust
use crate::render::Renderer;
use crate::parser::Document;
use crate::themes::Theme;

// Create renderer for PDF output
let renderer = Renderer::new("pdf", None)?;

// Load document and theme
let doc = Document::from_file(Path::new("cv.md"))?;
let theme = Theme::new("modern", "sharp")?;

// Render to output file
renderer.render(&doc, &theme, Path::new("output.pdf"))?;
```

## Format-Specific Features

### PDF (via Typst)
- Professional typesetting quality
- Embedded fonts
- Vector graphics support
- Fast compilation

### DOCX
- Native Word formatting
- Editable output
- Style preservation
- Table support

### HTML
- Web-ready output
- Print CSS included
- Mobile responsive
- SEO friendly

## Error Handling

- Invalid output format returns `CvError::InvalidFormat`
- Template loading failures are propagated with context
- File I/O errors are wrapped with descriptive messages

## Dependencies

- `typst`: PDF generation engine
- `docx-rs`: Word document creation (when implemented)
- `pulldown-cmark`: Markdown to HTML conversion
- Format-specific template files in `templates/`

## Adding a New Format

1. Create new file (e.g., `rtf.rs`)
2. Implement `RenderEngine` trait
3. Add to match statement in `Renderer::new()`
4. Update `CvError::InvalidFormat` message
5. Add tests for new format

## Template System

Each renderer can use templates:
- Default templates embedded in binary
- Custom templates via `--template` flag
- Templates receive document data and theme variables

## Performance Considerations

- PDF rendering is fastest (~100ms)
- DOCX generation requires more memory
- HTML is lightweight and quick
- All formats support streaming where possible
