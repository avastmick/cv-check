# Parser Module

This module handles parsing of markdown files with YAML frontmatter into structured documents.

## Purpose

Extracts and validates document metadata from YAML frontmatter and parses markdown content into an AST (Abstract Syntax Tree) for rendering.

## Module Structure

### `mod.rs` - Document Structure
Defines the core `Document` struct that represents a parsed CV or cover letter:
- `metadata`: Parsed YAML frontmatter as `DocumentMetadata`
- `content`: Raw markdown content (without frontmatter)
- `markdown_ast`: Parsed markdown as pulldown-cmark events

Key methods:
- `Document::from_file()` - Parse from file path
- `Document::from_string()` - Parse from string content
- `Document::validate()` - Ensure required fields are present

### `frontmatter.rs` - YAML Extraction
Handles extraction and parsing of YAML frontmatter:
- Expects documents to start with `---` delimiter
- Extracts YAML between delimiters
- Parses into `DocumentMetadata` structure
- Returns remaining content as markdown

### `markdown.rs` - Markdown Parsing
Converts markdown content to AST using pulldown-cmark:
- Enables tables, footnotes, strikethrough, and task lists
- Returns owned events for later rendering
- Preserves all markdown formatting

## Usage Example

```rust
use crate::parser::Document;

// Parse a CV file
let doc = Document::from_file(Path::new("cv.md"))?;

// Validate required fields
doc.validate()?;

// Access metadata
println!("Name: {}", doc.metadata.name);
println!("Email: {}", doc.metadata.email);

// Access parsed content
for event in &doc.markdown_ast {
    // Process markdown events
}
```

## Error Cases

- Missing frontmatter delimiters
- Invalid YAML syntax
- Missing required fields (name, email)
- File not found
- Invalid UTF-8 content

## Dependencies

- `pulldown-cmark`: Markdown parsing
- `serde_yaml`: YAML parsing
- `config`: Document metadata structures
- `error`: Error type definitions

## Design Decisions

1. **Two-Phase Parsing**: Separates frontmatter extraction from markdown parsing for clarity
2. **Owned Events**: Stores parsed markdown as owned events to avoid lifetime issues
3. **Validation**: Separate validation step allows for better error messages
4. **Path Context**: Keeps source path for better error reporting

## Potential Improvements

- [ ] Support for multiple frontmatter formats (TOML, JSON)
- [ ] Streaming parser for large documents
- [ ] Custom markdown extensions for CV-specific elements
- [ ] Frontmatter schema validation
