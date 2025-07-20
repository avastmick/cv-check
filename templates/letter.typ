// Cover Letter Template for Typst
// This template provides a professional, themeable cover letter layout

#let letter(
  // Sender Information
  name: "",
  email: "",
  phone: "",
  location: "",
  
  // Recipient Information
  recipient-name: "Hiring Manager",
  recipient-title: "",
  company: "",
  company-address: "",
  
  // Letter Metadata
  date: datetime.today().display("[month repr:long] [day], [year]"),
  subject: none,
  
  // Theme Configuration
  font-theme: "modern",
  color-theme: "modern",
  
  // Document Content
  body
) = {
  // Font Theme Definitions
  let font-themes = (
    classic: (
      header: "Georgia",
      body: "Times New Roman",
      header-size: 14pt,
      body-size: 11pt,
    ),
    modern: (
      header: "Inter",
      body: "Open Sans",
      header-size: 14pt,
      body-size: 11pt,
    ),
    sharp: (
      header: "Montserrat",
      body: "Roboto",
      header-size: 14pt,
      body-size: 11pt,
    ),
  )
  
  // Color Theme Definitions
  let color-themes = (
    classic: (
      primary: rgb("#2C3E50"),    // Navy
      secondary: rgb("#34495E"),   // Dark Gray
      accent: rgb("#8B0000"),      // Burgundy
      text: rgb("#2C2C2C"),        // Charcoal
      muted: rgb("#7F7F7F"),       // Gray
    ),
    modern: (
      primary: rgb("#0066CC"),     // Electric Blue
      secondary: rgb("#00A8A8"),   // Teal
      accent: rgb("#FF6B35"),      // Orange
      text: rgb("#333333"),        // Dark Gray
      muted: rgb("#666666"),       // Medium Gray
    ),
    sharp: (
      primary: rgb("#6B46C1"),     // Deep Purple
      secondary: rgb("#EC4899"),   // Hot Pink
      accent: rgb("#84CC16"),      // Lime
      text: rgb("#1A1A1A"),        // Near Black
      muted: rgb("#6B7280"),       // Cool Gray
    ),
  )
  
  // Get theme configurations
  let fonts = font-themes.at(font-theme)
  let colors = color-themes.at(color-theme)
  
  // Set document properties
  set document(title: "Cover Letter - " + name, author: name)
  set page(
    paper: "a4",
    margin: (top: 2.5cm, bottom: 2.5cm, left: 2.5cm, right: 2.5cm),
  )
  
  // Set text properties
  set text(
    font: fonts.body,
    size: fonts.body-size,
    fill: colors.text,
  )
  
  // Set paragraph spacing
  set par(
    justify: true,
    leading: 0.65em,
  )
  
  show par: set block(below: 1em)
  
  // Header with sender information
  align(right)[
    #text(font: fonts.header, size: fonts.header-size, weight: "bold", fill: colors.primary)[#name]
    #v(0.3em)
    #text(size: fonts.body-size - 1pt)[
      #email
      #if phone != "" [#linebreak()#phone]
      #if location != "" [#linebreak()#location]
    ]
  ]
  
  #v(2em)
  
  // Date
  align(left)[
    #text()[#date]
  ]
  
  #v(1.5em)
  
  // Recipient information
  align(left)[
    #text(weight: "medium")[
      #recipient-name
      #if recipient-title != "" [#linebreak()#recipient-title]
      #if company != "" [#linebreak()#company]
      #if company-address != "" [
        #for line in company-address.split("\n") [
          #linebreak()#line
        ]
      ]
    ]
  ]
  
  #v(1.5em)
  
  // Subject line (optional)
  #if subject != none [
    text(weight: "bold")[Re: #subject]
    #v(1.5em)
  ]
  
  // Letter body
  body
  
  #v(1.5em)
  
  // Closing
  align(left)[
    Sincerely,
    #v(3em)
    #text(font: fonts.header, weight: "medium")[#name]
  ]
}

// Helper function for letter paragraphs with proper spacing
#let para(content) = {
  block(below: 1em)[#content]
}