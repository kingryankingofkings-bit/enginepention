// Pention Engine - vkgen/emit.rs
// Requirement: PN-RND-001
// Decision:    ADR-0009, ADR-0010
//
// Emits Rust bindings from the parsed registry.
//
// One decision dominates the output shape: enumerations are emitted as
// #[repr(transparent)] newtypes with associated constants, NOT as Rust `enum`s.
//
// A Rust enum holding a value outside its declared set is undefined behaviour,
// and Vulkan drivers legitimately return values the registry does not list -
// vendor extensions in a driver newer than the header, VkResult codes added
// after these bindings were generated. Matching on such a value in a real
// `enum` is UB, not a surprise. The newtype keeps the same ergonomics at the
// call site while making an unrecognised value merely unrecognised.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::registry::{EnumKind, EnumValue, Registry, RequiredNames};

/// Everything the writers need to resolve a name, gathered once.
struct Context<'a> {
    required_types: &'a BTreeSet<String>,
    required_commands: &'a BTreeSet<String>,
    /// Every type name this run will actually define.
    defined: BTreeSet<String>,
    /// Constant names that appear as array extents, and so must be `usize`.
    size_constants: BTreeSet<String>,
    /// Resolved values of the API Constants block.
    constant_values: BTreeMap<String, i64>,
    /// Extension-name string constants inside the emitted scope.
    scoped_extension_constants: BTreeSet<String>,
    /// Names referenced but never defined. Collected during emission and
    /// reported afterwards: a binding that references a type it does not
    /// define will not compile, and finding out from rustc rather than from
    /// the generator wastes the reviewer's time.
    unknown: RefCell<BTreeSet<String>>,
}

impl Context<'_> {
    /// Names referenced but not defined, in sorted order.
    fn unknown_names(&self) -> Vec<String> {
        self.unknown.borrow().iter().cloned().collect()
    }
}

/// Fixed C types the registry refers to but does not define.
fn c_type_alias(name: &str) -> Option<&'static str> {
    Some(match name {
        "void" => "core::ffi::c_void",
        "char" => "core::ffi::c_char",
        "float" => "f32",
        "double" => "f64",
        "int8_t" => "i8",
        "uint8_t" => "u8",
        "int16_t" => "i16",
        "uint16_t" => "u16",
        "int32_t" => "i32",
        "uint32_t" => "u32",
        "int64_t" => "i64",
        "uint64_t" => "u64",
        "size_t" => "usize",
        "int" => "core::ffi::c_int",
        _ => return None,
    })
}

/// The generated module, plus anything the generator could not resolve.
pub struct Generated {
    pub source: String,
    /// Names referenced by the emitted code that this run did not define.
    /// Empty means the output is self-contained.
    pub unresolved: Vec<String>,
}

/// Renders the generated module.
pub fn emit(
    registry: &Registry,
    required: &RequiredNames,
    registry_sha256: &str,
    features: &[&str],
    extensions: &[&str],
) -> Generated {
    let context = build_context(registry, required, extensions);
    let mut out = String::with_capacity(1024 * 1024);

    write_header(&mut out, registry, registry_sha256, features, extensions);
    write_base_types(&mut out, registry);
    write_constants(&mut out, registry, &context);
    write_string_constants(&mut out, registry, &context);
    write_handles(&mut out, registry, required);
    write_enums(&mut out, registry, required);
    write_bitmasks(&mut out, registry, required);
    write_func_pointers(&mut out, registry, &context);
    write_structs(&mut out, registry, &context);
    write_commands(&mut out, registry, &context);
    write_dispatch_tables(&mut out, registry, &context);

    Generated { source: out, unresolved: context.unknown_names() }
}

