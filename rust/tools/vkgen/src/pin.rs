// Pention Engine - vkgen/pin.rs
// Requirement: PN-RND-001
// Decision:    ADR-0009, ADR-0010
//
// Reads the registry pin that build_scripts/fetch_vulkan_registry.py also
// reads. Compiled in rather than loaded at runtime: the generator must not be
// able to run against a pin file that has drifted from the one the binary was
// built with.

/// The pin file's text, as committed.
pub const PIN_FILE: &str = include_str!("../../../../build_scripts/vulkan_registry_pin.txt");

/// A parsed registry pin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pin {
    pub url: String,
    pub sha256: String,
    pub header_version: u32,
}

/// Parses `key = value` lines, ignoring blank lines and `#` comments.
///
/// Deliberately not a general configuration format: three keys, one line each.
/// Anything richer would need a parser, and a parser here would be a
/// dependency or a liability.
pub fn parse(text: &str) -> Result<Pin, String> {
    let mut url = None;
    let mut sha256 = None;
    let mut header_version = None;

    for (index, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("line {}: expected `key = value`", index + 1));
        };
        let value = value.trim().to_owned();
        match key.trim() {
            "url" => url = Some(value),
            "sha256" => sha256 = Some(value),
            "header_version" => {
                header_version = Some(
                    value
                        .parse::<u32>()
                        .map_err(|_| format!("line {}: header_version is not a number", index + 1))?,
                );
            }
            other => return Err(format!("line {}: unknown key `{other}`", index + 1)),
        }
    }

    let sha256 = sha256.ok_or_else(|| "missing `sha256`".to_owned())?;
    if sha256.len() != 64 || !sha256.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!("`sha256` is not 64 hexadecimal digits: {sha256}"));
    }

    Ok(Pin {
        url: url.ok_or_else(|| "missing `url`".to_owned())?,
        sha256,
        header_version: header_version.ok_or_else(|| "missing `header_version`".to_owned())?,
    })
}

/// The pin this binary was built with.
pub fn compiled_in() -> Pin {
    parse(PIN_FILE).expect("the committed pin file must parse")
}
