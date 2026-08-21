// Pention Engine - vkgen registry model tests
// Requirement: PN-RND-001
// Decision:    ADR-0009, ADR-0010
//
// Every test here runs against a synthetic registry written inline. The real
// vk.xml is not committed (see build_scripts/fetch_vulkan_registry.py), so a
// suite that needed it would be a suite that mostly does not run.

use vkgen::registry::{self, EnumValue};

/// Wraps a fragment in the minimum a registry needs to parse.
fn registry_document(body: &str) -> String {
    format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<registry>{body}</registry>")
}

fn parse(body: &str) -> registry::Registry {
    match registry::parse(&registry_document(body)) {
        Ok(parsed) => parsed,
        Err(error) => panic!("parse failed: {error}"),
    }
}

#[test]
fn the_extension_enum_formula_matches_the_registry_schema() {
    // Published formula: 1000000000 + (extnumber - 1) * 1000 + offset.
    // VK_KHR_swapchain is extension 2, and its first structure type is
    // VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR = 1000001000.
    assert_eq!(registry::extension_enum_value(2, 0, false), 1_000_001_000);
    assert_eq!(registry::extension_enum_value(1, 0, false), 1_000_000_000);
    assert_eq!(registry::extension_enum_value(1, 3, false), 1_000_000_003);
}

#[test]
fn a_negative_direction_flips_the_sign_of_an_extension_enum() {
    // Error codes are contributed as negative values; VK_ERROR_OUT_OF_DATE_KHR
    // is -1000001004.
    assert_eq!(registry::extension_enum_value(2, 4, true), -1_000_001_004);
}

#[test]
fn registry_integers_parse_in_every_form_the_schema_uses() {
    assert_eq!(registry::parse_integer("1000"), Some(1000));
    assert_eq!(registry::parse_integer("-4"), Some(-4));
    assert_eq!(registry::parse_integer("0x7FFFFFFF"), Some(0x7FFF_FFFF));
    assert_eq!(registry::parse_integer("1000000000ULL"), Some(1_000_000_000));
    // `~0U` is how the registry writes an all-ones sentinel.
    assert_eq!(registry::parse_integer("~0U"), Some(-1));
    assert_eq!(registry::parse_integer("~0ULL"), Some(-1));
}

#[test]
fn a_bit_position_is_kept_as_a_position_not_a_value() {
    // Resolving 1 << bitpos at parse time would lose the distinction between
    // a flag and an enumerator, and bit 63 would silently become negative.
    let parsed = parse(
        r#"<enums name="VkTestFlagBits" type="bitmask">
             <enum bitpos="0" name="VK_TEST_A_BIT"/>
             <enum bitpos="63" name="VK_TEST_B_BIT"/>
           </enums>"#,
    );
    let group = parsed.enum_groups.get("VkTestFlagBits").expect("group");
    assert_eq!(group.entries[0].value, EnumValue::BitPosition(0));
    assert_eq!(group.entries[1].value, EnumValue::BitPosition(63));
}

#[test]
fn a_nested_feature_element_does_not_overwrite_the_api_version() {
    // <feature> means two unrelated things. As a child of <registry> it is an
    // API version; inside a <require> block it names a member of a
    // physical-device feature struct, and the real registry has hundreds of
    // the latter against a handful of the former.
    let parsed = parse(
        r#"<feature api="vulkan" name="VK_VERSION_1_0" number="1.0">
             <require>
               <type name="VkInstance"/>
               <feature name="shaderInt64" struct="VkPhysicalDeviceFeatures"/>
             </require>
           </feature>"#,
    );
    assert_eq!(parsed.features.len(), 1);
    assert_eq!(parsed.features[0].name, "VK_VERSION_1_0");
    assert!(parsed.features[0].requirements[0].types.contains(&"VkInstance".to_owned()));
}

#[test]
fn a_feature_closure_pulls_in_what_it_depends_on() {
    // The registry layers the core versions behind internal features, so
    // asking for VK_VERSION_1_0 alone yields a fraction of the API unless the
    // dependency chain is followed.
    let parsed = parse(
        r#"<feature api="vulkan" name="VK_BASE_VERSION_1_0" number="1.0" apitype="internal">
             <require><type name="VkBaseType"/></require>
           </feature>
           <feature api="vulkan" name="VK_GRAPHICS_VERSION_1_0" number="1.0" apitype="internal"
                    depends="VK_BASE_VERSION_1_0">
             <require><type name="VkGraphicsType"/></require>
           </feature>
           <feature api="vulkan" name="VK_VERSION_1_0" number="1.0"
                    depends="VK_GRAPHICS_VERSION_1_0">
             <require><type name="VkCoreType"/></require>
           </feature>"#,
    );

    let closure = parsed.resolve_feature_closure(&["VK_VERSION_1_0"]);
    assert!(closure.contains("VK_VERSION_1_0"));
    assert!(closure.contains("VK_GRAPHICS_VERSION_1_0"));
    assert!(closure.contains("VK_BASE_VERSION_1_0"));

    let required = parsed.required_names(&["VK_VERSION_1_0"], &[]);
    for expected in ["VkCoreType", "VkGraphicsType", "VkBaseType"] {
        assert!(required.types.contains(expected), "closure dropped {expected}");
    }
}

