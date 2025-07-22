use cv_check::cli::display::SuggestionsBox;

#[test]
fn test_suggestions_box_new() {
    let suggestions = vec![
        "Add more quantifiable metrics to achievements".to_string(),
        "Include specific technologies used in each role".to_string(),
        "Consider reordering experiences by relevance".to_string(),
    ];

    let box_display = SuggestionsBox::new(&suggestions);
    assert_eq!(box_display.suggestions.len(), 3);
}

#[test]
fn test_suggestions_box_format() {
    let suggestions = vec![
        "Test suggestion 1".to_string(),
        "Test suggestion 2".to_string(),
    ];

    let box_display = SuggestionsBox::new(&suggestions);
    let formatted = box_display.format();

    // Check that the output contains the header
    assert!(formatted.contains("CV Suggestions"));

    // Check that suggestions are included
    assert!(formatted.contains("Test suggestion 1"));
    assert!(formatted.contains("Test suggestion 2"));

    // Check for box characters (corners and borders)
    assert!(formatted.contains("╭"));
    assert!(formatted.contains("╮"));
    assert!(formatted.contains("╰"));
    assert!(formatted.contains("╯"));
    assert!(formatted.contains("│"));
    assert!(formatted.contains("─"));
}

#[test]
fn test_suggestions_box_empty() {
    let suggestions: Vec<String> = vec![];

    let box_display = SuggestionsBox::new(&suggestions);
    let formatted = box_display.format();

    // Empty suggestions should still show a box with message
    assert!(formatted.contains("CV Suggestions"));
    assert!(formatted.contains("No suggestions available"));
}

#[test]
fn test_suggestions_box_width_constraint() {
    let long_suggestion = "This is a very long suggestion that should be wrapped to fit within the maximum width of the display box to ensure proper formatting".to_string();
    let suggestions = vec![long_suggestion];

    let box_display = SuggestionsBox::new(&suggestions);
    let formatted = box_display.format_plain();

    // Check that no line exceeds the maximum width (60 chars including borders)
    for line in formatted.lines() {
        // Count visual width, not byte length
        let visual_width = line.chars().count();
        println!("Visual width: {visual_width}, Line: '{line}'");
        assert!(
            visual_width <= 60,
            "Line too long (visual width: {visual_width}): {line}"
        );
    }
}

#[test]
fn test_display_suggestions() {
    use cv_check::cli::display;

    let suggestions = vec!["Suggestion 1".to_string(), "Suggestion 2".to_string()];

    // This should not panic
    display::show_suggestions(&suggestions);
}
