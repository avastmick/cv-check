use pulldown_cmark::{Event, Options, Parser};

/// Parses markdown content into a vector of events.
#[must_use]
pub fn parse_markdown(content: &str) -> Vec<Event<'static>> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    // Parse and convert to owned events
    let parser = Parser::new_ext(content, options);
    parser.map(Event::into_static).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pulldown_cmark::Event;

    #[test]
    fn test_parse_heading() {
        let content = "# Test Heading";
        let events = parse_markdown(content);

        // First ensure we have events
        assert!(!events.is_empty(), "Events should not be empty");

        // Check for heading events
        let has_heading = events
            .iter()
            .any(|e| matches!(e, Event::Start(pulldown_cmark::Tag::Heading { .. })));
        assert!(has_heading, "Should contain heading start tag");

        let has_text = events
            .iter()
            .any(|e| matches!(e, Event::Text(t) if t.as_ref() == "Test Heading"));
        assert!(has_text, "Should contain heading text");
    }

    #[test]
    fn test_parse_list() {
        let content = "- Item 1\n- Item 2";
        let events = parse_markdown(content);

        // Currently returns empty vec - this test should fail first
        assert!(!events.is_empty(), "Events should not be empty");
    }
}