fn build_context<'a>(
    registry: &Registry,
    required: &'a RequiredNames,
    extensions: &[&str],
) -> Context<'a> {
    let mut defined = BTreeSet::new();
    for base in &registry.base_types {
        if base.underlying.as_deref().and_then(c_type_alias).is_some() {
            defined.insert(base.name.clone());
        }
    }
    for handle in &registry.handles {
        if required.types.contains(&handle.name) {
            defined.insert(handle.name.clone());
        }
    }
    for group in registry.enum_groups.values() {
        if group.kind != EnumKind::Constants && required.types.contains(&group.name) {
            defined.insert(group.name.clone());
        }
    }
    for alias in &registry.bitmask_aliases {
        if required.types.contains(&alias.name) {
            defined.insert(alias.name.clone());
        }
    }
    for pointer in &registry.func_pointers {
        if required.types.contains(&pointer.name) {
            defined.insert(pointer.name.clone());
        }
    }
    for structure in &registry.structs {
        if required.types.contains(&structure.name) {
            defined.insert(structure.name.clone());
        }
    }

    // Array extents drive the width chosen for the constants they name, so
    // they have to be collected before any constant is written.
    let mut size_constants = BTreeSet::new();
    for structure in &registry.structs {
        if !required.types.contains(&structure.name) {
            continue;
        }
        for member in &structure.members {
            for extent in &member.array_sizes {
                // Only an identifier can name a constant. Anything else that
                // reaches here is a parse failure, and treating it as a
                // constant name would hide that behind valid-looking output.
                let is_identifier = !extent.is_empty()
                    && extent.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
                    && extent.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
                if is_identifier {
                    size_constants.insert(extent.clone());
                }
            }
        }
    }

    let mut constant_values = BTreeMap::new();
    if let Some(group) =
        registry.enum_groups.values().find(|group| group.kind == EnumKind::Constants)
    {
        for entry in &group.entries {
            if let Some(value) = resolve_value(&group.entries, &entry.value) {
                constant_values.insert(entry.name.clone(), value);
            }
        }
    }

    let wanted: BTreeSet<&str> = extensions.iter().copied().collect();
    let scoped_extension_constants = registry
        .string_constants
        .iter()
        .filter(|(_, value)| wanted.contains(value.as_str()))
        .map(|(name, _)| name.clone())
        .collect();

    Context {
        required_types: &required.types,
        required_commands: &required.commands,
        defined,
        size_constants,
        constant_values,
        scoped_extension_constants,
        unknown: RefCell::new(BTreeSet::new()),
    }
}

fn write_header(
    out: &mut String,
    registry: &Registry,
    registry_sha256: &str,
    features: &[&str],
    extensions: &[&str],
) {
    let version = registry
        .header_version
        .map(|v| v.to_string())
        .unwrap_or_else(|| "unknown".to_owned());

    let _ = writeln!(out, "// Pention Engine - GENERATED FILE, DO NOT EDIT BY HAND.");
    let _ = writeln!(out, "// Requirement: PN-RND-001");
    let _ = writeln!(out, "// Decision:    ADR-0009, ADR-0010");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// Produced by rust/tools/vkgen from the official Khronos Vulkan");
    let _ = writeln!(out, "// registry. Regenerate with:");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "//     python3 build_scripts/fetch_vulkan_registry.py");
    let _ = writeln!(out, "//     cargo run -p vkgen -- third_party/vk.xml --emit <path>");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// Registry provenance:");
    let _ = writeln!(out, "//   VK_HEADER_VERSION {version}");
    let _ = writeln!(out, "//   vk.xml sha256     {registry_sha256}");
    let _ = writeln!(out, "//   source            KhronosGroup/Vulkan-Headers registry/vk.xml");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// Scope:");
    for feature in features {
        let _ = writeln!(out, "//   feature   {feature}");
    }
    for extension in extensions {
        let _ = writeln!(out, "//   extension {extension}");
    }
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// The registry is a specification, not an implementation: no Khronos");
    let _ = writeln!(out, "// code is copied here, and no binding crate is used. See");
    let _ = writeln!(out, "// DEPENDENCY_BOUNDARY.md.");
    let _ = writeln!(out);
    let _ = writeln!(out, "#![allow(non_camel_case_types)]");
    let _ = writeln!(out, "#![allow(non_upper_case_globals)]");
    let _ = writeln!(out, "#![allow(dead_code)]");
    // Vulkan member and parameter names are camelCase in the specification.
    // Renaming them here would make every call site disagree with the
    // reference documentation, which is a worse trade than a lint allowance.
    let _ = writeln!(out, "#![allow(non_snake_case)]");
    let _ = writeln!(out);
}

fn write_base_types(out: &mut String, registry: &Registry) {
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out, "// Base types");
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out);

    for base in &registry.base_types {
        let Some(underlying) = base.underlying.as_deref() else { continue };
        let Some(rust) = c_type_alias(underlying) else { continue };
        let _ = writeln!(out, "pub type {} = {rust};", base.name);
    }
    let _ = writeln!(out);
}

