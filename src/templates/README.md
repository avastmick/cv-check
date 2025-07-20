# Templates Module

This module contains markdown templates for creating new CV and cover letter documents.

## Purpose

Provides starter templates that users can quickly customize to create their own CVs and cover letters with proper YAML frontmatter structure.

## Template Files

### `cv_template.md`
A comprehensive CV template including:
- Complete YAML frontmatter with all available fields
- Example sections (Experience, Education, Skills, etc.)
- Markdown formatting examples
- Comments explaining optional fields

### `letter_template.md`
A professional cover letter template featuring:
- Recipient information fields
- Date and subject line
- Standard letter structure
- Placeholder content for customization

## Usage

These templates are used by the `cv new` command:

```bash
# Create a new CV
cv new cv > my-cv.md

# Create a new cover letter
cv new letter > cover-letter.md
```

## Template Structure

Both templates follow this pattern:

1. **YAML Frontmatter** (between `---` delimiters)
   - Required fields (name, email)
   - Optional contact information
   - Theme configuration
   - Layout options

2. **Markdown Content**
   - Section headers
   - Lists and formatting examples
   - Placeholder text

## Customization Guide

Users should:
1. Replace all placeholder text with actual content
2. Remove any unused optional fields
3. Adjust theme settings to preference
4. Add/remove sections as needed

## Design Decisions

- **Comprehensive Examples**: Include all possible fields to show options
- **Clear Placeholders**: Use obvious placeholder text (e.g., "Your Name")
- **Comments**: Explain optional fields and formatting
- **Real-World Structure**: Follow common CV/letter conventions

## Maintenance

When updating templates:
- Ensure YAML fields match `config::DocumentMetadata`
- Test that templates parse successfully
- Keep placeholder text clear and consistent
- Update examples to show new features
