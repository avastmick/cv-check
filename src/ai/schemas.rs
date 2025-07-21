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
