use approx::assert_abs_diff_eq;
use cv_check::config::{DocumentMetadata, GlobalConfig, LayoutOptions, Margins, RecipientInfo};
use serde_yaml::Value;
use std::collections::HashMap;

#[test]
fn test_margins_default() {
    let margins = Margins::default();
    assert_abs_diff_eq!(margins.top, 1.5);
    assert_abs_diff_eq!(margins.bottom, 1.5);
    assert_abs_diff_eq!(margins.left, 2.0);
    assert_abs_diff_eq!(margins.right, 2.0);
}

#[test]
fn test_layout_options_default() {
    let layout = LayoutOptions::default();
    assert_eq!(layout.columns, 1);
    assert_abs_diff_eq!(layout.margins.top, 1.5);
    assert_abs_diff_eq!(layout.margins.bottom, 1.5);
    assert_abs_diff_eq!(layout.margins.left, 2.0);
    assert_abs_diff_eq!(layout.margins.right, 2.0);
    assert!(layout.sidebar.is_none());
}

#[test]
fn test_global_config_default() {
    let config = GlobalConfig::default();
    assert_eq!(config.default_font_theme, Some("modern".to_string()));
    assert_eq!(config.default_color_theme, Some("modern".to_string()));
    assert_eq!(config.pdf_engine, Some("typst".to_string()));
    assert!(config.custom_themes_dir.is_none());
    assert_eq!(config.output_dir, Some("./output".to_string()));
    assert_eq!(config.auto_open, Some(true));
}

#[test]
fn test_global_config_load_without_config_file() {
    // This test checks the fallback behavior when no config file exists
    let config = GlobalConfig::load().expect("Should load default config");

    // Should return default config when no file exists
    assert_eq!(config.default_font_theme, Some("modern".to_string()));
    assert_eq!(config.default_color_theme, Some("modern".to_string()));
    assert_eq!(config.pdf_engine, Some("typst".to_string()));
    assert_eq!(config.auto_open, Some(true));
}

#[test]
fn test_global_config_load_with_config_file() {
    // We can't easily test the actual config directory loading
    // since it depends on system directories, but we can test
    // the YAML parsing logic by testing the serialization/deserialization

    let test_config = GlobalConfig {
        default_font_theme: Some("classic".to_string()),
        default_color_theme: Some("sharp".to_string()),
        pdf_engine: Some("typst".to_string()),
        custom_themes_dir: Some("/custom/themes".to_string()),
        output_dir: Some("./custom_output".to_string()),
        auto_open: Some(false),
    };

    // Test serialization
    let yaml_content = serde_yaml::to_string(&test_config).expect("Failed to serialize config");

    // Test deserialization
    let deserialized: GlobalConfig =
        serde_yaml::from_str(&yaml_content).expect("Failed to deserialize config");

    assert_eq!(deserialized.default_font_theme, Some("classic".to_string()));
    assert_eq!(deserialized.default_color_theme, Some("sharp".to_string()));
    assert_eq!(deserialized.pdf_engine, Some("typst".to_string()));
    assert_eq!(
        deserialized.custom_themes_dir,
        Some("/custom/themes".to_string())
    );
    assert_eq!(deserialized.output_dir, Some("./custom_output".to_string()));
    assert_eq!(deserialized.auto_open, Some(false));
}

#[test]
fn test_document_metadata_basic() {
    let metadata = DocumentMetadata {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        phone: Some("+1 555-0123".to_string()),
        location: Some("New York, NY".to_string()),
        linkedin: Some("johndoe".to_string()),
        github: Some("johndoe".to_string()),
        website: Some("https://johndoe.dev".to_string()),
        font_theme: "modern".to_string(),
        color_theme: "classic".to_string(),
        layout: LayoutOptions::default(),
        recipient: None,
        date: None,
        subject: None,
        custom: HashMap::new(),
    };

    assert_eq!(metadata.name, "John Doe");
    assert_eq!(metadata.email, "john@example.com");
    assert_eq!(metadata.phone, Some("+1 555-0123".to_string()));
    assert_eq!(metadata.location, Some("New York, NY".to_string()));
    assert_eq!(metadata.linkedin, Some("johndoe".to_string()));
    assert_eq!(metadata.github, Some("johndoe".to_string()));
    assert_eq!(metadata.website, Some("https://johndoe.dev".to_string()));
    assert_eq!(metadata.font_theme, "modern");
    assert_eq!(metadata.color_theme, "classic");
    assert!(metadata.recipient.is_none());
    assert!(metadata.date.is_none());
    assert!(metadata.subject.is_none());
    assert!(metadata.custom.is_empty());
}

