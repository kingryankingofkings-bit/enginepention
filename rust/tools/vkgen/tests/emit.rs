// Pention Engine - vkgen emitter tests
// Requirement: PN-RND-001
// Decision:    ADR-0009, ADR-0010
//
// These assert properties of the emitted text rather than reproducing it. A
// test that pins the exact output would fail on every registry update without
// saying which property broke.

use vkgen::emit;
use vkgen::registry;

fn generate(body: &str, features: &[&str], extensions: &[&str]) -> String {
    let document =
        format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<registry>{body}</registry>");
    let parsed = match registry::parse(&document) {
        Ok(parsed) => parsed,
        Err(error) => panic!("parse failed: {error}"),
    };
    let required = parsed.required_names(features, extensions);
    let generated = emit::emit(&parsed, &required, "0".repeat(64).as_str(), features, extensions);
    assert!(
        generated.unresolved.is_empty(),
        "emitter left names undefined: {:?}",
        generated.unresolved
    );
    generated.source
}

#[test]
fn a_value_name_loses_the_prefix_its_group_implies() {
    assert_eq!(emit::shorten("VK_IMAGE_LAYOUT_GENERAL", "VkImageLayout"), "GENERAL");
    assert_eq!(
        emit::shorten("VK_STRUCTURE_TYPE_APPLICATION_INFO", "VkStructureType"),
        "APPLICATION_INFO"
    );
    // FlagBits is not part of the value prefix.
    assert_eq!(
        emit::shorten("VK_SHADER_STAGE_VERTEX_BIT", "VkShaderStageFlagBits"),
        "VERTEX_BIT"
    );
}

#[test]
fn a_shortened_name_that_would_not_be_an_identifier_is_left_alone() {
    // `VK_IMAGE_VIEW_TYPE_1D` would shorten to `1D`, which is not a Rust
    // identifier. A verbose constant beats a file that will not parse.
    assert_eq!(
        emit::shorten("VK_IMAGE_VIEW_TYPE_1D", "VkImageViewType"),
        "VK_IMAGE_VIEW_TYPE_1D"
    );
    // VkResult's values do not carry the group's prefix at all, so nothing is
    // stripped and the full name survives.
    assert_eq!(emit::shorten("VK_SUCCESS", "VkResult"), "VK_SUCCESS");
}

#[test]
fn command_names_become_snake_case_without_losing_acronyms() {
    assert_eq!(emit::snake_case("vkCreateInstance"), "create_instance");
    assert_eq!(emit::snake_case("vkCmdBindDescriptorSets"), "cmd_bind_descriptor_sets");
    assert_eq!(
        emit::snake_case("vkGetPhysicalDeviceSurfaceCapabilitiesKHR"),
        "get_physical_device_surface_capabilities_khr"
    );
    assert_eq!(emit::snake_case("vkQueuePresentKHR"), "queue_present_khr");
}

#[test]
fn an_enumeration_is_a_transparent_newtype_not_a_rust_enum() {
    // A Rust enum holding a value outside its declared set is undefined
    // behaviour, and drivers return values newer than the registry.
    let source = generate(
        r#"<enums name="VkResult" type="enum">
             <enum value="0" name="VK_SUCCESS"/>
             <enum value="-4" name="VK_ERROR_DEVICE_LOST"/>
           </enums>
           <feature api="vulkan" name="VK_VERSION_1_0" number="1.0">
             <require><type name="VkResult"/></require>
           </feature>"#,
        &["VK_VERSION_1_0"],
        &[],
    );
    assert!(source.contains("#[repr(transparent)]"));
    assert!(source.contains("pub struct VkResult(pub i32);"));
    assert!(!source.contains("pub enum VkResult"));
    assert!(source.contains("pub const VK_SUCCESS: Self = Self(0);"));
    assert!(source.contains("pub const VK_ERROR_DEVICE_LOST: Self = Self(-4);"));
}

#[test]
fn bit_sixty_three_of_a_64_bit_mask_is_emitted_unsigned() {
    // 1i64 << 63 is i64::MIN. Printed as a signed literal in a u64 position it
    // is `-9223372036854775808`, which does not compile.
    let source = generate(
        r#"<types>
             <type requires="VkTestFlagBits2" category="bitmask">
               typedef <type>VkFlags64</type> <name>VkTestFlags2</name>;
             </type>
           </types>
           <enums name="VkTestFlagBits2" type="bitmask" bitwidth="64">
             <enum bitpos="63" name="VK_TEST_TOP_BIT"/>
           </enums>
           <feature api="vulkan" name="VK_VERSION_1_0" number="1.0">
             <require>
               <type name="VkTestFlags2"/>
               <type name="VkTestFlagBits2"/>
             </require>
           </feature>"#,
        &["VK_VERSION_1_0"],
        &[],
    );
    assert!(source.contains("9223372036854775808"), "top bit missing:\n{source}");
    assert!(!source.contains("-9223372036854775808"));
}

#[test]
fn a_struct_defaults_to_the_stype_the_registry_fixes_for_it() {
    // A zeroed sType is VK_STRUCTURE_TYPE_APPLICATION_INFO, a valid value, so
    // forgetting to set it is not something the validation layers reliably
    // catch.
    let source = generate(
        r#"<types>
             <type category="struct" name="VkTestCreateInfo">
               <member values="VK_STRUCTURE_TYPE_TEST_CREATE_INFO">
                 <type>VkStructureType</type> <name>sType</name>
               </member>
               <member><type>uint32_t</type> <name>flags</name></member>
             </type>
           </types>
           <enums name="VkStructureType" type="enum">
             <enum value="0" name="VK_STRUCTURE_TYPE_APPLICATION_INFO"/>
             <enum value="99" name="VK_STRUCTURE_TYPE_TEST_CREATE_INFO"/>
           </enums>
           <feature api="vulkan" name="VK_VERSION_1_0" number="1.0">
             <require>
               <type name="VkStructureType"/>
               <type name="VkTestCreateInfo"/>
             </require>
           </feature>"#,
        &["VK_VERSION_1_0"],
        &[],
    );
    assert!(source.contains("pub struct VkTestCreateInfo {"));
    assert!(source.contains("value.sType = VkStructureType::TEST_CREATE_INFO;"));
}