fn write_handles(out: &mut String, registry: &Registry, required: &RequiredNames) {
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out, "// Handles");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// A dispatchable handle is an opaque pointer; a non-dispatchable one");
    let _ = writeln!(out, "// is a 64-bit integer EVEN ON 32-BIT TARGETS. Treating them alike is");
    let _ = writeln!(out, "// a silent ABI break on a 32-bit build, so the two are distinct here.");
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out);

    for handle in &registry.handles {
        if !required.types.contains(&handle.name) {
            continue;
        }
        if let Some(parent) = &handle.parent {
            let _ = writeln!(out, "/// Child of `{parent}`.");
        }
        let _ = writeln!(out, "#[repr(transparent)]");
        let _ = writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
        if handle.dispatchable {
            let _ = writeln!(out, "pub struct {}(pub *mut core::ffi::c_void);", handle.name);
            let _ = writeln!(out, "impl {} {{", handle.name);
            let _ = writeln!(out, "    pub const NULL: Self = Self(core::ptr::null_mut());");
            let _ = writeln!(out, "    #[inline]");
            let _ = writeln!(out, "    pub fn is_null(self) -> bool {{ self.0.is_null() }}");
            let _ = writeln!(out, "}}");
        } else {
            let _ = writeln!(out, "pub struct {}(pub u64);", handle.name);
            let _ = writeln!(out, "impl {} {{", handle.name);
            let _ = writeln!(out, "    pub const NULL: Self = Self(0);");
            let _ = writeln!(out, "    #[inline]");
            let _ = writeln!(out, "    pub fn is_null(self) -> bool {{ self.0 == 0 }}");
            let _ = writeln!(out, "}}");
        }
        let _ = writeln!(out);
    }
}

/// Resolves an entry to its concrete integer, following aliases.
fn resolve_value(group_entries: &[crate::registry::EnumEntry], value: &EnumValue) -> Option<i64> {
    match value {
        EnumValue::Value(v) => Some(*v),
        EnumValue::BitPosition(bit) => Some(1i64 << bit),
        EnumValue::Alias(target) => {
            let entry = group_entries.iter().find(|e| &e.name == target)?;
            // One level of indirection is enough for this registry; guarding
            // against a cycle rather than recursing without limit.
            match &entry.value {
                EnumValue::Alias(_) => None,
                other => resolve_value(group_entries, other),
            }
        }
    }
}

fn write_enums(out: &mut String, registry: &Registry, required: &RequiredNames) {
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out, "// Enumerations");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// Emitted as transparent newtypes rather than Rust enums. A driver");
    let _ = writeln!(out, "// may return a value this registry does not list - a vendor");
    let _ = writeln!(out, "// extension, or a code added after these bindings were generated -");
    let _ = writeln!(out, "// and holding such a value in a real enum is undefined behaviour,");
    let _ = writeln!(out, "// not merely surprising.");
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out);

    for group in registry.enum_groups.values() {
        if group.kind == EnumKind::Constants || !required.types.contains(&group.name) {
            continue;
        }

        // A plain enumeration is signed - VkResult's error codes are negative.
        // A FlagBits group is a set of bit positions and must be unsigned, or
        // bit 31 becomes a negative constant that will not coerce into the
        // Flags word it is meant to be combined into.
        let representation = match (group.kind, group.is_64_bit) {
            (EnumKind::Enumeration, _) => "i32",
            (_, true) => "u64",
            (_, false) => "u32",
        };

        let _ = writeln!(out, "#[repr(transparent)]");
        let _ = writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]");
        let _ = writeln!(out, "pub struct {}(pub {representation});", group.name);
        let _ = writeln!(out);
        let _ = writeln!(out, "impl {} {{", group.name);

        let mut emitted = BTreeSet::new();
        for entry in &group.entries {
            if !emitted.insert(entry.name.clone()) {
                continue;  // the registry repeats promoted values under both names
            }
            let Some(value) = resolve_value(&group.entries, &entry.value) else { continue };
            let literal = match representation {
                "i32" => format!("{}", value as i32),
                "u64" => format!("{}", value as u64),
                _ => format!("{}", value as u32),
            };
            let _ = writeln!(
                out,
                "    pub const {}: Self = Self({literal});",
                shorten(&entry.name, &group.name)
            );
        }
        let _ = writeln!(out, "}}");
        let _ = writeln!(out);

        if group.kind == EnumKind::Bitmask {
            let _ = writeln!(out, "impl core::ops::BitOr for {} {{", group.name);
            let _ = writeln!(out, "    type Output = Self;");
            let _ = writeln!(out, "    #[inline]");
            let _ = writeln!(out, "    fn bitor(self, rhs: Self) -> Self {{ Self(self.0 | rhs.0) }}");
            let _ = writeln!(out, "}}");
            let _ = writeln!(out);
        }
    }
}

