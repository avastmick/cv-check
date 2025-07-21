//! Tests for structured output functionality

use cv_check::ai::schema_gen::JsonSchema;
use cv_check::ai::schemas::OptimizedExperience;
use cv_check::ai::AIClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Simple test struct for validating structured outputs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct BookSummary {
    title: String,
    author: String,
    summary: String,
    key_themes: Vec<String>,
    rating: f32,
}

impl JsonSchema for BookSummary {
    fn schema() -> Value {
        use cv_check::ai::schema_gen::SchemaBuilder;

        let mut builder = SchemaBuilder::new();
        builder
            .string("title", "The title of the book")
            .string("author", "The author of the book")
            .string("summary", "A brief summary of the book")
            .string_array("key_themes", "Main themes explored in the book")
            .number("rating", "Rating out of 5", Some(0.0), Some(5.0));
        builder.build()
    }
}

#[tokio::test]
#[ignore = "Requires AI API credentials"]
async fn test_structured_output_simple() {
    // This test will only run if environment variables are set
    let Ok(mut client) = AIClient::from_env() else {
        eprintln!("Skipping test: AI environment variables not set");
        return;
    };

    // Create a simple request that should return structured data
    let model = std::env::var("AI_MODEL").expect("AI_MODEL not set");
    let mut request = openai_api_rs::v1::chat_completion::ChatCompletionRequest::new(
        model,
        vec![
            openai_api_rs::v1::chat_completion::ChatCompletionMessage {
                role: openai_api_rs::v1::chat_completion::MessageRole::system,
                content: openai_api_rs::v1::chat_completion::Content::Text(
                    "You are a book reviewer. Extract information about books and provide structured summaries.".to_string()
                ),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            openai_api_rs::v1::chat_completion::ChatCompletionMessage {
                role: openai_api_rs::v1::chat_completion::MessageRole::user,
                content: openai_api_rs::v1::chat_completion::Content::Text(
                    "Please provide a summary of '1984' by George Orwell.".to_string()
                ),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ],
    );

    // Add structured output format
    let response_format = BookSummary::response_format("book_summary");
    request = request.response_format(response_format);

    // Make the API call
    let response = client
        .chat_completion_raw(request)
        .await
        .expect("API call should succeed");

    // Extract and parse the response
    let content = response
        .choices
        .first()
        .and_then(|choice| choice.message.content.as_ref())
        .expect("Response should contain content");

    let book_summary: BookSummary =
        serde_json::from_str(content).expect("Response should deserialize to BookSummary");

    // Validate the response
    assert_eq!(book_summary.title, "1984");
    assert!(book_summary.author.contains("Orwell"));
    assert!(!book_summary.summary.is_empty());
    assert!(!book_summary.key_themes.is_empty());
    assert!(book_summary.rating >= 0.0 && book_summary.rating <= 5.0);
}

#[tokio::test]
#[ignore = "Requires AI API credentials"]
async fn test_structured_output_nested() {
    // Test with more complex nested structures
    let Ok(mut client) = AIClient::from_env() else {
        eprintln!("Skipping test: AI environment variables not set");
        return;
    };

    // Create a request for work experience optimization
    let test_experience = OptimizedExperience {
        title: "Software Engineer".to_string(),
        company: "Tech Corp".to_string(),
        duration: "2020 - Present".to_string(),
        highlights: vec![
            "Led development of microservices".to_string(),
            "Improved performance by 50%".to_string(),
        ],
        relevance_score: 0.9,
    };

    let model = std::env::var("AI_MODEL").expect("AI_MODEL not set");
    let mut request = openai_api_rs::v1::chat_completion::ChatCompletionRequest::new(
        model,
        vec![
            openai_api_rs::v1::chat_completion::ChatCompletionMessage {
                role: openai_api_rs::v1::chat_completion::MessageRole::system,
                content: openai_api_rs::v1::chat_completion::Content::Text(
                    "You are an HR expert. Optimize the given work experience for a specific job."
                        .to_string(),
                ),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            openai_api_rs::v1::chat_completion::ChatCompletionMessage {
                role: openai_api_rs::v1::chat_completion::MessageRole::user,
                content: openai_api_rs::v1::chat_completion::Content::Text(format!(
                    "Optimize this experience for a Senior Developer role: {test_experience:?}"
                )),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ],
    );

    // Add structured output format
    let response_format = OptimizedExperience::response_format("optimized_experience");
    request = request.response_format(response_format);

    // Make the API call
    let response = client
        .chat_completion_raw(request)
        .await
        .expect("API call should succeed");

    // Extract and parse the response
    let content = response
        .choices
        .first()
        .and_then(|choice| choice.message.content.as_ref())
        .expect("Response should contain content");

    let optimized: OptimizedExperience =
        serde_json::from_str(content).expect("Response should deserialize to OptimizedExperience");

    // Validate the response structure
    assert!(!optimized.title.is_empty());
    assert!(!optimized.company.is_empty());
    assert!(!optimized.duration.is_empty());
    assert!(!optimized.highlights.is_empty());
    assert!(optimized.relevance_score >= 0.0 && optimized.relevance_score <= 1.0);
}