#[test]
fn a_member_named_type_is_escaped_rather_than_renamed() {
    // VkDescriptorPoolSize::type collides with a Rust keyword. Renaming it
    // would make the binding disagree with the specification.
    let source = generate(
        r#"<types>
             <type category="struct" name="VkTestPoolSize">
               <member><type>uint32_t</type> <name>type</name></member>
             </type>
           </types>
           <feature api="vulkan" name="VK_VERSION_1_0" number="1.0">
             <require><type name="VkTestPoolSize"/></require>
           </feature>"#,
        &["VK_VERSION_1_0"],
        &[],
    );
    assert!(source.contains("pub r#type: u32,"), "keyword not escaped:\n{source}");
}

#[test]
fn a_multidimensional_array_keeps_c_extent_order() {
    // C `float m[3][4]` is three rows of four. Reversing the extents is a
    // silent layout bug that compiles and passes any test that only checks
    // the total size.
    let source = generate(
        r#"<types>
             <type category="struct" name="VkTestMatrix">
               <member><type>float</type> <name>matrix</name>[3][4]</member>
             </type>
           </types>
           <feature api="vulkan" name="VK_VERSION_1_0" number="1.0">
             <require><type name="VkTestMatrix"/></require>
           </feature>"#,
        &["VK_VERSION_1_0"],
        &[],
    );
    assert!(source.contains("pub matrix: [[f32; 4]; 3],"), "wrong extent order:\n{source}");
}

#[test]
fn a_callback_type_is_optional_so_that_a_zeroed_struct_is_sound() {
    // A bare `fn` is non-null in Rust. Zeroing a struct that holds one is
    // undefined behaviour, and VkAllocationCallbacks is routinely zeroed.
    let source = generate(
        r#"<types>
             <type category="funcpointer">
               <proto><type>void</type>* <name>PFN_vkTestAllocation</name></proto>
               <param><type>size_t</type> <name>size</name></param>
             </type>
           </types>
           <feature api="vulkan" name="VK_VERSION_1_0" number="1.0">
             <require><type name="PFN_vkTestAllocation"/></require>
           </feature>"#,
        &["VK_VERSION_1_0"],
        &[],
    );
    assert!(source.contains(
        "pub type PFN_vkTestAllocation = Option<unsafe extern \"system\" fn(size: usize) \
         -> *mut core::ffi::c_void>;"
    ), "callback not optional:\n{source}");
}

#[test]
fn a_command_taking_no_handle_lands_in_the_entry_table() {
    // vkCreateInstance's first parameter is a `const VkInstanceCreateInfo*`.
    // Classifying by "starts with Vk" files the one command that must be
    // reachable before an instance exists under the table that needs one.
    let source = generate(
        r#"<types>
             <type category="handle">
               VK_DEFINE_HANDLE(<name>VkInstance</name>)
             </type>
             <type category="struct" name="VkInstanceCreateInfo">
               <member><type>uint32_t</type> <name>flags</name></member>
             </type>
           </types>
           <commands>
             <command>
               <proto><type>uint32_t</type> <name>vkCreateInstance</name></proto>
               <param>const <type>VkInstanceCreateInfo</type>* <name>pCreateInfo</name></param>
               <param><type>VkInstance</type>* <name>pInstance</name></param>
             </command>
             <command>
               <proto><type>void</type> <name>vkDestroyInstance</name></proto>
               <param><type>VkInstance</type> <name>instance</name></param>
             </command>
           </commands>
           <feature api="vulkan" name="VK_VERSION_1_0" number="1.0">
             <require>
               <type name="VkInstance"/>
               <type name="VkInstanceCreateInfo"/>
               <command name="vkCreateInstance"/>
               <command name="vkDestroyInstance"/>
             </require>
           </feature>"#,
        &["VK_VERSION_1_0"],
        &[],
    );

    let entry = source.split("pub struct EntryFns {").nth(1).expect("EntryFns");
    let entry_body = entry.split('}').next().expect("body");
    assert!(entry_body.contains("create_instance"), "EntryFns:\n{entry_body}");
    assert!(!entry_body.contains("destroy_instance"));

    let instance = source.split("pub struct InstanceFns {").nth(1).expect("InstanceFns");
    let instance_body = instance.split('}').next().expect("body");
    assert!(instance_body.contains("destroy_instance"), "InstanceFns:\n{instance_body}");
}

#[test]
fn the_provenance_header_names_the_registry_it_came_from() {
    // A generated file that does not say what it was generated from makes the
    // provenance ledger unverifiable.
    let source = generate(
        r#"<feature api="vulkan" name="VK_VERSION_1_0" number="1.0"><require/></feature>"#,
        &["VK_VERSION_1_0"],
        &[],
    );
    assert!(source.contains("GENERATED FILE, DO NOT EDIT BY HAND"));
    assert!(source.contains(&"0".repeat(64)));
    assert!(source.contains("KhronosGroup/Vulkan-Headers registry/vk.xml"));
    assert!(source.contains("feature   VK_VERSION_1_0"));
}
