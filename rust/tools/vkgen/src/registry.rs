// Pention Engine - vkgen/registry.rs
// Requirement: PN-RND-001
// Decision:    ADR-0009, ADR-0010
//
// A typed model of vk.xml, built from the pull parser's event stream.
//
// Two things about the registry shape drive this design:
//
// 1. Declarations are MIXED CONTENT. A parameter is written as
//        <param>const <type>VkFoo</type>* <name>pFoo</name>[<enum>N</enum>]</param>
//    with the const, the asterisks and the array brackets as loose text around
//    the child elements. So a declaration is reassembled from the event
//    sequence rather than read out of attributes.
//
// 2. The registry describes TWO APIs. Elements carry api="vulkan",
//    api="vulkansc", or api="vulkan,vulkansc", and an absent attribute means
//    all of them. Vulkan SC is a different API with different definitions of
//    the same names. Mixing them yields bindings that compile and are wrong, so
//    the filter is applied at every element that carries the attribute.

use std::collections::{BTreeMap, BTreeSet};

use crate::xml::{Event, ParseError, Reader, Start};

/// The API variant these bindings target. Vulkan SC is deliberately excluded.
pub const TARGET_API: &str = "vulkan";

/// A C declaration reassembled from mixed content.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Declaration {
    /// The referenced type, e.g. `uint32_t` or `VkInstance`.
    pub type_name: String,
    /// The declared identifier, e.g. `pPhysicalDevices`.
    pub name: String,
    pub is_const: bool,
    pub pointer_depth: usize,
    /// Array extents, each either a literal or an enum name.
    pub array_sizes: Vec<String>,
    /// The original C text, kept verbatim so the generated output can cite it
    /// and a reviewer can check the reconstruction without opening vk.xml.
    pub c_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handle {
    pub name: String,
    pub parent: Option<String>,
    /// True for VK_DEFINE_HANDLE, false for VK_DEFINE_NON_DISPATCHABLE_HANDLE.
    /// The distinction matters: a dispatchable handle is a pointer, a
    /// non-dispatchable one is a 64-bit integer even on 32-bit targets.
    pub dispatchable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseType {
    pub name: String,
    /// None for opaque forward declarations such as `struct ANativeWindow;`.
    pub underlying: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitmaskAlias {
    pub name: String,
    /// The VkFlags or VkFlags64 base.
    pub base: String,
    /// The corresponding FlagBits enum, when one exists.
    pub bits: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumValue {
    /// An explicit value, which may be negative (error codes are).
    Value(i64),
    /// A bit position; the value is `1 << position`.
    BitPosition(u32),
    /// An alias of another value in the same group.
    Alias(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumEntry {
    pub name: String,
    pub value: EnumValue,
    /// The C type the registry declares for this constant, e.g. `uint32_t`.
    ///
    /// Only the API Constants block carries one. Without it the emitter would
    /// have to guess a width from the magnitude, which gets `VK_WHOLE_SIZE`
    /// (`~0ULL`) and `VK_ATTACHMENT_UNUSED` (`~0U`) wrong in opposite
    /// directions.
    pub c_type: Option<String>,
    /// The `value` attribute exactly as written.
    ///
    /// Kept because not every constant is an integer: `VK_LOD_CLAMP_NONE` is
    /// `1000.0F`, and an integer-only model drops it silently.
    pub raw_value: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumKind {
    /// A closed enumeration.
    Enumeration,
    /// A set of flag bits.
    Bitmask,
    /// The unnamed constant block (`<enums name="API Constants">`).
    Constants,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumGroup {
    pub name: String,
    pub kind: EnumKind,
    /// True when the underlying width is 64 bits.
    pub is_64_bit: bool,
    pub entries: Vec<EnumEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructType {
    pub name: String,
    pub is_union: bool,
    pub members: Vec<Declaration>,
    /// The sType value this struct must carry, when the registry fixes one.
    pub structure_type: Option<String>,
}

/// A `typedef`'d C function pointer, e.g. `PFN_vkAllocationFunction`.
///
/// VkAllocationCallbacks holds five of these, so a bindings crate that skips
/// them cannot express a custom allocator at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncPointer {
    pub name: String,
    /// The return type, with an empty `name`.
    pub return_type: Declaration,
    pub parameters: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<Declaration>,
    /// Present when this command is an alias of another.
    pub alias_of: Option<String>,
}

/// One `<require>` block: the names a feature or extension brings in.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Requirement {
    pub types: Vec<String>,
    pub commands: Vec<String>,
    /// Enum values contributed to an existing group by this feature/extension.
    pub extended_enums: Vec<ExtendedEnum>,
}

/// A value added to an existing enum group by a feature or extension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendedEnum {
    pub name: String,
    pub extends: String,
    pub value: EnumValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feature {
    /// e.g. `VK_VERSION_1_3`.
    pub name: String,
    /// e.g. `1.3`.
    pub number: String,
    /// Features this one builds on, from the `depends` attribute.
    ///
    /// This registry layers the core versions: VK_VERSION_1_0 depends on
    /// VK_GRAPHICS_VERSION_1_0, which depends on VK_COMPUTE_VERSION_1_0, which
    /// depends on VK_BASE_VERSION_1_0. Those internal features carry a large
    /// share of the actual type and command requirements, so selecting
    /// VK_VERSION_1_0 alone yields a fraction of the API.
    pub depends: Vec<String>,
    /// True for the `apitype="internal"` layering features, which are not
    /// versions a caller would ask for by name.
    pub internal: bool,
    pub requirements: Vec<Requirement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extension {
    pub name: String,
    pub number: i64,
    /// `instance` or `device`.
    pub kind: Option<String>,
    pub supported: Option<String>,
    pub requirements: Vec<Requirement>,
}

/// The whole registry, filtered to [`TARGET_API`].
#[derive(Debug, Clone, Default)]
pub struct Registry {
    pub header_version: Option<u32>,
    pub base_types: Vec<BaseType>,
    pub func_pointers: Vec<FuncPointer>,
    pub handles: Vec<Handle>,
    pub bitmask_aliases: Vec<BitmaskAlias>,
    pub enum_groups: BTreeMap<String, EnumGroup>,
    pub structs: Vec<StructType>,
    pub commands: Vec<Command>,
    pub features: Vec<Feature>,
    pub extensions: Vec<Extension>,
    /// Type and command aliases, e.g. the KHR name of a promoted symbol.
    pub type_aliases: BTreeMap<String, String>,
    /// String constants declared inside feature and extension require blocks,
    /// e.g. `VK_KHR_SWAPCHAIN_EXTENSION_NAME` -> `VK_KHR_swapchain`.
    ///
    /// These are not enum values and belong to no group, but enabling an
    /// extension means passing exactly this string, so dropping them would
    /// leave a caller to retype it by hand.
    pub string_constants: BTreeMap<String, String>,
}

/// The registry's formula for an extension-contributed enum value.
///
/// Defined by the Vulkan specification's registry schema: values start at
/// 1000000000, each extension owns a block of 1000, and `dir="-"` marks the
/// negative error codes. Reproduced here rather than hard-coding results,
/// because every extension enum in the generated output depends on it.
pub fn extension_enum_value(extension_number: i64, offset: i64, negative: bool) -> i64 {
    const BASE: i64 = 1_000_000_000;
    const BLOCK: i64 = 1_000;
    let value = BASE + (extension_number - 1) * BLOCK + offset;
    if negative {
        -value
    } else {
        value
    }
}

/// Parses a registry integer, which may be decimal, hexadecimal, or suffixed.
///
/// The registry writes `0x7FFFFFFF`, `1000`, `(~0ULL)`, `-4`, and
/// `1000000000ULL` in the same attribute position.
///
/// The suffix must be stripped *after* the base is known, not before: `F` is
/// both a float suffix and a hexadecimal digit, so trimming it up front turns
/// `0x7FFFFFFF` into `0x7` and `0x0000001F` into `0x0000001`. That is a silent
/// wrong answer in every constant it touches, not a parse failure.
pub fn parse_integer(text: &str) -> Option<i64> {
    let mut text = text.trim();

    // The registry parenthesises its complement forms: `(~0ULL)`.
    while let Some(inner) = text.strip_prefix('(').and_then(|rest| rest.strip_suffix(')')) {
        text = inner.trim();
    }

    let (negative, text) = match text.strip_prefix('-') {
        Some(rest) => (true, rest.trim()),
        None => (false, text),
    };

    if let Some(rest) = text.strip_prefix('~') {
        let base = if rest.trim().is_empty() { 0 } else { parse_integer(rest)? };
        let value = !base;
        return Some(if negative { -value } else { value });
    }

    let value = if let Some(hex) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        let digits: String = hex.chars().take_while(char::is_ascii_hexdigit).collect();
        if digits.is_empty() || !is_integer_suffix(&hex[digits.len()..]) {
            return None;
        }
        // Parsed as unsigned then reinterpreted, so 0xFFFFFFFFFFFFFFFF does
        // not overflow.
        u64::from_str_radix(&digits, 16).ok()? as i64
    } else {
        let digits: String = text.chars().take_while(char::is_ascii_digit).collect();
        if digits.is_empty() || !is_integer_suffix(&text[digits.len()..]) {
            return None;
        }
        digits.parse::<i64>().ok()?
    };

    Some(if negative { -value } else { value })
}

/// True when `text` is a C integer suffix, i.e. only `U` and `L` in any case.
///
/// `F` is deliberately absent: a trailing `F` means the literal is a float,
/// and a float is not an integer that happens to have a suffix.
fn is_integer_suffix(text: &str) -> bool {
    text.chars().all(|c| matches!(c, 'u' | 'U' | 'l' | 'L'))
}

/// Splits `[3][4]` into `["3", "4"]`, in the order C declares them.
///
/// Returns nothing for text that is not an extent list, which is most of the
/// mixed content in a declaration.
fn split_array_extents(text: &str) -> Vec<String> {
    let mut extents = Vec::new();
    let mut rest = text;
    while let Some(open) = rest.find('[') {
        let after = &rest[open + 1..];
        let Some(close) = after.find(']') else { break };
        let inner = after[..close].trim();
        if !inner.is_empty() {
            extents.push(inner.to_owned());
        }
        rest = &after[close + 1..];
    }
    extents
}

/// Returns the contents of a registry string literal, or None if unquoted.
///
/// The registry writes these as `value="&quot;VK_KHR_surface&quot;"`, so the
/// quotes survive entity decoding and are part of the attribute value.
fn unquote(text: &str) -> Option<String> {
    let trimmed = text.trim();
    let inner = trimmed.strip_prefix('"')?.strip_suffix('"')?;
    Some(inner.to_owned())
}

/// Accumulates the text and child elements of one declaration.
#[derive(Default)]
struct DeclarationBuilder {
    type_name: String,
    name: String,
    array_sizes: Vec<String>,
    text: String,
    saw_name: bool,
}

impl DeclarationBuilder {
    fn finish(self) -> Declaration {
        let c_text = self.text.split_whitespace().collect::<Vec<_>>().join(" ");
        Declaration {
            is_const: self.text.contains("const"),
            pointer_depth: self.text.matches('*').count(),
            type_name: self.type_name,
            name: self.name,
            array_sizes: self.array_sizes,
            c_text,
        }
    }
}

/// Parses vk.xml into a [`Registry`], keeping only [`TARGET_API`] elements.
pub fn parse(source: &str) -> Result<Registry, ParseError> {
    let mut reader = Reader::new(source);
    let mut registry = Registry::default();
    // The element path, so handlers know their context without a state machine.
    let mut path: Vec<String> = Vec::new();

    // Scratch state for the element currently being assembled.
    let mut current_enum_group: Option<EnumGroup> = None;
    let mut current_struct: Option<StructType> = None;
    let mut current_command: Option<Command> = None;
    let mut current_func_pointer: Option<FuncPointer> = None;
    let mut current_feature: Option<Feature> = None;
    let mut current_extension: Option<Extension> = None;
    let mut current_requirement: Option<Requirement> = None;
    let mut declaration: Option<DeclarationBuilder> = None;
    let mut type_builder: Option<(Start<'_>, String)> = None;
    let mut in_proto = false;
    let mut skip_depth: Option<usize> = None;

    while let Some(event) = reader.next_event()? {
        match event {
            Event::Start(start) => {
                path.push(start.name.to_owned());

                // Once inside an element that does not apply to our API, skip
                // everything until it closes.
                if skip_depth.is_some() {
                    continue;
                }
                if !start.applies_to_api(TARGET_API) {
                    skip_depth = Some(path.len());
                    continue;
                }

                match start.name {
                    "types" | "commands" | "extensions" | "require" if start.name == "require" => {
                        current_requirement = Some(Requirement::default());
                    }
                    "type" if parent_is(&path, "types") => {
                        type_builder = Some((start.clone(), String::new()));
                        // A funcpointer is written like a command - <proto>
                        // and <param> children - not as raw typedef text, so
                        // it reuses the same declaration machinery.
                        if start.attribute("category") == Some("funcpointer") {
                            current_func_pointer = Some(FuncPointer {
                                name: String::new(),
                                return_type: Declaration::default(),
                                parameters: Vec::new(),
                            });
                        }
                        if let Some(name) = start.attribute("name") {
                            if let Some(alias) = start.attribute("alias") {
                                registry.type_aliases.insert(name.to_owned(), alias.to_owned());
                            }
                            if start.attribute("category") == Some("struct")
                                || start.attribute("category") == Some("union")
                            {
                                current_struct = Some(StructType {
                                    name: name.to_owned(),
                                    is_union: start.attribute("category") == Some("union"),
                                    members: Vec::new(),
                                    structure_type: None,
                                });
                            }
                        }
                    }
                    "member" => {
                        if let Some(values) = start.attribute("values") {
                            if let Some(structure) = current_struct.as_mut() {
                                if structure.structure_type.is_none() {
                                    structure.structure_type = Some(values.to_owned());
                                }
                            }
                        }
                        declaration = Some(DeclarationBuilder::default());
                    }
                    "enums" => {
                        let name = start.attribute("name").unwrap_or_default().to_owned();
                        let kind = match start.attribute("type") {
                            Some("bitmask") => EnumKind::Bitmask,
                            Some("enum") => EnumKind::Enumeration,
                            _ => EnumKind::Constants,
                        };
                        current_enum_group = Some(EnumGroup {
                            name,
                            kind,
                            is_64_bit: start.attribute("bitwidth") == Some("64"),
                            entries: Vec::new(),
                        });
                    }
                    "enum" => handle_enum(&start, &path, &mut registry, &mut current_enum_group,
                                          &mut current_requirement, &current_extension,
                                          &mut declaration),
                    "command" if parent_is(&path, "commands") => {
                        if let (Some(name), Some(alias)) =
                            (start.attribute("name"), start.attribute("alias"))
                        {
                            registry.commands.push(Command {
                                name: name.to_owned(),
                                return_type: String::new(),
                                parameters: Vec::new(),
                                alias_of: Some(alias.to_owned()),
                            });
                        } else {
                            current_command = Some(Command {
                                name: String::new(),
                                return_type: String::new(),
                                parameters: Vec::new(),
                                alias_of: None,
                            });
                        }
                    }
                    "command" => {
                        if let (Some(requirement), Some(name)) =
                            (current_requirement.as_mut(), start.attribute("name"))
                        {
                            requirement.commands.push(name.to_owned());
                        }
                    }
                    "proto" => {
                        in_proto = true;
                        declaration = Some(DeclarationBuilder::default());
                    }
                    // <implicitexternsyncparams> also contains <param>, but
                    // its contents are English prose about external
                    // synchronisation, not a parameter list. Seven commands
                    // carry one, and reading them as parameters produces
                    // members with no type and no name.
                    "param" if parent_is(&path, "command") || parent_is(&path, "type") => {
                        declaration = Some(DeclarationBuilder::default());
                    }
                    // <feature> is used for two unrelated things. As a direct
                    // child of <registry> it is an API version. Inside a
                    // <require> block it is a physical-device feature-struct
                    // member, e.g. <feature name="shaderInt64" struct="..."/>,
                    // and there are 456 of those against 9 real versions.
                    // Without the parent test the nested ones overwrite the
                    // version being assembled and the requirements are lost.
                    "feature" if parent_is(&path, "registry") => {
                        current_feature = Some(Feature {
                            name: start.attribute("name").unwrap_or_default().to_owned(),
                            number: start.attribute("number").unwrap_or_default().to_owned(),
                            depends: parse_depends(start.attribute("depends").unwrap_or_default()),
                            internal: start.attribute("apitype") == Some("internal"),
                            requirements: Vec::new(),
                        });
                    }
                    "extension" => {
                        current_extension = Some(Extension {
                            name: start.attribute("name").unwrap_or_default().to_owned(),
                            number: start
                                .attribute("number")
                                .and_then(parse_integer)
                                .unwrap_or(0),
                            kind: start.attribute("type").map(str::to_owned),
                            supported: start.attribute("supported").map(str::to_owned),
                            requirements: Vec::new(),
                        });
                    }
                    _ => {}
                }

                // A <type name="..."/> reference inside a <require> block.
                if start.name == "type" && !parent_is(&path, "types") {
                    if let (Some(requirement), Some(name)) =
                        (current_requirement.as_mut(), start.attribute("name"))
                    {
                        requirement.types.push(name.to_owned());
                    }
                }
            }

            Event::Text(text) => {
                if skip_depth.is_some() {
                    continue;
                }
                if let Some((_, buffer)) = type_builder.as_mut() {
                    buffer.push_str(&text);
                }
                if let Some(builder) = declaration.as_mut() {
                    // <comment> children are prose, not part of the
                    // declaration. VkWriteDescriptorSet::pBufferInfo is
                    // documented as "...{UNIFORM,STORAGE}_BUFFER[_DYNAMIC]
                    // descriptor types", and reading that bracket as an array
                    // extent yields `*const [VkDescriptorBufferInfo; _DYNAMIC]`.
                    if path.last().map(String::as_str) == Some("comment") {
                        continue;
                    }
                    builder.text.push_str(&text);
                    match path.last().map(String::as_str) {
                        Some("type") => builder.type_name.push_str(text.trim()),
                        Some("name") => {
                            builder.name.push_str(text.trim());
                            builder.saw_name = true;
                        }
                        Some("enum") => builder.array_sizes.push(text.trim().to_owned()),
                        Some("member") | Some("param") | Some("proto") => {
                            // Array extents written as literals. A member can
                            // carry more than one - VkTransformMatrixKHR is
                            // `matrix[3][4]` - and reading `3][4` as a single
                            // extent produces `[f32; 3][4]`, which is not Rust.
                            if builder.saw_name {
                                for extent in split_array_extents(text.trim()) {
                                    builder.array_sizes.push(extent);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            Event::End(name) => {
                if let Some(depth) = skip_depth {
                    if path.len() == depth {
                        skip_depth = None;
                    }
                    path.pop();
                    continue;
                }

                match name {
                    "type" if parent_is_after_pop(&path, "types") => {
                        if let Some(pointer) = current_func_pointer.take() {
                            if !pointer.name.is_empty() {
                                registry.func_pointers.push(pointer);
                            }
                        }
                        if let Some((start, body)) = type_builder.take() {
                            finish_type(&start, &body, &mut registry, &mut current_struct);
                        }
                    }
                    "member" => {
                        if let (Some(builder), Some(structure)) =
                            (declaration.take(), current_struct.as_mut())
                        {
                            structure.members.push(builder.finish());
                        }
                    }
                    "proto" => {
                        in_proto = false;
                        if let Some(builder) = declaration.take() {
                            let finished = builder.finish();
                            if let Some(pointer) = current_func_pointer.as_mut() {
                                pointer.name = finished.name.clone();
                                pointer.return_type = finished;
                            } else if let Some(command) = current_command.as_mut() {
                                command.return_type = finished.type_name;
                                command.name = finished.name;
                            }
                        }
                    }
                    "param" => {
                        if let Some(builder) = declaration.take() {
                            let finished = builder.finish();
                            if let Some(pointer) = current_func_pointer.as_mut() {
                                pointer.parameters.push(finished);
                            } else if let Some(command) = current_command.as_mut() {
                                command.parameters.push(finished);
                            }
                        }
                    }
                    "command" => {
                        if let Some(command) = current_command.take() {
                            if !command.name.is_empty() {
                                registry.commands.push(command);
                            }
                        }
                    }
                    "enums" => {
                        if let Some(group) = current_enum_group.take() {
                            if !group.name.is_empty() {
                                registry
                                    .enum_groups
                                    .entry(group.name.clone())
                                    .and_modify(|existing| {
                                        existing.entries.extend(group.entries.clone())
                                    })
                                    .or_insert(group);
                            }
                        }
                    }
                    "require" => {
                        if let Some(requirement) = current_requirement.take() {
                            if let Some(feature) = current_feature.as_mut() {
                                feature.requirements.push(requirement);
                            } else if let Some(extension) = current_extension.as_mut() {
                                extension.requirements.push(requirement);
                            }
                        }
                    }
                    "feature" if parent_is(&path, "registry") => {
                        if let Some(feature) = current_feature.take() {
                            registry.features.push(feature);
                        }
                    }
                    "extension" => {
                        if let Some(extension) = current_extension.take() {
                            registry.extensions.push(extension);
                        }
                    }
                    _ => {}
                }
                let _ = in_proto;
                path.pop();
            }
        }
    }

    Ok(registry)
}

fn parent_is(path: &[String], name: &str) -> bool {
    path.len() >= 2 && path[path.len() - 2] == name
}

/// Same test, for use in an End handler where the element is still on the path.
fn parent_is_after_pop(path: &[String], name: &str) -> bool {
    parent_is(path, name)
}

#[allow(clippy::too_many_arguments)]
fn handle_enum(
    start: &Start<'_>,
    path: &[String],
    registry: &mut Registry,
    current_enum_group: &mut Option<EnumGroup>,
    current_requirement: &mut Option<Requirement>,
    current_extension: &Option<Extension>,
    declaration: &mut Option<DeclarationBuilder>,
) {
    let Some(name) = start.attribute("name") else { return };

    // An <enum> inside a declaration is an array extent, handled as text.
    if declaration.is_some() && path.iter().rev().nth(1).map(String::as_str) == Some("member") {
        return;
    }

    let value = if let Some(alias) = start.attribute("alias") {
        Some(EnumValue::Alias(alias.to_owned()))
    } else if let Some(bitpos) = start.attribute("bitpos").and_then(parse_integer) {
        Some(EnumValue::BitPosition(bitpos as u32))
    } else if let Some(offset) = start.attribute("offset").and_then(parse_integer) {
        // Contributed by an extension: the value is computed from the
        // extension's number, not written literally.
        let extension_number = start
            .attribute("extnumber")
            .and_then(parse_integer)
            .or_else(|| current_extension.as_ref().map(|e| e.number))
            .unwrap_or(0);
        let negative = start.attribute("dir") == Some("-");
        Some(EnumValue::Value(extension_enum_value(extension_number, offset, negative)))
    } else {
        start.attribute("value").and_then(parse_integer).map(EnumValue::Value)
    };

    // A quoted value is a string constant, not an enumerator.
    if let Some(literal) = start.attribute("value").and_then(|text| unquote(&text)) {
        registry.string_constants.insert(name.to_owned(), literal);
        return;
    }

    let value = match value {
        Some(value) => value,
        // Non-integer constants (`1000.0F`) have no integer form. Keep them
        // with a zero placeholder; the emitter reads `raw_value` instead.
        None if start.attribute("value").is_some() => EnumValue::Value(0),
        None => return,
    };

    if let Some(extends) = start.attribute("extends") {
        if let Some(requirement) = current_requirement.as_mut() {
            requirement.extended_enums.push(ExtendedEnum {
                name: name.to_owned(),
                extends: extends.to_owned(),
                value: value.clone(),
            });
        }
        // Also fold it into the group so a consumer reading enum_groups alone
        // sees the complete set.
        if let Some(group) = registry.enum_groups.get_mut(extends) {
            if !group.entries.iter().any(|e| e.name == name) {
                group.entries.push(EnumEntry {
                    name: name.to_owned(),
                    value,
                    c_type: start.attribute("type").map(str::to_owned),
                    raw_value: start.attribute("value").map(str::to_owned),
                });
            }
        }
        return;
    }

    if let Some(group) = current_enum_group.as_mut() {
        group.entries.push(EnumEntry {
            name: name.to_owned(),
            value,
            c_type: start.attribute("type").map(str::to_owned),
            raw_value: start.attribute("value").map(str::to_owned),
        });
    }
}

fn finish_type(
    start: &Start<'_>,
    body: &str,
    registry: &mut Registry,
    current_struct: &mut Option<StructType>,
) {
    let category = start.attribute("category").unwrap_or_default();

    match category {
        "handle" => {
            // The name is a <name> child; the macro tells us dispatchability.
            if let Some(name) = extract_tagged(body, start) {
                registry.handles.push(Handle {
                    name,
                    parent: start.attribute("parent").map(str::to_owned),
                    dispatchable: !body.contains("NON_DISPATCHABLE"),
                });
            }
        }
        "basetype" => {
            if let Some(name) = extract_tagged(body, start) {
                let underlying = body
                    .split_whitespace()
                    .nth(1)
                    .filter(|token| *token != &name)
                    .map(str::to_owned);
                registry.base_types.push(BaseType { name, underlying });
            }
        }
        "bitmask" => {
            if let Some(name) = extract_tagged(body, start) {
                let base = if body.contains("VkFlags64") { "VkFlags64" } else { "VkFlags" };
                registry.bitmask_aliases.push(BitmaskAlias {
                    name,
                    base: base.to_owned(),
                    bits: start.attribute("requires").or_else(|| start.attribute("bitvalues"))
                        .map(str::to_owned),
                });
            }
        }
        "define" => {
            // VK_HEADER_VERSION is written as `#define VK_HEADER_VERSION 360`.
            if body.contains("VK_HEADER_VERSION") && !body.contains("COMPLETE") {
                if let Some(version) = body
                    .split_whitespace()
                    .last()
                    .and_then(|token| token.trim_end_matches("</type>").parse::<u32>().ok())
                {
                    registry.header_version.get_or_insert(version);
                }
            }
        }
        "struct" | "union" => {
            if let Some(structure) = current_struct.take() {
                registry.structs.push(structure);
            }
        }
        _ => {}
    }
}

/// Recovers a type's name, which is either an attribute or the text of a
/// `<name>` child that the caller has already folded into `body`.
fn extract_tagged(body: &str, start: &Start<'_>) -> Option<String> {
    if let Some(name) = start.attribute("name") {
        return Some(name.to_owned());
    }
    // The body text for a handle is `VK_DEFINE_HANDLE(VkInstance)`; for a
    // bitmask, `typedef VkFlags VkFooFlags;`.
    let cleaned = body.replace(['(', ')', ';'], " ");
    cleaned
        .split_whitespace()
        .rev()
        .find(|token| token.starts_with("Vk"))
        .map(str::to_owned)
}

impl Registry {
    /// Every name required by the named features and extensions.
    ///
    /// Scoping the generated output by feature rather than by a hand-written
    /// list is what makes this a generator rather than a transcription: the
    /// registry already records which symbols each version and extension
    /// brings in, so that record is used instead of being duplicated.
    pub fn required_names(&self, features: &[&str], extensions: &[&str]) -> RequiredNames {
        let mut required = RequiredNames::default();
        let selected = self.resolve_feature_closure(features);

        for feature in &self.features {
            if selected.contains(&feature.name) {
                for requirement in &feature.requirements {
                    required.absorb(requirement);
                }
            }
        }
        for extension in &self.extensions {
            if extensions.contains(&extension.name.as_str()) {
                for requirement in &extension.requirements {
                    required.absorb(requirement);
                }
            }
        }
        required
    }

    /// Expands the requested features to include everything they depend on.
    ///
    /// Necessary because the core versions are layered over internal features
    /// (see [`Feature::depends`]); asking for VK_VERSION_1_0 without following
    /// the chain silently produces a fraction of the API.
    pub fn resolve_feature_closure(&self, requested: &[&str]) -> BTreeSet<String> {
        let by_name: BTreeMap<&str, &Feature> =
            self.features.iter().map(|f| (f.name.as_str(), f)).collect();

        let mut closure = BTreeSet::new();
        let mut pending: Vec<String> = requested.iter().map(|name| (*name).to_owned()).collect();

        while let Some(name) = pending.pop() {
            if !closure.insert(name.clone()) {
                continue;  // already visited; also terminates on a dependency cycle
            }
            if let Some(feature) = by_name.get(name.as_str()) {
                pending.extend(feature.depends.iter().cloned());
            }
        }
        closure
    }
}

/// Splits a `depends` expression into the names it mentions.
///
/// Feature dependencies are plain conjunctions such as
/// `VK_VERSION_1_2+VK_GRAPHICS_VERSION_1_3`. Extension dependencies can be
/// full boolean expressions with commas and parentheses; taking every
/// identifier is deliberately conservative - over-including a dependency
/// yields a superset of the API, whereas missing one yields bindings that do
/// not compile.
fn parse_depends(expression: &str) -> Vec<String> {
    expression
        .split(['+', ',', '(', ')'])
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::to_owned)
        .collect()
}

#[derive(Debug, Clone, Default)]
pub struct RequiredNames {
    pub types: BTreeSet<String>,
    pub commands: BTreeSet<String>,
    pub extended_enums: Vec<ExtendedEnum>,
}

impl RequiredNames {
    fn absorb(&mut self, requirement: &Requirement) {
        self.types.extend(requirement.types.iter().cloned());
        self.commands.extend(requirement.commands.iter().cloned());
        self.extended_enums.extend(requirement.extended_enums.iter().cloned());
    }
}
