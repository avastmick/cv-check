# Color Theme System

## Overview

Color themes provide coordinated color palettes that create visual hierarchy and professional appeal. Each theme is carefully designed to work across different industries and personal brands.

## Built-in Themes

### Classic
Traditional and conservative, perfect for established industries.

```yaml
color_theme: classic
```

- **Primary**: Navy Blue (#2C3E50)
- **Secondary**: Dark Gray (#34495E)
- **Accent**: Burgundy (#8B0000)
- **Text**: Charcoal (#2C2C2C)
- **Background**: Warm White (#FAFAFA)
- **Best for**: Finance, law, government, traditional corporations

### Modern
Clean and contemporary, ideal for technology and innovation.

```yaml
color_theme: modern
```

- **Primary**: Electric Blue (#0066CC)
- **Secondary**: Teal (#00A8A8)
- **Accent**: Orange (#FF6B35)
- **Text**: Dark Gray (#333333)
- **Background**: Pure White (#FFFFFF)
- **Best for**: Technology, startups, design, modern corporations

### Sharp
Bold and distinctive, makes a memorable impression.

```yaml
color_theme: sharp
```

- **Primary**: Deep Purple (#6B46C1)
- **Secondary**: Hot Pink (#EC4899)
- **Accent**: Lime Green (#84CC16)
- **Text**: Near Black (#1A1A1A)
- **Background**: Cool White (#F8FAFC)
- **Best for**: Creative industries, marketing, personal branding

## Theme Specifications

### Color Properties

Each theme defines:

```yaml
colors:
  primary: "#0066CC"      # Headers, section titles
  secondary: "#00A8A8"    # Subheadings, highlights
  accent: "#FF6B35"       # Links, special elements
  text: "#333333"         # Body text
  muted: "#6B7280"        # Secondary text
  background: "#FFFFFF"   # Page background
  surface: "#F3F4F6"      # Boxes, cards
  border: "#E5E7EB"       # Dividers, lines
```

### Custom Color Configuration

Override specific colors:

```yaml
---
color_theme: modern
colors:
  primary: "#1E40AF"  # Override just the primary color
---
```

Or define a completely custom palette:

```yaml
---
colors:
  primary: "#D97706"
  secondary: "#DC2626"
  accent: "#059669"
  text: "#1F2937"
  muted: "#6B7280"
  background: "#FFFBF3"
---
```

## Color Theory Guidelines

### Contrast Ratios
- Text on background: Minimum 7:1 (WCAG AAA)
- Headers on background: Minimum 4.5:1 (WCAG AA)
- Ensure readability in print and digital

### Color Harmony
- **Complementary**: Primary and accent colors opposite on color wheel
- **Analogous**: Colors adjacent for subtle variation
- **Triadic**: Three evenly spaced colors for vibrancy

### Professional Considerations
- **Print-friendly**: Colors that reproduce well in CMYK
- **Grayscale-safe**: Maintain hierarchy when printed in B&W
- **Cultural sensitivity**: Consider color meanings globally

## Implementation Details

### Color Spaces
```yaml
# RGB for digital
primary_rgb: "0, 102, 204"

# HSL for variations
primary_hsl: "210, 100%, 40%"

# CMYK for print
primary_cmyk: "100, 50, 0, 20"
```

### Shade Generation
Automatically generate color variations:
- **Lighter**: For backgrounds, hover states
- **Darker**: For emphasis, active states
- **Muted**: For secondary content

### Accessibility
- All themes tested for WCAG compliance
- High contrast mode available
- Color-blind friendly palettes

## Examples

### Classic Theme Applied
```
╔════════════════════════════════════════╗
║ JOHN DOE                               ║  <- Navy (#2C3E50)
║ Senior Software Engineer               ║  <- Dark Gray (#34495E)
╠════════════════════════════════════════╣
║ EXPERIENCE                             ║  <- Navy (#2C3E50)
║                                        ║
║ Tech Corp | 2020-Present               ║  <- Charcoal (#2C2C2C)
║ Led development teams...               ║  <- Charcoal (#2C2C2C)
║                                        ║
║ Key Achievement                        ║  <- Burgundy accent (#8B0000)
╚════════════════════════════════════════╝
```

### Modern Theme Applied
```
┌────────────────────────────────────────┐
│ John Doe                               │  <- Electric Blue (#0066CC)
│ Senior Software Engineer               │  <- Teal (#00A8A8)
├────────────────────────────────────────┤
│ Experience                             │  <- Electric Blue (#0066CC)
│                                        │
│ Tech Corp • 2020-Present               │  <- Dark Gray (#333333)
│ Led development teams...               │  <- Dark Gray (#333333)
│                                        │
│ View Portfolio →                       │  <- Orange link (#FF6B35)
└────────────────────────────────────────┘
```

### Sharp Theme Applied
```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ JOHN DOE                               ┃  <- Deep Purple (#6B46C1)
┃ Creative Director                      ┃  <- Hot Pink (#EC4899)
┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃ EXPERIENCE                             ┃  <- Deep Purple (#6B46C1)
┃                                        ┃
┃ Design Studio ▪ 2020-Present           ┃  <- Near Black (#1A1A1A)
┃ Leading creative innovation...         ┃  <- Near Black (#1A1A1A)
┃                                        ┃
┃ ★ Award Winner                         ┃  <- Lime accent (#84CC16)
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

## Theme Combinations

### Recommended Pairings
- **Classic Font + Classic Color**: Traditional, timeless
- **Modern Font + Modern Color**: Clean, contemporary
- **Sharp Font + Sharp Color**: Bold, creative
- **Modern Font + Classic Color**: Updated traditional
- **Classic Font + Modern Color**: Established but current

### Industry Guidelines
- **Finance/Law**: Classic or Classic/Modern hybrid
- **Technology**: Modern or Sharp
- **Creative**: Sharp or Modern/Sharp hybrid
- **Academia**: Classic with subtle modern touches
- **Healthcare**: Modern with calming colors