#[test]
fn test_document_metadata_with_recipient() {
    let recipient = RecipientInfo {
        name: "Hiring Manager".to_string(),
        title: Some("Senior Recruiter".to_string()),
        company: Some("Tech Corp".to_string()),
        address: Some("123 Tech Street, Tech City, TC 12345".to_string()),
    };

    let metadata = DocumentMetadata {
        name: "Jane Smith".to_string(),
        email: "jane@example.com".to_string(),
        phone: None,
        location: None,
        linkedin: None,
        github: None,
        website: None,
        font_theme: "classic".to_string(),
        color_theme: "sharp".to_string(),
        layout: LayoutOptions::default(),
        recipient: Some(recipient),
        date: Some("2025-07-17".to_string()),
        subject: Some("Application for Software Engineer Position".to_string()),
        custom: HashMap::new(),
    };

    assert_eq!(metadata.name, "Jane Smith");
    assert_eq!(metadata.email, "jane@example.com");
    assert!(metadata.phone.is_none());
    assert!(metadata.recipient.is_some());

    let recipient_info = metadata
        .recipient
        .as_ref()
        .expect("Recipient should be present");
    assert_eq!(recipient_info.name, "Hiring Manager");
    assert_eq!(recipient_info.title, Some("Senior Recruiter".to_string()));
    assert_eq!(recipient_info.company, Some("Tech Corp".to_string()));
    assert_eq!(
        recipient_info.address,
        Some("123 Tech Street, Tech City, TC 12345".to_string())
    );

    assert_eq!(metadata.date, Some("2025-07-17".to_string()));
    assert_eq!(
        metadata.subject,
        Some("Application for Software Engineer Position".to_string())
    );
}

#[test]
fn test_document_metadata_with_custom_fields() {
    let mut custom_fields = HashMap::new();
    custom_fields.insert(
        "portfolio".to_string(),
        Value::String("https://portfolio.example.com".to_string()),
    );
    custom_fields.insert(
        "years_experience".to_string(),
        Value::Number(serde_yaml::Number::from(5)),
    );
    custom_fields.insert("remote_work".to_string(), Value::Bool(true));

    let metadata = DocumentMetadata {
        name: "Alex Johnson".to_string(),
        email: "alex@example.com".to_string(),
        phone: None,
        location: None,
        linkedin: None,
        github: None,
        website: None,
        font_theme: "sharp".to_string(),
        color_theme: "modern".to_string(),
        layout: LayoutOptions::default(),
        recipient: None,
        date: None,
        subject: None,
        custom: custom_fields,
    };

    assert_eq!(metadata.custom.len(), 3);
    assert_eq!(
        metadata.custom.get("portfolio"),
        Some(&Value::String("https://portfolio.example.com".to_string()))
    );
    assert_eq!(
        metadata.custom.get("years_experience"),
        Some(&Value::Number(serde_yaml::Number::from(5)))
    );
    assert_eq!(metadata.custom.get("remote_work"), Some(&Value::Bool(true)));
}

#[test]
fn test_document_metadata_serialization() {
    let metadata = DocumentMetadata {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        phone: Some("+1 555-0123".to_string()),
        location: Some("Test City".to_string()),
        linkedin: Some("testuser".to_string()),
        github: Some("testuser".to_string()),
        website: Some("https://test.example.com".to_string()),
        font_theme: "modern".to_string(),
        color_theme: "classic".to_string(),
        layout: LayoutOptions {
            columns: 2,
            margins: Margins {
                top: 2.0,
                bottom: 2.0,
                left: 2.5,
                right: 2.5,
            },
            sidebar: Some("right".to_string()),
        },
        recipient: None,
        date: None,
        subject: None,
        custom: HashMap::new(),
    };

    // Test serialization
    let yaml_content = serde_yaml::to_string(&metadata).expect("Failed to serialize metadata");

    // Test deserialization
    let deserialized: DocumentMetadata =
        serde_yaml::from_str(&yaml_content).expect("Failed to deserialize metadata");

    assert_eq!(deserialized.name, "Test User");
    assert_eq!(deserialized.email, "test@example.com");
    assert_eq!(deserialized.phone, Some("+1 555-0123".to_string()));
    assert_eq!(deserialized.layout.columns, 2);
    assert_abs_diff_eq!(deserialized.layout.margins.top, 2.0);
    assert_eq!(deserialized.layout.sidebar, Some("right".to_string()));
}

#[test]
fn test_layout_options_custom() {
    let layout = LayoutOptions {
        columns: 3,
        margins: Margins {
            top: 3.0,
            bottom: 3.0,
            left: 3.5,
            right: 3.5,
        },
        sidebar: Some("left".to_string()),
    };

    assert_eq!(layout.columns, 3);
    assert_abs_diff_eq!(layout.margins.top, 3.0);
    assert_abs_diff_eq!(layout.margins.bottom, 3.0);
    assert_abs_diff_eq!(layout.margins.left, 3.5);
    assert_abs_diff_eq!(layout.margins.right, 3.5);
    assert_eq!(layout.sidebar, Some("left".to_string()));
}

