//! JSON schema definitions for structured AI outputs

use crate::ai::schema_gen::{JsonSchema, SchemaBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The complete tailored CV response from the AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TailoredCV {
    /// Optimized professional summary targeting the job
    pub professional_summary: String,

    /// Reordered and optimized experiences
    pub experiences: Vec<OptimizedExperience>,

    /// Skills extracted and prioritized for the job
    pub skills: Vec<String>,

    /// Key keywords to include for ATS optimization
    pub keywords: Vec<String>,

    /// Additional suggestions for CV improvement
    pub suggestions: Vec<String>,
}

/// An individual work experience optimized for the job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedExperience {
    /// Job title
    pub title: String,

    /// Company name
    pub company: String,

    /// Duration (e.g., "2020 - Present")
    pub duration: String,

    /// Bullet points highlighting relevant achievements
    pub highlights: Vec<String>,

    /// How relevant this experience is to the target job (0.0 - 1.0)
    pub relevance_score: f32,
}

impl JsonSchema for OptimizedExperience {
    fn schema() -> Value {
        let mut builder = SchemaBuilder::new();
        builder
            .string("title", "Job title")
            .string("company", "Company name")
            .string("duration", "Duration (e.g., '2020 - Present')")
            .string_array(
                "highlights",
                "Bullet points highlighting relevant achievements",
            )
            .number(
                "relevance_score",
                "How relevant this experience is to the target job",
                Some(0.0),
                Some(1.0),
            );
        builder.build()
    }
}

