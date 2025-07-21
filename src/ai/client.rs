//! OpenAI-compatible API client for CV tailoring

use crate::ai::prompts::{create_user_prompt, SYSTEM_PROMPT};
use crate::ai::schema_gen::JsonSchema;
use crate::ai::schemas::TailoredCV;
use crate::ai::{AIError, Result};
use log::{debug, error, info};
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};

/// OpenAI-compatible API client
pub struct AIClient {
    pub(crate) client: OpenAIClient,
    pub(crate) model: String,
}

impl AIClient {
    /// Create a new AI client from environment variables
    ///
    /// # Errors
    ///
    /// Returns an error if any required environment variables are not set or if client creation fails
    pub fn from_env() -> Result<Self> {
        let endpoint =
            std::env::var("AI_ENDPOINT").map_err(|_| AIError::EnvVar("AI_ENDPOINT".to_string()))?;
        let api_key =
            std::env::var("AI_API_KEY").map_err(|_| AIError::EnvVar("AI_API_KEY".to_string()))?;
        let model =
            std::env::var("AI_MODEL").map_err(|_| AIError::EnvVar("AI_MODEL".to_string()))?;

        info!("Creating AI client with endpoint: {endpoint}");
        info!("Using model: {model}");
        debug!("API key length: {}", api_key.len());

        let client = OpenAIClient::builder()
            .with_endpoint(endpoint)
            .with_api_key(api_key)
            .build()
            .map_err(|e| AIError::InvalidResponse(format!("Failed to build client: {e}")))?;

        info!("AI client created successfully");
        Ok(Self { client, model })
    }

    /// Make a raw chat completion request with a custom response format
    ///
    /// This method allows direct access to the `OpenAI` API with custom schemas,
    /// useful for cases beyond CV tailoring such as extracting structured data
    /// from documents or implementing custom AI workflows.
    ///
    /// ONLY USED IN INTEGRATION TESTS
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails
    #[allow(dead_code)] // `allow(dead_code)` exception
    pub async fn chat_completion_raw(
        &mut self,
        request: ChatCompletionRequest,
    ) -> Result<chat_completion::ChatCompletionResponse> {
        self.client
            .chat_completion(request)
            .await
            .map_err(|e| AIError::InvalidResponse(format!("API request failed: {e}")))
    }

    /// Tailor a CV for a specific job description
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or if the response cannot be parsed
    pub async fn tailor_cv(
        &mut self,
        cv_content: &str,
        job_description: &str,
    ) -> Result<TailoredCV> {
        let mut request = ChatCompletionRequest::new(
            self.model.clone(),
            vec![
                chat_completion::ChatCompletionMessage {
                    role: chat_completion::MessageRole::system,
                    content: chat_completion::Content::Text(SYSTEM_PROMPT.to_string()),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                },
                chat_completion::ChatCompletionMessage {
                    role: chat_completion::MessageRole::user,
                    content: chat_completion::Content::Text(create_user_prompt(
                        cv_content,
                        job_description,
                    )),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                },
            ],
        );

        // Set up structured output using the generated JSON schema
        let response_format = TailoredCV::response_format("tailored_cv");
        debug!(
            "Request response format: {}",
            serde_json::to_string_pretty(&response_format).unwrap_or_default()
        );
        request = request.response_format(response_format);

        info!("Sending request to AI API endpoint");
        debug!("Request: {request:?}");

        let response = match self.client.chat_completion(request).await {
            Ok(resp) => {
                debug!("Got successful response from API");
                resp
            }
            Err(e) => {
                error!("API request failed: {e}");
                error!("Error type: {e:?}");
                return Err(AIError::InvalidResponse(format!("API request failed: {e}")));
            }
        };

        debug!("Received response from API");

        // Extract the content from the response
        let content = response
            .choices
            .first()
            .and_then(|choice| choice.message.content.as_ref())
            .ok_or_else(|| {
                error!("No content in API response");
                debug!("Full response: {response:?}");
                AIError::InvalidResponse("No content in response".to_string())
            })?;

        debug!("Response content: {content}");

        // Strip markdown code blocks if present
        let json_content = if content.starts_with("```json") && content.ends_with("```") {
            info!("Stripping markdown JSON code block");
            content
                .trim_start_matches("```json")
                .trim_end_matches("```")
                .trim()
        } else if content.starts_with("```") && content.ends_with("```") {
            info!("Stripping markdown code block");
            content
                .trim_start_matches("```")
                .trim_end_matches("```")
                .trim()
        } else {
            content
        };

        debug!("Cleaned content: {json_content}");

        // Parse the JSON content into our structured type
        info!("Parsing response JSON");
        let tailored_cv: TailoredCV = serde_json::from_str(json_content).map_err(|e| {
            error!("Failed to parse response JSON: {e}");
            error!("Raw content: {content}");
            error!("Cleaned content: {json_content}");
            debug!(
                "Expected schema: {}",
                serde_json::to_string_pretty(&TailoredCV::schema()).unwrap_or_default()
            );
            e
        })?;

        info!("Successfully parsed tailored CV");
        Ok(tailored_cv)
    }
}
