// Pention Engine - vkgen/xml.rs
// Requirement: PN-RND-001
// Decision:    ADR-0009 (no crates.io dependencies), ADR-0010
//
// A pull parser for the subset of XML that vk.xml actually uses.
//
// This exists because the dependency boundary permits no crates.io packages,
// and the registry is 3.3 MB of XML that has to be read somehow. It is
// deliberately NOT a general-purpose XML implementation: no namespaces, no
// DTD processing, no external entities. Those are absent from vk.xml, and
// implementing them would be a larger surface to get wrong for no benefit.
//
// What it does handle is the thing that matters here: MIXED CONTENT. The
// registry writes parameter declarations as
//
//     <param><type>VkPhysicalDevice</type>* <name>pPhysicalDevices</name></param>
//
// where the pointer asterisk is loose text between two elements. A parser that
// only reported elements, or that discarded inter-element text, would silently
// turn every pointer parameter into a value parameter. So text is reported in
// document order alongside elements, and the caller reassembles the C
// declaration from the sequence.

use std::fmt;

/// A parse failure, with enough position information to find it in a 3 MB file.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.line, self.column, self.message)
    }
}

impl std::error::Error for ParseError {}

/// One attribute. The name borrows from the input; the value is owned because
/// entity references may need decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute<'a> {
    pub name: &'a str,
    pub value: String,
}

/// An opening tag and its attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Start<'a> {
    pub name: &'a str,
    pub attributes: Vec<Attribute<'a>>,
}

impl<'a> Start<'a> {
    /// Value of an attribute, or None if absent.
    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|a| a.name == name)
            .map(|a| a.value.as_str())
    }

    /// True when the attribute is absent, or present and containing `value` in
    /// its comma-separated list.
    ///
    /// The registry marks API applicability as `api="vulkan,vulkansc"`, and an
    /// absent attribute means "all APIs". Mixing the two variants produces a
    /// binding set that compiles and is wrong, so this predicate is used at
    /// every element that carries the attribute.
    pub fn applies_to_api(&self, value: &str) -> bool {
        match self.attribute("api") {
            None => true,
            Some(list) => list.split(',').any(|entry| entry.trim() == value),
        }
    }
}

/// A parse event, reported in document order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event<'a> {
    Start(Start<'a>),
    End(&'a str),
    /// Character data with entity references already decoded.
    Text(String),
}

/// Pull parser over an XML document.
pub struct Reader<'a> {
    input: &'a str,
    bytes: &'a [u8],
    position: usize,
    /// Set when a self-closing tag has reported its Start and still owes an End.
    pending_end: Option<&'a str>,
}