impl JsonSchema for TailoredCV {
    fn schema() -> Value {
        let mut builder = SchemaBuilder::new();
        builder
            .string("professional_summary", "A tailored professional summary that highlights skills and experience relevant to the job")
            .object_array("experiences", "Reordered and optimized experiences", &OptimizedExperience::schema())
            .string_array("skills", "Skills relevant to the job, ordered by importance")
            .string_array("keywords", "Keywords from the job description to include in the CV")
            .string_array("suggestions", "Additional suggestions for improving the CV");
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_optimized_experience_serialization() {
        let experience = OptimizedExperience {
            title: "Senior Developer".to_string(),
            company: "Tech Corp".to_string(),
            duration: "2020 - Present".to_string(),
            highlights: vec![
                "Led team of 5".to_string(),
                "Improved performance by 50%".to_string(),
            ],
            relevance_score: 0.95,
        };

        let json = serde_json::to_value(&experience).expect("Failed to serialize");

        assert_eq!(json["title"], "Senior Developer");
        assert_eq!(json["company"], "Tech Corp");
        assert_eq!(json["duration"], "2020 - Present");
        assert_eq!(
            json["highlights"]
                .as_array()
                .expect("highlights should be array")
                .len(),
            2
        );
        assert!(
            (json["relevance_score"]
                .as_f64()
                .expect("relevance_score should be number")
                - 0.95)
                .abs()
                < 0.00001
        );
    }

    #[test]
    fn test_optimized_experience_deserialization() {
        let json = json!({
            "title": "Software Engineer",
            "company": "StartupXYZ",
            "duration": "2018 - 2020",
            "highlights": ["Built API", "Reduced costs"],
            "relevance_score": 0.75
        });

        let experience: OptimizedExperience =
            serde_json::from_value(json).expect("Failed to deserialize");

        assert_eq!(experience.title, "Software Engineer");
        assert_eq!(experience.company, "StartupXYZ");
        assert_eq!(experience.duration, "2018 - 2020");
        assert_eq!(experience.highlights.len(), 2);
        assert!((experience.relevance_score - 0.75).abs() < f32::EPSILON);
    }

    #[test]
    fn test_tailored_cv_serialization() {
        let cv = TailoredCV {
            professional_summary: "Experienced developer...".to_string(),
            experiences: vec![OptimizedExperience {
                title: "Lead Dev".to_string(),
                company: "BigCo".to_string(),
                duration: "2021 - Present".to_string(),
                highlights: vec!["Achievement 1".to_string()],
                relevance_score: 0.9,
            }],
            skills: vec!["Rust".to_string(), "Python".to_string()],
            keywords: vec!["agile".to_string(), "cloud".to_string()],
            suggestions: vec!["Add more metrics".to_string()],
        };

        let json = serde_json::to_value(&cv).expect("Failed to serialize");

        assert_eq!(json["professional_summary"], "Experienced developer...");
        assert_eq!(
            json["experiences"]
                .as_array()
                .expect("experiences should be array")
                .len(),
            1
        );
        assert_eq!(
            json["skills"]
                .as_array()
                .expect("skills should be array")
                .len(),
            2
        );
        assert_eq!(
            json["keywords"]
                .as_array()
                .expect("keywords should be array")
                .len(),
            2
        );
        assert_eq!(
            json["suggestions"]
                .as_array()
                .expect("suggestions should be array")
                .len(),
            1
        );
    }

    #[test]
    fn test_tailored_cv_deserialization() {
        let json = json!({
            "professional_summary": "Summary text",
            "experiences": [{
                "title": "Developer",
                "company": "Company",
                "duration": "2020 - 2021",
                "highlights": ["Did stuff"],
                "relevance_score": 0.8
            }],
            "skills": ["Java", "Spring"],
            "keywords": ["microservices"],
            "suggestions": ["Improve formatting"]
        });

        let cv: TailoredCV = serde_json::from_value(json).expect("Failed to deserialize");

        assert_eq!(cv.professional_summary, "Summary text");
        assert_eq!(cv.experiences.len(), 1);
        assert_eq!(cv.skills.len(), 2);
        assert_eq!(cv.keywords.len(), 1);
        assert_eq!(cv.suggestions.len(), 1);
    }

    #[test]
    fn test_optimized_experience_schema() {
        let schema = OptimizedExperience::schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["title"].is_object());
        assert!(schema["properties"]["company"].is_object());
        assert!(schema["properties"]["duration"].is_object());
        assert!(schema["properties"]["highlights"].is_object());
        assert!(schema["properties"]["relevance_score"].is_object());

        // Check relevance_score constraints
        assert_eq!(schema["properties"]["relevance_score"]["minimum"], 0.0);
        assert_eq!(schema["properties"]["relevance_score"]["maximum"], 1.0);

        // Check required fields
        let required = schema["required"]
            .as_array()
            .expect("required should be array");
        assert_eq!(required.len(), 5);
        assert!(required.contains(&json!("title")));
        assert!(required.contains(&json!("company")));
        assert!(required.contains(&json!("duration")));
        assert!(required.contains(&json!("highlights")));
        assert!(required.contains(&json!("relevance_score")));
    }

    #[test]
    fn test_tailored_cv_schema() {
        let schema = TailoredCV::schema();

        assert_eq!(schema["type"], "object");
        assert!(schema["properties"]["professional_summary"].is_object());
        assert!(schema["properties"]["experiences"].is_object());
        assert!(schema["properties"]["skills"].is_object());
        assert!(schema["properties"]["keywords"].is_object());
        assert!(schema["properties"]["suggestions"].is_object());

        // Check experiences is array of objects
        assert_eq!(schema["properties"]["experiences"]["type"], "array");
        assert!(schema["properties"]["experiences"]["items"].is_object());

        // Check required fields
        let required = schema["required"]
            .as_array()
            .expect("required should be array");
        assert_eq!(required.len(), 5);
        assert!(required.contains(&json!("professional_summary")));
        assert!(required.contains(&json!("experiences")));
        assert!(required.contains(&json!("skills")));
        assert!(required.contains(&json!("keywords")));
        assert!(required.contains(&json!("suggestions")));
    }

    #[test]
    fn test_response_format() {
        let format = TailoredCV::response_format("test_cv");

        assert_eq!(format["type"], "json_schema");
        assert_eq!(format["json_schema"]["name"], "test_cv");
        assert_eq!(format["json_schema"]["strict"], true);
        assert!(format["json_schema"]["schema"].is_object());
    }
}