fn write_bitmasks(out: &mut String, registry: &Registry, required: &RequiredNames) {
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out, "// Bitmasks");
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out);

    for alias in &registry.bitmask_aliases {
        if !required.types.contains(&alias.name) {
            continue;
        }
        let width = if alias.base == "VkFlags64" { "u64" } else { "u32" };

        let _ = writeln!(out, "#[repr(transparent)]");
        let _ = writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]");
        let _ = writeln!(out, "pub struct {}(pub {width});", alias.name);
        let _ = writeln!(out);
        let _ = writeln!(out, "impl {} {{", alias.name);
        let _ = writeln!(out, "    pub const EMPTY: Self = Self(0);");
        let _ = writeln!(out, "    #[inline]");
        let _ = writeln!(out, "    pub const fn contains(self, other: Self) -> bool {{");
        let _ = writeln!(out, "        (self.0 & other.0) == other.0");
        let _ = writeln!(out, "    }}");

        if let Some(bits_name) = &alias.bits {
            if let Some(group) = registry.enum_groups.get(bits_name) {
                let mut emitted = BTreeSet::new();
                for entry in &group.entries {
                    if !emitted.insert(entry.name.clone()) {
                        continue;
                    }
                    let Some(value) = resolve_value(&group.entries, &entry.value) else { continue };
                    // Bit 63 of a 64-bit mask is 1i64 << 63, which is i64::MIN.
                    // Printed as a signed literal it becomes `-9223372036854775808`
                    // in a u64 position, which does not compile. The flag is a bit
                    // pattern, not a number, so reinterpret rather than negate.
                    let literal = if alias.base == "VkFlags64" {
                        format!("{}", value as u64)
                    } else {
                        format!("{}", value as u32)
                    };
                    let _ = writeln!(
                        out,
                        "    pub const {}: Self = Self({literal});",
                        shorten(&entry.name, bits_name)
                    );
                }
            }
        }
        let _ = writeln!(out, "}}");
        let _ = writeln!(out);
        if let Some(bits_name) = &alias.bits {
            if required.types.contains(bits_name) {
                let _ = writeln!(out, "impl From<{bits_name}> for {} {{", alias.name);
                let _ = writeln!(out, "    #[inline]");
                let _ = writeln!(out, "    fn from(bits: {bits_name}) -> Self {{ Self(bits.0) }}");
                let _ = writeln!(out, "}}");
                let _ = writeln!(out);
            }
        }
        let _ = writeln!(out, "impl core::ops::BitOr for {} {{", alias.name);
        let _ = writeln!(out, "    type Output = Self;");
        let _ = writeln!(out, "    #[inline]");
        let _ = writeln!(out, "    fn bitor(self, rhs: Self) -> Self {{ Self(self.0 | rhs.0) }}");
        let _ = writeln!(out, "}}");
        let _ = writeln!(out);
    }
}

/// Strips the group's implied prefix from a value name.
///
/// `VK_IMAGE_LAYOUT_GENERAL` in group `VkImageLayout` becomes `GENERAL`. When
/// the result would not be a valid identifier - a leading digit, or an empty
/// string - the original name is kept, because a binding that does not compile
/// is worse than a verbose one.
pub fn shorten(value_name: &str, group_name: &str) -> String {
    let prefix = screaming_snake_prefix(group_name);
    let candidate = value_name
        .strip_prefix(&format!("{prefix}_"))
        .unwrap_or(value_name);

    if candidate.is_empty() || candidate.starts_with(|c: char| c.is_ascii_digit()) {
        return value_name.to_owned();
    }
    candidate.to_owned()
}

