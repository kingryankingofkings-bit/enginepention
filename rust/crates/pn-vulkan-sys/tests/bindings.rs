// Pention Engine - pn-vulkan-sys binding tests
// Requirement: PN-RND-001, PN-RND-002
// Decision:    ADR-0009, ADR-0010
//
// Every expected value here comes from the Vulkan specification, not from the
// generator's own output. A test that asserted what vkgen happened to produce
// would pass for a generator that is consistently wrong.
//
// What these cannot check is whether a driver agrees: there is no Vulkan
// implementation in this environment (BLOCK-002, BLOCK-004 in
// docs/ENVIRONMENT_BASELINE.md). Layout and value correctness is checked here;
// end-to-end correctness stays unverified until it runs against a real ICD.

use core::mem::{align_of, size_of};

use pn_vulkan_sys::*;

#[test]
fn result_codes_match_the_specification() {
    assert_eq!(VkResult::VK_SUCCESS.0, 0);
    assert_eq!(VkResult::VK_NOT_READY.0, 1);
    assert_eq!(VkResult::VK_TIMEOUT.0, 2);
    assert_eq!(VkResult::VK_INCOMPLETE.0, 5);
    assert_eq!(VkResult::VK_ERROR_OUT_OF_HOST_MEMORY.0, -1);
    assert_eq!(VkResult::VK_ERROR_DEVICE_LOST.0, -4);
    assert_eq!(VkResult::VK_ERROR_FORMAT_NOT_SUPPORTED.0, -11);
}

#[test]
fn an_extension_contributed_value_uses_the_registry_formula() {
    // 1000000000 + (extnumber - 1) * 1000 + offset. VK_KHR_swapchain is
    // extension 2, so its first structure type is 1000001000 and its
    // out-of-date error is -1000001004.
    assert_eq!(VkStructureType::SWAPCHAIN_CREATE_INFO_KHR.0, 1_000_001_000);
    assert_eq!(VkResult::VK_ERROR_OUT_OF_DATE_KHR.0, -1_000_001_004);
    // VK_KHR_surface is extension 1.
    assert_eq!(VkResult::VK_ERROR_SURFACE_LOST_KHR.0, -1_000_000_000);
}

#[test]
fn a_dispatchable_handle_is_pointer_sized_and_a_non_dispatchable_one_is_not() {
    // The distinction is in the specification's handle macros: a
    // non-dispatchable handle is 64 bits on every target, including 32-bit
    // ones. Collapsing the two is a silent ABI break.
    assert_eq!(size_of::<VkInstance>(), size_of::<*mut core::ffi::c_void>());
    assert_eq!(size_of::<VkDevice>(), size_of::<*mut core::ffi::c_void>());
    assert_eq!(size_of::<VkBuffer>(), 8);
    assert_eq!(size_of::<VkDeviceMemory>(), 8);
}

#[test]
fn a_null_handle_is_all_zero_bits() {
    assert!(VkInstance::NULL.is_null());
    assert!(VkBuffer::NULL.is_null());
    assert_eq!(VkBuffer::NULL.0, 0);
}

#[test]
fn a_struct_defaults_to_its_own_structure_type() {
    // A zeroed sType is VK_STRUCTURE_TYPE_APPLICATION_INFO, which is a valid
    // value, so an unset sType is not something the validation layers will
    // reliably catch.
    assert_eq!(
        VkApplicationInfo::default().sType,
        VkStructureType::APPLICATION_INFO
    );
    assert_eq!(
        VkInstanceCreateInfo::default().sType,
        VkStructureType::INSTANCE_CREATE_INFO
    );
    assert_eq!(
        VkSwapchainCreateInfoKHR::default().sType,
        VkStructureType::SWAPCHAIN_CREATE_INFO_KHR
    );
    assert!(VkApplicationInfo::default().pNext.is_null());
}

#[test]
fn simple_structs_have_the_layout_c_would_give_them() {
    // VkOffset2D is two int32_t; VkExtent3D is three uint32_t. These are the
    // shapes a mis-parsed member list would change first.
    assert_eq!(size_of::<VkOffset2D>(), 8);
    assert_eq!(size_of::<VkExtent2D>(), 8);
    assert_eq!(size_of::<VkExtent3D>(), 12);
    assert_eq!(size_of::<VkRect2D>(), 16);
    assert_eq!(align_of::<VkOffset2D>(), 4);
}

#[test]
fn fixed_size_arrays_use_the_constants_the_specification_names() {
    assert_eq!(VK_UUID_SIZE, 16);
    assert_eq!(VK_LUID_SIZE, 8);
    assert_eq!(VK_MAX_EXTENSION_NAME_SIZE, 256);
    assert_eq!(VK_MAX_PHYSICAL_DEVICE_NAME_SIZE, 256);
    assert_eq!(VK_MAX_MEMORY_TYPES, 32);
    assert_eq!(VK_MAX_MEMORY_HEAPS, 16);

    assert_eq!(size_of::<VkExtensionProperties>(), VK_MAX_EXTENSION_NAME_SIZE + 4);
}

