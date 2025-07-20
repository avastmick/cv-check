# Font Theme System

## Overview

Font themes provide pre-configured, professional font combinations that work well together. Users can select a theme by name or create custom combinations.

## Built-in Themes

### Classic
Professional and timeless, suitable for traditional industries.

```yaml
font_theme: classic
```

- **Headers**: Georgia or Garamond
- **Body**: Times New Roman or Palatino
- **Characteristics**: Serif fonts, formal appearance
- **Best for**: Law, academia, government, traditional corporations

### Modern
Clean and contemporary, perfect for tech and creative fields.

```yaml
font_theme: modern
```

- **Headers**: Inter or Helvetica Neue
- **Body**: Open Sans or Source Sans Pro
- **Characteristics**: Sans-serif, excellent readability
- **Best for**: Technology, startups, design, modern corporations

### Sharp
Bold and distinctive, makes a strong impression.

```yaml
font_theme: sharp
```

- **Headers**: Montserrat or Raleway
- **Body**: Roboto or Lato
- **Characteristics**: Geometric sans-serif, strong personality
- **Best for**: Creative industries, marketing, bold personal brands

## Theme Specifications

### Font Properties

Each theme defines:

```yaml
fonts:
  header:
    family: "Inter"
    weight_regular: 400
    weight_bold: 700
    size_name: 28pt
    size_section: 16pt
    size_subsection: 14pt
    letter_spacing: -0.02em
    
  body:
    family: "Open Sans"
    weight_regular: 400
    weight_bold: 600
    size_normal: 11pt
    size_small: 10pt
    line_height: 1.5
    
  accent:
    family: "Roboto Mono"  # For special elements
    size: 10pt
```

### Custom Font Configuration

Users can override any font setting:

```yaml
---
font_theme: modern
fonts:
  header:
    family: "Playfair Display"  # Override just the header font
---
```

Or define a completely custom theme:

```yaml
---
fonts:
  header:
    family: "Custom Font"
    weight_regular: 300
    weight_bold: 700
  body:
    family: "Another Font"
    weight_regular: 400
---
```

## Font Pairing Guidelines

### Contrast
- Pair serif headers with sans-serif body (Classic)
- Or use different weights of the same family (Sharp)

### Harmony
- Ensure x-heights are similar
- Match font personalities (playful/serious)

### Hierarchy
- Headers should be distinctive
- Body text must be highly readable
- Use weight and size for emphasis

## Implementation Details

### Font Loading Priority
1. System fonts (fastest)
2. Bundled fonts (reliable)
3. Web fonts (fallback)

### Fallback Chains
```yaml
header:
  family: "Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
body:
  family: "Open Sans, Helvetica, Arial, sans-serif"
```

### Performance Considerations
- Subset fonts to required characters
- Use variable fonts when available
- Optimize for common character sets

## Examples

### Classic Theme Applied
```
╔════════════════════════════════════════╗
║          JOHN DOE                      ║  <- Georgia, 28pt
║     Senior Software Engineer           ║  <- Georgia, 14pt
╠════════════════════════════════════════╣
║ EXPERIENCE                             ║  <- Georgia, 16pt, bold
║                                        ║
║ Tech Corp | 2020-Present               ║  <- Times New Roman, 11pt
║ Led development of cloud platforms...  ║  <- Times New Roman, 11pt
╚════════════════════════════════════════╝
```

### Modern Theme Applied
```
┌────────────────────────────────────────┐
│ John Doe                               │  <- Inter, 28pt, tight spacing
│ Senior Software Engineer               │  <- Inter, 14pt, light
├────────────────────────────────────────┤
│ Experience                             │  <- Inter, 16pt, medium
│                                        │
│ Tech Corp • 2020-Present               │  <- Open Sans, 11pt
│ Led development of cloud platforms...  │  <- Open Sans, 11pt, 1.5 line height
└────────────────────────────────────────┘
```