#[test]
fn a_funcpointer_keeps_its_full_signature() {
    let parsed = parse(
        r#"<types>
             <type category="funcpointer">
               <proto><type>void</type>* <name>PFN_vkTestAllocation</name></proto>
               <param><type>void</type>* <name>pUserData</name></param>
               <param><type>size_t</type> <name>size</name></param>
             </type>
           </types>"#,
    );
    assert_eq!(parsed.func_pointers.len(), 1);
    let pointer = &parsed.func_pointers[0];
    assert_eq!(pointer.name, "PFN_vkTestAllocation");
    assert_eq!(pointer.return_type.type_name, "void");
    assert_eq!(pointer.return_type.pointer_depth, 1);
    assert_eq!(pointer.parameters.len(), 2);
    assert_eq!(pointer.parameters[0].name, "pUserData");
    assert_eq!(pointer.parameters[0].pointer_depth, 1);
    assert_eq!(pointer.parameters[1].type_name, "size_t");
    assert_eq!(pointer.parameters[1].pointer_depth, 0);
}

#[test]
fn external_synchronisation_prose_is_not_read_as_a_parameter() {
    // <implicitexternsyncparams> holds English sentences inside <param> tags.
    // Seven core commands carry one; reading them as parameters produces
    // members with neither a type nor a name.
    let parsed = parse(
        r#"<commands>
             <command>
               <proto><type>void</type> <name>vkDestroyTest</name></proto>
               <param><type>VkInstance</type> <name>instance</name></param>
               <implicitexternsyncparams>
                 <param>all sname:VkPhysicalDevice objects enumerated from pname:instance</param>
               </implicitexternsyncparams>
             </command>
           </commands>"#,
    );
    let command = parsed.commands.iter().find(|c| c.name == "vkDestroyTest").expect("command");
    assert_eq!(command.parameters.len(), 1);
    assert_eq!(command.parameters[0].name, "instance");
}

#[test]
fn a_quoted_value_is_a_string_constant_not_an_enumerator() {
    let parsed = parse(
        r#"<extensions>
             <extension name="VK_KHR_test" number="7" supported="vulkan">
               <require>
                 <enum value="&quot;VK_KHR_test&quot;" name="VK_KHR_TEST_EXTENSION_NAME"/>
                 <enum value="1" name="VK_KHR_TEST_SPEC_VERSION"/>
               </require>
             </extension>
           </extensions>"#,
    );
    assert_eq!(
        parsed.string_constants.get("VK_KHR_TEST_EXTENSION_NAME").map(String::as_str),
        Some("VK_KHR_test")
    );
    assert!(!parsed.string_constants.contains_key("VK_KHR_TEST_SPEC_VERSION"));
}

#[test]
fn a_non_integer_constant_survives_with_its_text() {
    // VK_LOD_CLAMP_NONE is 1000.0F. An integer-only model drops it silently,
    // and the emitted bindings are then missing a constant callers need.
    let parsed = parse(
        r#"<enums name="API Constants">
             <enum type="float" value="1000.0F" name="VK_LOD_CLAMP_NONE"/>
             <enum type="uint64_t" value="(~0ULL)" name="VK_WHOLE_SIZE"/>
           </enums>"#,
    );
    let group = parsed.enum_groups.get("API Constants").expect("constants");
    let clamp = group.entries.iter().find(|e| e.name == "VK_LOD_CLAMP_NONE").expect("clamp");
    assert_eq!(clamp.raw_value.as_deref(), Some("1000.0F"));
    assert_eq!(clamp.c_type.as_deref(), Some("float"));
}

#[test]
fn declarations_that_do_not_apply_to_this_api_are_skipped() {
    // vk.xml describes Vulkan and Vulkan SC from one file. Emitting the SC
    // variants would produce a binding for an API this project does not target.
    let parsed = parse(
        r#"<enums name="VkTest" type="enum">
             <enum value="0" name="VK_TEST_BOTH"/>
             <enum value="1" name="VK_TEST_SC_ONLY" api="vulkansc"/>
           </enums>"#,
    );
    let group = parsed.enum_groups.get("VkTest").expect("group");
    let names: Vec<&str> = group.entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, vec!["VK_TEST_BOTH"]);
}