/// `VkImageLayout` -> `VK_IMAGE_LAYOUT`, matching the registry's value naming.
///
/// A version suffix moves: `VkPipelineStageFlagBits2` names its values
/// `VK_PIPELINE_STAGE_2_*`, not `VK_PIPELINE_STAGE_FLAG_BITS2_*`. Leaving the
/// digit where it sits makes the prefix match nothing, and every 64-bit flag
/// constant then keeps its full name while its 32-bit sibling is shortened.
fn screaming_snake_prefix(group_name: &str) -> String {
    let digit_count = group_name.chars().rev().take_while(char::is_ascii_digit).count();
    let (stem, digits) = group_name.split_at(group_name.len() - digit_count);
    let trimmed = stem.trim_end_matches("FlagBits");

    let mut out = String::with_capacity(trimmed.len() * 2 + digits.len() + 1);
    for (index, character) in trimmed.char_indices() {
        if character.is_ascii_uppercase() && index != 0 {
            out.push('_');
        }
        out.push(character.to_ascii_uppercase());
    }
    if !digits.is_empty() {
        out.push('_');
        out.push_str(digits);
    }
    out
}

// ===================================================================
// Type mapping
// ===================================================================

/// Rust keywords that appear as Vulkan member names.
///
/// `VkDescriptorPoolSize::type` is the one that actually bites; the rest are
/// listed so a future registry addition does not produce a file that will not
/// parse.
fn escape_identifier(name: &str) -> String {
    const KEYWORDS: &[&str] = &[
        "as", "box", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
        "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
        "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true", "type",
        "unsafe", "use", "where", "while", "async", "await", "final", "macro", "override",
        "priv", "typeof", "unsized", "virtual", "yield",
    ];
    if KEYWORDS.contains(&name) {
        format!("r#{name}")
    } else {
        name.to_owned()
    }
}

/// Renders one declaration as a Rust type.
///
/// Arrays are built innermost-first: C `char x[A][B]` is an array of A arrays
/// of B chars, so the Rust type is `[[c_char; B]; A]`. Reversing this is a
/// silent layout bug on any multidimensional member, of which the registry has
/// several.
fn rust_type(declaration: &crate::registry::Declaration, context: &Context) -> String {
    let mut rendered = map_type_name(&declaration.type_name, context);

    for extent in declaration.array_sizes.iter().rev() {
        rendered = format!("[{rendered}; {}]", array_extent(extent, context));
    }
    for _ in 0..declaration.pointer_depth {
        rendered = if declaration.is_const {
            format!("*const {rendered}")
        } else {
            format!("*mut {rendered}")
        };
    }
    rendered
}

fn map_type_name(name: &str, context: &Context) -> String {
    if let Some(mapped) = c_type_alias(name) {
        return mapped.to_owned();
    }
    if context.defined.contains(name) {
        return name.to_owned();
    }
    // Unknown to this scope. Recorded rather than guessed: emitting a
    // plausible stand-in would compile and be wrong at the ABI level.
    context.unknown.borrow_mut().insert(name.to_owned());
    name.to_owned()
}

/// Renders an array extent, which is either a literal or a constant's name.
fn array_extent(extent: &str, context: &Context) -> String {
    if extent.chars().all(|c| c.is_ascii_digit()) {
        return extent.to_owned();
    }
    if context.size_constants.contains(extent) {
        return extent.to_owned();
    }
    // A constant the emitter did not classify as a size: fall back to its
    // resolved literal so the output still compiles, and say so in the file.
    match context.constant_values.get(extent) {
        Some(value) => format!("{value} /* {extent} */"),
        None => {
            context.unknown.borrow_mut().insert(extent.to_owned());
            extent.to_owned()
        }
    }
}

/// `vkCmdBindDescriptorSets` -> `cmd_bind_descriptor_sets`.
pub fn snake_case(name: &str) -> String {
    let trimmed = name.strip_prefix("vk").unwrap_or(name);
    let characters: Vec<char> = trimmed.chars().collect();
    let mut out = String::with_capacity(trimmed.len() + 8);

    for (index, &character) in characters.iter().enumerate() {
        if character.is_ascii_uppercase() && index != 0 {
            let previous = characters[index - 1];
            let next_is_lower = characters.get(index + 1).is_some_and(|c| c.is_ascii_lowercase());
            // Break before a new word, and also at the tail of an acronym that
            // runs into one: `...SurfaceKHR` stays whole, `KHRSurface` splits.
            if previous.is_ascii_lowercase() || previous.is_ascii_digit() || next_is_lower {
                out.push('_');
            }
        }
        out.push(character.to_ascii_lowercase());
    }
    out
}

