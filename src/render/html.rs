use crate::parser::Document;
use crate::render::RenderEngine;
use crate::themes::Theme;
use anyhow::Result;
use pulldown_cmark::html;
use std::path::Path;

pub struct HtmlRenderer {
    _template: Option<String>,
}

impl HtmlRenderer {
    /// Creates a new HTML renderer with optional custom template.
    ///
    /// # Errors
    ///
    /// Returns an error if the template file cannot be read.
    pub fn new(template_path: Option<&Path>) -> Result<Self> {
        let template = if let Some(path) = template_path {
            Some(std::fs::read_to_string(path)?)
        } else {
            None
        };

        Ok(Self {
            _template: template,
        })
    }

    fn generate_html(doc: &Document, theme: &Theme) -> String {
        let mut html_output = String::new();
        html::push_html(&mut html_output, doc.markdown_ast.iter().cloned());

        // Build complete HTML document
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - CV</title>
    <style>
        :root {{
            --primary: {};
            --secondary: {};
            --accent: {};
            --text: {};
            --muted: {};
            --background: {};
        }}
        
        body {{
            font-family: {}, sans-serif;
            color: var(--text);
            background: var(--background);
            line-height: 1.6;
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
        }}
        
        h1 {{
            font-family: {}, sans-serif;
            color: var(--primary);
            border-bottom: 2px solid var(--primary);
            padding-bottom: 0.5rem;
        }}
        
        h2 {{
            color: var(--secondary);
            margin-top: 2rem;
        }}
        
        a {{
            color: var(--accent);
            text-decoration: none;
        }}
        
        a:hover {{
            text-decoration: underline;
        }}
        
        .header {{
            text-align: center;
            margin-bottom: 2rem;
        }}
        
        .contact {{
            color: var(--muted);
            font-size: 0.9rem;
        }}
    </style>
</head>
<body>
    <div class="header">
        <h1>{}</h1>
        <div class="contact">
            {} | {}
            {} {}
        </div>
    </div>
    
    {}
</body>
</html>"#,
            doc.metadata.name,
            theme.color.primary,
            theme.color.secondary,
            theme.color.accent,
            theme.color.text,
            theme.color.muted,
            theme.color.background,
            theme.font.body.family,
            theme.font.header.family,
            doc.metadata.name,
            doc.metadata.email,
            doc.metadata.phone.as_deref().unwrap_or(""),
            doc.metadata.location.as_deref().unwrap_or(""),
            if let Some(linkedin) = &doc.metadata.linkedin {
                format!(r#"| <a href="https://linkedin.com/in/{linkedin}">LinkedIn</a>"#)
            } else {
                String::new()
            },
            html_output
        )
    }
}

impl RenderEngine for HtmlRenderer {
    fn render(&self, doc: &Document, theme: &Theme, output: &Path) -> Result<()> {
        let html = Self::generate_html(doc, theme);
        std::fs::write(output, html)?;
        Ok(())
    }
}