#[test]
fn sentinel_constants_keep_the_width_the_specification_gives_them() {
    // VK_WHOLE_SIZE is ~0ULL and VK_ATTACHMENT_UNUSED is ~0U. Inferring the
    // width from the magnitude gets one of the two wrong whichever way it
    // guesses.
    assert_eq!(VK_WHOLE_SIZE, u64::MAX);
    assert_eq!(VK_ATTACHMENT_UNUSED, u32::MAX);
    assert_eq!(VK_REMAINING_MIP_LEVELS, u32::MAX);
    assert_eq!(VK_QUEUE_FAMILY_IGNORED, u32::MAX);
    assert_eq!(VK_TRUE, 1);
    assert_eq!(VK_FALSE, 0);
    assert_eq!(VK_LOD_CLAMP_NONE, 1000.0f32);
}

#[test]
fn flag_bits_combine_into_the_flags_word_they_belong_to() {
    let usage = VkBufferUsageFlags::from(VkBufferUsageFlagBits::TRANSFER_DST_BIT)
        | VkBufferUsageFlags::from(VkBufferUsageFlagBits::VERTEX_BUFFER_BIT);

    assert!(usage.contains(VkBufferUsageFlags::TRANSFER_DST_BIT));
    assert!(usage.contains(VkBufferUsageFlags::VERTEX_BUFFER_BIT));
    assert!(!usage.contains(VkBufferUsageFlags::INDEX_BUFFER_BIT));

    // Published bit positions: TRANSFER_SRC is bit 0, TRANSFER_DST bit 1.
    assert_eq!(VkBufferUsageFlagBits::TRANSFER_SRC_BIT.0, 0x0000_0001);
    assert_eq!(VkBufferUsageFlagBits::TRANSFER_DST_BIT.0, 0x0000_0002);
}

#[test]
fn a_64_bit_flags_word_reaches_its_top_bit() {
    // VkPipelineStageFlags2 and VkAccessFlags2 are 64 bits wide precisely so
    // that synchronization2 could keep adding bits. Emitting them as 32-bit
    // truncates silently.
    assert_eq!(size_of::<VkPipelineStageFlags2>(), 8);
    assert_eq!(size_of::<VkAccessFlags2>(), 8);
    assert_eq!(VkPipelineStageFlagBits2::ALL_COMMANDS_BIT.0, 0x0000_0000_0001_0000);
}

#[test]
fn an_unlisted_value_is_representable_rather_than_undefined() {
    // The reason enumerations are newtypes and not Rust enums: a driver may
    // return a value this registry does not list, and holding one in a real
    // enum is undefined behaviour rather than merely surprising.
    let from_a_newer_driver = VkResult(-1_000_999_999);
    assert_ne!(from_a_newer_driver, VkResult::VK_SUCCESS);
    assert_eq!(from_a_newer_driver.0, -1_000_999_999);
}

#[test]
fn a_callback_slot_is_null_when_the_struct_is_zeroed() {
    // Callbacks are Option-wrapped so that zeroing is sound; a bare `fn` is a
    // non-null type and zeroing one is undefined behaviour.
    let callbacks = VkAllocationCallbacks::default();
    assert!(callbacks.pfnAllocation.is_none());
    assert!(callbacks.pfnFree.is_none());
    assert_eq!(size_of::<PFN_vkAllocationFunction>(), size_of::<*mut core::ffi::c_void>());
}

#[test]
fn extension_name_constants_are_nul_terminated() {
    // Rust string literals are not NUL-terminated. Handing one to
    // vkCreateInstance reads past the end of the allocation.
    assert_eq!(VK_KHR_SWAPCHAIN_EXTENSION_NAME, b"VK_KHR_swapchain\0");
    assert_eq!(VK_KHR_SURFACE_EXTENSION_NAME, b"VK_KHR_surface\0");
    assert_eq!(*VK_KHR_SWAPCHAIN_EXTENSION_NAME.last().expect("non-empty"), 0);
}

#[test]
fn dispatch_tables_are_split_by_the_loader_that_can_resolve_them() {
    // vkCreateInstance must be reachable before an instance exists, so it
    // cannot live in the instance table. vkCmdDraw is device-level and is
    // fetched from vkGetDeviceProcAddr, which returns a pointer into the
    // driver rather than into the loader's dispatch trampoline.
    let entry = EntryFns {
        create_instance: None,
        get_device_proc_addr: None,
        get_instance_proc_addr: None,
        enumerate_instance_version: None,
        enumerate_instance_layer_properties: None,
        enumerate_instance_extension_properties: None,
    };
    assert!(entry.create_instance.is_none());

    // Compile-time proof that these fields exist on the tables named. If a
    // command were classified into the wrong table this would not build.
    fn accepts_instance_field(fns: &InstanceFns) -> bool {
        fns.destroy_instance.is_none() && fns.create_device.is_none()
    }
    fn accepts_device_field(fns: &DeviceFns) -> bool {
        fns.cmd_draw.is_none() && fns.queue_submit.is_none() && fns.create_swapchain_khr.is_none()
    }
    let _ = accepts_instance_field;
    let _ = accepts_device_field;
}

#[test]
fn a_command_signature_is_a_system_abi_function_pointer() {
    // extern "system" is VKAPI_PTR: stdcall on 32-bit Windows, the platform C
    // ABI elsewhere. The size assertion is weak on its own; the value of this
    // test is that it does not compile if the type is not a function pointer.
    assert_eq!(size_of::<PFN_vkCreateInstance>(), size_of::<*mut core::ffi::c_void>());
    assert_eq!(size_of::<Option<PFN_vkCreateInstance>>(), size_of::<PFN_vkCreateInstance>());
}
