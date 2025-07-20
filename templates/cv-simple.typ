// Simple CV Template for Typst that handles raw text content
// This template is used when we have markdown content that hasn't been converted to Typst format

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
  
  // Header
  align(center)[
    #text(font: fonts.header, size: fonts.header-size, weight: "bold", fill: colors.primary)[#name]
    
    #v(0.5em)
    
    #text(size: 10pt)[
      #email
      #if phone != "" [ | #phone]
      #if location != "" [ | #location]
    ]
    
    #if website != none or linkedin != none or github != none [
      #v(0.3em)
      #text(size: 10pt, fill: colors.secondary)[
        #if website != none [#link("https://" + website)[#website]]
        #if website != none and (linkedin != none or github != none) [ | ]
        #if linkedin != none [LinkedIn: #linkedin]
        #if linkedin != none and github != none [ | ]
        #if github != none [GitHub: #github]
      ]
    ]
  ]
  
  #v(1em)
  #line(length: 100%, stroke: 0.5pt + colors.muted)
  #v(1em)
  
  // Body content - display as raw text to avoid conflicts with markdown syntax
  #raw(body, lang: "markdown")
}