// ===================================================================
// Constants
// ===================================================================

fn write_constants(out: &mut String, registry: &Registry, context: &Context) {
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out, "// API constants");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// Names used as array extents are emitted as usize so they can be");
    let _ = writeln!(out, "// written directly in an array type; the rest keep the width the");
    let _ = writeln!(out, "// registry declares. Guessing the width from the magnitude would");
    let _ = writeln!(out, "// get VK_WHOLE_SIZE (~0ULL) and VK_ATTACHMENT_UNUSED (~0U) wrong in");
    let _ = writeln!(out, "// opposite directions.");
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out);

    let Some(group) = registry
        .enum_groups
        .values()
        .find(|group| group.kind == EnumKind::Constants)
    else {
        let _ = writeln!(out);
        return;
    };

    for entry in &group.entries {
        let raw = entry.raw_value.as_deref().unwrap_or_default();

        // Floating-point constants have no integer form; VK_LOD_CLAMP_NONE is
        // written `1000.0F`.
        if raw.contains('.') {
            let literal = raw.trim_end_matches(['F', 'f']);
            let _ = writeln!(out, "pub const {}: f32 = {literal}f32;", entry.name);
            continue;
        }

        let Some(value) = resolve_value(&group.entries, &entry.value) else { continue };

        if context.size_constants.contains(&entry.name) {
            let _ = writeln!(out, "pub const {}: usize = {};", entry.name, value as u64);
            continue;
        }
        match entry.c_type.as_deref() {
            Some("uint64_t") => {
                let _ = writeln!(out, "pub const {}: u64 = {};", entry.name, value as u64);
            }
            Some("float") => {
                let _ = writeln!(out, "pub const {}: f32 = {value}f32;", entry.name);
            }
            _ => {
                let _ = writeln!(out, "pub const {}: u32 = {};", entry.name, value as u32);
            }
        }
    }
    let _ = writeln!(out);
}

fn write_string_constants(out: &mut String, registry: &Registry, context: &Context) {
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out, "// Extension names");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// NUL-terminated so they can be handed to Vulkan directly. Rust");
    let _ = writeln!(out, "// string literals are not NUL-terminated, and passing one to");
    let _ = writeln!(out, "// vkCreateInstance reads past the end of the allocation.");
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out);

    for (name, value) in &registry.string_constants {
        if !context.scoped_extension_constants.contains(name) {
            continue;
        }
        let _ = writeln!(out, "pub const {name}: &[u8] = b\"{value}\\0\";");
    }
    let _ = writeln!(out);
}

// ===================================================================
// Function pointers declared by the registry
// ===================================================================

fn write_func_pointers(out: &mut String, registry: &Registry, context: &Context) {
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out, "// Callback types");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// Wrapped in Option so that a null pointer is representable. A bare");
    let _ = writeln!(out, "// `fn` is a non-null type in Rust: zeroing a struct that holds one");
    let _ = writeln!(out, "// is undefined behaviour, and VkAllocationCallbacks is routinely");
    let _ = writeln!(out, "// zeroed.");
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out);

    for pointer in &registry.func_pointers {
        if !context.required_types.contains(&pointer.name) {
            continue;
        }
        let parameters = pointer
            .parameters
            .iter()
            .map(|parameter| {
                format!("{}: {}", escape_identifier(&parameter.name), rust_type(parameter, context))
            })
            .collect::<Vec<_>>()
            .join(", ");
        let returns = render_return(&pointer.return_type, context);
        let _ = writeln!(
            out,
            "pub type {} = Option<unsafe extern \"system\" fn({parameters}){returns}>;",
            pointer.name
        );
    }
    let _ = writeln!(out);
}

fn render_return(declaration: &crate::registry::Declaration, context: &Context) -> String {
    if declaration.type_name == "void" && declaration.pointer_depth == 0 {
        String::new()
    } else {
        format!(" -> {}", rust_type(declaration, context))
    }
}

// ===================================================================
// Structs and unions
// ===================================================================

