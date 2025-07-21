//! JSON Schema generation for structured outputs

use serde_json::{json, Value};

/// Trait for types that can generate their own JSON Schema
pub trait JsonSchema {
    /// Generate a JSON Schema for this type
    fn schema() -> Value;

    /// Generate a complete response format object for `OpenAI` API
    #[must_use]
    fn response_format(name: &str) -> Value {
        json!({
            "type": "json_schema",
            "json_schema": {
                "name": name,
                "schema": Self::schema(),
                "strict": true
            }
        })
    }
}

/// Helper struct to build JSON schemas programmatically
pub struct SchemaBuilder {
    properties: Value,
    required: Vec<String>,
}

impl SchemaBuilder {
    /// Create a new schema builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            properties: json!({}),
            required: Vec::new(),
        }
    }

    /// Add a string property
    pub fn string(&mut self, name: &str, description: &str) -> &mut Self {
        self.properties[name] = json!({
            "type": "string",
            "description": description
        });
        self.required.push(name.to_string());
        self
    }

    /// Add an array of strings property
    pub fn string_array(&mut self, name: &str, description: &str) -> &mut Self {
        self.properties[name] = json!({
            "type": "array",
            "items": {"type": "string"},
            "description": description
        });
        self.required.push(name.to_string());
        self
    }

    /// Add a number property
    pub fn number(
        &mut self,
        name: &str,
        description: &str,
        min: Option<f64>,
        max: Option<f64>,
    ) -> &mut Self {
        let mut prop = json!({
            "type": "number",
            "description": description
        });

        if let Some(min_val) = min {
            prop["minimum"] = json!(min_val);
        }
        if let Some(max_val) = max {
            prop["maximum"] = json!(max_val);
        }

        self.properties[name] = prop;
        self.required.push(name.to_string());
        self
    }

    /// Add an array of objects property
    pub fn object_array(
        &mut self,
        name: &str,
        description: &str,
        item_schema: &Value,
    ) -> &mut Self {
        self.properties[name] = json!({
            "type": "array",
            "description": description,
            "items": item_schema
        });
        self.required.push(name.to_string());
        self
    }

    /// Build the final schema
    #[must_use]
    pub fn build(self) -> Value {
        json!({
            "type": "object",
            "properties": self.properties,
            "required": self.required,
            "additionalProperties": false
        })
    }
}

impl Default for SchemaBuilder {
    fn default() -> Self {
        Self::new()
    }
}
