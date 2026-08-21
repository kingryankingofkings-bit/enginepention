// Pention Engine - vkgen/main.rs
// Requirement: PN-RND-001
// Decision:    ADR-0009, ADR-0010
//
// Generates Rust Vulkan bindings from the official Khronos registry.
//
//     python3 build_scripts/fetch_vulkan_registry.py
//     cargo run -p vkgen -- third_party/vk.xml \
//         --emit rust/crates/pn-vulkan-sys/src/generated.rs
//
// With no --emit the tool reports what it parsed and exits, which is what CI
// runs when the registry is not present.

use std::process::ExitCode;

use vkgen::registry::{self, EnumValue, Registry, RequiredNames};
use vkgen::{emit, pin, sha256};

/// Scope of the first generated slice: core Vulkan plus the two extensions
/// needed to present anything to a window.
const FEATURES: &[&str] =
    &["VK_VERSION_1_0", "VK_VERSION_1_1", "VK_VERSION_1_2", "VK_VERSION_1_3"];
const EXTENSIONS: &[&str] = &["VK_KHR_surface", "VK_KHR_swapchain"];

struct Options {
    registry_path: String,
    output_path: Option<String>,
    allow_unpinned: bool,
}

fn usage() {
    eprintln!("usage: vkgen <path-to-vk.xml> [--emit <output.rs>] [--allow-unpinned-registry]");
    eprintln!();
    eprintln!("Fetch the registry first:");
    eprintln!("  python3 build_scripts/fetch_vulkan_registry.py");
}

fn parse_arguments() -> Result<Options, String> {
    let mut registry_path = None;
    let mut output_path = None;
    let mut allow_unpinned = false;

    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--emit" => {
                let value = arguments.next().ok_or("--emit needs a path")?;
                output_path = Some(value);
            }
            "--allow-unpinned-registry" => allow_unpinned = true,
            "-h" | "--help" => return Err("help".to_owned()),
            other if other.starts_with('-') => {
                return Err(format!("unknown option `{other}`"));
            }
            other => {
                if registry_path.is_some() {
                    return Err(format!("unexpected extra argument `{other}`"));
                }
                registry_path = Some(other.to_owned());
            }
        }
    }

    Ok(Options {
        registry_path: registry_path.ok_or("no registry path given")?,
        output_path,
        allow_unpinned,
    })
}

fn main() -> ExitCode {
    let options = match parse_arguments() {
        Ok(options) => options,
        Err(message) => {
            if message != "help" {
                eprintln!("vkgen: {message}");
                eprintln!();
            }
            usage();
            return ExitCode::from(2);
        }
    };

    // Read as bytes: the hash is over the file as fetched, not over whatever a
    // lossy decode would have produced.
    let bytes = match std::fs::read(&options.registry_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("vkgen: could not read {}: {error}", options.registry_path);
            return ExitCode::from(1);
        }
    };

    let pinned = pin::compiled_in();
    let actual_sha256 = sha256::hex(&bytes);
    if actual_sha256 != pinned.sha256 {
        eprintln!("vkgen: registry hash does not match the pin");
        eprintln!("  expected {}", pinned.sha256);
        eprintln!("  actual   {actual_sha256}");
        if !options.allow_unpinned {
            eprintln!();
            eprintln!("Generating bindings from an unpinned registry would make the");
            eprintln!("provenance header a fiction. Update");
            eprintln!("build_scripts/vulkan_registry_pin.txt deliberately, or pass");
            eprintln!("--allow-unpinned-registry for a throwaway experiment.");
            return ExitCode::from(1);
        }
        eprintln!("--allow-unpinned-registry given; continuing");
    }

    let source = match String::from_utf8(bytes) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("vkgen: {} is not valid UTF-8: {error}", options.registry_path);
            return ExitCode::from(1);
        }
    };

    let registry = match registry::parse(&source) {
        Ok(registry) => registry,
        Err(error) => {
            eprintln!("{}:{error}", options.registry_path);
            return ExitCode::from(1);
        }
    };

    if let Some(version) = registry.header_version {
        if version != pinned.header_version && !options.allow_unpinned {
            eprintln!("vkgen: VK_HEADER_VERSION {version} does not match the pinned");
            eprintln!("       {}. The pin file disagrees with itself.", pinned.header_version);
            return ExitCode::from(1);
        }
    }

    let required = registry.required_names(FEATURES, EXTENSIONS);
    report(&registry, &required, &options.registry_path);

    let Some(output_path) = options.output_path else {
        return ExitCode::SUCCESS;
    };

    let generated = emit::emit(&registry, &required, &actual_sha256, FEATURES, EXTENSIONS);
    if !generated.unresolved.is_empty() {
        eprintln!();
        eprintln!("vkgen: {} name(s) referenced but not defined:", generated.unresolved.len());
        for name in &generated.unresolved {
            eprintln!("  {name}");
        }
        eprintln!();
        eprintln!("The emitted file will not compile. Either widen the scope so these");
        eprintln!("are pulled in, or teach the emitter to render them.");
        return ExitCode::from(1);
    }
    let generated = generated.source;
    if let Some(parent) = std::path::Path::new(&output_path).parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(error) = std::fs::create_dir_all(parent) {
                eprintln!("vkgen: could not create {}: {error}", parent.display());
                return ExitCode::from(1);
            }
        }
    }
    if let Err(error) = std::fs::write(&output_path, generated.as_bytes()) {
        eprintln!("vkgen: could not write {output_path}: {error}");
        return ExitCode::from(1);
    }

    println!();
    println!("wrote {output_path} ({} bytes, {} lines)", generated.len(), generated.lines().count());
    ExitCode::SUCCESS
}

