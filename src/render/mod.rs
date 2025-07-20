pub mod docx;
pub mod html;
pub mod pdf;

use crate::error::CvError;
use crate::parser::Document;
use crate::themes::Theme;
use anyhow::Result;
use std::path::Path;

pub trait RenderEngine {
    /// Renders a document to the specified output path.
    ///
    /// # Errors
    ///
    /// Returns an error if rendering fails.
    fn render(&self, doc: &Document, theme: &Theme, output: &Path) -> Result<()>;
}

pub struct Renderer {
    engine: Box<dyn RenderEngine>,
}

impl Renderer {
    /// Creates a new renderer for the specified format.
    ///
    /// # Errors
    ///
    /// Returns an error if the format is unsupported or template cannot be loaded.
    pub fn new(format: &str, template: Option<&Path>) -> Result<Self> {
        let engine: Box<dyn RenderEngine> = match format {
            "pdf" => Box::new(pdf::PdfRenderer::new(template)?),
            "docx" => Box::new(docx::DocxRenderer::new(template)?),
            "html" => Box::new(html::HtmlRenderer::new(template)?),
            _ => {
                return Err(CvError::InvalidFormat {
                    format: format.to_string(),
                }
                .into())
            }
        };

        Ok(Self { engine })
    }

    /// Renders a document using the configured render engine.
    ///
    /// # Errors
    ///
    /// Returns an error if rendering fails.
    pub fn render(&self, doc: &Document, theme: &Theme, output: &Path) -> Result<()> {
        self.engine.render(doc, theme, output)
    }
}
