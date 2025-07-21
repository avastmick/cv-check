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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_schema_builder_new() {
        let builder = SchemaBuilder::new();
        let schema = builder.build();

        assert_eq!(schema["type"], "object");
        assert_eq!(schema["properties"], json!({}));
        assert_eq!(schema["required"], json!([]));
        assert_eq!(schema["additionalProperties"], false);
    }

    #[test]
    fn test_schema_builder_string() {
        let mut builder = SchemaBuilder::new();
        builder.string("name", "The person's name");
        let schema = builder.build();

        assert_eq!(schema["properties"]["name"]["type"], "string");
        assert_eq!(
            schema["properties"]["name"]["description"],
            "The person's name"
        );
        assert!(schema["required"]
            .as_array()
            .expect("required should be array")
            .contains(&json!("name")));
    }

    #[test]
    fn test_schema_builder_string_array() {
        let mut builder = SchemaBuilder::new();
        builder.string_array("skills", "List of skills");
        let schema = builder.build();

        assert_eq!(schema["properties"]["skills"]["type"], "array");
        assert_eq!(schema["properties"]["skills"]["items"]["type"], "string");
        assert_eq!(
            schema["properties"]["skills"]["description"],
            "List of skills"
        );
        assert!(schema["required"]
            .as_array()
            .expect("required should be array")
            .contains(&json!("skills")));
    }

    #[test]
    fn test_schema_builder_number() {
        let mut builder = SchemaBuilder::new();
        builder.number("score", "Test score", Some(0.0), Some(100.0));
        let schema = builder.build();

        assert_eq!(schema["properties"]["score"]["type"], "number");
        assert_eq!(schema["properties"]["score"]["description"], "Test score");
        assert_eq!(schema["properties"]["score"]["minimum"], 0.0);
        assert_eq!(schema["properties"]["score"]["maximum"], 100.0);
        assert!(schema["required"]
            .as_array()
            .expect("required should be array")
            .contains(&json!("score")));
    }

    #[test]
    fn test_schema_builder_number_no_bounds() {
        let mut builder = SchemaBuilder::new();
        builder.number("value", "Any number", None, None);
        let schema = builder.build();

        assert_eq!(schema["properties"]["value"]["type"], "number");
        assert_eq!(schema["properties"]["value"]["description"], "Any number");
        assert!(schema["properties"]["value"]["minimum"].is_null());
        assert!(schema["properties"]["value"]["maximum"].is_null());
    }

    #[test]
    fn test_schema_builder_object_array() {
        let mut builder = SchemaBuilder::new();
        let item_schema = json!({
            "type": "object",
            "properties": {
                "id": {"type": "number"},
                "name": {"type": "string"}
            }
        });
        builder.object_array("items", "List of items", &item_schema);
        let schema = builder.build();

        assert_eq!(schema["properties"]["items"]["type"], "array");
        assert_eq!(
            schema["properties"]["items"]["description"],
            "List of items"
        );
        assert_eq!(schema["properties"]["items"]["items"], item_schema);
        assert!(schema["required"]
            .as_array()
            .expect("required should be array")
            .contains(&json!("items")));
    }

    #[test]
    fn test_schema_builder_chaining() {
        let mut builder = SchemaBuilder::new();
        builder
            .string("name", "Full name")
            .number("age", "Age in years", Some(0.0), Some(150.0))
            .string_array("hobbies", "List of hobbies");
        let schema = builder.build();

        assert_eq!(schema["properties"]["name"]["type"], "string");
        assert_eq!(schema["properties"]["age"]["type"], "number");
        assert_eq!(schema["properties"]["hobbies"]["type"], "array");

        let required = schema["required"]
            .as_array()
            .expect("required should be array");
        assert_eq!(required.len(), 3);
        assert!(required.contains(&json!("name")));
        assert!(required.contains(&json!("age")));
        assert!(required.contains(&json!("hobbies")));
    }

    #[test]
    fn test_response_format() {
        // Create a simple test type that implements JsonSchema
        struct TestType;
        impl JsonSchema for TestType {
            fn schema() -> Value {
                json!({
                    "type": "object",
                    "properties": {
                        "test": {"type": "string"}
                    }
                })
            }
        }

        let format = TestType::response_format("test_response");

        assert_eq!(format["type"], "json_schema");
        assert_eq!(format["json_schema"]["name"], "test_response");
        assert_eq!(format["json_schema"]["strict"], true);
        assert_eq!(format["json_schema"]["schema"]["type"], "object");
        assert_eq!(
            format["json_schema"]["schema"]["properties"]["test"]["type"],
            "string"
        );
    }

    #[test]
    fn test_schema_builder_default() {
        let builder1 = SchemaBuilder::new();
        let builder2 = SchemaBuilder::default();

        let schema1 = builder1.build();
        let schema2 = builder2.build();

        assert_eq!(schema1, schema2);
    }
}
