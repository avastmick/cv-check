// CV Template for Typst
// This template provides a modern, themeable CV layout

#let cv(
  // Personal Information
  name: "",
  email: "",
  phone: "",
  location: "",
  linkedin: none,
  github: none,
  website: none,
  
  // Theme Configuration
  font-theme: "modern",
  
  // Color values (passed from Rust theme system)
  color-primary: rgb("#0066CC"),
  color-secondary: rgb("#00A8A8"),
  color-accent: rgb("#FF6B35"),
  color-text: rgb("#333333"),
  color-muted: rgb("#666666"),
  
  // Layout Options
  columns: 1,
  
  // Document Content
  body
) = {
  // Font Theme Definitions
  let font-themes = (
    classic: (
      header: "Georgia",
      body: "Times New Roman",
      header-size: 28pt,
      section-size: 16pt,
      body-size: 11pt,
    ),
    modern: (
      header: "Inter",
      body: "Open Sans",
      header-size: 28pt,
      section-size: 16pt,
      body-size: 11pt,
    ),
    sharp: (
      header: "Montserrat",
      body: "Roboto",
      header-size: 28pt,
      section-size: 16pt,
      body-size: 11pt,
    ),
  )
  
  // Get font configuration
  let fonts = font-themes.at(font-theme)
  
  // Use color values passed from Rust
  let colors = (
    primary: color-primary,
    secondary: color-secondary,
    accent: color-accent,
    text: color-text,
    muted: color-muted,
  )
  
  // Set document properties
  set document(title: name, author: name)
  set page(
    paper: "a4",
    margin: (top: 1.5cm, bottom: 1.5cm, left: 2cm, right: 2cm),
  )
  
  // Set text properties
  set text(
    font: fonts.body,
    size: fonts.body-size,
    fill: colors.text,
  )
  
  // Configure headings
  show heading.where(level: 1): it => {
    set text(
      font: fonts.header,
      size: fonts.section-size,
      fill: colors.primary,
      weight: "bold",
    )
    block(below: 0.5em)[
      #it.body
      #line(length: 100%, stroke: 0.5pt + colors.primary)
    ]
  }
  
  show heading.where(level: 2): it => {
    set text(
      font: fonts.header,
      size: fonts.body-size + 2pt,
      fill: colors.secondary,
      weight: "semibold",
    )
    block(above: 0.8em, below: 0.4em)[#it.body]
  }
  
  // Header section with personal information
  align(center)[
    #set text(font: fonts.header)
    
    // Name
    #text(size: fonts.header-size, weight: "bold", fill: colors.primary)[#name]
    
    #v(0.5em)
    
    // Contact information
    #text(size: fonts.body-size, fill: colors.text)[
      #email
      #if phone != "" [ | #phone ]
      #if location != "" [ | #location ]
    ]
    
    // Social links
    #if linkedin != none or github != none or website != none [
      #text(size: fonts.body-size - 1pt, fill: colors.secondary)[
        #if linkedin != none [
          #link("https://linkedin.com/in/" + linkedin)[LinkedIn: #linkedin]
        ]
        #if github != none [
          #if linkedin != none [ | ]
          #link("https://github.com/" + github)[GitHub: #github]
        ]
        #if website != none [
          #if linkedin != none or github != none [ | ]
          #link(website)[#website]
        ]
      ]
    ]
  ]
  
  #v(1em)
  
  // Main content
  if columns == 2 {
    // Two-column layout
    columns(2, gutter: 1.5em)[
      #body
    ]
  } else {
    // Single-column layout
    body
  }
}

// Helper function for job entries
#let job(
  title: "",
  company: "",
  dates: "",
  location: "",
  description: []
) = {
  block(above: 0.8em)[
    grid(
      columns: (1fr, auto),
      text(weight: "semibold")[#title],
      text(style: "italic")[#dates],
    )
    grid(
      columns: (1fr, auto),
      text(weight: "medium")[#company],
      if location != "" [#text(size: 0.9em)[#location]],
    )
    #description
  ]
}

// Helper function for education entries
#let education(
  degree: "",
  school: "",
  dates: "",
  location: "",
  details: []
) = {
  block(above: 0.8em)[
    grid(
      columns: (1fr, auto),
      text(weight: "semibold")[#degree],
      text(style: "italic")[#dates],
    )
    grid(
      columns: (1fr, auto),
      text(weight: "medium")[#school],
      if location != "" [#text(size: 0.9em)[#location]],
    )
    #details
  ]
}

// Helper function for skill sections
#let skills(..categories) = {
  for (category, items) in categories.named() {
    block(above: 0.5em)[
      text(weight: "semibold")[#category:] #items
    ]
  }
}