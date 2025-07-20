# User Guide

## Getting Started

This guide will help you create professional CVs and cover letters using our theme-based generation system.

## Quick Start

### 1. Create Your CV

Create a file named `cv.md` with your content:

```markdown
---
# Personal Information
name: Jane Smith
email: jane.smith@example.com
phone: +1 (555) 123-4567
location: New York, NY
linkedin: janesmith
github: janesmith

# Theme Selection
font_theme: modern
color_theme: modern

# Layout Options
layout:
  columns: 1
---

# Professional Summary

Innovative software engineer with 10+ years of experience building scalable web applications and leading development teams.

# Experience

## Senior Software Engineer
**Tech Innovations Inc.** | *Jan 2020 - Present*

- Led development of microservices architecture serving 5M+ users
- Mentored team of 8 engineers
- Reduced deployment time by 70% through CI/CD improvements

## Software Engineer
**StartupXYZ** | *Jun 2016 - Dec 2019*

- Built core product features using React and Node.js
- Implemented real-time data processing pipeline
- Contributed to open source projects

# Education

## M.S. Computer Science
**Stanford University** | *2014 - 2016*

## B.S. Computer Science
**UC Berkeley** | *2010 - 2014*

# Skills

**Languages**: JavaScript, Python, Go, Rust
**Frameworks**: React, Node.js, Django, FastAPI
**Tools**: Docker, Kubernetes, AWS, PostgreSQL
```

### 2. Create Your Cover Letter

Create a file named `cover-letter.md`:

```markdown
---
# Recipient Information
recipient:
  name: Hiring Manager
  title: Engineering Team
  company: Dream Company Inc.
  address: |
    123 Main Street
    San Francisco, CA 94105

# Your Information (reuse from CV)
name: Jane Smith
email: jane.smith@example.com
phone: +1 (555) 123-4567
location: New York, NY

# Theme (match your CV)
font_theme: modern
color_theme: modern

# Letter metadata
date: January 15, 2024
subject: Senior Software Engineer Position
---

Dear Hiring Manager,

I am writing to express my strong interest in the Senior Software Engineer position at Dream Company Inc. With over 10 years of experience building scalable applications and leading development teams, I am excited about the opportunity to contribute to your innovative projects.

In my current role at Tech Innovations Inc., I have:
- Architected microservices handling 5M+ daily active users
- Led a team of 8 engineers through successful product launches
- Reduced operational costs by 40% through optimization

Your company's focus on [specific company initiative] aligns perfectly with my experience in [relevant experience]. I am particularly drawn to [specific aspect of company/role].

I would welcome the opportunity to discuss how my skills and experience can contribute to your team's continued success.

Thank you for considering my application.

Sincerely,
Jane Smith
```

### 3. Generate Your Documents

```bash
# Generate CV
cv build cv.md

# Generate cover letter
cv build cover-letter.md

# Watch for changes and auto-rebuild
cv watch cv.md

# Generate specific format
cv build cv.md --format pdf
cv build cv.md --format docx
cv build cv.md --format html
```

## Theme Configuration

### Choosing Themes

Select from our pre-designed themes:

#### Font Themes
- **classic**: Traditional serif fonts (Georgia/Times New Roman)
- **modern**: Clean sans-serif fonts (Inter/Open Sans)  
- **sharp**: Bold geometric fonts (Montserrat/Roboto)

#### Color Themes
- **classic**: Navy and burgundy (finance, law, government)
- **modern**: Blue and teal (tech, startups, design)
- **sharp**: Purple and pink (creative, marketing, bold brands)

### Mixing Themes

You can mix font and color themes:

```yaml
font_theme: modern
color_theme: classic
```

### Custom Themes

Override specific settings:

```yaml
# Use modern fonts but customize header
font_theme: modern
fonts:
  header:
    family: "Playfair Display"
    size_name: 32pt

# Use modern colors but customize primary
color_theme: modern
colors:
  primary: "#0A66C2"  # LinkedIn blue
```

## Layout Options

### Single Column (Default)
Traditional layout, maximum compatibility:

```yaml
layout:
  columns: 1
```

### Two Column
Modern layout with sidebar:

```yaml
layout:
  columns: 2
  sidebar: left  # or right
```

### Margins
Adjust page margins:

```yaml
layout:
  margins:
    top: 1.5cm
    bottom: 1.5cm
    left: 2cm
    right: 2cm
```

## Advanced Customization

### Custom Sections

Add custom sections using Markdown headers:

```markdown
# Certifications

- AWS Certified Solutions Architect
- Google Cloud Professional
- Certified Kubernetes Administrator

# Publications

- "Scaling Microservices" - Tech Journal 2023
- "Real-time Data Processing" - Conference Paper 2022
```

### Highlighting

Use standard Markdown formatting:

```markdown
**Bold** for emphasis
*Italic* for titles
`Code` for technical terms
> Blockquotes for testimonials
```

### Lists

```markdown
# Unordered lists for skills
- Python
- JavaScript
- Go

# Ordered lists for process
1. Analyze requirements
2. Design architecture
3. Implement solution
```

## Tips and Best Practices

### Content Guidelines

1. **Be Concise**: Use bullet points and short paragraphs
2. **Quantify Results**: Include numbers and percentages
3. **Action Verbs**: Start bullets with strong verbs
4. **Relevance**: Tailor content to the position

### Theme Selection

- **Classic**: Traditional industries (finance, law)
- **Modern**: Tech companies and startups
- **Sharp**: Creative roles and agencies

### File Organization

```
your-resume/
├── cv.md                 # Your main CV
├── cover-letter.md       # Cover letter template
├── cv-tech.md           # Tech-focused variant
├── cv-manager.md        # Management-focused variant
└── output/              # Generated PDFs
```

### Version Control

Track changes with Git:

```bash
git init
git add cv.md cover-letter.md
git commit -m "Initial CV and cover letter"
```

## Troubleshooting

### Common Issues

**Fonts not displaying correctly**
- Ensure font files are installed
- Check font name spelling
- Try a different font theme

**Colors look different in print**
- Use print preview
- Adjust color values for CMYK
- Test on target printer

**Content overflow**
- Reduce font sizes
- Use compact margins
- Consider two-column layout
- Create summary version

### Getting Help

1. Check documentation in `docs/` directory
2. Review examples in `examples/` directory
3. Submit issues on GitHub
4. Contact support

## Examples

Find complete examples in the `examples/` directory:

- `examples/cv-software-engineer.md` - Tech industry CV
- `examples/cv-manager.md` - Management position CV
- `examples/cv-designer.md` - Creative industry CV
- `examples/cover-letter-tech.md` - Tech cover letter
- `examples/cv-academic.md` - Academic CV

Each example demonstrates different themes and layouts appropriate for the industry.