# Themes Module

This module manages visual themes for CV and cover letter generation, including fonts and colors.

## Purpose

Provides a flexible theming system that allows users to customize the visual appearance of their documents through predefined font and color combinations.

## Module Structure

### `mod.rs` - Theme Orchestration
Defines the main `Theme` struct that combines font and color themes:
- `Theme::new()` - Creates theme from font and color theme names
- `Theme::available_themes()` - Lists all available theme options
- Validates theme names and provides helpful error messages

### `font.rs` - Font Themes
Manages typography settings:
- **FontTheme** - Container for header and body font specifications
- **FontSpec** - Detailed font configuration including:
  - Font family
  - Weight variations (regular, bold)
  - Size hierarchy (name, section, subsection, normal, small)
  - Line height and letter spacing

Available font themes:
- **classic** - Georgia (headers) + Times New Roman (body)
- **modern** - Inter (headers) + Open Sans (body)
- **sharp** - Montserrat (headers) + Roboto (body)

### `color.rs` - Color Themes
Manages color palettes:
- **ColorTheme** - Complete color system including:
  - Primary (headers, links)
  - Secondary (emphasis)
  - Accent (highlights)
  - Text (body content)
  - Muted (secondary text)
  - Background
  - Surface (cards, sections)
  - Border

Available color themes:
- **classic** - Navy & Burgundy (traditional professional)
- **modern** - Electric Blue & Teal (tech-focused)
- **sharp** - Deep Purple & Hot Pink (creative/bold)

## Usage Example

```rust
use crate::themes::Theme;

// Create a theme combination
let theme = Theme::new("modern", "sharp")?;

// Access font settings
println!("Header font: {}", theme.font.header.family);
println!("Body size: {}", theme.font.body.size_normal);

// Access color settings
println!("Primary color: {}", theme.color.primary);
println!("Background: {}", theme.color.background);

// List available themes
let (fonts, colors) = Theme::available_themes();
println!("Font themes: {:?}", fonts);
println!("Color themes: {:?}", colors);
```

## Theme Definitions

### Font Specifications
Each font theme defines a complete typographic system:
- Carefully chosen font pairings
- Consistent size hierarchy
- Optimized line heights
- Professional weight variations

### Color Systems
Each color theme provides:
- High contrast for readability
- Professional color combinations
- Print-friendly selections
- Consistent color relationships

## Error Handling

- Unknown theme names return `CvError::UnknownTheme`
- Error messages include list of available themes
- Graceful fallbacks for missing fonts

## Design Principles

1. **Predefined Themes**: Limited, curated selection prevents bad combinations
2. **Semantic Colors**: Colors have meaning (primary, secondary, etc.)
3. **Print-First**: All themes work well in print
4. **Accessibility**: High contrast ratios for readability

## Font Files

The actual font files (.ttf) are stored in the `/fonts/` directory and embedded in the binary during compilation.

## Extending Themes

To add a new theme:

1. **Font Theme**:
   - Add new match arm in `FontTheme::load()`
   - Define complete `FontSpec` for header and body
   - Add to `AVAILABLE_THEMES` constant

2. **Color Theme**:
   - Add new match arm in `ColorTheme::load()`
   - Define all 8 color values
   - Add to `AVAILABLE_THEMES` constant

## Future Enhancements

- [ ] Theme inheritance/extension
- [ ] Custom theme loading from YAML
- [ ] Theme preview generation
- [ ] Dark mode variants
- [ ] Industry-specific themes (academic, tech, creative)