#[test]
fn test_recipient_info_complete() {
    let recipient = RecipientInfo {
        name: "Dr. Sarah Wilson".to_string(),
        title: Some("Head of Engineering".to_string()),
        company: Some("Innovation Labs".to_string()),
        address: Some("456 Innovation Drive, Tech Valley, TV 67890".to_string()),
    };

    assert_eq!(recipient.name, "Dr. Sarah Wilson");
    assert_eq!(recipient.title, Some("Head of Engineering".to_string()));
    assert_eq!(recipient.company, Some("Innovation Labs".to_string()));
    assert_eq!(
        recipient.address,
        Some("456 Innovation Drive, Tech Valley, TV 67890".to_string())
    );
}

#[test]
fn test_recipient_info_minimal() {
    let recipient = RecipientInfo {
        name: "Hiring Team".to_string(),
        title: None,
        company: None,
        address: None,
    };

    assert_eq!(recipient.name, "Hiring Team");
    assert!(recipient.title.is_none());
    assert!(recipient.company.is_none());
    assert!(recipient.address.is_none());
}

#[test]
fn test_document_metadata_empty_name_email() {
    // Test edge case where name or email might be empty strings
    let metadata = DocumentMetadata {
        name: String::new(),
        email: String::new(),
        phone: None,
        location: None,
        linkedin: None,
        github: None,
        website: None,
        font_theme: "modern".to_string(),
        color_theme: "modern".to_string(),
        layout: LayoutOptions::default(),
        recipient: None,
        date: None,
        subject: None,
        custom: HashMap::new(),
    };

    // Empty strings are allowed in the struct, validation happens elsewhere
    assert_eq!(metadata.name, "");
    assert_eq!(metadata.email, "");
}

#[test]
fn test_margins_extreme_values() {
    let margins = Margins {
        top: 0.0,
        bottom: 0.0,
        left: 100.0,
        right: 100.0,
    };

    assert_abs_diff_eq!(margins.top, 0.0);
    assert_abs_diff_eq!(margins.bottom, 0.0);
    assert_abs_diff_eq!(margins.left, 100.0);
    assert_abs_diff_eq!(margins.right, 100.0);
}

#[test]
fn test_layout_options_zero_columns() {
    let layout = LayoutOptions {
        columns: 0, // Edge case
        margins: Margins::default(),
        sidebar: None,
    };

    assert_eq!(layout.columns, 0);
}

#[test]
fn test_global_config_partial() {
    // Test that GlobalConfig handles partial config files
    let partial_config = GlobalConfig {
        default_font_theme: None,
        default_color_theme: Some("sharp".to_string()),
        pdf_engine: None,
        custom_themes_dir: None,
        output_dir: None,
        auto_open: None,
    };

    assert!(partial_config.default_font_theme.is_none());
    assert_eq!(
        partial_config.default_color_theme,
        Some("sharp".to_string())
    );
    assert!(partial_config.pdf_engine.is_none());
    assert!(partial_config.auto_open.is_none());
}

#[test]
fn test_document_metadata_invalid_theme_names() {
    // Test metadata with potentially invalid theme names
    let metadata = DocumentMetadata {
        name: "Test".to_string(),
        email: "test@test.com".to_string(),
        phone: None,
        location: None,
        linkedin: None,
        github: None,
        website: None,
        font_theme: "nonexistent-theme".to_string(),
        color_theme: String::new(), // Empty theme name
        layout: LayoutOptions::default(),
        recipient: None,
        date: None,
        subject: None,
        custom: HashMap::new(),
    };

    // The struct itself accepts any string, validation happens in theme loading
    assert_eq!(metadata.font_theme, "nonexistent-theme");
    assert_eq!(metadata.color_theme, "");
}

#[test]
fn test_custom_fields_with_nested_values() {
    let mut custom_fields = HashMap::new();

    // Create nested YAML values
    let skills = vec![
        Value::String("Rust".to_string()),
        Value::String("Python".to_string()),
        Value::String("JavaScript".to_string()),
    ];
    custom_fields.insert("skills".to_string(), Value::Sequence(skills));

    let mut certifications = serde_yaml::Mapping::new();
    certifications.insert(
        Value::String("AWS".to_string()),
        Value::String("Solutions Architect".to_string()),
    );
    certifications.insert(
        Value::String("Google".to_string()),
        Value::String("Cloud Engineer".to_string()),
    );
    custom_fields.insert("certifications".to_string(), Value::Mapping(certifications));

    let metadata = DocumentMetadata {
        name: "Test".to_string(),
        email: "test@test.com".to_string(),
        phone: None,
        location: None,
        linkedin: None,
        github: None,
        website: None,
        font_theme: "modern".to_string(),
        color_theme: "modern".to_string(),
        layout: LayoutOptions::default(),
        recipient: None,
        date: None,
        subject: None,
        custom: custom_fields,
    };

    // Verify nested structures are preserved
    assert_eq!(metadata.custom.len(), 2);
    assert!(matches!(
        metadata.custom.get("skills"),
        Some(Value::Sequence(_))
    ));
    assert!(matches!(
        metadata.custom.get("certifications"),
        Some(Value::Mapping(_))
    ));
}