fn report(registry: &Registry, required: &RequiredNames, path: &str) {
    println!("registry: {path}");
    match registry.header_version {
        Some(version) => println!("  VK_HEADER_VERSION  {version}"),
        None => println!("  VK_HEADER_VERSION  (not found)"),
    }
    println!("  base types         {}", registry.base_types.len());
    println!("  handles            {}", registry.handles.len());
    println!("  bitmask aliases    {}", registry.bitmask_aliases.len());
    println!("  enum groups        {}", registry.enum_groups.len());
    println!("  structs and unions {}", registry.structs.len());
    println!("  commands           {}", registry.commands.len());
    println!("  features           {}", registry.features.len());
    println!("  extensions         {}", registry.extensions.len());
    println!("  type aliases       {}", registry.type_aliases.len());
    println!("  string constants   {}", registry.string_constants.len());
    println!("  func pointers      {}", registry.func_pointers.len());

    let total_enum_values: usize =
        registry.enum_groups.values().map(|group| group.entries.len()).sum();
    println!("  enum values        {total_enum_values}");

    println!();
    println!("scoped to core 1.0-1.3 plus VK_KHR_surface and VK_KHR_swapchain:");
    println!("  required types     {}", required.types.len());
    println!("  required commands  {}", required.commands.len());
    println!("  extension enums    {}", required.extended_enums.len());

    // A few spot checks printed for a human to eyeball. The same facts are
    // asserted by the test suite; printing them makes an unexpected registry
    // change visible when someone runs the tool by hand.
    println!();
    println!("spot checks:");
    if let Some(result) = registry.enum_groups.get("VkResult") {
        for wanted in ["VK_SUCCESS", "VK_ERROR_DEVICE_LOST"] {
            let value = result
                .entries
                .iter()
                .find(|entry| entry.name == wanted)
                .map(|entry| format!("{:?}", entry.value))
                .unwrap_or_else(|| "MISSING".to_owned());
            println!("  {wanted:<32} {value}");
        }
    }
    if let Some(command) = registry.commands.iter().find(|c| c.name == "vkCreateInstance") {
        println!(
            "  {:<32} -> {} ({} params)",
            "vkCreateInstance",
            command.return_type,
            command.parameters.len()
        );
    }
    if let Some(structure) = registry.structs.iter().find(|s| s.name == "VkApplicationInfo") {
        println!(
            "  {:<32} {} members, sType {:?}",
            "VkApplicationInfo",
            structure.members.len(),
            structure.structure_type
        );
    }
    if let Some(swapchain) = registry.enum_groups.get("VkStructureType").and_then(|group| {
        group.entries.iter().find(|e| e.name == "VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR")
    }) {
        // Contributed by VK_KHR_swapchain (extension 2), so its value is
        // computed from the extension number rather than written literally.
        match swapchain.value {
            EnumValue::Value(value) => {
                println!("  {:<32} {value}", "SWAPCHAIN_CREATE_INFO_KHR")
            }
            ref other => println!("  {:<32} {other:?}", "SWAPCHAIN_CREATE_INFO_KHR"),
        }
    }
}