fn write_structs(out: &mut String, registry: &Registry, context: &Context) {
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out, "// Structures");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// No Debug or PartialEq derive: several of these embed unions, for");
    let _ = writeln!(out, "// which neither can be generated, and a surface that is derived on");
    let _ = writeln!(out, "// some types and not others is worse than one that is uniform.");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// Default zeroes the struct and then writes the sType the registry");
    let _ = writeln!(out, "// fixes for it. A zeroed sType is VK_STRUCTURE_TYPE_APPLICATION_INFO,");
    let _ = writeln!(out, "// which is a valid value and therefore not a mistake the validation");
    let _ = writeln!(out, "// layers can reliably catch.");
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out);

    for structure in &registry.structs {
        if !context.required_types.contains(&structure.name) {
            continue;
        }

        let _ = writeln!(out, "#[repr(C)]");
        let _ = writeln!(out, "#[derive(Clone, Copy)]");
        let keyword = if structure.is_union { "union" } else { "struct" };
        let _ = writeln!(out, "pub {keyword} {} {{", structure.name);
        for member in &structure.members {
            let _ = writeln!(
                out,
                "    pub {}: {},",
                escape_identifier(&member.name),
                rust_type(member, context)
            );
        }
        let _ = writeln!(out, "}}");
        let _ = writeln!(out);

        let _ = writeln!(out, "impl Default for {} {{", structure.name);
        let _ = writeln!(out, "    #[inline]");
        let _ = writeln!(out, "    fn default() -> Self {{");
        match &structure.structure_type {
            Some(value) => {
                let _ = writeln!(
                    out,
                    "        let mut value: Self = unsafe {{ core::mem::zeroed() }};"
                );
                let _ = writeln!(
                    out,
                    "        value.sType = VkStructureType::{};",
                    shorten(value, "VkStructureType")
                );
                let _ = writeln!(out, "        value");
            }
            None => {
                let _ = writeln!(out, "        unsafe {{ core::mem::zeroed() }}");
            }
        }
        let _ = writeln!(out, "    }}");
        let _ = writeln!(out, "}}");
        let _ = writeln!(out);
    }
}

// ===================================================================
// Commands
// ===================================================================

/// Which loader a command must be fetched from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Level {
    /// Available before any instance exists.
    Entry,
    /// Fetched with vkGetInstanceProcAddr once an instance exists.
    Instance,
    /// Fetched with vkGetDeviceProcAddr, which skips the loader's dispatch
    /// trampoline and is measurably cheaper on hot paths.
    Device,
}

/// Classifies a command by the type of its first parameter.
///
/// The rule the Vulkan loader documents: a command whose first parameter is a
/// VkDevice or one of its children is device-level; one that takes any other
/// handle is instance-level; one that takes no handle at all exists before any
/// handle does. The two proc-address getters are excluded by name because they
/// are how the tables get populated in the first place.
///
/// The test is "is a handle", not "starts with Vk". vkCreateInstance's first
/// parameter is a `const VkInstanceCreateInfo*`, and reading that as a handle
/// files the one command that must be reachable before an instance exists
/// under the table that needs an instance to load.
fn level_of(command: &crate::registry::Command, handles: &BTreeSet<String>) -> Level {
    if command.name == "vkGetInstanceProcAddr" || command.name == "vkGetDeviceProcAddr" {
        return Level::Entry;
    }
    let Some(first) = command.parameters.first() else { return Level::Entry };
    if first.pointer_depth != 0 || !handles.contains(&first.type_name) {
        return Level::Entry;
    }
    match first.type_name.as_str() {
        "VkDevice" | "VkQueue" | "VkCommandBuffer" => Level::Device,
        _ => Level::Instance,
    }
}

fn write_commands(out: &mut String, registry: &Registry, context: &Context) {
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out, "// Command signatures");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// `extern \"system\"` matches VKAPI_PTR: stdcall on 32-bit Windows,");
    let _ = writeln!(out, "// the platform C ABI everywhere else. `extern \"C\"` would be a");
    let _ = writeln!(out, "// stack-corrupting mismatch on exactly one supported target.");
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out);

    for command in scoped_commands(registry, context) {
        let parameters = command
            .parameters
            .iter()
            .map(|parameter| {
                format!("{}: {}", escape_identifier(&parameter.name), rust_type(parameter, context))
            })
            .collect::<Vec<_>>()
            .join(", ");
        let returns = if command.return_type == "void" {
            String::new()
        } else {
            format!(" -> {}", map_type_name(&command.return_type, context))
        };
        let _ = writeln!(
            out,
            "pub type PFN_{} = unsafe extern \"system\" fn({parameters}){returns};",
            command.name
        );
    }
    let _ = writeln!(out);
}

