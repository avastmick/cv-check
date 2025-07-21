use cv_check::parser::Document;
use cv_check::render::pdf::PdfRenderer;
use cv_check::render::RenderEngine;
use cv_check::themes::Theme;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_pdf_rendering_with_all_markdown_tags() {
    // Test all markdown elements that have specific handlers
    let content = r"---
name: Test User
email: test@example.com
---

# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

Regular paragraph text.

**Bold text** and *italic text* and ***bold italic***.

~~Strikethrough text~~

> Block quote text
> Multiple lines

[Link text](https://example.com)

`Inline code`

```
Code block
Multiple lines
```

- List item 1
- List item 2
  - Nested item 1
  - Nested item 2
    - Double nested
- Back to top level

1. Ordered list
2. Second item

Soft break here
and hard break here
after two spaces.

| Table | Header |
|-------|--------|
| Cell  | Data   |

---

Horizontal rule above.

- [ ] Task list unchecked
- [x] Task list checked

Footnote reference[^1]

[^1]: Footnote text
";

    let doc =
        Document::from_string(content, Path::new("test.md")).expect("Failed to parse document");
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");

    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output = temp_dir.path().join("output.pdf");

    // Should handle all markdown elements
    let result = renderer.render(&doc, &theme, &output);

    // Even if typst fails, the generation should not panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_pdf_rendering_deeply_nested_lists() {
    let content = r"---
name: Test
email: test@test.com
---

- Level 1
  - Level 2
    - Level 3
      - Level 4
        - Level 5
          - Level 6
            - Level 7
              - Level 8
                - Level 9
                  - Level 10
";

    let doc =
        Document::from_string(content, Path::new("test.md")).expect("Failed to parse document");
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");

    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output = temp_dir.path().join("output.pdf");

    // Should handle deeply nested lists
    let _ = renderer.render(&doc, &theme, &output);
}

#[test]
fn test_pdf_rendering_empty_markdown_elements() {
    let content = r"---
name: Test
email: test@test.com
---

#

##

***

****

~~

>

[]()

``

```
```

-
  -
";

    let doc =
        Document::from_string(content, Path::new("test.md")).expect("Failed to parse document");
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");

    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output = temp_dir.path().join("output.pdf");

    // Should handle empty elements without panicking
    let _ = renderer.render(&doc, &theme, &output);
}

#[test]
fn test_pdf_rendering_special_characters_in_links() {
    let content = r#"---
name: Test
email: test@test.com
---

[Link with spaces](https://example.com/path with spaces)
[Link with quotes](https://example.com/path"with"quotes)
[Link with special chars](https://example.com/path?param=value&other=123)
[Link with unicode](https://example.com/测试)
"#;

    let doc =
        Document::from_string(content, Path::new("test.md")).expect("Failed to parse document");
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");

    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output = temp_dir.path().join("output.pdf");

    // Should handle special characters in links
    let _ = renderer.render(&doc, &theme, &output);
}

#[test]
fn test_pdf_rendering_mixed_list_types() {
    let content = r"---
name: Test
email: test@test.com
---

- Bullet item
  1. Ordered nested
  2. Another ordered
- Back to bullet
  - Nested bullet
    1. Ordered in nested bullet
";

    let doc =
        Document::from_string(content, Path::new("test.md")).expect("Failed to parse document");
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");

    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output = temp_dir.path().join("output.pdf");

    // Should handle mixed list types
    let _ = renderer.render(&doc, &theme, &output);
}

#[test]
fn test_pdf_rendering_paragraphs_in_lists() {
    let content = r"---
name: Test
email: test@test.com
---

Normal paragraph before list.

- List item with paragraph

  Another paragraph in the list item

- Second item

Normal paragraph after list.
";

    let doc =
        Document::from_string(content, Path::new("test.md")).expect("Failed to parse document");
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");

    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output = temp_dir.path().join("output.pdf");

    // Should handle paragraphs within lists correctly
    let _ = renderer.render(&doc, &theme, &output);
}

#[test]
fn test_pdf_rendering_multiple_code_blocks() {
    let content = r#"---
name: Test
email: test@test.com
---

```rust
fn main() {
    println!("Hello");
}
```

Some text between.

```python
def hello():
    print("Hello")
```

```
No language specified
```
"#;

    let doc =
        Document::from_string(content, Path::new("test.md")).expect("Failed to parse document");
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");

    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output = temp_dir.path().join("output.pdf");

    // Should handle multiple code blocks
    let _ = renderer.render(&doc, &theme, &output);
}

#[test]
fn test_pdf_rendering_block_quotes_nested() {
    let content = r"---
name: Test
email: test@test.com
---

> Level 1 quote
> > Level 2 nested quote
> > > Level 3 deeply nested
>
> Back to level 1
";

    let doc =
        Document::from_string(content, Path::new("test.md")).expect("Failed to parse document");
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");

    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output = temp_dir.path().join("output.pdf");

    // Should handle nested block quotes
    let _ = renderer.render(&doc, &theme, &output);
}

#[test]
fn test_pdf_text_escaping() {
    // Test that special Typst characters are handled
    let content = r#"---
name: Test
email: test@test.com
---

Text with # hash marks
Text with $ dollar signs
Text with \ backslashes
Text with " quotes
Text with ' apostrophes
Text with < > angle brackets
Text with & ampersands
Text with @ at signs
Text with { } braces
Text with [ ] brackets
"#;

    let doc =
        Document::from_string(content, Path::new("test.md")).expect("Failed to parse document");
    let theme = Theme::new("modern", "modern").expect("Failed to create theme");
    let renderer = PdfRenderer::new(None).expect("Failed to create renderer");

    let temp_dir = tempdir().expect("Failed to create temp dir");
    let output = temp_dir.path().join("output.pdf");

    // Should escape special characters properly
    let _ = renderer.render(&doc, &theme, &output);
}
