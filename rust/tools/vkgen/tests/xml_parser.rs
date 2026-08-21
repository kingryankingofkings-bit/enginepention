// Pention Engine - vkgen XML parser tests
// Requirement: PN-RND-001
// Decision:    ADR-0009, ADR-0010

use vkgen::xml::{decode_entities, Event, Reader};

/// Collects every event, failing the test on a parse error.
fn events(input: &str) -> Vec<Event<'_>> {
    let mut reader = Reader::new(input);
    let mut collected = Vec::new();
    loop {
        match reader.next_event() {
            Ok(None) => break,
            Ok(Some(event)) => collected.push(event),
            Err(error) => panic!("parse failed: {error}"),
        }
    }
    collected
}

/// Names only, for tests that do not care about attributes or text.
fn element_names(input: &str) -> Vec<String> {
    events(input)
        .into_iter()
        .filter_map(|event| match event {
            Event::Start(start) => Some(start.name.to_owned()),
            _ => None,
        })
        .collect()
}

#[test]
fn reads_a_single_element() {
    let names = element_names("<root></root>");
    assert_eq!(names, vec!["root"]);
}

#[test]
fn self_closing_tag_reports_start_then_end() {
    // Consumers should not need a special case for self-closing tags, so the
    // parser reports them exactly as it would report an empty element pair.
    let collected = events("<a/>");
    assert_eq!(collected.len(), 2);
    assert!(matches!(&collected[0], Event::Start(s) if s.name == "a"));
    assert!(matches!(&collected[1], Event::End("a")));
}

#[test]
fn reads_attributes_with_either_quote_style() {
    let collected = events(r#"<e one="1" two='2' three = "3" />"#);
    let Event::Start(start) = &collected[0] else {
        panic!("expected a start element");
    };
    assert_eq!(start.attribute("one"), Some("1"));
    assert_eq!(start.attribute("two"), Some("2"));
    assert_eq!(start.attribute("three"), Some("3"));
    assert_eq!(start.attribute("absent"), None);
}

#[test]
fn preserves_mixed_content_in_document_order() {
    // The property the registry actually depends on. In
    //   <param><type>VkPhysicalDevice</type>* <name>p</name></param>
    // the pointer asterisk is loose text between two elements. A parser that
    // dropped inter-element text would silently turn every pointer parameter
    // into a value parameter.
    let collected = events("<param><type>VkPhysicalDevice</type>* <name>p</name></param>");

    let mut sequence = Vec::new();
    for event in &collected {
        match event {
            Event::Start(s) => sequence.push(format!("<{}>", s.name)),
            Event::End(name) => sequence.push(format!("</{name}>")),
            Event::Text(text) => sequence.push(format!("text({text:?})")),
        }
    }

    assert_eq!(
        sequence,
        vec![
            "<param>",
            "<type>",
            "text(\"VkPhysicalDevice\")",
            "</type>",
            "text(\"* \")",
            "<name>",
            "text(\"p\")",
            "</name>",
            "</param>",
        ]
    );
}

#[test]
fn skips_comments_declarations_and_processing_instructions() {
    let names = element_names(
        r#"<?xml version="1.0"?><!DOCTYPE x><!-- a comment --><root><!--another--></root>"#,
    );
    assert_eq!(names, vec!["root"]);
}

#[test]
fn a_comment_containing_markup_is_not_parsed_as_markup() {
    let names = element_names("<root><!-- <fake attr='x'/> --><real/></root>");
    assert_eq!(names, vec!["root", "real"]);
}

#[test]
fn cdata_is_literal() {
    let collected = events("<root><![CDATA[<not/> &amp; raw]]></root>");
    let text: Vec<_> = collected
        .iter()
        .filter_map(|e| match e {
            Event::Text(t) => Some(t.as_str()),
            _ => None,
        })
        .collect();
    // No element parsing, and no entity decoding, inside CDATA.
    assert_eq!(text, vec!["<not/> &amp; raw"]);
}

#[test]
fn decodes_the_predefined_entities() {
    assert_eq!(decode_entities("a &lt; b &gt; c"), "a < b > c");
    assert_eq!(decode_entities("&amp;&quot;&apos;"), "&\"'");
    assert_eq!(decode_entities("no entities here"), "no entities here");
}

#[test]
fn decodes_numeric_character_references() {
    assert_eq!(decode_entities("&#65;&#66;"), "AB");
    assert_eq!(decode_entities("&#x41;&#X42;"), "AB");
}

#[test]
fn malformed_entities_are_preserved_rather_than_dropped() {
    // Losing data silently is worse than passing something odd through, because
    // the caller can see the latter.
    assert_eq!(decode_entities("bare & ampersand"), "bare & ampersand");
    assert_eq!(decode_entities("&unknown;"), "&unknown;");
    assert_eq!(decode_entities("&#zz;"), "&#zz;");
}

#[test]
fn entities_are_decoded_inside_attribute_values() {
    let collected = events(r#"<e note="a &lt; b &amp; c"/>"#);
    let Event::Start(start) = &collected[0] else {
        panic!("expected a start element");
    };
    assert_eq!(start.attribute("note"), Some("a < b & c"));
}

#[test]
fn api_filter_treats_an_absent_attribute_as_universal() {
    // Mixing the vulkan and vulkansc variants produces bindings that compile
    // and are wrong, so this predicate is load-bearing.
    let collected = events(r#"<a/><b api="vulkan"/><c api="vulkansc"/><d api="vulkan,vulkansc"/>"#);
    let starts: Vec<_> = collected
        .iter()
        .filter_map(|e| match e {
            Event::Start(s) => Some(s),
            _ => None,
        })
        .collect();

    assert!(starts[0].applies_to_api("vulkan"), "absent api means all APIs");
    assert!(starts[1].applies_to_api("vulkan"));
    assert!(!starts[2].applies_to_api("vulkan"), "vulkansc-only must be excluded");
    assert!(starts[3].applies_to_api("vulkan"));
    assert!(starts[3].applies_to_api("vulkansc"));
}

#[test]
fn nesting_is_reported_with_matching_ends() {
    let collected = events("<a><b><c/></b></a>");
    let mut depth = 0i32;
    let mut deepest = 0i32;
    for event in &collected {
        match event {
            Event::Start(_) => {
                depth += 1;
                deepest = deepest.max(depth);
            }
            Event::End(_) => depth -= 1,
            Event::Text(_) => {}
        }
    }
    assert_eq!(depth, 0, "every start must be matched by an end");
    assert_eq!(deepest, 3);
}

#[test]
fn errors_report_a_line_and_column() {
    let mut reader = Reader::new("<a>\n<b unterminated\n");
    let mut error = None;
    loop {
        match reader.next_event() {
            Ok(None) => break,
            Ok(Some(_)) => continue,
            Err(e) => {
                error = Some(e);
                break;
            }
        }
    }
    let error = error.expect("malformed input must produce an error, not silence");
    assert!(error.line >= 2, "expected the error on line 2 or later, got {}", error.line);
}

#[test]
fn an_unterminated_comment_is_an_error_not_a_silent_truncation() {
    let mut reader = Reader::new("<root><!-- never closed");
    let mut saw_error = false;
    loop {
        match reader.next_event() {
            Ok(None) => break,
            Ok(Some(_)) => continue,
            Err(_) => {
                saw_error = true;
                break;
            }
        }
    }
    assert!(saw_error);
}