fn scoped_commands<'a>(
    registry: &'a Registry,
    context: &Context,
) -> Vec<&'a crate::registry::Command> {
    registry
        .commands
        .iter()
        .filter(|command| {
            command.alias_of.is_none() && context.required_commands.contains(&command.name)
        })
        .collect()
}

fn write_dispatch_tables(out: &mut String, registry: &Registry, context: &Context) {
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out, "// Dispatch tables");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// Vulkan has no link-time symbols worth using: the loader resolves");
    let _ = writeln!(out, "// everything through vkGetInstanceProcAddr and vkGetDeviceProcAddr.");
    let _ = writeln!(out, "// Device-level commands are deliberately fetched from the device");
    let _ = writeln!(out, "// getter, which returns a pointer straight into the driver instead");
    let _ = writeln!(out, "// of one into the loader's dispatch trampoline.");
    let _ = writeln!(out, "//");
    let _ = writeln!(out, "// A missing entry is None rather than a panic. Every one of these");
    let _ = writeln!(out, "// is legitimately absent on some driver, and a table that refuses");
    let _ = writeln!(out, "// to load because an unused command is missing is unusable.");
    let _ = writeln!(out, "// ---------------------------------------------------------------");
    let _ = writeln!(out);

    let handles: BTreeSet<String> =
        registry.handles.iter().map(|handle| handle.name.clone()).collect();
    let commands = scoped_commands(registry, context);
    let by_level = |level: Level| -> Vec<&crate::registry::Command> {
        commands.iter().copied().filter(|c| level_of(c, &handles) == level).collect()
    };

    write_table(out, "EntryFns", &by_level(Level::Entry), "VkInstance");
    write_table(out, "InstanceFns", &by_level(Level::Instance), "VkInstance");
    write_table(out, "DeviceFns", &by_level(Level::Device), "VkDevice");
}

fn write_table(
    out: &mut String,
    table_name: &str,
    commands: &[&crate::registry::Command],
    handle_type: &str,
) {
    let getter_type = if handle_type == "VkDevice" {
        "PFN_vkGetDeviceProcAddr"
    } else {
        "PFN_vkGetInstanceProcAddr"
    };

    let _ = writeln!(out, "#[derive(Clone, Copy)]");
    let _ = writeln!(out, "pub struct {table_name} {{");
    for command in commands {
        let _ = writeln!(
            out,
            "    pub {}: Option<PFN_{}>,",
            snake_case(&command.name),
            command.name
        );
    }
    let _ = writeln!(out, "}}");
    let _ = writeln!(out);

    let _ = writeln!(out, "impl {table_name} {{");
    let _ = writeln!(out, "    /// Resolves every command in this table.");
    let _ = writeln!(out, "    ///");
    let _ = writeln!(out, "    /// # Safety");
    let _ = writeln!(out, "    ///");
    let _ = writeln!(out, "    /// `getter` must be the loader's real entry point, and `handle`");
    let _ = writeln!(out, "    /// must be a live {handle_type} obtained from it (or");
    let _ = writeln!(out, "    /// {handle_type}::NULL where the specification allows it).");
    let _ = writeln!(out, "    /// The returned pointers are only valid while `handle` is.");
    let _ = writeln!(out, "    #[allow(unused_variables)]");
    let _ = writeln!(
        out,
        "    pub unsafe fn load(getter: {getter_type}, handle: {handle_type}) -> Self {{"
    );
    let _ = writeln!(out, "        Self {{");
    for command in commands {
        let _ = writeln!(
            out,
            "            {}: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_{}>>(",
            snake_case(&command.name),
            command.name
        );
        let _ = writeln!(
            out,
            "                getter(handle, b\"{}\\0\".as_ptr().cast::<core::ffi::c_char>()),",
            command.name
        );
        let _ = writeln!(out, "            ),");
    }
    let _ = writeln!(out, "        }}");
    let _ = writeln!(out, "    }}");
    let _ = writeln!(out, "}}");
    let _ = writeln!(out);
}
