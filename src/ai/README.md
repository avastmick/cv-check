# AI Module

This module provides AI-powered CV tailoring functionality using OpenAI-compatible APIs.

## Purpose

Automatically optimize CVs for specific job descriptions by:
- Extracting requirements from job description PDFs
- Analyzing CV content against job requirements
- Generating tailored CV content with optimized keywords
- Ensuring ATS (Applicant Tracking System) compatibility

## Module Structure

### `mod.rs` - Module Interface
Re-exports public types and functions for the AI module.

### `client.rs` - AI API Client
Manages communication with OpenAI-compatible APIs:
- **AIClient** - Main client struct handling API requests
- **tailor_cv()** - Sends CV + job description for AI processing
- Supports structured JSON outputs for reliability
- Configurable via environment variables

### `pdf_parser.rs` - PDF Text Extraction
Extracts text content from job description PDFs:
- **extract_text_from_pdf()** - Main extraction function
- Handles various PDF formats and encodings
- Returns clean text suitable for AI processing

### `prompts.rs` - AI Prompt Engineering
Contains carefully crafted prompts for CV optimization:
- **SYSTEM_PROMPT** - Establishes AI as HR expert
- **generate_user_prompt()** - Creates specific tailoring prompts
- Includes instructions for structured output generation

### `schema_gen.rs` - JSON Schema Generation
Provides trait and utilities for generating JSON schemas:
- **JsonSchema** trait - For types that can generate schemas
- **SchemaBuilder** - Programmatic schema construction
- Ensures AI responses match expected structure

### `schemas.rs` - Structured Output Types
Defines the data structures for AI responses:
- **TailoredCV** - Complete tailored CV structure
- **OptimizedExperience** - Individual experience entries
- **OptimizedSkill** - Skills with relevance scores
- All types implement serde for JSON serialization

## Configuration

The AI module requires these environment variables:

```bash
# Required
AI_ENDPOINT=https://api.openai.com/v1  # API endpoint
AI_MODEL=gpt-4o-2024-08-06            # Model name
AI_API_KEY=your-api-key-here          # API key

# Optional
AI_MAX_TOKENS=4000                    # Max response tokens
AI_TEMPERATURE=0.7                    # Response creativity
```

## Usage Example

```rust
use crate::ai::client::AIClient;

// Initialize client from environment
let client = AIClient::from_env()?;

// Tailor CV to job description
let tailored = client.tailor_cv(
    &cv_content,
    &job_description
).await?;

// Access optimized content
println!("Summary: {}", tailored.professional_summary);
for exp in &tailored.experiences {
    println!("- {} at {}", exp.title, exp.company);
}
```

## Error Handling

- API errors return `CvError::AIClient` with context
- PDF parsing errors return `CvError::PdfExtraction`
- Missing environment variables return configuration errors
- Network timeouts handled gracefully with retries

## Design Decisions

1. **Structured Outputs**: Uses JSON schemas to ensure reliable, parseable responses
2. **Environment Config**: Keeps API keys secure via environment variables
3. **Async/Await**: All API calls are asynchronous for better performance
4. **HR Expertise**: Prompts designed with professional HR knowledge
5. **Modular Design**: Each component has a single, clear responsibility

## Dependencies

- `reqwest`: HTTP client for API calls
- `tokio`: Async runtime
- `serde_json`: JSON serialization
- `pdf_extract`: PDF text extraction
- `anyhow`: Error handling

## Future Enhancements

- [ ] Support for multiple AI providers (Anthropic, Cohere, etc.)
- [ ] Caching of API responses to reduce costs
- [ ] Batch processing of multiple job descriptions
- [ ] Fine-tuning support for industry-specific CVs
- [ ] Cover letter generation from CV + job description