impl<'a> Reader<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, bytes: input.as_bytes(), position: 0, pending_end: None }
    }

    /// Byte offset of the next unread character. Exposed for diagnostics.
    pub fn position(&self) -> usize {
        self.position
    }

    fn line_and_column(&self, offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;
        for &byte in &self.bytes[..offset.min(self.bytes.len())] {
            if byte == b'\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        (line, column)
    }

    fn error<T>(&self, offset: usize, message: impl Into<String>) -> Result<T, ParseError> {
        let (line, column) = self.line_and_column(offset);
        Err(ParseError { message: message.into(), line, column })
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.position).copied()
    }

    fn starts_with(&self, prefix: &str) -> bool {
        self.input[self.position..].starts_with(prefix)
    }

    /// Advances past `needle`, returning the text that preceded it.
    fn take_until(&mut self, needle: &str) -> Option<&'a str> {
        let rest = &self.input[self.position..];
        let found = rest.find(needle)?;
        let content = &rest[..found];
        self.position += found + needle.len();
        Some(content)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
            self.position += 1;
        }
    }

    /// Returns the next event, or None at end of input.
    pub fn next_event(&mut self) -> Result<Option<Event<'a>>, ParseError> {
        if let Some(name) = self.pending_end.take() {
            return Ok(Some(Event::End(name)));
        }

        if self.position >= self.bytes.len() {
            return Ok(None);
        }

        if self.peek() != Some(b'<') {
            return self.read_text().map(Some);
        }

        // Declarations, comments and CDATA are all introduced by '<' plus a
        // marker; dispatch on the marker before treating it as a tag.
        if self.starts_with("<?") {
            let start = self.position;
            if self.take_until("?>").is_none() {
                return self.error(start, "unterminated processing instruction");
            }
            return self.next_event();
        }
        if self.starts_with("<!--") {
            let start = self.position;
            self.position += 4;
            if self.take_until("-->").is_none() {
                return self.error(start, "unterminated comment");
            }
            return self.next_event();
        }
        if self.starts_with("<![CDATA[") {
            let start = self.position;
            self.position += "<![CDATA[".len();
            let Some(content) = self.take_until("]]>") else {
                return self.error(start, "unterminated CDATA section");
            };
            // CDATA is literal by definition - no entity decoding.
            return Ok(Some(Event::Text(content.to_owned())));
        }
        if self.starts_with("<!") {
            let start = self.position;
            if self.take_until(">").is_none() {
                return self.error(start, "unterminated declaration");
            }
            return self.next_event();
        }
        if self.starts_with("</") {
            let start = self.position;
            self.position += 2;
            let Some(content) = self.take_until(">") else {
                return self.error(start, "unterminated closing tag");
            };
            return Ok(Some(Event::End(content.trim())));
        }

        self.read_start_tag().map(Some)
    }

    fn read_text(&mut self) -> Result<Event<'a>, ParseError> {
        let rest = &self.input[self.position..];
        let end = rest.find('<').unwrap_or(rest.len());
        let raw = &rest[..end];
        self.position += end;
        Ok(Event::Text(decode_entities(raw)))
    }

    fn read_start_tag(&mut self) -> Result<Event<'a>, ParseError> {
        let tag_start = self.position;
        self.position += 1; // consume '<'

        let name_start = self.position;
        while let Some(byte) = self.peek() {
            if byte.is_ascii_whitespace() || byte == b'>' || byte == b'/' {
                break;
            }
            self.position += 1;
        }
        if self.position == name_start {
            return self.error(tag_start, "element name expected after '<'");
        }
        let name = &self.input[name_start..self.position];

        let mut attributes = Vec::new();
        loop {
            self.skip_whitespace();
            match self.peek() {
                None => return self.error(tag_start, "unterminated start tag"),
                Some(b'>') => {
                    self.position += 1;
                    break;
                }
                Some(b'/') => {
                    self.position += 1;
                    if self.peek() != Some(b'>') {
                        return self.error(self.position, "expected '>' after '/'");
                    }
                    self.position += 1;
                    // A self-closing tag is reported as Start followed by End,
                    // so consumers need no special case for it.
                    self.pending_end = Some(name);
                    break;
                }
                Some(_) => attributes.push(self.read_attribute()?),
            }
        }

        Ok(Event::Start(Start { name, attributes }))
    }

    fn read_attribute(&mut self) -> Result<Attribute<'a>, ParseError> {
        let name_start = self.position;
        while let Some(byte) = self.peek() {
            if byte.is_ascii_whitespace() || byte == b'=' || byte == b'>' || byte == b'/' {
                break;
            }
            self.position += 1;
        }
        if self.position == name_start {
            return self.error(self.position, "attribute name expected");
        }
        let name = &self.input[name_start..self.position];

        self.skip_whitespace();
        if self.peek() != Some(b'=') {
            return self.error(self.position, format!("expected '=' after attribute '{name}'"));
        }
        self.position += 1;
        self.skip_whitespace();

        let quote = match self.peek() {
            Some(q @ (b'"' | b'\'')) => q,
            _ => {
                return self.error(
                    self.position,
                    format!("expected a quoted value for attribute '{name}'"),
                )
            }
        };
        self.position += 1;

        let value_start = self.position;
        while let Some(byte) = self.peek() {
            if byte == quote {
                break;
            }
            self.position += 1;
        }
        if self.peek() != Some(quote) {
            return self.error(value_start, format!("unterminated value for attribute '{name}'"));
        }
        let raw = &self.input[value_start..self.position];
        self.position += 1;

        Ok(Attribute { name, value: decode_entities(raw) })
    }
}

/// Expands the five predefined entities and numeric character references.
///
/// Returns the input unchanged when it contains no '&', which is the
/// overwhelming majority of a 3 MB registry, so the common path allocates once
/// rather than scanning character by character.
pub fn decode_entities(input: &str) -> String {
    if !input.contains('&') {
        return input.to_owned();
    }

    let mut output = String::with_capacity(input.len());
    let mut rest = input;

    while let Some(ampersand) = rest.find('&') {
        output.push_str(&rest[..ampersand]);
        let tail = &rest[ampersand..];

        let Some(semicolon) = tail.find(';') else {
            // A bare '&' is malformed XML, but discarding the remainder would
            // lose data. Preserve it verbatim and let the caller notice.
            output.push_str(tail);
            return output;
        };

        let entity = &tail[1..semicolon];
        match entity {
            "lt" => output.push('<'),
            "gt" => output.push('>'),
            "amp" => output.push('&'),
            "quot" => output.push('"'),
            "apos" => output.push('\''),
            _ => {
                let decoded = entity
                    .strip_prefix("#x")
                    .or_else(|| entity.strip_prefix("#X"))
                    .and_then(|hex| u32::from_str_radix(hex, 16).ok())
                    .or_else(|| entity.strip_prefix('#').and_then(|d| d.parse::<u32>().ok()))
                    .and_then(char::from_u32);
                match decoded {
                    Some(character) => output.push(character),
                    // Unknown entity: keep it literal rather than dropping it.
                    None => output.push_str(&tail[..=semicolon]),
                }
            }
        }
        rest = &tail[semicolon + 1..];
    }

    output.push_str(rest);
    output
}
