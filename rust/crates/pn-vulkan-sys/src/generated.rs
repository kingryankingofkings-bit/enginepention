// Pention Engine - GENERATED FILE, DO NOT EDIT BY HAND.
// Requirement: PN-RND-001
// Decision:    ADR-0009, ADR-0010
//
// Produced by rust/tools/vkgen from the official Khronos Vulkan
// registry. Regenerate with:
//
//     python3 build_scripts/fetch_vulkan_registry.py
//     cargo run -p vkgen -- third_party/vk.xml --emit <path>
//
// Registry provenance:
//   VK_HEADER_VERSION 360
//   vk.xml sha256     65d829561fa4b9e01a15e1327d9e6744f66b025b08c5c7ad13636bf0a8b15c62
//   source            KhronosGroup/Vulkan-Headers registry/vk.xml
//
// Scope:
//   feature   VK_VERSION_1_0
//   feature   VK_VERSION_1_1
//   feature   VK_VERSION_1_2
//   feature   VK_VERSION_1_3
//   extension VK_KHR_surface
//   extension VK_KHR_swapchain
//
// The registry is a specification, not an implementation: no Khronos
// code is copied here, and no binding crate is used. See
// DEPENDENCY_BOUNDARY.md.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_snake_case)]

// ---------------------------------------------------------------
// Base types
// ---------------------------------------------------------------

pub type VkSampleMask = u32;
pub type VkBool32 = u32;
pub type VkFlags = u32;
pub type VkFlags64 = u64;
pub type VkDeviceSize = u64;
pub type VkDeviceAddress = u64;

// ---------------------------------------------------------------
// API constants
//
// Names used as array extents are emitted as usize so they can be
// written directly in an array type; the rest keep the width the
// registry declares. Guessing the width from the magnitude would
// get VK_WHOLE_SIZE (~0ULL) and VK_ATTACHMENT_UNUSED (~0U) wrong in
// opposite directions.
// ---------------------------------------------------------------

pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const VK_UUID_SIZE: usize = 16;
pub const VK_LUID_SIZE: usize = 8;
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;
pub const VK_MAX_MEMORY_TYPES: usize = 32;
pub const VK_MAX_MEMORY_HEAPS: usize = 16;
pub const VK_LOD_CLAMP_NONE: f32 = 1000.0f32;
pub const VK_REMAINING_MIP_LEVELS: u32 = 4294967295;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = 4294967295;
pub const VK_REMAINING_3D_SLICES_EXT: u32 = 4294967295;
pub const VK_WHOLE_SIZE: u64 = 18446744073709551615;
pub const VK_ATTACHMENT_UNUSED: u32 = 4294967295;
pub const VK_TRUE: u32 = 1;
pub const VK_FALSE: u32 = 0;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = 4294967295;
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = 4294967294;
pub const VK_QUEUE_FAMILY_FOREIGN_EXT: u32 = 4294967293;
pub const VK_SUBPASS_EXTERNAL: u32 = 4294967295;
pub const VK_MAX_DEVICE_GROUP_SIZE: usize = 32;
pub const VK_MAX_DRIVER_NAME_SIZE: usize = 256;
pub const VK_MAX_DRIVER_INFO_SIZE: usize = 256;
pub const VK_SHADER_UNUSED_KHR: u32 = 4294967295;
pub const VK_MAX_GLOBAL_PRIORITY_SIZE: u32 = 16;
pub const VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;
pub const VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR: u32 = 32;
pub const VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR: u32 = 7;
pub const VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR: u32 = 3;
pub const VK_SHADER_INDEX_UNUSED_AMDX: u32 = 4294967295;
pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV: u32 = 4294967295;
pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX: u32 = 128;
pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX: u32 = 128;
pub const VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM: u32 = 128;
pub const VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM: u32 = 3;
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV: f32 = 0.25f32;
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV: f32 = 0.50f32;
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV: f32 = 0.75f32;
pub const VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM: u32 = 128;
pub const VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM: u32 = 4;

// ---------------------------------------------------------------
// Extension names
//
// NUL-terminated so they can be handed to Vulkan directly. Rust
// string literals are not NUL-terminated, and passing one to
// vkCreateInstance reads past the end of the allocation.
// ---------------------------------------------------------------

pub const VK_KHR_SURFACE_EXTENSION_NAME: &[u8] = b"VK_KHR_surface\0";
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &[u8] = b"VK_KHR_swapchain\0";

// ---------------------------------------------------------------
// Handles
//
// A dispatchable handle is an opaque pointer; a non-dispatchable one
// is a 64-bit integer EVEN ON 32-BIT TARGETS. Treating them alike is
// a silent ABI break on a 32-bit build, so the two are distinct here.
// ---------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkInstance(pub *mut core::ffi::c_void);
impl VkInstance {
    pub const NULL: Self = Self(core::ptr::null_mut());
    #[inline]
    pub fn is_null(self) -> bool { self.0.is_null() }
}

/// Child of `VkInstance`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkPhysicalDevice(pub *mut core::ffi::c_void);
impl VkPhysicalDevice {
    pub const NULL: Self = Self(core::ptr::null_mut());
    #[inline]
    pub fn is_null(self) -> bool { self.0.is_null() }
}

/// Child of `VkPhysicalDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkDevice(pub *mut core::ffi::c_void);
impl VkDevice {
    pub const NULL: Self = Self(core::ptr::null_mut());
    #[inline]
    pub fn is_null(self) -> bool { self.0.is_null() }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkQueue(pub *mut core::ffi::c_void);
impl VkQueue {
    pub const NULL: Self = Self(core::ptr::null_mut());
    #[inline]
    pub fn is_null(self) -> bool { self.0.is_null() }
}

/// Child of `VkCommandPool`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkCommandBuffer(pub *mut core::ffi::c_void);
impl VkCommandBuffer {
    pub const NULL: Self = Self(core::ptr::null_mut());
    #[inline]
    pub fn is_null(self) -> bool { self.0.is_null() }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkDeviceMemory(pub u64);
impl VkDeviceMemory {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkCommandPool(pub u64);
impl VkCommandPool {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkBuffer(pub u64);
impl VkBuffer {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkBufferView(pub u64);
impl VkBufferView {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkImage(pub u64);
impl VkImage {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkImageView(pub u64);
impl VkImageView {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkShaderModule(pub u64);
impl VkShaderModule {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkPipeline(pub u64);
impl VkPipeline {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkPipelineLayout(pub u64);
impl VkPipelineLayout {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkSampler(pub u64);
impl VkSampler {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDescriptorPool`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkDescriptorSet(pub u64);
impl VkDescriptorSet {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkDescriptorSetLayout(pub u64);
impl VkDescriptorSetLayout {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkDescriptorPool(pub u64);
impl VkDescriptorPool {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkFence(pub u64);
impl VkFence {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkSemaphore(pub u64);
impl VkSemaphore {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkEvent(pub u64);
impl VkEvent {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkQueryPool(pub u64);
impl VkQueryPool {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkFramebuffer(pub u64);
impl VkFramebuffer {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkRenderPass(pub u64);
impl VkRenderPass {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkPipelineCache(pub u64);
impl VkPipelineCache {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkDescriptorUpdateTemplate(pub u64);
impl VkDescriptorUpdateTemplate {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkSamplerYcbcrConversion(pub u64);
impl VkSamplerYcbcrConversion {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkPrivateDataSlot(pub u64);
impl VkPrivateDataSlot {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkInstance`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkSurfaceKHR(pub u64);
impl VkSurfaceKHR {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

/// Child of `VkDevice`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VkSwapchainKHR(pub u64);
impl VkSwapchainKHR {
    pub const NULL: Self = Self(0);
    #[inline]
    pub fn is_null(self) -> bool { self.0 == 0 }
}

// ---------------------------------------------------------------
// Enumerations
//
// Emitted as transparent newtypes rather than Rust enums. A driver
// may return a value this registry does not list - a vendor
// extension, or a code added after these bindings were generated -
// and holding such a value in a real enum is undefined behaviour,
// not merely surprising.
// ---------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkAccessFlagBits(pub u32);

impl VkAccessFlagBits {
    pub const INDIRECT_COMMAND_READ_BIT: Self = Self(1);
    pub const INDEX_READ_BIT: Self = Self(2);
    pub const VERTEX_ATTRIBUTE_READ_BIT: Self = Self(4);
    pub const UNIFORM_READ_BIT: Self = Self(8);
    pub const INPUT_ATTACHMENT_READ_BIT: Self = Self(16);
    pub const SHADER_READ_BIT: Self = Self(32);
    pub const SHADER_WRITE_BIT: Self = Self(64);
    pub const COLOR_ATTACHMENT_READ_BIT: Self = Self(128);
    pub const COLOR_ATTACHMENT_WRITE_BIT: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_READ_BIT: Self = Self(512);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: Self = Self(1024);
    pub const TRANSFER_READ_BIT: Self = Self(2048);
    pub const TRANSFER_WRITE_BIT: Self = Self(4096);
    pub const HOST_READ_BIT: Self = Self(8192);
    pub const HOST_WRITE_BIT: Self = Self(16384);
    pub const MEMORY_READ_BIT: Self = Self(32768);
    pub const MEMORY_WRITE_BIT: Self = Self(65536);
    pub const NONE: Self = Self(0);
    pub const TRANSFORM_FEEDBACK_WRITE_BIT_EXT: Self = Self(33554432);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT: Self = Self(67108864);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT: Self = Self(134217728);
    pub const CONDITIONAL_RENDERING_READ_BIT_EXT: Self = Self(1048576);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: Self = Self(524288);
    pub const ACCELERATION_STRUCTURE_READ_BIT_KHR: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_WRITE_BIT_KHR: Self = Self(4194304);
    pub const SHADING_RATE_IMAGE_READ_BIT_NV: Self = Self(8388608);
    pub const ACCELERATION_STRUCTURE_READ_BIT_NV: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_WRITE_BIT_NV: Self = Self(4194304);
    pub const FRAGMENT_DENSITY_MAP_READ_BIT_EXT: Self = Self(16777216);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: Self = Self(8388608);
    pub const COMMAND_PREPROCESS_READ_BIT_NV: Self = Self(131072);
    pub const COMMAND_PREPROCESS_WRITE_BIT_NV: Self = Self(262144);
    pub const NONE_KHR: Self = Self(0);
    pub const COMMAND_PREPROCESS_READ_BIT_EXT: Self = Self(131072);
    pub const COMMAND_PREPROCESS_WRITE_BIT_EXT: Self = Self(262144);
}

impl core::ops::BitOr for VkAccessFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkAccessFlagBits2(pub u64);

impl VkAccessFlagBits2 {
    pub const NONE: Self = Self(0);
    pub const INDIRECT_COMMAND_READ_BIT: Self = Self(1);
    pub const INDEX_READ_BIT: Self = Self(2);
    pub const VERTEX_ATTRIBUTE_READ_BIT: Self = Self(4);
    pub const UNIFORM_READ_BIT: Self = Self(8);
    pub const INPUT_ATTACHMENT_READ_BIT: Self = Self(16);
    pub const SHADER_READ_BIT: Self = Self(32);
    pub const SHADER_WRITE_BIT: Self = Self(64);
    pub const COLOR_ATTACHMENT_READ_BIT: Self = Self(128);
    pub const COLOR_ATTACHMENT_WRITE_BIT: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_READ_BIT: Self = Self(512);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: Self = Self(1024);
    pub const TRANSFER_READ_BIT: Self = Self(2048);
    pub const TRANSFER_WRITE_BIT: Self = Self(4096);
    pub const HOST_READ_BIT: Self = Self(8192);
    pub const HOST_WRITE_BIT: Self = Self(16384);
    pub const MEMORY_READ_BIT: Self = Self(32768);
    pub const MEMORY_WRITE_BIT: Self = Self(65536);
    pub const SHADER_SAMPLED_READ_BIT: Self = Self(4294967296);
    pub const SHADER_STORAGE_READ_BIT: Self = Self(8589934592);
    pub const SHADER_STORAGE_WRITE_BIT: Self = Self(17179869184);
    pub const VIDEO_DECODE_READ_BIT_KHR: Self = Self(34359738368);
    pub const VIDEO_DECODE_WRITE_BIT_KHR: Self = Self(68719476736);
    pub const SAMPLER_HEAP_READ_BIT_EXT: Self = Self(144115188075855872);
    pub const RESOURCE_HEAP_READ_BIT_EXT: Self = Self(288230376151711744);
    pub const RESERVED_46_BIT_INTEL: Self = Self(70368744177664);
    pub const VIDEO_ENCODE_READ_BIT_KHR: Self = Self(137438953472);
    pub const VIDEO_ENCODE_WRITE_BIT_KHR: Self = Self(274877906944);
    pub const RESERVED_53_BIT_KHR: Self = Self(9007199254740992);
    pub const RESERVED_54_BIT_KHR: Self = Self(18014398509481984);
    pub const SHADER_TILE_ATTACHMENT_READ_BIT_QCOM: Self = Self(2251799813685248);
    pub const SHADER_TILE_ATTACHMENT_WRITE_BIT_QCOM: Self = Self(4503599627370496);
    pub const NONE_KHR: Self = Self(0);
    pub const INDIRECT_COMMAND_READ_BIT_KHR: Self = Self(1);
    pub const INDEX_READ_BIT_KHR: Self = Self(2);
    pub const VERTEX_ATTRIBUTE_READ_BIT_KHR: Self = Self(4);
    pub const UNIFORM_READ_BIT_KHR: Self = Self(8);
    pub const INPUT_ATTACHMENT_READ_BIT_KHR: Self = Self(16);
    pub const SHADER_READ_BIT_KHR: Self = Self(32);
    pub const SHADER_WRITE_BIT_KHR: Self = Self(64);
    pub const COLOR_ATTACHMENT_READ_BIT_KHR: Self = Self(128);
    pub const COLOR_ATTACHMENT_WRITE_BIT_KHR: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR: Self = Self(512);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR: Self = Self(1024);
    pub const TRANSFER_READ_BIT_KHR: Self = Self(2048);
    pub const TRANSFER_WRITE_BIT_KHR: Self = Self(4096);
    pub const HOST_READ_BIT_KHR: Self = Self(8192);
    pub const HOST_WRITE_BIT_KHR: Self = Self(16384);
    pub const MEMORY_READ_BIT_KHR: Self = Self(32768);
    pub const MEMORY_WRITE_BIT_KHR: Self = Self(65536);
    pub const SHADER_SAMPLED_READ_BIT_KHR: Self = Self(4294967296);
    pub const SHADER_STORAGE_READ_BIT_KHR: Self = Self(8589934592);
    pub const SHADER_STORAGE_WRITE_BIT_KHR: Self = Self(17179869184);
    pub const TRANSFORM_FEEDBACK_WRITE_BIT_EXT: Self = Self(33554432);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT: Self = Self(67108864);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT: Self = Self(134217728);
    pub const CONDITIONAL_RENDERING_READ_BIT_EXT: Self = Self(1048576);
    pub const COMMAND_PREPROCESS_READ_BIT_NV: Self = Self(131072);
    pub const COMMAND_PREPROCESS_WRITE_BIT_NV: Self = Self(262144);
    pub const COMMAND_PREPROCESS_READ_BIT_EXT: Self = Self(131072);
    pub const COMMAND_PREPROCESS_WRITE_BIT_EXT: Self = Self(262144);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: Self = Self(8388608);
    pub const SHADING_RATE_IMAGE_READ_BIT_NV: Self = Self(8388608);
    pub const ACCELERATION_STRUCTURE_READ_BIT_KHR: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_WRITE_BIT_KHR: Self = Self(4194304);
    pub const ACCELERATION_STRUCTURE_READ_BIT_NV: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_WRITE_BIT_NV: Self = Self(4194304);
    pub const FRAGMENT_DENSITY_MAP_READ_BIT_EXT: Self = Self(16777216);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: Self = Self(524288);
    pub const DESCRIPTOR_BUFFER_READ_BIT_EXT: Self = Self(2199023255552);
    pub const INVOCATION_MASK_READ_BIT_HUAWEI: Self = Self(549755813888);
    pub const SHADER_BINDING_TABLE_READ_BIT_KHR: Self = Self(1099511627776);
    pub const MICROMAP_READ_BIT_EXT: Self = Self(17592186044416);
    pub const MICROMAP_WRITE_BIT_EXT: Self = Self(35184372088832);
    pub const OPTICAL_FLOW_READ_BIT_NV: Self = Self(4398046511104);
    pub const OPTICAL_FLOW_WRITE_BIT_NV: Self = Self(8796093022208);
    pub const DATA_GRAPH_READ_BIT_ARM: Self = Self(140737488355328);
    pub const DATA_GRAPH_WRITE_BIT_ARM: Self = Self(281474976710656);
    pub const MEMORY_DECOMPRESSION_READ_BIT_EXT: Self = Self(36028797018963968);
    pub const MEMORY_DECOMPRESSION_WRITE_BIT_EXT: Self = Self(72057594037927936);
    pub const RESERVED_62_BIT_EXT: Self = Self(4611686018427387904);
    pub const RESERVED_63_BIT_EXT: Self = Self(9223372036854775808);
    pub const RESERVED_60_BIT_KHR: Self = Self(1152921504606846976);
    pub const RESERVED_61_BIT_KHR: Self = Self(2305843009213693952);
    pub const RESERVED_28_BIT_AMD: Self = Self(268435456);
    pub const RESERVED_29_BIT_AMD: Self = Self(536870912);
    pub const RESERVED_49_BIT_ARM: Self = Self(562949953421312);
    pub const RESERVED_50_BIT_ARM: Self = Self(1125899906842624);
}

impl core::ops::BitOr for VkAccessFlagBits2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkAttachmentDescriptionFlagBits(pub u32);

impl VkAttachmentDescriptionFlagBits {
    pub const MAY_ALIAS_BIT: Self = Self(1);
    pub const RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR: Self = Self(2);
    pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR: Self = Self(4);
}

impl core::ops::BitOr for VkAttachmentDescriptionFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkAttachmentLoadOp(pub i32);

impl VkAttachmentLoadOp {
    pub const LOAD: Self = Self(0);
    pub const CLEAR: Self = Self(1);
    pub const DONT_CARE: Self = Self(2);
    pub const NONE: Self = Self(1000400000);
    pub const NONE_EXT: Self = Self(1000400000);
    pub const NONE_KHR: Self = Self(1000400000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkAttachmentStoreOp(pub i32);

impl VkAttachmentStoreOp {
    pub const STORE: Self = Self(0);
    pub const DONT_CARE: Self = Self(1);
    pub const NONE: Self = Self(1000301000);
    pub const NONE_KHR: Self = Self(1000301000);
    pub const NONE_QCOM: Self = Self(1000301000);
    pub const NONE_EXT: Self = Self(1000301000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkBlendFactor(pub i32);

impl VkBlendFactor {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const SRC_COLOR: Self = Self(2);
    pub const ONE_MINUS_SRC_COLOR: Self = Self(3);
    pub const DST_COLOR: Self = Self(4);
    pub const ONE_MINUS_DST_COLOR: Self = Self(5);
    pub const SRC_ALPHA: Self = Self(6);
    pub const ONE_MINUS_SRC_ALPHA: Self = Self(7);
    pub const DST_ALPHA: Self = Self(8);
    pub const ONE_MINUS_DST_ALPHA: Self = Self(9);
    pub const CONSTANT_COLOR: Self = Self(10);
    pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(11);
    pub const CONSTANT_ALPHA: Self = Self(12);
    pub const ONE_MINUS_CONSTANT_ALPHA: Self = Self(13);
    pub const SRC_ALPHA_SATURATE: Self = Self(14);
    pub const SRC1_COLOR: Self = Self(15);
    pub const ONE_MINUS_SRC1_COLOR: Self = Self(16);
    pub const SRC1_ALPHA: Self = Self(17);
    pub const ONE_MINUS_SRC1_ALPHA: Self = Self(18);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkBlendOp(pub i32);

impl VkBlendOp {
    pub const ADD: Self = Self(0);
    pub const SUBTRACT: Self = Self(1);
    pub const REVERSE_SUBTRACT: Self = Self(2);
    pub const MIN: Self = Self(3);
    pub const MAX: Self = Self(4);
    pub const ZERO_EXT: Self = Self(1000148000);
    pub const SRC_EXT: Self = Self(1000148001);
    pub const DST_EXT: Self = Self(1000148002);
    pub const SRC_OVER_EXT: Self = Self(1000148003);
    pub const DST_OVER_EXT: Self = Self(1000148004);
    pub const SRC_IN_EXT: Self = Self(1000148005);
    pub const DST_IN_EXT: Self = Self(1000148006);
    pub const SRC_OUT_EXT: Self = Self(1000148007);
    pub const DST_OUT_EXT: Self = Self(1000148008);
    pub const SRC_ATOP_EXT: Self = Self(1000148009);
    pub const DST_ATOP_EXT: Self = Self(1000148010);
    pub const XOR_EXT: Self = Self(1000148011);
    pub const MULTIPLY_EXT: Self = Self(1000148012);
    pub const SCREEN_EXT: Self = Self(1000148013);
    pub const OVERLAY_EXT: Self = Self(1000148014);
    pub const DARKEN_EXT: Self = Self(1000148015);
    pub const LIGHTEN_EXT: Self = Self(1000148016);
    pub const COLORDODGE_EXT: Self = Self(1000148017);
    pub const COLORBURN_EXT: Self = Self(1000148018);
    pub const HARDLIGHT_EXT: Self = Self(1000148019);
    pub const SOFTLIGHT_EXT: Self = Self(1000148020);
    pub const DIFFERENCE_EXT: Self = Self(1000148021);
    pub const EXCLUSION_EXT: Self = Self(1000148022);
    pub const INVERT_EXT: Self = Self(1000148023);
    pub const INVERT_RGB_EXT: Self = Self(1000148024);
    pub const LINEARDODGE_EXT: Self = Self(1000148025);
    pub const LINEARBURN_EXT: Self = Self(1000148026);
    pub const VIVIDLIGHT_EXT: Self = Self(1000148027);
    pub const LINEARLIGHT_EXT: Self = Self(1000148028);
    pub const PINLIGHT_EXT: Self = Self(1000148029);
    pub const HARDMIX_EXT: Self = Self(1000148030);
    pub const HSL_HUE_EXT: Self = Self(1000148031);
    pub const HSL_SATURATION_EXT: Self = Self(1000148032);
    pub const HSL_COLOR_EXT: Self = Self(1000148033);
    pub const HSL_LUMINOSITY_EXT: Self = Self(1000148034);
    pub const PLUS_EXT: Self = Self(1000148035);
    pub const PLUS_CLAMPED_EXT: Self = Self(1000148036);
    pub const PLUS_CLAMPED_ALPHA_EXT: Self = Self(1000148037);
    pub const PLUS_DARKER_EXT: Self = Self(1000148038);
    pub const MINUS_EXT: Self = Self(1000148039);
    pub const MINUS_CLAMPED_EXT: Self = Self(1000148040);
    pub const CONTRAST_EXT: Self = Self(1000148041);
    pub const INVERT_OVG_EXT: Self = Self(1000148042);
    pub const RED_EXT: Self = Self(1000148043);
    pub const GREEN_EXT: Self = Self(1000148044);
    pub const BLUE_EXT: Self = Self(1000148045);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkBorderColor(pub i32);

impl VkBorderColor {
    pub const FLOAT_TRANSPARENT_BLACK: Self = Self(0);
    pub const INT_TRANSPARENT_BLACK: Self = Self(1);
    pub const FLOAT_OPAQUE_BLACK: Self = Self(2);
    pub const INT_OPAQUE_BLACK: Self = Self(3);
    pub const FLOAT_OPAQUE_WHITE: Self = Self(4);
    pub const INT_OPAQUE_WHITE: Self = Self(5);
    pub const FLOAT_CUSTOM_EXT: Self = Self(1000287003);
    pub const INT_CUSTOM_EXT: Self = Self(1000287004);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkBufferCreateFlagBits(pub u32);

impl VkBufferCreateFlagBits {
    pub const SPARSE_BINDING_BIT: Self = Self(1);
    pub const SPARSE_RESIDENCY_BIT: Self = Self(2);
    pub const SPARSE_ALIASED_BIT: Self = Self(4);
    pub const PROTECTED_BIT: Self = Self(8);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: Self = Self(16);
    pub const RESERVED_7_BIT_IMG: Self = Self(128);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT: Self = Self(16);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: Self = Self(16);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(32);
    pub const VIDEO_PROFILE_INDEPENDENT_BIT_KHR: Self = Self(64);
}

impl core::ops::BitOr for VkBufferCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkBufferUsageFlagBits(pub u32);

impl VkBufferUsageFlagBits {
    pub const TRANSFER_SRC_BIT: Self = Self(1);
    pub const TRANSFER_DST_BIT: Self = Self(2);
    pub const UNIFORM_TEXEL_BUFFER_BIT: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER_BIT: Self = Self(8);
    pub const UNIFORM_BUFFER_BIT: Self = Self(16);
    pub const STORAGE_BUFFER_BIT: Self = Self(32);
    pub const INDEX_BUFFER_BIT: Self = Self(64);
    pub const VERTEX_BUFFER_BIT: Self = Self(128);
    pub const INDIRECT_BUFFER_BIT: Self = Self(256);
    pub const SHADER_DEVICE_ADDRESS_BIT: Self = Self(131072);
    pub const VIDEO_DECODE_SRC_BIT_KHR: Self = Self(8192);
    pub const VIDEO_DECODE_DST_BIT_KHR: Self = Self(16384);
    pub const TRANSFORM_FEEDBACK_BUFFER_BIT_EXT: Self = Self(2048);
    pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT: Self = Self(4096);
    pub const CONDITIONAL_RENDERING_BIT_EXT: Self = Self(512);
    pub const EXECUTION_GRAPH_SCRATCH_BIT_AMDX: Self = Self(33554432);
    pub const DESCRIPTOR_HEAP_BIT_EXT: Self = Self(268435456);
    pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR: Self = Self(524288);
    pub const ACCELERATION_STRUCTURE_STORAGE_BIT_KHR: Self = Self(1048576);
    pub const SHADER_BINDING_TABLE_BIT_KHR: Self = Self(1024);
    pub const RAY_TRACING_BIT_NV: Self = Self(1024);
    pub const SHADER_DEVICE_ADDRESS_BIT_EXT: Self = Self(131072);
    pub const SHADER_DEVICE_ADDRESS_BIT_KHR: Self = Self(131072);
    pub const VIDEO_ENCODE_DST_BIT_KHR: Self = Self(32768);
    pub const VIDEO_ENCODE_SRC_BIT_KHR: Self = Self(65536);
    pub const SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(2097152);
    pub const RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(4194304);
    pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(67108864);
    pub const MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT: Self = Self(8388608);
    pub const MICROMAP_STORAGE_BIT_EXT: Self = Self(16777216);
    pub const TILE_MEMORY_BIT_QCOM: Self = Self(134217728);
}

impl core::ops::BitOr for VkBufferUsageFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkChromaLocation(pub i32);

impl VkChromaLocation {
    pub const COSITED_EVEN: Self = Self(0);
    pub const MIDPOINT: Self = Self(1);
    pub const COSITED_EVEN_KHR: Self = Self(0);
    pub const MIDPOINT_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkColorComponentFlagBits(pub u32);

impl VkColorComponentFlagBits {
    pub const R_BIT: Self = Self(1);
    pub const G_BIT: Self = Self(2);
    pub const B_BIT: Self = Self(4);
    pub const A_BIT: Self = Self(8);
}

impl core::ops::BitOr for VkColorComponentFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkColorSpaceKHR(pub i32);

impl VkColorSpaceKHR {
    pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR: Self = Self(0);
    pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR: Self = Self(0);
    pub const VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT: Self = Self(1000104001);
    pub const VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT: Self = Self(1000104002);
    pub const VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT: Self = Self(1000104003);
    pub const VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT: Self = Self(1000104004);
    pub const VK_COLOR_SPACE_BT709_LINEAR_EXT: Self = Self(1000104005);
    pub const VK_COLOR_SPACE_BT709_NONLINEAR_EXT: Self = Self(1000104006);
    pub const VK_COLOR_SPACE_BT2020_LINEAR_EXT: Self = Self(1000104007);
    pub const VK_COLOR_SPACE_HDR10_ST2084_EXT: Self = Self(1000104008);
    pub const VK_COLOR_SPACE_DOLBYVISION_EXT: Self = Self(1000104009);
    pub const VK_COLOR_SPACE_HDR10_HLG_EXT: Self = Self(1000104010);
    pub const VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT: Self = Self(1000104011);
    pub const VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT: Self = Self(1000104012);
    pub const VK_COLOR_SPACE_PASS_THROUGH_EXT: Self = Self(1000104013);
    pub const VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1000104014);
    pub const VK_COLOR_SPACE_DCI_P3_LINEAR_EXT: Self = Self(1000104003);
    pub const VK_COLOR_SPACE_DISPLAY_NATIVE_AMD: Self = Self(1000213000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkCommandBufferLevel(pub i32);

impl VkCommandBufferLevel {
    pub const PRIMARY: Self = Self(0);
    pub const SECONDARY: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkCommandBufferResetFlagBits(pub u32);

impl VkCommandBufferResetFlagBits {
    pub const RELEASE_RESOURCES_BIT: Self = Self(1);
}

impl core::ops::BitOr for VkCommandBufferResetFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkCommandBufferUsageFlagBits(pub u32);

impl VkCommandBufferUsageFlagBits {
    pub const ONE_TIME_SUBMIT_BIT: Self = Self(1);
    pub const RENDER_PASS_CONTINUE_BIT: Self = Self(2);
    pub const SIMULTANEOUS_USE_BIT: Self = Self(4);
    pub const RESERVED_3_BIT_HUAWEI: Self = Self(8);
    pub const RESERVED_4_BIT_HUAWEI: Self = Self(16);
}

impl core::ops::BitOr for VkCommandBufferUsageFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkCommandPoolCreateFlagBits(pub u32);

impl VkCommandPoolCreateFlagBits {
    pub const TRANSIENT_BIT: Self = Self(1);
    pub const RESET_COMMAND_BUFFER_BIT: Self = Self(2);
    pub const PROTECTED_BIT: Self = Self(4);
}

impl core::ops::BitOr for VkCommandPoolCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkCommandPoolResetFlagBits(pub u32);

impl VkCommandPoolResetFlagBits {
    pub const RELEASE_RESOURCES_BIT: Self = Self(1);
    pub const RESERVED_1_BIT_COREAVI: Self = Self(2);
}

impl core::ops::BitOr for VkCommandPoolResetFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkCompareOp(pub i32);

impl VkCompareOp {
    pub const NEVER: Self = Self(0);
    pub const LESS: Self = Self(1);
    pub const EQUAL: Self = Self(2);
    pub const LESS_OR_EQUAL: Self = Self(3);
    pub const GREATER: Self = Self(4);
    pub const NOT_EQUAL: Self = Self(5);
    pub const GREATER_OR_EQUAL: Self = Self(6);
    pub const ALWAYS: Self = Self(7);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkComponentSwizzle(pub i32);

impl VkComponentSwizzle {
    pub const IDENTITY: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const ONE: Self = Self(2);
    pub const R: Self = Self(3);
    pub const G: Self = Self(4);
    pub const B: Self = Self(5);
    pub const A: Self = Self(6);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkCompositeAlphaFlagBitsKHR(pub u32);

impl VkCompositeAlphaFlagBitsKHR {
    pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: Self = Self(1);
    pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: Self = Self(2);
    pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: Self = Self(4);
    pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR: Self = Self(8);
}

impl core::ops::BitOr for VkCompositeAlphaFlagBitsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkCullModeFlagBits(pub u32);

impl VkCullModeFlagBits {
    pub const NONE: Self = Self(0);
    pub const FRONT_BIT: Self = Self(1);
    pub const BACK_BIT: Self = Self(2);
    pub const FRONT_AND_BACK: Self = Self(3);
}

impl core::ops::BitOr for VkCullModeFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkDependencyFlagBits(pub u32);

impl VkDependencyFlagBits {
    pub const BY_REGION_BIT: Self = Self(1);
    pub const DEVICE_GROUP_BIT: Self = Self(4);
    pub const VIEW_LOCAL_BIT: Self = Self(2);
    pub const VIEW_LOCAL_BIT_KHR: Self = Self(2);
    pub const DEVICE_GROUP_BIT_KHR: Self = Self(4);
    pub const FEEDBACK_LOOP_BIT_EXT: Self = Self(8);
    pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_BIT_KHR: Self = Self(32);
    pub const ASYMMETRIC_EVENT_BIT_KHR: Self = Self(64);
    pub const EXTENSION_586_BIT_IMG: Self = Self(16);
}

impl core::ops::BitOr for VkDependencyFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkDescriptorBindingFlagBits(pub u32);

impl VkDescriptorBindingFlagBits {
    pub const UPDATE_AFTER_BIND_BIT: Self = Self(1);
    pub const UPDATE_UNUSED_WHILE_PENDING_BIT: Self = Self(2);
    pub const PARTIALLY_BOUND_BIT: Self = Self(4);
    pub const VARIABLE_DESCRIPTOR_COUNT_BIT: Self = Self(8);
    pub const UPDATE_AFTER_BIND_BIT_EXT: Self = Self(1);
    pub const UPDATE_UNUSED_WHILE_PENDING_BIT_EXT: Self = Self(2);
    pub const PARTIALLY_BOUND_BIT_EXT: Self = Self(4);
    pub const VARIABLE_DESCRIPTOR_COUNT_BIT_EXT: Self = Self(8);
    pub const RESERVED_4_BIT_QCOM: Self = Self(16);
}

impl core::ops::BitOr for VkDescriptorBindingFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkDescriptorPoolCreateFlagBits(pub u32);

impl VkDescriptorPoolCreateFlagBits {
    pub const FREE_DESCRIPTOR_SET_BIT: Self = Self(1);
    pub const UPDATE_AFTER_BIND_BIT: Self = Self(2);
    pub const UPDATE_AFTER_BIND_BIT_EXT: Self = Self(2);
    pub const HOST_ONLY_BIT_VALVE: Self = Self(4);
    pub const HOST_ONLY_BIT_EXT: Self = Self(4);
    pub const ALLOW_OVERALLOCATION_SETS_BIT_NV: Self = Self(8);
    pub const ALLOW_OVERALLOCATION_POOLS_BIT_NV: Self = Self(16);
}

impl core::ops::BitOr for VkDescriptorPoolCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkDescriptorSetLayoutCreateFlagBits(pub u32);

impl VkDescriptorSetLayoutCreateFlagBits {
    pub const UPDATE_AFTER_BIND_POOL_BIT: Self = Self(2);
    pub const PUSH_DESCRIPTOR_BIT: Self = Self(1);
    pub const PUSH_DESCRIPTOR_BIT_KHR: Self = Self(1);
    pub const UPDATE_AFTER_BIND_POOL_BIT_EXT: Self = Self(2);
    pub const DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(16);
    pub const EMBEDDED_IMMUTABLE_SAMPLERS_BIT_EXT: Self = Self(32);
    pub const HOST_ONLY_POOL_BIT_VALVE: Self = Self(4);
    pub const INDIRECT_BINDABLE_BIT_NV: Self = Self(128);
    pub const HOST_ONLY_POOL_BIT_EXT: Self = Self(4);
    pub const PER_STAGE_BIT_NV: Self = Self(64);
}

impl core::ops::BitOr for VkDescriptorSetLayoutCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkDescriptorType(pub i32);

impl VkDescriptorType {
    pub const SAMPLER: Self = Self(0);
    pub const COMBINED_IMAGE_SAMPLER: Self = Self(1);
    pub const SAMPLED_IMAGE: Self = Self(2);
    pub const STORAGE_IMAGE: Self = Self(3);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(5);
    pub const UNIFORM_BUFFER: Self = Self(6);
    pub const STORAGE_BUFFER: Self = Self(7);
    pub const UNIFORM_BUFFER_DYNAMIC: Self = Self(8);
    pub const STORAGE_BUFFER_DYNAMIC: Self = Self(9);
    pub const INPUT_ATTACHMENT: Self = Self(10);
    pub const INLINE_UNIFORM_BLOCK: Self = Self(1000138000);
    pub const INLINE_UNIFORM_BLOCK_EXT: Self = Self(1000138000);
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
    pub const MUTABLE_VALVE: Self = Self(1000351000);
    pub const SAMPLE_WEIGHT_IMAGE_QCOM: Self = Self(1000440000);
    pub const BLOCK_MATCH_IMAGE_QCOM: Self = Self(1000440001);
    pub const TENSOR_ARM: Self = Self(1000460000);
    pub const MUTABLE_EXT: Self = Self(1000351000);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_NV: Self = Self(1000570000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkDescriptorUpdateTemplateType(pub i32);

impl VkDescriptorUpdateTemplateType {
    pub const DESCRIPTOR_SET: Self = Self(0);
    pub const PUSH_DESCRIPTORS: Self = Self(1);
    pub const PUSH_DESCRIPTORS_KHR: Self = Self(1);
    pub const DESCRIPTOR_SET_KHR: Self = Self(0);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkDeviceGroupPresentModeFlagBitsKHR(pub u32);

impl VkDeviceGroupPresentModeFlagBitsKHR {
    pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR: Self = Self(1);
    pub const VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR: Self = Self(2);
    pub const VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR: Self = Self(4);
    pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR: Self = Self(8);
}

impl core::ops::BitOr for VkDeviceGroupPresentModeFlagBitsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkDeviceQueueCreateFlagBits(pub u32);

impl VkDeviceQueueCreateFlagBits {
    pub const PROTECTED_BIT: Self = Self(1);
    pub const RESERVED_1_BIT_QCOM: Self = Self(2);
    pub const INTERNALLY_SYNCHRONIZED_BIT_KHR: Self = Self(4);
}

impl core::ops::BitOr for VkDeviceQueueCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkDriverId(pub i32);

impl VkDriverId {
    pub const AMD_PROPRIETARY: Self = Self(1);
    pub const AMD_OPEN_SOURCE: Self = Self(2);
    pub const MESA_RADV: Self = Self(3);
    pub const NVIDIA_PROPRIETARY: Self = Self(4);
    pub const INTEL_PROPRIETARY_WINDOWS: Self = Self(5);
    pub const INTEL_OPEN_SOURCE_MESA: Self = Self(6);
    pub const IMAGINATION_PROPRIETARY: Self = Self(7);
    pub const QUALCOMM_PROPRIETARY: Self = Self(8);
    pub const ARM_PROPRIETARY: Self = Self(9);
    pub const GOOGLE_SWIFTSHADER: Self = Self(10);
    pub const GGP_PROPRIETARY: Self = Self(11);
    pub const BROADCOM_PROPRIETARY: Self = Self(12);
    pub const MESA_LLVMPIPE: Self = Self(13);
    pub const MOLTENVK: Self = Self(14);
    pub const COREAVI_PROPRIETARY: Self = Self(15);
    pub const JUICE_PROPRIETARY: Self = Self(16);
    pub const VERISILICON_PROPRIETARY: Self = Self(17);
    pub const MESA_TURNIP: Self = Self(18);
    pub const MESA_V3DV: Self = Self(19);
    pub const MESA_PANVK: Self = Self(20);
    pub const SAMSUNG_PROPRIETARY: Self = Self(21);
    pub const MESA_VENUS: Self = Self(22);
    pub const MESA_DOZEN: Self = Self(23);
    pub const MESA_NVK: Self = Self(24);
    pub const IMAGINATION_OPEN_SOURCE_MESA: Self = Self(25);
    pub const MESA_HONEYKRISP: Self = Self(26);
    pub const VULKAN_SC_EMULATION_ON_VULKAN: Self = Self(27);
    pub const MESA_KOSMICKRISP: Self = Self(28);
    pub const MESA_GFXSTREAM: Self = Self(29);
    pub const APE_SOFT: Self = Self(30);
    pub const RESERVED_31: Self = Self(31);
    pub const AMD_PROPRIETARY_KHR: Self = Self(1);
    pub const AMD_OPEN_SOURCE_KHR: Self = Self(2);
    pub const MESA_RADV_KHR: Self = Self(3);
    pub const NVIDIA_PROPRIETARY_KHR: Self = Self(4);
    pub const INTEL_PROPRIETARY_WINDOWS_KHR: Self = Self(5);
    pub const INTEL_OPEN_SOURCE_MESA_KHR: Self = Self(6);
    pub const IMAGINATION_PROPRIETARY_KHR: Self = Self(7);
    pub const QUALCOMM_PROPRIETARY_KHR: Self = Self(8);
    pub const ARM_PROPRIETARY_KHR: Self = Self(9);
    pub const GOOGLE_SWIFTSHADER_KHR: Self = Self(10);
    pub const GGP_PROPRIETARY_KHR: Self = Self(11);
    pub const BROADCOM_PROPRIETARY_KHR: Self = Self(12);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkDynamicState(pub i32);

impl VkDynamicState {
    pub const VIEWPORT: Self = Self(0);
    pub const SCISSOR: Self = Self(1);
    pub const LINE_WIDTH: Self = Self(2);
    pub const DEPTH_BIAS: Self = Self(3);
    pub const BLEND_CONSTANTS: Self = Self(4);
    pub const DEPTH_BOUNDS: Self = Self(5);
    pub const STENCIL_COMPARE_MASK: Self = Self(6);
    pub const STENCIL_WRITE_MASK: Self = Self(7);
    pub const STENCIL_REFERENCE: Self = Self(8);
    pub const CULL_MODE: Self = Self(1000267000);
    pub const FRONT_FACE: Self = Self(1000267001);
    pub const PRIMITIVE_TOPOLOGY: Self = Self(1000267002);
    pub const VIEWPORT_WITH_COUNT: Self = Self(1000267003);
    pub const SCISSOR_WITH_COUNT: Self = Self(1000267004);
    pub const VERTEX_INPUT_BINDING_STRIDE: Self = Self(1000267005);
    pub const DEPTH_TEST_ENABLE: Self = Self(1000267006);
    pub const DEPTH_WRITE_ENABLE: Self = Self(1000267007);
    pub const DEPTH_COMPARE_OP: Self = Self(1000267008);
    pub const DEPTH_BOUNDS_TEST_ENABLE: Self = Self(1000267009);
    pub const STENCIL_TEST_ENABLE: Self = Self(1000267010);
    pub const STENCIL_OP: Self = Self(1000267011);
    pub const RASTERIZER_DISCARD_ENABLE: Self = Self(1000377001);
    pub const DEPTH_BIAS_ENABLE: Self = Self(1000377002);
    pub const PRIMITIVE_RESTART_ENABLE: Self = Self(1000377004);
    pub const LINE_STIPPLE: Self = Self(1000259000);
    pub const VIEWPORT_W_SCALING_NV: Self = Self(1000087000);
    pub const DISCARD_RECTANGLE_EXT: Self = Self(1000099000);
    pub const DISCARD_RECTANGLE_ENABLE_EXT: Self = Self(1000099001);
    pub const DISCARD_RECTANGLE_MODE_EXT: Self = Self(1000099002);
    pub const SAMPLE_LOCATIONS_EXT: Self = Self(1000143000);
    pub const RAY_TRACING_PIPELINE_STACK_SIZE_KHR: Self = Self(1000347000);
    pub const VIEWPORT_SHADING_RATE_PALETTE_NV: Self = Self(1000164004);
    pub const VIEWPORT_COARSE_SAMPLE_ORDER_NV: Self = Self(1000164006);
    pub const EXCLUSIVE_SCISSOR_ENABLE_NV: Self = Self(1000205000);
    pub const EXCLUSIVE_SCISSOR_NV: Self = Self(1000205001);
    pub const FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226000);
    pub const LINE_STIPPLE_EXT: Self = Self(1000259000);
    pub const CULL_MODE_EXT: Self = Self(1000267000);
    pub const FRONT_FACE_EXT: Self = Self(1000267001);
    pub const PRIMITIVE_TOPOLOGY_EXT: Self = Self(1000267002);
    pub const VIEWPORT_WITH_COUNT_EXT: Self = Self(1000267003);
    pub const SCISSOR_WITH_COUNT_EXT: Self = Self(1000267004);
    pub const VERTEX_INPUT_BINDING_STRIDE_EXT: Self = Self(1000267005);
    pub const DEPTH_TEST_ENABLE_EXT: Self = Self(1000267006);
    pub const DEPTH_WRITE_ENABLE_EXT: Self = Self(1000267007);
    pub const DEPTH_COMPARE_OP_EXT: Self = Self(1000267008);
    pub const DEPTH_BOUNDS_TEST_ENABLE_EXT: Self = Self(1000267009);
    pub const STENCIL_TEST_ENABLE_EXT: Self = Self(1000267010);
    pub const STENCIL_OP_EXT: Self = Self(1000267011);
    pub const VERTEX_INPUT_EXT: Self = Self(1000352000);
    pub const PATCH_CONTROL_POINTS_EXT: Self = Self(1000377000);
    pub const RASTERIZER_DISCARD_ENABLE_EXT: Self = Self(1000377001);
    pub const DEPTH_BIAS_ENABLE_EXT: Self = Self(1000377002);
    pub const LOGIC_OP_EXT: Self = Self(1000377003);
    pub const PRIMITIVE_RESTART_ENABLE_EXT: Self = Self(1000377004);
    pub const COLOR_WRITE_ENABLE_EXT: Self = Self(1000381000);
    pub const DEPTH_CLAMP_ENABLE_EXT: Self = Self(1000455003);
    pub const POLYGON_MODE_EXT: Self = Self(1000455004);
    pub const RASTERIZATION_SAMPLES_EXT: Self = Self(1000455005);
    pub const SAMPLE_MASK_EXT: Self = Self(1000455006);
    pub const ALPHA_TO_COVERAGE_ENABLE_EXT: Self = Self(1000455007);
    pub const ALPHA_TO_ONE_ENABLE_EXT: Self = Self(1000455008);
    pub const LOGIC_OP_ENABLE_EXT: Self = Self(1000455009);
    pub const COLOR_BLEND_ENABLE_EXT: Self = Self(1000455010);
    pub const COLOR_BLEND_EQUATION_EXT: Self = Self(1000455011);
    pub const COLOR_WRITE_MASK_EXT: Self = Self(1000455012);
    pub const TESSELLATION_DOMAIN_ORIGIN_EXT: Self = Self(1000455002);
    pub const RASTERIZATION_STREAM_EXT: Self = Self(1000455013);
    pub const CONSERVATIVE_RASTERIZATION_MODE_EXT: Self = Self(1000455014);
    pub const EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT: Self = Self(1000455015);
    pub const DEPTH_CLIP_ENABLE_EXT: Self = Self(1000455016);
    pub const SAMPLE_LOCATIONS_ENABLE_EXT: Self = Self(1000455017);
    pub const COLOR_BLEND_ADVANCED_EXT: Self = Self(1000455018);
    pub const PROVOKING_VERTEX_MODE_EXT: Self = Self(1000455019);
    pub const LINE_RASTERIZATION_MODE_EXT: Self = Self(1000455020);
    pub const LINE_STIPPLE_ENABLE_EXT: Self = Self(1000455021);
    pub const DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT: Self = Self(1000455022);
    pub const VIEWPORT_W_SCALING_ENABLE_NV: Self = Self(1000455023);
    pub const VIEWPORT_SWIZZLE_NV: Self = Self(1000455024);
    pub const COVERAGE_TO_COLOR_ENABLE_NV: Self = Self(1000455025);
    pub const COVERAGE_TO_COLOR_LOCATION_NV: Self = Self(1000455026);
    pub const COVERAGE_MODULATION_MODE_NV: Self = Self(1000455027);
    pub const COVERAGE_MODULATION_TABLE_ENABLE_NV: Self = Self(1000455028);
    pub const COVERAGE_MODULATION_TABLE_NV: Self = Self(1000455029);
    pub const SHADING_RATE_IMAGE_ENABLE_NV: Self = Self(1000455030);
    pub const REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV: Self = Self(1000455031);
    pub const COVERAGE_REDUCTION_MODE_NV: Self = Self(1000455032);
    pub const ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT: Self = Self(1000524000);
    pub const LINE_STIPPLE_KHR: Self = Self(1000259000);
    pub const DEPTH_CLAMP_RANGE_EXT: Self = Self(1000582000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkEventCreateFlagBits(pub u32);

impl VkEventCreateFlagBits {
    pub const DEVICE_ONLY_BIT: Self = Self(1);
    pub const DEVICE_ONLY_BIT_KHR: Self = Self(1);
}

impl core::ops::BitOr for VkEventCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkExternalFenceFeatureFlagBits(pub u32);

impl VkExternalFenceFeatureFlagBits {
    pub const EXPORTABLE_BIT: Self = Self(1);
    pub const IMPORTABLE_BIT: Self = Self(2);
    pub const EXPORTABLE_BIT_KHR: Self = Self(1);
    pub const IMPORTABLE_BIT_KHR: Self = Self(2);
}

impl core::ops::BitOr for VkExternalFenceFeatureFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkExternalFenceHandleTypeFlagBits(pub u32);

impl VkExternalFenceHandleTypeFlagBits {
    pub const OPAQUE_FD_BIT: Self = Self(1);
    pub const OPAQUE_WIN32_BIT: Self = Self(2);
    pub const OPAQUE_WIN32_KMT_BIT: Self = Self(4);
    pub const SYNC_FD_BIT: Self = Self(8);
    pub const OPAQUE_FD_BIT_KHR: Self = Self(1);
    pub const OPAQUE_WIN32_BIT_KHR: Self = Self(2);
    pub const OPAQUE_WIN32_KMT_BIT_KHR: Self = Self(4);
    pub const SYNC_FD_BIT_KHR: Self = Self(8);
    pub const SCI_SYNC_OBJ_BIT_NV: Self = Self(16);
    pub const SCI_SYNC_FENCE_BIT_NV: Self = Self(32);
}

impl core::ops::BitOr for VkExternalFenceHandleTypeFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkExternalMemoryFeatureFlagBits(pub u32);

impl VkExternalMemoryFeatureFlagBits {
    pub const DEDICATED_ONLY_BIT: Self = Self(1);
    pub const EXPORTABLE_BIT: Self = Self(2);
    pub const IMPORTABLE_BIT: Self = Self(4);
    pub const DEDICATED_ONLY_BIT_KHR: Self = Self(1);
    pub const EXPORTABLE_BIT_KHR: Self = Self(2);
    pub const IMPORTABLE_BIT_KHR: Self = Self(4);
}

impl core::ops::BitOr for VkExternalMemoryFeatureFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkExternalMemoryHandleTypeFlagBits(pub u32);

impl VkExternalMemoryHandleTypeFlagBits {
    pub const OPAQUE_FD_BIT: Self = Self(1);
    pub const OPAQUE_WIN32_BIT: Self = Self(2);
    pub const OPAQUE_WIN32_KMT_BIT: Self = Self(4);
    pub const D3D11_TEXTURE_BIT: Self = Self(8);
    pub const D3D11_TEXTURE_KMT_BIT: Self = Self(16);
    pub const D3D12_HEAP_BIT: Self = Self(32);
    pub const D3D12_RESOURCE_BIT: Self = Self(64);
    pub const OPAQUE_FD_BIT_KHR: Self = Self(1);
    pub const OPAQUE_WIN32_BIT_KHR: Self = Self(2);
    pub const OPAQUE_WIN32_KMT_BIT_KHR: Self = Self(4);
    pub const D3D11_TEXTURE_BIT_KHR: Self = Self(8);
    pub const D3D11_TEXTURE_KMT_BIT_KHR: Self = Self(16);
    pub const D3D12_HEAP_BIT_KHR: Self = Self(32);
    pub const D3D12_RESOURCE_BIT_KHR: Self = Self(64);
    pub const DMA_BUF_BIT_EXT: Self = Self(512);
    pub const ANDROID_HARDWARE_BUFFER_BIT_ANDROID: Self = Self(1024);
    pub const HOST_ALLOCATION_BIT_EXT: Self = Self(128);
    pub const HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT: Self = Self(256);
    pub const ZIRCON_VMO_BIT_FUCHSIA: Self = Self(2048);
    pub const RDMA_ADDRESS_BIT_NV: Self = Self(4096);
    pub const SCI_BUF_BIT_NV: Self = Self(8192);
    pub const OH_NATIVE_BUFFER_BIT_OHOS: Self = Self(32768);
    pub const SCREEN_BUFFER_BIT_QNX: Self = Self(16384);
    pub const MTLBUFFER_BIT_EXT: Self = Self(65536);
    pub const MTLTEXTURE_BIT_EXT: Self = Self(131072);
    pub const MTLHEAP_BIT_EXT: Self = Self(262144);
}

impl core::ops::BitOr for VkExternalMemoryHandleTypeFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkExternalSemaphoreFeatureFlagBits(pub u32);

impl VkExternalSemaphoreFeatureFlagBits {
    pub const EXPORTABLE_BIT: Self = Self(1);
    pub const IMPORTABLE_BIT: Self = Self(2);
    pub const EXPORTABLE_BIT_KHR: Self = Self(1);
    pub const IMPORTABLE_BIT_KHR: Self = Self(2);
}

impl core::ops::BitOr for VkExternalSemaphoreFeatureFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkExternalSemaphoreHandleTypeFlagBits(pub u32);

impl VkExternalSemaphoreHandleTypeFlagBits {
    pub const OPAQUE_FD_BIT: Self = Self(1);
    pub const OPAQUE_WIN32_BIT: Self = Self(2);
    pub const OPAQUE_WIN32_KMT_BIT: Self = Self(4);
    pub const D3D12_FENCE_BIT: Self = Self(8);
    pub const D3D11_FENCE_BIT: Self = Self(8);
    pub const SYNC_FD_BIT: Self = Self(16);
    pub const OPAQUE_FD_BIT_KHR: Self = Self(1);
    pub const OPAQUE_WIN32_BIT_KHR: Self = Self(2);
    pub const OPAQUE_WIN32_KMT_BIT_KHR: Self = Self(4);
    pub const D3D12_FENCE_BIT_KHR: Self = Self(8);
    pub const SYNC_FD_BIT_KHR: Self = Self(16);
    pub const ZIRCON_EVENT_BIT_FUCHSIA: Self = Self(128);
    pub const SCI_SYNC_OBJ_BIT_NV: Self = Self(32);
}

impl core::ops::BitOr for VkExternalSemaphoreHandleTypeFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkFenceCreateFlagBits(pub u32);

impl VkFenceCreateFlagBits {
    pub const SIGNALED_BIT: Self = Self(1);
}

impl core::ops::BitOr for VkFenceCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkFenceImportFlagBits(pub u32);

impl VkFenceImportFlagBits {
    pub const TEMPORARY_BIT: Self = Self(1);
    pub const TEMPORARY_BIT_KHR: Self = Self(1);
}

impl core::ops::BitOr for VkFenceImportFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkFilter(pub i32);

impl VkFilter {
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
    pub const CUBIC_IMG: Self = Self(1000015000);
    pub const CUBIC_EXT: Self = Self(1000015000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkFormat(pub i32);

impl VkFormat {
    pub const UNDEFINED: Self = Self(0);
    pub const R4G4_UNORM_PACK8: Self = Self(1);
    pub const R4G4B4A4_UNORM_PACK16: Self = Self(2);
    pub const B4G4R4A4_UNORM_PACK16: Self = Self(3);
    pub const R5G6B5_UNORM_PACK16: Self = Self(4);
    pub const B5G6R5_UNORM_PACK16: Self = Self(5);
    pub const R5G5B5A1_UNORM_PACK16: Self = Self(6);
    pub const B5G5R5A1_UNORM_PACK16: Self = Self(7);
    pub const A1R5G5B5_UNORM_PACK16: Self = Self(8);
    pub const R8_UNORM: Self = Self(9);
    pub const R8_SNORM: Self = Self(10);
    pub const R8_USCALED: Self = Self(11);
    pub const R8_SSCALED: Self = Self(12);
    pub const R8_UINT: Self = Self(13);
    pub const R8_SINT: Self = Self(14);
    pub const R8_SRGB: Self = Self(15);
    pub const R8G8_UNORM: Self = Self(16);
    pub const R8G8_SNORM: Self = Self(17);
    pub const R8G8_USCALED: Self = Self(18);
    pub const R8G8_SSCALED: Self = Self(19);
    pub const R8G8_UINT: Self = Self(20);
    pub const R8G8_SINT: Self = Self(21);
    pub const R8G8_SRGB: Self = Self(22);
    pub const R8G8B8_UNORM: Self = Self(23);
    pub const R8G8B8_SNORM: Self = Self(24);
    pub const R8G8B8_USCALED: Self = Self(25);
    pub const R8G8B8_SSCALED: Self = Self(26);
    pub const R8G8B8_UINT: Self = Self(27);
    pub const R8G8B8_SINT: Self = Self(28);
    pub const R8G8B8_SRGB: Self = Self(29);
    pub const B8G8R8_UNORM: Self = Self(30);
    pub const B8G8R8_SNORM: Self = Self(31);
    pub const B8G8R8_USCALED: Self = Self(32);
    pub const B8G8R8_SSCALED: Self = Self(33);
    pub const B8G8R8_UINT: Self = Self(34);
    pub const B8G8R8_SINT: Self = Self(35);
    pub const B8G8R8_SRGB: Self = Self(36);
    pub const R8G8B8A8_UNORM: Self = Self(37);
    pub const R8G8B8A8_SNORM: Self = Self(38);
    pub const R8G8B8A8_USCALED: Self = Self(39);
    pub const R8G8B8A8_SSCALED: Self = Self(40);
    pub const R8G8B8A8_UINT: Self = Self(41);
    pub const R8G8B8A8_SINT: Self = Self(42);
    pub const R8G8B8A8_SRGB: Self = Self(43);
    pub const B8G8R8A8_UNORM: Self = Self(44);
    pub const B8G8R8A8_SNORM: Self = Self(45);
    pub const B8G8R8A8_USCALED: Self = Self(46);
    pub const B8G8R8A8_SSCALED: Self = Self(47);
    pub const B8G8R8A8_UINT: Self = Self(48);
    pub const B8G8R8A8_SINT: Self = Self(49);
    pub const B8G8R8A8_SRGB: Self = Self(50);
    pub const A8B8G8R8_UNORM_PACK32: Self = Self(51);
    pub const A8B8G8R8_SNORM_PACK32: Self = Self(52);
    pub const A8B8G8R8_USCALED_PACK32: Self = Self(53);
    pub const A8B8G8R8_SSCALED_PACK32: Self = Self(54);
    pub const A8B8G8R8_UINT_PACK32: Self = Self(55);
    pub const A8B8G8R8_SINT_PACK32: Self = Self(56);
    pub const A8B8G8R8_SRGB_PACK32: Self = Self(57);
    pub const A2R10G10B10_UNORM_PACK32: Self = Self(58);
    pub const A2R10G10B10_SNORM_PACK32: Self = Self(59);
    pub const A2R10G10B10_USCALED_PACK32: Self = Self(60);
    pub const A2R10G10B10_SSCALED_PACK32: Self = Self(61);
    pub const A2R10G10B10_UINT_PACK32: Self = Self(62);
    pub const A2R10G10B10_SINT_PACK32: Self = Self(63);
    pub const A2B10G10R10_UNORM_PACK32: Self = Self(64);
    pub const A2B10G10R10_SNORM_PACK32: Self = Self(65);
    pub const A2B10G10R10_USCALED_PACK32: Self = Self(66);
    pub const A2B10G10R10_SSCALED_PACK32: Self = Self(67);
    pub const A2B10G10R10_UINT_PACK32: Self = Self(68);
    pub const A2B10G10R10_SINT_PACK32: Self = Self(69);
    pub const R16_UNORM: Self = Self(70);
    pub const R16_SNORM: Self = Self(71);
    pub const R16_USCALED: Self = Self(72);
    pub const R16_SSCALED: Self = Self(73);
    pub const R16_UINT: Self = Self(74);
    pub const R16_SINT: Self = Self(75);
    pub const R16_SFLOAT: Self = Self(76);
    pub const R16G16_UNORM: Self = Self(77);
    pub const R16G16_SNORM: Self = Self(78);
    pub const R16G16_USCALED: Self = Self(79);
    pub const R16G16_SSCALED: Self = Self(80);
    pub const R16G16_UINT: Self = Self(81);
    pub const R16G16_SINT: Self = Self(82);
    pub const R16G16_SFLOAT: Self = Self(83);
    pub const R16G16B16_UNORM: Self = Self(84);
    pub const R16G16B16_SNORM: Self = Self(85);
    pub const R16G16B16_USCALED: Self = Self(86);
    pub const R16G16B16_SSCALED: Self = Self(87);
    pub const R16G16B16_UINT: Self = Self(88);
    pub const R16G16B16_SINT: Self = Self(89);
    pub const R16G16B16_SFLOAT: Self = Self(90);
    pub const R16G16B16A16_UNORM: Self = Self(91);
    pub const R16G16B16A16_SNORM: Self = Self(92);
    pub const R16G16B16A16_USCALED: Self = Self(93);
    pub const R16G16B16A16_SSCALED: Self = Self(94);
    pub const R16G16B16A16_UINT: Self = Self(95);
    pub const R16G16B16A16_SINT: Self = Self(96);
    pub const R16G16B16A16_SFLOAT: Self = Self(97);
    pub const R32_UINT: Self = Self(98);
    pub const R32_SINT: Self = Self(99);
    pub const R32_SFLOAT: Self = Self(100);
    pub const R32G32_UINT: Self = Self(101);
    pub const R32G32_SINT: Self = Self(102);
    pub const R32G32_SFLOAT: Self = Self(103);
    pub const R32G32B32_UINT: Self = Self(104);
    pub const R32G32B32_SINT: Self = Self(105);
    pub const R32G32B32_SFLOAT: Self = Self(106);
    pub const R32G32B32A32_UINT: Self = Self(107);
    pub const R32G32B32A32_SINT: Self = Self(108);
    pub const R32G32B32A32_SFLOAT: Self = Self(109);
    pub const R64_UINT: Self = Self(110);
    pub const R64_SINT: Self = Self(111);
    pub const R64_SFLOAT: Self = Self(112);
    pub const R64G64_UINT: Self = Self(113);
    pub const R64G64_SINT: Self = Self(114);
    pub const R64G64_SFLOAT: Self = Self(115);
    pub const R64G64B64_UINT: Self = Self(116);
    pub const R64G64B64_SINT: Self = Self(117);
    pub const R64G64B64_SFLOAT: Self = Self(118);
    pub const R64G64B64A64_UINT: Self = Self(119);
    pub const R64G64B64A64_SINT: Self = Self(120);
    pub const R64G64B64A64_SFLOAT: Self = Self(121);
    pub const B10G11R11_UFLOAT_PACK32: Self = Self(122);
    pub const E5B9G9R9_UFLOAT_PACK32: Self = Self(123);
    pub const D16_UNORM: Self = Self(124);
    pub const X8_D24_UNORM_PACK32: Self = Self(125);
    pub const D32_SFLOAT: Self = Self(126);
    pub const S8_UINT: Self = Self(127);
    pub const D16_UNORM_S8_UINT: Self = Self(128);
    pub const D24_UNORM_S8_UINT: Self = Self(129);
    pub const D32_SFLOAT_S8_UINT: Self = Self(130);
    pub const BC1_RGB_UNORM_BLOCK: Self = Self(131);
    pub const BC1_RGB_SRGB_BLOCK: Self = Self(132);
    pub const BC1_RGBA_UNORM_BLOCK: Self = Self(133);
    pub const BC1_RGBA_SRGB_BLOCK: Self = Self(134);
    pub const BC2_UNORM_BLOCK: Self = Self(135);
    pub const BC2_SRGB_BLOCK: Self = Self(136);
    pub const BC3_UNORM_BLOCK: Self = Self(137);
    pub const BC3_SRGB_BLOCK: Self = Self(138);
    pub const BC4_UNORM_BLOCK: Self = Self(139);
    pub const BC4_SNORM_BLOCK: Self = Self(140);
    pub const BC5_UNORM_BLOCK: Self = Self(141);
    pub const BC5_SNORM_BLOCK: Self = Self(142);
    pub const BC6H_UFLOAT_BLOCK: Self = Self(143);
    pub const BC6H_SFLOAT_BLOCK: Self = Self(144);
    pub const BC7_UNORM_BLOCK: Self = Self(145);
    pub const BC7_SRGB_BLOCK: Self = Self(146);
    pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Self(147);
    pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Self(148);
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self(149);
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self(150);
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self(151);
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self(152);
    pub const EAC_R11_UNORM_BLOCK: Self = Self(153);
    pub const EAC_R11_SNORM_BLOCK: Self = Self(154);
    pub const EAC_R11G11_UNORM_BLOCK: Self = Self(155);
    pub const EAC_R11G11_SNORM_BLOCK: Self = Self(156);
    pub const ASTC_4x4_UNORM_BLOCK: Self = Self(157);
    pub const ASTC_4x4_SRGB_BLOCK: Self = Self(158);
    pub const ASTC_5x4_UNORM_BLOCK: Self = Self(159);
    pub const ASTC_5x4_SRGB_BLOCK: Self = Self(160);
    pub const ASTC_5x5_UNORM_BLOCK: Self = Self(161);
    pub const ASTC_5x5_SRGB_BLOCK: Self = Self(162);
    pub const ASTC_6x5_UNORM_BLOCK: Self = Self(163);
    pub const ASTC_6x5_SRGB_BLOCK: Self = Self(164);
    pub const ASTC_6x6_UNORM_BLOCK: Self = Self(165);
    pub const ASTC_6x6_SRGB_BLOCK: Self = Self(166);
    pub const ASTC_8x5_UNORM_BLOCK: Self = Self(167);
    pub const ASTC_8x5_SRGB_BLOCK: Self = Self(168);
    pub const ASTC_8x6_UNORM_BLOCK: Self = Self(169);
    pub const ASTC_8x6_SRGB_BLOCK: Self = Self(170);
    pub const ASTC_8x8_UNORM_BLOCK: Self = Self(171);
    pub const ASTC_8x8_SRGB_BLOCK: Self = Self(172);
    pub const ASTC_10x5_UNORM_BLOCK: Self = Self(173);
    pub const ASTC_10x5_SRGB_BLOCK: Self = Self(174);
    pub const ASTC_10x6_UNORM_BLOCK: Self = Self(175);
    pub const ASTC_10x6_SRGB_BLOCK: Self = Self(176);
    pub const ASTC_10x8_UNORM_BLOCK: Self = Self(177);
    pub const ASTC_10x8_SRGB_BLOCK: Self = Self(178);
    pub const ASTC_10x10_UNORM_BLOCK: Self = Self(179);
    pub const ASTC_10x10_SRGB_BLOCK: Self = Self(180);
    pub const ASTC_12x10_UNORM_BLOCK: Self = Self(181);
    pub const ASTC_12x10_SRGB_BLOCK: Self = Self(182);
    pub const ASTC_12x12_UNORM_BLOCK: Self = Self(183);
    pub const ASTC_12x12_SRGB_BLOCK: Self = Self(184);
    pub const G8B8G8R8_422_UNORM: Self = Self(1000156000);
    pub const B8G8R8G8_422_UNORM: Self = Self(1000156001);
    pub const G8_B8_R8_3PLANE_420_UNORM: Self = Self(1000156002);
    pub const G8_B8R8_2PLANE_420_UNORM: Self = Self(1000156003);
    pub const G8_B8_R8_3PLANE_422_UNORM: Self = Self(1000156004);
    pub const G8_B8R8_2PLANE_422_UNORM: Self = Self(1000156005);
    pub const G8_B8_R8_3PLANE_444_UNORM: Self = Self(1000156006);
    pub const R10X6_UNORM_PACK16: Self = Self(1000156007);
    pub const R10X6G10X6_UNORM_2PACK16: Self = Self(1000156008);
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16: Self = Self(1000156009);
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: Self = Self(1000156010);
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: Self = Self(1000156011);
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: Self = Self(1000156012);
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: Self = Self(1000156013);
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: Self = Self(1000156014);
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: Self = Self(1000156015);
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: Self = Self(1000156016);
    pub const R12X4_UNORM_PACK16: Self = Self(1000156017);
    pub const R12X4G12X4_UNORM_2PACK16: Self = Self(1000156018);
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16: Self = Self(1000156019);
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: Self = Self(1000156020);
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: Self = Self(1000156021);
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: Self = Self(1000156022);
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: Self = Self(1000156023);
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: Self = Self(1000156024);
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: Self = Self(1000156025);
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: Self = Self(1000156026);
    pub const G16B16G16R16_422_UNORM: Self = Self(1000156027);
    pub const B16G16R16G16_422_UNORM: Self = Self(1000156028);
    pub const G16_B16_R16_3PLANE_420_UNORM: Self = Self(1000156029);
    pub const G16_B16R16_2PLANE_420_UNORM: Self = Self(1000156030);
    pub const G16_B16_R16_3PLANE_422_UNORM: Self = Self(1000156031);
    pub const G16_B16R16_2PLANE_422_UNORM: Self = Self(1000156032);
    pub const G16_B16_R16_3PLANE_444_UNORM: Self = Self(1000156033);
    pub const G8_B8R8_2PLANE_444_UNORM: Self = Self(1000330000);
    pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16: Self = Self(1000330001);
    pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16: Self = Self(1000330002);
    pub const G16_B16R16_2PLANE_444_UNORM: Self = Self(1000330003);
    pub const A4R4G4B4_UNORM_PACK16: Self = Self(1000340000);
    pub const A4B4G4R4_UNORM_PACK16: Self = Self(1000340001);
    pub const ASTC_4x4_SFLOAT_BLOCK: Self = Self(1000066000);
    pub const ASTC_5x4_SFLOAT_BLOCK: Self = Self(1000066001);
    pub const ASTC_5x5_SFLOAT_BLOCK: Self = Self(1000066002);
    pub const ASTC_6x5_SFLOAT_BLOCK: Self = Self(1000066003);
    pub const ASTC_6x6_SFLOAT_BLOCK: Self = Self(1000066004);
    pub const ASTC_8x5_SFLOAT_BLOCK: Self = Self(1000066005);
    pub const ASTC_8x6_SFLOAT_BLOCK: Self = Self(1000066006);
    pub const ASTC_8x8_SFLOAT_BLOCK: Self = Self(1000066007);
    pub const ASTC_10x5_SFLOAT_BLOCK: Self = Self(1000066008);
    pub const ASTC_10x6_SFLOAT_BLOCK: Self = Self(1000066009);
    pub const ASTC_10x8_SFLOAT_BLOCK: Self = Self(1000066010);
    pub const ASTC_10x10_SFLOAT_BLOCK: Self = Self(1000066011);
    pub const ASTC_12x10_SFLOAT_BLOCK: Self = Self(1000066012);
    pub const ASTC_12x12_SFLOAT_BLOCK: Self = Self(1000066013);
    pub const A1B5G5R5_UNORM_PACK16: Self = Self(1000470000);
    pub const A8_UNORM: Self = Self(1000470001);
    pub const PVRTC1_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054000);
    pub const PVRTC1_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054001);
    pub const PVRTC2_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054002);
    pub const PVRTC2_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054003);
    pub const PVRTC1_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054004);
    pub const PVRTC1_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054005);
    pub const PVRTC2_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054006);
    pub const PVRTC2_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054007);
    pub const ASTC_4x4_SFLOAT_BLOCK_EXT: Self = Self(1000066000);
    pub const ASTC_5x4_SFLOAT_BLOCK_EXT: Self = Self(1000066001);
    pub const ASTC_5x5_SFLOAT_BLOCK_EXT: Self = Self(1000066002);
    pub const ASTC_6x5_SFLOAT_BLOCK_EXT: Self = Self(1000066003);
    pub const ASTC_6x6_SFLOAT_BLOCK_EXT: Self = Self(1000066004);
    pub const ASTC_8x5_SFLOAT_BLOCK_EXT: Self = Self(1000066005);
    pub const ASTC_8x6_SFLOAT_BLOCK_EXT: Self = Self(1000066006);
    pub const ASTC_8x8_SFLOAT_BLOCK_EXT: Self = Self(1000066007);
    pub const ASTC_10x5_SFLOAT_BLOCK_EXT: Self = Self(1000066008);
    pub const ASTC_10x6_SFLOAT_BLOCK_EXT: Self = Self(1000066009);
    pub const ASTC_10x8_SFLOAT_BLOCK_EXT: Self = Self(1000066010);
    pub const ASTC_10x10_SFLOAT_BLOCK_EXT: Self = Self(1000066011);
    pub const ASTC_12x10_SFLOAT_BLOCK_EXT: Self = Self(1000066012);
    pub const ASTC_12x12_SFLOAT_BLOCK_EXT: Self = Self(1000066013);
    pub const G8B8G8R8_422_UNORM_KHR: Self = Self(1000156000);
    pub const B8G8R8G8_422_UNORM_KHR: Self = Self(1000156001);
    pub const G8_B8_R8_3PLANE_420_UNORM_KHR: Self = Self(1000156002);
    pub const G8_B8R8_2PLANE_420_UNORM_KHR: Self = Self(1000156003);
    pub const G8_B8_R8_3PLANE_422_UNORM_KHR: Self = Self(1000156004);
    pub const G8_B8R8_2PLANE_422_UNORM_KHR: Self = Self(1000156005);
    pub const G8_B8_R8_3PLANE_444_UNORM_KHR: Self = Self(1000156006);
    pub const R10X6_UNORM_PACK16_KHR: Self = Self(1000156007);
    pub const R10X6G10X6_UNORM_2PACK16_KHR: Self = Self(1000156008);
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: Self = Self(1000156009);
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: Self = Self(1000156010);
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: Self = Self(1000156011);
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: Self = Self(1000156012);
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: Self = Self(1000156013);
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: Self = Self(1000156014);
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: Self = Self(1000156015);
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: Self = Self(1000156016);
    pub const R12X4_UNORM_PACK16_KHR: Self = Self(1000156017);
    pub const R12X4G12X4_UNORM_2PACK16_KHR: Self = Self(1000156018);
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: Self = Self(1000156019);
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: Self = Self(1000156020);
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: Self = Self(1000156021);
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: Self = Self(1000156022);
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: Self = Self(1000156023);
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: Self = Self(1000156024);
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: Self = Self(1000156025);
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: Self = Self(1000156026);
    pub const G16B16G16R16_422_UNORM_KHR: Self = Self(1000156027);
    pub const B16G16R16G16_422_UNORM_KHR: Self = Self(1000156028);
    pub const G16_B16_R16_3PLANE_420_UNORM_KHR: Self = Self(1000156029);
    pub const G16_B16R16_2PLANE_420_UNORM_KHR: Self = Self(1000156030);
    pub const G16_B16_R16_3PLANE_422_UNORM_KHR: Self = Self(1000156031);
    pub const G16_B16R16_2PLANE_422_UNORM_KHR: Self = Self(1000156032);
    pub const G16_B16_R16_3PLANE_444_UNORM_KHR: Self = Self(1000156033);
    pub const ASTC_3x3x3_UNORM_BLOCK_EXT: Self = Self(1000288000);
    pub const ASTC_3x3x3_SRGB_BLOCK_EXT: Self = Self(1000288001);
    pub const ASTC_3x3x3_SFLOAT_BLOCK_EXT: Self = Self(1000288002);
    pub const ASTC_4x3x3_UNORM_BLOCK_EXT: Self = Self(1000288003);
    pub const ASTC_4x3x3_SRGB_BLOCK_EXT: Self = Self(1000288004);
    pub const ASTC_4x3x3_SFLOAT_BLOCK_EXT: Self = Self(1000288005);
    pub const ASTC_4x4x3_UNORM_BLOCK_EXT: Self = Self(1000288006);
    pub const ASTC_4x4x3_SRGB_BLOCK_EXT: Self = Self(1000288007);
    pub const ASTC_4x4x3_SFLOAT_BLOCK_EXT: Self = Self(1000288008);
    pub const ASTC_4x4x4_UNORM_BLOCK_EXT: Self = Self(1000288009);
    pub const ASTC_4x4x4_SRGB_BLOCK_EXT: Self = Self(1000288010);
    pub const ASTC_4x4x4_SFLOAT_BLOCK_EXT: Self = Self(1000288011);
    pub const ASTC_5x4x4_UNORM_BLOCK_EXT: Self = Self(1000288012);
    pub const ASTC_5x4x4_SRGB_BLOCK_EXT: Self = Self(1000288013);
    pub const ASTC_5x4x4_SFLOAT_BLOCK_EXT: Self = Self(1000288014);
    pub const ASTC_5x5x4_UNORM_BLOCK_EXT: Self = Self(1000288015);
    pub const ASTC_5x5x4_SRGB_BLOCK_EXT: Self = Self(1000288016);
    pub const ASTC_5x5x4_SFLOAT_BLOCK_EXT: Self = Self(1000288017);
    pub const ASTC_5x5x5_UNORM_BLOCK_EXT: Self = Self(1000288018);
    pub const ASTC_5x5x5_SRGB_BLOCK_EXT: Self = Self(1000288019);
    pub const ASTC_5x5x5_SFLOAT_BLOCK_EXT: Self = Self(1000288020);
    pub const ASTC_6x5x5_UNORM_BLOCK_EXT: Self = Self(1000288021);
    pub const ASTC_6x5x5_SRGB_BLOCK_EXT: Self = Self(1000288022);
    pub const ASTC_6x5x5_SFLOAT_BLOCK_EXT: Self = Self(1000288023);
    pub const ASTC_6x6x5_UNORM_BLOCK_EXT: Self = Self(1000288024);
    pub const ASTC_6x6x5_SRGB_BLOCK_EXT: Self = Self(1000288025);
    pub const ASTC_6x6x5_SFLOAT_BLOCK_EXT: Self = Self(1000288026);
    pub const ASTC_6x6x6_UNORM_BLOCK_EXT: Self = Self(1000288027);
    pub const ASTC_6x6x6_SRGB_BLOCK_EXT: Self = Self(1000288028);
    pub const ASTC_6x6x6_SFLOAT_BLOCK_EXT: Self = Self(1000288029);
    pub const G8_B8R8_2PLANE_444_UNORM_EXT: Self = Self(1000330000);
    pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT: Self = Self(1000330001);
    pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT: Self = Self(1000330002);
    pub const G16_B16R16_2PLANE_444_UNORM_EXT: Self = Self(1000330003);
    pub const A4R4G4B4_UNORM_PACK16_EXT: Self = Self(1000340000);
    pub const A4B4G4R4_UNORM_PACK16_EXT: Self = Self(1000340001);
    pub const R8_BOOL_ARM: Self = Self(1000460000);
    pub const R16_SFLOAT_FPENCODING_BFLOAT16_ARM: Self = Self(1000460001);
    pub const R8_SFLOAT_FPENCODING_FLOAT8E4M3_ARM: Self = Self(1000460002);
    pub const R8_SFLOAT_FPENCODING_FLOAT8E5M2_ARM: Self = Self(1000460003);
    pub const R16G16_SFIXED5_NV: Self = Self(1000464000);
    pub const R16G16_S10_5_NV: Self = Self(1000464000);
    pub const A1B5G5R5_UNORM_PACK16_KHR: Self = Self(1000470000);
    pub const A8_UNORM_KHR: Self = Self(1000470001);
    pub const R10X6_UINT_PACK16_ARM: Self = Self(1000609000);
    pub const R10X6G10X6_UINT_2PACK16_ARM: Self = Self(1000609001);
    pub const R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM: Self = Self(1000609002);
    pub const R12X4_UINT_PACK16_ARM: Self = Self(1000609003);
    pub const R12X4G12X4_UINT_2PACK16_ARM: Self = Self(1000609004);
    pub const R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM: Self = Self(1000609005);
    pub const R14X2_UINT_PACK16_ARM: Self = Self(1000609006);
    pub const R14X2G14X2_UINT_2PACK16_ARM: Self = Self(1000609007);
    pub const R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM: Self = Self(1000609008);
    pub const R14X2_UNORM_PACK16_ARM: Self = Self(1000609009);
    pub const R14X2G14X2_UNORM_2PACK16_ARM: Self = Self(1000609010);
    pub const R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM: Self = Self(1000609011);
    pub const G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM: Self = Self(1000609012);
    pub const G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM: Self = Self(1000609013);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkFormatFeatureFlagBits(pub u32);

impl VkFormatFeatureFlagBits {
    pub const SAMPLED_IMAGE_BIT: Self = Self(1);
    pub const STORAGE_IMAGE_BIT: Self = Self(2);
    pub const STORAGE_IMAGE_ATOMIC_BIT: Self = Self(4);
    pub const UNIFORM_TEXEL_BUFFER_BIT: Self = Self(8);
    pub const STORAGE_TEXEL_BUFFER_BIT: Self = Self(16);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_BIT: Self = Self(32);
    pub const VERTEX_BUFFER_BIT: Self = Self(64);
    pub const COLOR_ATTACHMENT_BIT: Self = Self(128);
    pub const COLOR_ATTACHMENT_BLEND_BIT: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: Self = Self(512);
    pub const BLIT_SRC_BIT: Self = Self(1024);
    pub const BLIT_DST_BIT: Self = Self(2048);
    pub const SAMPLED_IMAGE_FILTER_LINEAR_BIT: Self = Self(4096);
    pub const TRANSFER_SRC_BIT: Self = Self(16384);
    pub const TRANSFER_DST_BIT: Self = Self(32768);
    pub const MIDPOINT_CHROMA_SAMPLES_BIT: Self = Self(131072);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: Self = Self(262144);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: Self = Self(524288);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: Self = Self(1048576);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: Self = Self(2097152);
    pub const DISJOINT_BIT: Self = Self(4194304);
    pub const COSITED_CHROMA_SAMPLES_BIT: Self = Self(8388608);
    pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT: Self = Self(65536);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG: Self = Self(8192);
    pub const VIDEO_DECODE_OUTPUT_BIT_KHR: Self = Self(33554432);
    pub const VIDEO_DECODE_DPB_BIT_KHR: Self = Self(67108864);
    pub const TRANSFER_SRC_BIT_KHR: Self = Self(16384);
    pub const TRANSFER_DST_BIT_KHR: Self = Self(32768);
    pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT: Self = Self(65536);
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR: Self = Self(536870912);
    pub const MIDPOINT_CHROMA_SAMPLES_BIT_KHR: Self = Self(131072);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: Self = Self(262144);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: Self = Self(524288);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: Self = Self(1048576);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: Self = Self(2097152);
    pub const DISJOINT_BIT_KHR: Self = Self(4194304);
    pub const COSITED_CHROMA_SAMPLES_BIT_KHR: Self = Self(8388608);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: Self = Self(8192);
    pub const FRAGMENT_DENSITY_MAP_BIT_EXT: Self = Self(16777216);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(1073741824);
    pub const VIDEO_ENCODE_INPUT_BIT_KHR: Self = Self(134217728);
    pub const VIDEO_ENCODE_DPB_BIT_KHR: Self = Self(268435456);
}

impl core::ops::BitOr for VkFormatFeatureFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkFormatFeatureFlagBits2(pub u64);

impl VkFormatFeatureFlagBits2 {
    pub const SAMPLED_IMAGE_BIT: Self = Self(1);
    pub const STORAGE_IMAGE_BIT: Self = Self(2);
    pub const STORAGE_IMAGE_ATOMIC_BIT: Self = Self(4);
    pub const UNIFORM_TEXEL_BUFFER_BIT: Self = Self(8);
    pub const STORAGE_TEXEL_BUFFER_BIT: Self = Self(16);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_BIT: Self = Self(32);
    pub const VERTEX_BUFFER_BIT: Self = Self(64);
    pub const COLOR_ATTACHMENT_BIT: Self = Self(128);
    pub const COLOR_ATTACHMENT_BLEND_BIT: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: Self = Self(512);
    pub const BLIT_SRC_BIT: Self = Self(1024);
    pub const BLIT_DST_BIT: Self = Self(2048);
    pub const SAMPLED_IMAGE_FILTER_LINEAR_BIT: Self = Self(4096);
    pub const TRANSFER_SRC_BIT: Self = Self(16384);
    pub const TRANSFER_DST_BIT: Self = Self(32768);
    pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT: Self = Self(65536);
    pub const MIDPOINT_CHROMA_SAMPLES_BIT: Self = Self(131072);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: Self = Self(262144);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: Self = Self(524288);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: Self = Self(1048576);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: Self = Self(2097152);
    pub const DISJOINT_BIT: Self = Self(4194304);
    pub const COSITED_CHROMA_SAMPLES_BIT: Self = Self(8388608);
    pub const STORAGE_READ_WITHOUT_FORMAT_BIT: Self = Self(2147483648);
    pub const STORAGE_WRITE_WITHOUT_FORMAT_BIT: Self = Self(4294967296);
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON_BIT: Self = Self(8589934592);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT: Self = Self(8192);
    pub const HOST_IMAGE_TRANSFER_BIT: Self = Self(70368744177664);
    pub const VIDEO_DECODE_OUTPUT_BIT_KHR: Self = Self(33554432);
    pub const VIDEO_DECODE_DPB_BIT_KHR: Self = Self(67108864);
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR: Self = Self(536870912);
    pub const FRAGMENT_DENSITY_MAP_BIT_EXT: Self = Self(16777216);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(1073741824);
    pub const HOST_IMAGE_TRANSFER_BIT_EXT: Self = Self(70368744177664);
    pub const VIDEO_ENCODE_INPUT_BIT_KHR: Self = Self(134217728);
    pub const VIDEO_ENCODE_DPB_BIT_KHR: Self = Self(268435456);
    pub const BLOCK_MATCHING_SXD_BIT_QCOM: Self = Self(17592186044416);
    pub const SAMPLED_IMAGE_BIT_KHR: Self = Self(1);
    pub const STORAGE_IMAGE_BIT_KHR: Self = Self(2);
    pub const STORAGE_IMAGE_ATOMIC_BIT_KHR: Self = Self(4);
    pub const UNIFORM_TEXEL_BUFFER_BIT_KHR: Self = Self(8);
    pub const STORAGE_TEXEL_BUFFER_BIT_KHR: Self = Self(16);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR: Self = Self(32);
    pub const VERTEX_BUFFER_BIT_KHR: Self = Self(64);
    pub const COLOR_ATTACHMENT_BIT_KHR: Self = Self(128);
    pub const COLOR_ATTACHMENT_BLEND_BIT_KHR: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_BIT_KHR: Self = Self(512);
    pub const BLIT_SRC_BIT_KHR: Self = Self(1024);
    pub const BLIT_DST_BIT_KHR: Self = Self(2048);
    pub const SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR: Self = Self(4096);
    pub const TRANSFER_SRC_BIT_KHR: Self = Self(16384);
    pub const TRANSFER_DST_BIT_KHR: Self = Self(32768);
    pub const MIDPOINT_CHROMA_SAMPLES_BIT_KHR: Self = Self(131072);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: Self = Self(262144);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: Self = Self(524288);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: Self = Self(1048576);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: Self = Self(2097152);
    pub const DISJOINT_BIT_KHR: Self = Self(4194304);
    pub const COSITED_CHROMA_SAMPLES_BIT_KHR: Self = Self(8388608);
    pub const STORAGE_READ_WITHOUT_FORMAT_BIT_KHR: Self = Self(2147483648);
    pub const STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR: Self = Self(4294967296);
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR: Self = Self(8589934592);
    pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR: Self = Self(65536);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: Self = Self(8192);
    pub const ACCELERATION_STRUCTURE_RADIUS_BUFFER_BIT_NV: Self = Self(2251799813685248);
    pub const LINEAR_COLOR_ATTACHMENT_BIT_NV: Self = Self(274877906944);
    pub const WEIGHT_IMAGE_BIT_QCOM: Self = Self(17179869184);
    pub const WEIGHT_SAMPLED_IMAGE_BIT_QCOM: Self = Self(34359738368);
    pub const BLOCK_MATCHING_BIT_QCOM: Self = Self(68719476736);
    pub const BOX_FILTER_SAMPLED_BIT_QCOM: Self = Self(137438953472);
    pub const TENSOR_SHADER_BIT_ARM: Self = Self(549755813888);
    pub const TENSOR_IMAGE_ALIASING_BIT_ARM: Self = Self(8796093022208);
    pub const OPTICAL_FLOW_IMAGE_BIT_NV: Self = Self(1099511627776);
    pub const OPTICAL_FLOW_VECTOR_BIT_NV: Self = Self(2199023255552);
    pub const OPTICAL_FLOW_COST_BIT_NV: Self = Self(4398046511104);
    pub const TENSOR_DATA_GRAPH_BIT_ARM: Self = Self(281474976710656);
    pub const RESERVED_60_BIT_EXT: Self = Self(1152921504606846976);
    pub const COPY_IMAGE_INDIRECT_DST_BIT_KHR: Self = Self(576460752303423488);
    pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR: Self = Self(562949953421312);
    pub const VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR: Self = Self(1125899906842624);
    pub const SAMPLED_IMAGE_FILTER_LINEAR_2D_BIT_IMG: Self = Self(35184372088832);
    pub const DEPTH_COPY_ON_COMPUTE_QUEUE_BIT_KHR: Self = Self(4503599627370496);
    pub const DEPTH_COPY_ON_TRANSFER_QUEUE_BIT_KHR: Self = Self(9007199254740992);
    pub const STENCIL_COPY_ON_COMPUTE_QUEUE_BIT_KHR: Self = Self(18014398509481984);
    pub const STENCIL_COPY_ON_TRANSFER_QUEUE_BIT_KHR: Self = Self(36028797018963968);
    pub const DATA_GRAPH_OPTICAL_FLOW_IMAGE_BIT_ARM: Self = Self(72057594037927936);
    pub const DATA_GRAPH_OPTICAL_FLOW_VECTOR_BIT_ARM: Self = Self(144115188075855872);
    pub const DATA_GRAPH_OPTICAL_FLOW_COST_BIT_ARM: Self = Self(288230376151711744);
    pub const RESERVED_47_BIT_ARM: Self = Self(140737488355328);
    pub const RESERVED_61_BIT_HUAWEI: Self = Self(2305843009213693952);
}

impl core::ops::BitOr for VkFormatFeatureFlagBits2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkFramebufferCreateFlagBits(pub u32);

impl VkFramebufferCreateFlagBits {
    pub const IMAGELESS_BIT: Self = Self(1);
    pub const IMAGELESS_BIT_KHR: Self = Self(1);
}

impl core::ops::BitOr for VkFramebufferCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkFrontFace(pub i32);

impl VkFrontFace {
    pub const COUNTER_CLOCKWISE: Self = Self(0);
    pub const CLOCKWISE: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkImageAspectFlagBits(pub u32);

impl VkImageAspectFlagBits {
    pub const COLOR_BIT: Self = Self(1);
    pub const DEPTH_BIT: Self = Self(2);
    pub const STENCIL_BIT: Self = Self(4);
    pub const METADATA_BIT: Self = Self(8);
    pub const PLANE_0_BIT: Self = Self(16);
    pub const PLANE_1_BIT: Self = Self(32);
    pub const PLANE_2_BIT: Self = Self(64);
    pub const NONE: Self = Self(0);
    pub const PLANE_0_BIT_KHR: Self = Self(16);
    pub const PLANE_1_BIT_KHR: Self = Self(32);
    pub const PLANE_2_BIT_KHR: Self = Self(64);
    pub const MEMORY_PLANE_0_BIT_EXT: Self = Self(128);
    pub const MEMORY_PLANE_1_BIT_EXT: Self = Self(256);
    pub const MEMORY_PLANE_2_BIT_EXT: Self = Self(512);
    pub const MEMORY_PLANE_3_BIT_EXT: Self = Self(1024);
    pub const NONE_KHR: Self = Self(0);
    pub const RESERVED_11_BIT_HUAWEI: Self = Self(2048);
}

impl core::ops::BitOr for VkImageAspectFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkImageCreateFlagBits(pub u32);

impl VkImageCreateFlagBits {
    pub const SPARSE_BINDING_BIT: Self = Self(1);
    pub const SPARSE_RESIDENCY_BIT: Self = Self(2);
    pub const SPARSE_ALIASED_BIT: Self = Self(4);
    pub const MUTABLE_FORMAT_BIT: Self = Self(8);
    pub const CUBE_COMPATIBLE_BIT: Self = Self(16);
    pub const ALIAS_BIT: Self = Self(1024);
    pub const SPLIT_INSTANCE_BIND_REGIONS_BIT: Self = Self(64);
    pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT: Self = Self(32);
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE_BIT: Self = Self(128);
    pub const EXTENDED_USAGE_BIT: Self = Self(256);
    pub const PROTECTED_BIT: Self = Self(2048);
    pub const DISJOINT_BIT: Self = Self(512);
    pub const CORNER_SAMPLED_BIT_NV: Self = Self(8192);
    pub const SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: Self = Self(64);
    pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR: Self = Self(32);
    pub const RESERVED_21_BIT_IMG: Self = Self(2097152);
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR: Self = Self(128);
    pub const EXTENDED_USAGE_BIT_KHR: Self = Self(256);
    pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_EXT: Self = Self(65536);
    pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT: Self = Self(4096);
    pub const DISJOINT_BIT_KHR: Self = Self(512);
    pub const ALIAS_BIT_KHR: Self = Self(1024);
    pub const SUBSAMPLED_BIT_EXT: Self = Self(16384);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(65536);
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT: Self = Self(262144);
    pub const VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT: Self = Self(131072);
    pub const FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM: Self = Self(32768);
    pub const VIDEO_PROFILE_INDEPENDENT_BIT_KHR: Self = Self(1048576);
    pub const FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT: Self = Self(32768);
    pub const ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR: Self = Self(4194304);
    pub const RESERVED_19_BIT_NV: Self = Self(524288);
}

impl core::ops::BitOr for VkImageCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkImageLayout(pub i32);

impl VkImageLayout {
    pub const UNDEFINED: Self = Self(0);
    pub const GENERAL: Self = Self(1);
    pub const COLOR_ATTACHMENT_OPTIMAL: Self = Self(2);
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(3);
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = Self(4);
    pub const SHADER_READ_ONLY_OPTIMAL: Self = Self(5);
    pub const TRANSFER_SRC_OPTIMAL: Self = Self(6);
    pub const TRANSFER_DST_OPTIMAL: Self = Self(7);
    pub const PREINITIALIZED: Self = Self(8);
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1000117000);
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: Self = Self(1000117001);
    pub const DEPTH_ATTACHMENT_OPTIMAL: Self = Self(1000241000);
    pub const DEPTH_READ_ONLY_OPTIMAL: Self = Self(1000241001);
    pub const STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1000241002);
    pub const STENCIL_READ_ONLY_OPTIMAL: Self = Self(1000241003);
    pub const READ_ONLY_OPTIMAL: Self = Self(1000314000);
    pub const ATTACHMENT_OPTIMAL: Self = Self(1000314001);
    pub const RENDERING_LOCAL_READ: Self = Self(1000232000);
    pub const PRESENT_SRC_KHR: Self = Self(1000001002);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(1000024000);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(1000024001);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(1000024002);
    pub const SHARED_PRESENT_KHR: Self = Self(1000111000);
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000117000);
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR: Self = Self(1000117001);
    pub const SHADING_RATE_OPTIMAL_NV: Self = Self(1000164003);
    pub const FRAGMENT_DENSITY_MAP_OPTIMAL_EXT: Self = Self(1000218000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000164003);
    pub const RENDERING_LOCAL_READ_KHR: Self = Self(1000232000);
    pub const DEPTH_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000241000);
    pub const DEPTH_READ_ONLY_OPTIMAL_KHR: Self = Self(1000241001);
    pub const STENCIL_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000241002);
    pub const STENCIL_READ_ONLY_OPTIMAL_KHR: Self = Self(1000241003);
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(1000299000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1000299001);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1000299002);
    pub const READ_ONLY_OPTIMAL_KHR: Self = Self(1000314000);
    pub const ATTACHMENT_OPTIMAL_KHR: Self = Self(1000314001);
    pub const ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT: Self = Self(1000339000);
    pub const TENSOR_ALIASING_ARM: Self = Self(1000460000);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_KHR: Self = Self(1000553000);
    pub const ZERO_INITIALIZED_EXT: Self = Self(1000620000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkImageTiling(pub i32);

impl VkImageTiling {
    pub const OPTIMAL: Self = Self(0);
    pub const LINEAR: Self = Self(1);
    pub const DRM_FORMAT_MODIFIER_EXT: Self = Self(1000158000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkImageType(pub i32);

impl VkImageType {
    pub const VK_IMAGE_TYPE_1D: Self = Self(0);
    pub const VK_IMAGE_TYPE_2D: Self = Self(1);
    pub const VK_IMAGE_TYPE_3D: Self = Self(2);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkImageUsageFlagBits(pub u32);

impl VkImageUsageFlagBits {
    pub const TRANSFER_SRC_BIT: Self = Self(1);
    pub const TRANSFER_DST_BIT: Self = Self(2);
    pub const SAMPLED_BIT: Self = Self(4);
    pub const STORAGE_BIT: Self = Self(8);
    pub const COLOR_ATTACHMENT_BIT: Self = Self(16);
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: Self = Self(32);
    pub const TRANSIENT_ATTACHMENT_BIT: Self = Self(64);
    pub const INPUT_ATTACHMENT_BIT: Self = Self(128);
    pub const HOST_TRANSFER_BIT: Self = Self(4194304);
    pub const VIDEO_DECODE_DST_BIT_KHR: Self = Self(1024);
    pub const VIDEO_DECODE_SRC_BIT_KHR: Self = Self(2048);
    pub const VIDEO_DECODE_DPB_BIT_KHR: Self = Self(4096);
    pub const SHADING_RATE_IMAGE_BIT_NV: Self = Self(256);
    pub const FRAGMENT_DENSITY_MAP_BIT_EXT: Self = Self(512);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(256);
    pub const HOST_TRANSFER_BIT_EXT: Self = Self(4194304);
    pub const VIDEO_ENCODE_DST_BIT_KHR: Self = Self(8192);
    pub const VIDEO_ENCODE_SRC_BIT_KHR: Self = Self(16384);
    pub const VIDEO_ENCODE_DPB_BIT_KHR: Self = Self(32768);
    pub const ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: Self = Self(524288);
    pub const INVOCATION_MASK_BIT_HUAWEI: Self = Self(262144);
    pub const SAMPLE_WEIGHT_BIT_QCOM: Self = Self(1048576);
    pub const SAMPLE_BLOCK_MATCH_BIT_QCOM: Self = Self(2097152);
    pub const RESERVED_24_BIT_COREAVI: Self = Self(16777216);
    pub const TENSOR_ALIASING_BIT_ARM: Self = Self(8388608);
    pub const RESERVED_28_BIT_EXT: Self = Self(268435456);
    pub const TILE_MEMORY_BIT_QCOM: Self = Self(134217728);
    pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR: Self = Self(33554432);
    pub const VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR: Self = Self(67108864);
    pub const RESERVED_29_BIT_KHR: Self = Self(536870912);
    pub const RESERVED_30_BIT_KHR: Self = Self(1073741824);
    pub const RESERVED_16_BIT_HUAWEI: Self = Self(65536);
    pub const RESERVED_17_BIT_HUAWEI: Self = Self(131072);
}

impl core::ops::BitOr for VkImageUsageFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkImageViewCreateFlagBits(pub u32);

impl VkImageViewCreateFlagBits {
    pub const FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT: Self = Self(1);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(4);
    pub const FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT: Self = Self(2);
}

impl core::ops::BitOr for VkImageViewCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkImageViewType(pub i32);

impl VkImageViewType {
    pub const VK_IMAGE_VIEW_TYPE_1D: Self = Self(0);
    pub const VK_IMAGE_VIEW_TYPE_2D: Self = Self(1);
    pub const VK_IMAGE_VIEW_TYPE_3D: Self = Self(2);
    pub const CUBE: Self = Self(3);
    pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY: Self = Self(4);
    pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY: Self = Self(5);
    pub const CUBE_ARRAY: Self = Self(6);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkIndexType(pub i32);

impl VkIndexType {
    pub const UINT16: Self = Self(0);
    pub const UINT32: Self = Self(1);
    pub const UINT8: Self = Self(1000265000);
    pub const NONE_KHR: Self = Self(1000165000);
    pub const NONE_NV: Self = Self(1000165000);
    pub const UINT8_EXT: Self = Self(1000265000);
    pub const UINT8_KHR: Self = Self(1000265000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkInstanceCreateFlagBits(pub u32);

impl VkInstanceCreateFlagBits {
    pub const ENUMERATE_PORTABILITY_BIT_KHR: Self = Self(1);
    pub const RESERVED_616_BIT_EXT: Self = Self(2);
}

impl core::ops::BitOr for VkInstanceCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkInternalAllocationType(pub i32);

impl VkInternalAllocationType {
    pub const EXECUTABLE: Self = Self(0);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkLogicOp(pub i32);

impl VkLogicOp {
    pub const CLEAR: Self = Self(0);
    pub const AND: Self = Self(1);
    pub const AND_REVERSE: Self = Self(2);
    pub const COPY: Self = Self(3);
    pub const AND_INVERTED: Self = Self(4);
    pub const NO_OP: Self = Self(5);
    pub const XOR: Self = Self(6);
    pub const OR: Self = Self(7);
    pub const NOR: Self = Self(8);
    pub const EQUIVALENT: Self = Self(9);
    pub const INVERT: Self = Self(10);
    pub const OR_REVERSE: Self = Self(11);
    pub const COPY_INVERTED: Self = Self(12);
    pub const OR_INVERTED: Self = Self(13);
    pub const NAND: Self = Self(14);
    pub const SET: Self = Self(15);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkMemoryAllocateFlagBits(pub u32);

impl VkMemoryAllocateFlagBits {
    pub const DEVICE_MASK_BIT: Self = Self(1);
    pub const DEVICE_ADDRESS_BIT: Self = Self(2);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: Self = Self(4);
    pub const DEVICE_MASK_BIT_KHR: Self = Self(1);
    pub const DEVICE_ADDRESS_BIT_KHR: Self = Self(2);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: Self = Self(4);
    pub const ZERO_INITIALIZE_BIT_EXT: Self = Self(8);
}

impl core::ops::BitOr for VkMemoryAllocateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkMemoryHeapFlagBits(pub u32);

impl VkMemoryHeapFlagBits {
    pub const DEVICE_LOCAL_BIT: Self = Self(1);
    pub const MULTI_INSTANCE_BIT: Self = Self(2);
    pub const MULTI_INSTANCE_BIT_KHR: Self = Self(2);
    pub const TILE_MEMORY_BIT_QCOM: Self = Self(8);
}

impl core::ops::BitOr for VkMemoryHeapFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkMemoryMapFlagBits(pub u32);

impl VkMemoryMapFlagBits {
    pub const PLACED_BIT_EXT: Self = Self(1);
}

impl core::ops::BitOr for VkMemoryMapFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkMemoryPropertyFlagBits(pub u32);

impl VkMemoryPropertyFlagBits {
    pub const DEVICE_LOCAL_BIT: Self = Self(1);
    pub const HOST_VISIBLE_BIT: Self = Self(2);
    pub const HOST_COHERENT_BIT: Self = Self(4);
    pub const HOST_CACHED_BIT: Self = Self(8);
    pub const LAZILY_ALLOCATED_BIT: Self = Self(16);
    pub const PROTECTED_BIT: Self = Self(32);
    pub const DEVICE_COHERENT_BIT_AMD: Self = Self(64);
    pub const DEVICE_UNCACHED_BIT_AMD: Self = Self(128);
    pub const RDMA_CAPABLE_BIT_NV: Self = Self(256);
}

impl core::ops::BitOr for VkMemoryPropertyFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkObjectType(pub i32);

impl VkObjectType {
    pub const UNKNOWN: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
    pub const PHYSICAL_DEVICE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const QUEUE: Self = Self(4);
    pub const SEMAPHORE: Self = Self(5);
    pub const COMMAND_BUFFER: Self = Self(6);
    pub const FENCE: Self = Self(7);
    pub const DEVICE_MEMORY: Self = Self(8);
    pub const BUFFER: Self = Self(9);
    pub const IMAGE: Self = Self(10);
    pub const EVENT: Self = Self(11);
    pub const QUERY_POOL: Self = Self(12);
    pub const BUFFER_VIEW: Self = Self(13);
    pub const IMAGE_VIEW: Self = Self(14);
    pub const SHADER_MODULE: Self = Self(15);
    pub const PIPELINE_CACHE: Self = Self(16);
    pub const PIPELINE_LAYOUT: Self = Self(17);
    pub const RENDER_PASS: Self = Self(18);
    pub const PIPELINE: Self = Self(19);
    pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    pub const SAMPLER: Self = Self(21);
    pub const DESCRIPTOR_POOL: Self = Self(22);
    pub const DESCRIPTOR_SET: Self = Self(23);
    pub const FRAMEBUFFER: Self = Self(24);
    pub const COMMAND_POOL: Self = Self(25);
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1000085000);
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1000156000);
    pub const PRIVATE_DATA_SLOT: Self = Self(1000295000);
    pub const SURFACE_KHR: Self = Self(1000000000);
    pub const SWAPCHAIN_KHR: Self = Self(1000001000);
    pub const DISPLAY_KHR: Self = Self(1000002000);
    pub const DISPLAY_MODE_KHR: Self = Self(1000002001);
    pub const DEBUG_REPORT_CALLBACK_EXT: Self = Self(1000011000);
    pub const VIDEO_SESSION_KHR: Self = Self(1000023000);
    pub const VIDEO_SESSION_PARAMETERS_KHR: Self = Self(1000023001);
    pub const CU_MODULE_NVX: Self = Self(1000029000);
    pub const CU_FUNCTION_NVX: Self = Self(1000029001);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self = Self(1000085000);
    pub const DEBUG_UTILS_MESSENGER_EXT: Self = Self(1000128000);
    pub const GPA_SESSION_AMD: Self = Self(1000133000);
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
    pub const SAMPLER_YCBCR_CONVERSION_KHR: Self = Self(1000156000);
    pub const VALIDATION_CACHE_EXT: Self = Self(1000160000);
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
    pub const PERFORMANCE_CONFIGURATION_INTEL: Self = Self(1000210000);
    pub const DEFERRED_OPERATION_KHR: Self = Self(1000268000);
    pub const INDIRECT_COMMANDS_LAYOUT_NV: Self = Self(1000277000);
    pub const PRIVATE_DATA_SLOT_EXT: Self = Self(1000295000);
    pub const CUDA_MODULE_NV: Self = Self(1000307000);
    pub const CUDA_FUNCTION_NV: Self = Self(1000307001);
    pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366000);
    pub const MICROMAP_EXT: Self = Self(1000396000);
    pub const TENSOR_ARM: Self = Self(1000460000);
    pub const TENSOR_VIEW_ARM: Self = Self(1000460001);
    pub const OPTICAL_FLOW_SESSION_NV: Self = Self(1000464000);
    pub const SHADER_EXT: Self = Self(1000482000);
    pub const PIPELINE_BINARY_KHR: Self = Self(1000483000);
    pub const SEMAPHORE_SCI_SYNC_POOL_NV: Self = Self(1000489000);
    pub const DATA_GRAPH_PIPELINE_SESSION_ARM: Self = Self(1000507000);
    pub const EXTERNAL_COMPUTE_QUEUE_NV: Self = Self(1000556000);
    pub const INDIRECT_COMMANDS_LAYOUT_EXT: Self = Self(1000572000);
    pub const INDIRECT_EXECUTION_SET_EXT: Self = Self(1000572001);
    pub const SHADER_INSTRUMENTATION_ARM: Self = Self(1000607000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPeerMemoryFeatureFlagBits(pub u32);

impl VkPeerMemoryFeatureFlagBits {
    pub const COPY_SRC_BIT: Self = Self(1);
    pub const COPY_DST_BIT: Self = Self(2);
    pub const GENERIC_SRC_BIT: Self = Self(4);
    pub const GENERIC_DST_BIT: Self = Self(8);
    pub const COPY_SRC_BIT_KHR: Self = Self(1);
    pub const COPY_DST_BIT_KHR: Self = Self(2);
    pub const GENERIC_SRC_BIT_KHR: Self = Self(4);
    pub const GENERIC_DST_BIT_KHR: Self = Self(8);
}

impl core::ops::BitOr for VkPeerMemoryFeatureFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPhysicalDeviceType(pub i32);

impl VkPhysicalDeviceType {
    pub const OTHER: Self = Self(0);
    pub const INTEGRATED_GPU: Self = Self(1);
    pub const DISCRETE_GPU: Self = Self(2);
    pub const VIRTUAL_GPU: Self = Self(3);
    pub const CPU: Self = Self(4);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPipelineBindPoint(pub i32);

impl VkPipelineBindPoint {
    pub const GRAPHICS: Self = Self(0);
    pub const COMPUTE: Self = Self(1);
    pub const EXECUTION_GRAPH_AMDX: Self = Self(1000134000);
    pub const RAY_TRACING_KHR: Self = Self(1000165000);
    pub const RAY_TRACING_NV: Self = Self(1000165000);
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(1000369003);
    pub const DATA_GRAPH_ARM: Self = Self(1000507000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPipelineCacheCreateFlagBits(pub u32);

impl VkPipelineCacheCreateFlagBits {
    pub const EXTERNALLY_SYNCHRONIZED_BIT: Self = Self(1);
    pub const EXTERNALLY_SYNCHRONIZED_BIT_EXT: Self = Self(1);
    pub const INTERNALLY_SYNCHRONIZED_MERGE_BIT_KHR: Self = Self(8);
}

impl core::ops::BitOr for VkPipelineCacheCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPipelineCacheHeaderVersion(pub i32);

impl VkPipelineCacheHeaderVersion {
    pub const ONE: Self = Self(1);
    pub const DATA_GRAPH_QCOM: Self = Self(1000629000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPipelineColorBlendStateCreateFlagBits(pub u32);

impl VkPipelineColorBlendStateCreateFlagBits {
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM: Self = Self(1);
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT: Self = Self(1);
}

impl core::ops::BitOr for VkPipelineColorBlendStateCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPipelineCreateFlagBits(pub u32);

impl VkPipelineCreateFlagBits {
    pub const DISABLE_OPTIMIZATION_BIT: Self = Self(1);
    pub const ALLOW_DERIVATIVES_BIT: Self = Self(2);
    pub const DERIVATIVE_BIT: Self = Self(4);
    pub const DISPATCH_BASE_BIT: Self = Self(16);
    pub const DISPATCH_BASE: Self = Self(16);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX_BIT: Self = Self(8);
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT: Self = Self(256);
    pub const EARLY_RETURN_ON_FAILURE_BIT: Self = Self(512);
    pub const NO_PROTECTED_ACCESS_BIT: Self = Self(134217728);
    pub const PROTECTED_ACCESS_ONLY_BIT: Self = Self(1073741824);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR: Self = Self(8);
    pub const DISPATCH_BASE_BIT_KHR: Self = Self(16);
    pub const DISPATCH_BASE_KHR: Self = Self(16);
    pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR: Self = Self(16384);
    pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR: Self = Self(32768);
    pub const RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR: Self = Self(65536);
    pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR: Self = Self(131072);
    pub const RAY_TRACING_SKIP_TRIANGLES_BIT_KHR: Self = Self(4096);
    pub const RAY_TRACING_SKIP_AABBS_BIT_KHR: Self = Self(8192);
    pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR: Self = Self(524288);
    pub const DEFER_COMPILE_BIT_NV: Self = Self(32);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: Self = Self(4194304);
    pub const VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: Self = Self(4194304);
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(2097152);
    pub const VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(2097152);
    pub const CAPTURE_STATISTICS_BIT_KHR: Self = Self(64);
    pub const CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR: Self = Self(128);
    pub const INDIRECT_BINDABLE_BIT_NV: Self = Self(262144);
    pub const LIBRARY_BIT_KHR: Self = Self(2048);
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT: Self = Self(256);
    pub const EARLY_RETURN_ON_FAILURE_BIT_EXT: Self = Self(512);
    pub const DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(536870912);
    pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT: Self = Self(8388608);
    pub const LINK_TIME_OPTIMIZATION_BIT_EXT: Self = Self(1024);
    pub const RAY_TRACING_ALLOW_MOTION_BIT_NV: Self = Self(1048576);
    pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: Self = Self(33554432);
    pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: Self = Self(67108864);
    pub const RAY_TRACING_OPACITY_MICROMAP_BIT_EXT: Self = Self(16777216);
    pub const RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV: Self = Self(268435456);
    pub const NO_PROTECTED_ACCESS_BIT_EXT: Self = Self(134217728);
    pub const PROTECTED_ACCESS_ONLY_BIT_EXT: Self = Self(1073741824);
    pub const RAY_TRACING_OPACITY_MICROMAP_BIT_KHR: Self = Self(16777216);
}

impl core::ops::BitOr for VkPipelineCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPipelineCreationFeedbackFlagBits(pub u32);

impl VkPipelineCreationFeedbackFlagBits {
    pub const VALID_BIT: Self = Self(1);
    pub const APPLICATION_PIPELINE_CACHE_HIT_BIT: Self = Self(2);
    pub const BASE_PIPELINE_ACCELERATION_BIT: Self = Self(4);
    pub const VALID_BIT_EXT: Self = Self(1);
    pub const APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT: Self = Self(2);
    pub const BASE_PIPELINE_ACCELERATION_BIT_EXT: Self = Self(4);
}

impl core::ops::BitOr for VkPipelineCreationFeedbackFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPipelineDepthStencilStateCreateFlagBits(pub u32);

impl VkPipelineDepthStencilStateCreateFlagBits {
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM: Self = Self(1);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM: Self = Self(2);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT: Self = Self(1);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT: Self = Self(2);
}

impl core::ops::BitOr for VkPipelineDepthStencilStateCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPipelineLayoutCreateFlagBits(pub u32);

impl VkPipelineLayoutCreateFlagBits {
    pub const INDEPENDENT_SETS_BIT_EXT: Self = Self(2);
    pub const NO_TASK_SHADER_BIT_KHR: Self = Self(4);
}

impl core::ops::BitOr for VkPipelineLayoutCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPipelineShaderStageCreateFlagBits(pub u32);

impl VkPipelineShaderStageCreateFlagBits {
    pub const ALLOW_VARYING_SUBGROUP_SIZE_BIT: Self = Self(1);
    pub const REQUIRE_FULL_SUBGROUPS_BIT: Self = Self(2);
    pub const ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT: Self = Self(1);
    pub const REQUIRE_FULL_SUBGROUPS_BIT_EXT: Self = Self(2);
    pub const RESERVED_3_BIT_KHR: Self = Self(8);
}

impl core::ops::BitOr for VkPipelineShaderStageCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPipelineStageFlagBits(pub u32);

impl VkPipelineStageFlagBits {
    pub const TOP_OF_PIPE_BIT: Self = Self(1);
    pub const DRAW_INDIRECT_BIT: Self = Self(2);
    pub const VERTEX_INPUT_BIT: Self = Self(4);
    pub const VERTEX_SHADER_BIT: Self = Self(8);
    pub const TESSELLATION_CONTROL_SHADER_BIT: Self = Self(16);
    pub const TESSELLATION_EVALUATION_SHADER_BIT: Self = Self(32);
    pub const GEOMETRY_SHADER_BIT: Self = Self(64);
    pub const FRAGMENT_SHADER_BIT: Self = Self(128);
    pub const EARLY_FRAGMENT_TESTS_BIT: Self = Self(256);
    pub const LATE_FRAGMENT_TESTS_BIT: Self = Self(512);
    pub const COLOR_ATTACHMENT_OUTPUT_BIT: Self = Self(1024);
    pub const COMPUTE_SHADER_BIT: Self = Self(2048);
    pub const TRANSFER_BIT: Self = Self(4096);
    pub const BOTTOM_OF_PIPE_BIT: Self = Self(8192);
    pub const HOST_BIT: Self = Self(16384);
    pub const ALL_GRAPHICS_BIT: Self = Self(32768);
    pub const ALL_COMMANDS_BIT: Self = Self(65536);
    pub const NONE: Self = Self(0);
    pub const TRANSFORM_FEEDBACK_BIT_EXT: Self = Self(16777216);
    pub const CONDITIONAL_RENDERING_BIT_EXT: Self = Self(262144);
    pub const ACCELERATION_STRUCTURE_BUILD_BIT_KHR: Self = Self(33554432);
    pub const RAY_TRACING_SHADER_BIT_KHR: Self = Self(2097152);
    pub const SHADING_RATE_IMAGE_BIT_NV: Self = Self(4194304);
    pub const RAY_TRACING_SHADER_BIT_NV: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_BUILD_BIT_NV: Self = Self(33554432);
    pub const TASK_SHADER_BIT_NV: Self = Self(524288);
    pub const MESH_SHADER_BIT_NV: Self = Self(1048576);
    pub const FRAGMENT_DENSITY_PROCESS_BIT_EXT: Self = Self(8388608);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(4194304);
    pub const COMMAND_PREPROCESS_BIT_NV: Self = Self(131072);
    pub const NONE_KHR: Self = Self(0);
    pub const TASK_SHADER_BIT_EXT: Self = Self(524288);
    pub const MESH_SHADER_BIT_EXT: Self = Self(1048576);
    pub const COMMAND_PREPROCESS_BIT_EXT: Self = Self(131072);
}

impl core::ops::BitOr for VkPipelineStageFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPipelineStageFlagBits2(pub u64);

impl VkPipelineStageFlagBits2 {
    pub const NONE: Self = Self(0);
    pub const TOP_OF_PIPE_BIT: Self = Self(1);
    pub const DRAW_INDIRECT_BIT: Self = Self(2);
    pub const VERTEX_INPUT_BIT: Self = Self(4);
    pub const VERTEX_SHADER_BIT: Self = Self(8);
    pub const TESSELLATION_CONTROL_SHADER_BIT: Self = Self(16);
    pub const TESSELLATION_EVALUATION_SHADER_BIT: Self = Self(32);
    pub const GEOMETRY_SHADER_BIT: Self = Self(64);
    pub const FRAGMENT_SHADER_BIT: Self = Self(128);
    pub const EARLY_FRAGMENT_TESTS_BIT: Self = Self(256);
    pub const LATE_FRAGMENT_TESTS_BIT: Self = Self(512);
    pub const COLOR_ATTACHMENT_OUTPUT_BIT: Self = Self(1024);
    pub const COMPUTE_SHADER_BIT: Self = Self(2048);
    pub const ALL_TRANSFER_BIT: Self = Self(4096);
    pub const TRANSFER_BIT: Self = Self(4096);
    pub const BOTTOM_OF_PIPE_BIT: Self = Self(8192);
    pub const HOST_BIT: Self = Self(16384);
    pub const ALL_GRAPHICS_BIT: Self = Self(32768);
    pub const ALL_COMMANDS_BIT: Self = Self(65536);
    pub const COPY_BIT: Self = Self(4294967296);
    pub const RESOLVE_BIT: Self = Self(8589934592);
    pub const BLIT_BIT: Self = Self(17179869184);
    pub const CLEAR_BIT: Self = Self(34359738368);
    pub const INDEX_INPUT_BIT: Self = Self(68719476736);
    pub const VERTEX_ATTRIBUTE_INPUT_BIT: Self = Self(137438953472);
    pub const PRE_RASTERIZATION_SHADERS_BIT: Self = Self(274877906944);
    pub const VIDEO_DECODE_BIT_KHR: Self = Self(67108864);
    pub const VIDEO_ENCODE_BIT_KHR: Self = Self(134217728);
    pub const RESERVED_50_BIT_KHR: Self = Self(1125899906842624);
    pub const NONE_KHR: Self = Self(0);
    pub const TOP_OF_PIPE_BIT_KHR: Self = Self(1);
    pub const DRAW_INDIRECT_BIT_KHR: Self = Self(2);
    pub const VERTEX_INPUT_BIT_KHR: Self = Self(4);
    pub const VERTEX_SHADER_BIT_KHR: Self = Self(8);
    pub const TESSELLATION_CONTROL_SHADER_BIT_KHR: Self = Self(16);
    pub const TESSELLATION_EVALUATION_SHADER_BIT_KHR: Self = Self(32);
    pub const GEOMETRY_SHADER_BIT_KHR: Self = Self(64);
    pub const FRAGMENT_SHADER_BIT_KHR: Self = Self(128);
    pub const EARLY_FRAGMENT_TESTS_BIT_KHR: Self = Self(256);
    pub const LATE_FRAGMENT_TESTS_BIT_KHR: Self = Self(512);
    pub const COLOR_ATTACHMENT_OUTPUT_BIT_KHR: Self = Self(1024);
    pub const COMPUTE_SHADER_BIT_KHR: Self = Self(2048);
    pub const ALL_TRANSFER_BIT_KHR: Self = Self(4096);
    pub const TRANSFER_BIT_KHR: Self = Self(4096);
    pub const BOTTOM_OF_PIPE_BIT_KHR: Self = Self(8192);
    pub const HOST_BIT_KHR: Self = Self(16384);
    pub const ALL_GRAPHICS_BIT_KHR: Self = Self(32768);
    pub const ALL_COMMANDS_BIT_KHR: Self = Self(65536);
    pub const COPY_BIT_KHR: Self = Self(4294967296);
    pub const RESOLVE_BIT_KHR: Self = Self(8589934592);
    pub const BLIT_BIT_KHR: Self = Self(17179869184);
    pub const CLEAR_BIT_KHR: Self = Self(34359738368);
    pub const INDEX_INPUT_BIT_KHR: Self = Self(68719476736);
    pub const VERTEX_ATTRIBUTE_INPUT_BIT_KHR: Self = Self(137438953472);
    pub const PRE_RASTERIZATION_SHADERS_BIT_KHR: Self = Self(274877906944);
    pub const TRANSFORM_FEEDBACK_BIT_EXT: Self = Self(16777216);
    pub const CONDITIONAL_RENDERING_BIT_EXT: Self = Self(262144);
    pub const COMMAND_PREPROCESS_BIT_NV: Self = Self(131072);
    pub const COMMAND_PREPROCESS_BIT_EXT: Self = Self(131072);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(4194304);
    pub const SHADING_RATE_IMAGE_BIT_NV: Self = Self(4194304);
    pub const ACCELERATION_STRUCTURE_BUILD_BIT_KHR: Self = Self(33554432);
    pub const RAY_TRACING_SHADER_BIT_KHR: Self = Self(2097152);
    pub const RAY_TRACING_SHADER_BIT_NV: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_BUILD_BIT_NV: Self = Self(33554432);
    pub const FRAGMENT_DENSITY_PROCESS_BIT_EXT: Self = Self(8388608);
    pub const TASK_SHADER_BIT_NV: Self = Self(524288);
    pub const MESH_SHADER_BIT_NV: Self = Self(1048576);
    pub const TASK_SHADER_BIT_EXT: Self = Self(524288);
    pub const MESH_SHADER_BIT_EXT: Self = Self(1048576);
    pub const SUBPASS_SHADER_BIT_HUAWEI: Self = Self(549755813888);
    pub const SUBPASS_SHADING_BIT_HUAWEI: Self = Self(549755813888);
    pub const INVOCATION_MASK_BIT_HUAWEI: Self = Self(1099511627776);
    pub const ACCELERATION_STRUCTURE_COPY_BIT_KHR: Self = Self(268435456);
    pub const MICROMAP_BUILD_BIT_EXT: Self = Self(1073741824);
    pub const CLUSTER_CULLING_SHADER_BIT_HUAWEI: Self = Self(2199023255552);
    pub const OPTICAL_FLOW_BIT_NV: Self = Self(536870912);
    pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_BIT_NV: Self = Self(17592186044416);
    pub const DATA_GRAPH_BIT_ARM: Self = Self(4398046511104);
    pub const COPY_INDIRECT_BIT_KHR: Self = Self(70368744177664);
    pub const MEMORY_DECOMPRESSION_BIT_EXT: Self = Self(35184372088832);
    pub const RESERVED_49_BIT_EXT: Self = Self(562949953421312);
    pub const RESERVED_47_BIT_KHR: Self = Self(140737488355328);
    pub const RESERVED_31_BIT_AMD: Self = Self(2147483648);
    pub const RESERVED_43_BIT_ARM: Self = Self(8796093022208);
    pub const RESERVED_48_BIT_HUAWEI: Self = Self(281474976710656);
}

impl core::ops::BitOr for VkPipelineStageFlagBits2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPointClippingBehavior(pub i32);

impl VkPointClippingBehavior {
    pub const ALL_CLIP_PLANES: Self = Self(0);
    pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
    pub const ALL_CLIP_PLANES_KHR: Self = Self(0);
    pub const USER_CLIP_PLANES_ONLY_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPolygonMode(pub i32);

impl VkPolygonMode {
    pub const FILL: Self = Self(0);
    pub const LINE: Self = Self(1);
    pub const POINT: Self = Self(2);
    pub const FILL_RECTANGLE_NV: Self = Self(1000153000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPresentModeKHR(pub i32);

impl VkPresentModeKHR {
    pub const VK_PRESENT_MODE_IMMEDIATE_KHR: Self = Self(0);
    pub const VK_PRESENT_MODE_MAILBOX_KHR: Self = Self(1);
    pub const VK_PRESENT_MODE_FIFO_KHR: Self = Self(2);
    pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR: Self = Self(3);
    pub const VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR: Self = Self(1000111000);
    pub const VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR: Self = Self(1000111001);
    pub const VK_PRESENT_MODE_FIFO_LATEST_READY_EXT: Self = Self(1000361000);
    pub const VK_PRESENT_MODE_FIFO_LATEST_READY_KHR: Self = Self(1000361000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkPrimitiveTopology(pub i32);

impl VkPrimitiveTopology {
    pub const POINT_LIST: Self = Self(0);
    pub const LINE_LIST: Self = Self(1);
    pub const LINE_STRIP: Self = Self(2);
    pub const TRIANGLE_LIST: Self = Self(3);
    pub const TRIANGLE_STRIP: Self = Self(4);
    pub const TRIANGLE_FAN: Self = Self(5);
    pub const LINE_LIST_WITH_ADJACENCY: Self = Self(6);
    pub const LINE_STRIP_WITH_ADJACENCY: Self = Self(7);
    pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = Self(8);
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = Self(9);
    pub const PATCH_LIST: Self = Self(10);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkQueryControlFlagBits(pub u32);

impl VkQueryControlFlagBits {
    pub const PRECISE_BIT: Self = Self(1);
}

impl core::ops::BitOr for VkQueryControlFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkQueryPipelineStatisticFlagBits(pub u32);

impl VkQueryPipelineStatisticFlagBits {
    pub const INPUT_ASSEMBLY_VERTICES_BIT: Self = Self(1);
    pub const INPUT_ASSEMBLY_PRIMITIVES_BIT: Self = Self(2);
    pub const VERTEX_SHADER_INVOCATIONS_BIT: Self = Self(4);
    pub const GEOMETRY_SHADER_INVOCATIONS_BIT: Self = Self(8);
    pub const GEOMETRY_SHADER_PRIMITIVES_BIT: Self = Self(16);
    pub const CLIPPING_INVOCATIONS_BIT: Self = Self(32);
    pub const CLIPPING_PRIMITIVES_BIT: Self = Self(64);
    pub const FRAGMENT_SHADER_INVOCATIONS_BIT: Self = Self(128);
    pub const TESSELLATION_CONTROL_SHADER_PATCHES_BIT: Self = Self(256);
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: Self = Self(512);
    pub const COMPUTE_SHADER_INVOCATIONS_BIT: Self = Self(1024);
    pub const TASK_SHADER_INVOCATIONS_BIT_EXT: Self = Self(2048);
    pub const MESH_SHADER_INVOCATIONS_BIT_EXT: Self = Self(4096);
    pub const CLUSTER_CULLING_SHADER_INVOCATIONS_BIT_HUAWEI: Self = Self(8192);
}

impl core::ops::BitOr for VkQueryPipelineStatisticFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkQueryPoolCreateFlagBits(pub u32);

impl VkQueryPoolCreateFlagBits {
    pub const RESET_BIT_KHR: Self = Self(1);
}

impl core::ops::BitOr for VkQueryPoolCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkQueryResultFlagBits(pub u32);

impl VkQueryResultFlagBits {
    pub const VK_QUERY_RESULT_64_BIT: Self = Self(1);
    pub const WAIT_BIT: Self = Self(2);
    pub const WITH_AVAILABILITY_BIT: Self = Self(4);
    pub const PARTIAL_BIT: Self = Self(8);
    pub const WITH_STATUS_BIT_KHR: Self = Self(16);
}

impl core::ops::BitOr for VkQueryResultFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkQueryType(pub i32);

impl VkQueryType {
    pub const OCCLUSION: Self = Self(0);
    pub const PIPELINE_STATISTICS: Self = Self(1);
    pub const TIMESTAMP: Self = Self(2);
    pub const RESULT_STATUS_ONLY_KHR: Self = Self(1000023000);
    pub const TRANSFORM_FEEDBACK_STREAM_EXT: Self = Self(1000028004);
    pub const PERFORMANCE_QUERY_KHR: Self = Self(1000116000);
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR: Self = Self(1000150001);
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV: Self = Self(1000165000);
    pub const TIME_ELAPSED_QCOM: Self = Self(1000173000);
    pub const PERFORMANCE_QUERY_INTEL: Self = Self(1000210000);
    pub const VIDEO_ENCODE_FEEDBACK_KHR: Self = Self(1000299000);
    pub const MESH_PRIMITIVES_GENERATED_EXT: Self = Self(1000328000);
    pub const PRIMITIVES_GENERATED_EXT: Self = Self(1000382000);
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR: Self = Self(1000386000);
    pub const ACCELERATION_STRUCTURE_SIZE_KHR: Self = Self(1000386001);
    pub const MICROMAP_SERIALIZATION_SIZE_EXT: Self = Self(1000396000);
    pub const MICROMAP_COMPACTED_SIZE_EXT: Self = Self(1000396001);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkQueueFlagBits(pub u32);

impl VkQueueFlagBits {
    pub const GRAPHICS_BIT: Self = Self(1);
    pub const COMPUTE_BIT: Self = Self(2);
    pub const TRANSFER_BIT: Self = Self(4);
    pub const SPARSE_BINDING_BIT: Self = Self(8);
    pub const PROTECTED_BIT: Self = Self(16);
    pub const VIDEO_DECODE_BIT_KHR: Self = Self(32);
    pub const VIDEO_ENCODE_BIT_KHR: Self = Self(64);
    pub const RESERVED_7_BIT_QCOM: Self = Self(128);
    pub const OPTICAL_FLOW_BIT_NV: Self = Self(256);
    pub const DATA_GRAPH_BIT_ARM: Self = Self(1024);
    pub const RESERVED_12_BIT_EXT: Self = Self(4096);
    pub const RESERVED_9_BIT_EXT: Self = Self(512);
    pub const RESERVED_13_BIT_EXT: Self = Self(8192);
    pub const RESERVED_11_BIT_ARM: Self = Self(2048);
    pub const RESERVED_14_BIT_EXT: Self = Self(16384);
}

impl core::ops::BitOr for VkQueueFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkRenderPassCreateFlagBits(pub u32);

impl VkRenderPassCreateFlagBits {
    pub const RESERVED_3_BIT_IMG: Self = Self(8);
    pub const RESERVED_0_BIT_KHR: Self = Self(1);
    pub const TRANSFORM_BIT_QCOM: Self = Self(2);
    pub const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE: Self = Self(4);
}

impl core::ops::BitOr for VkRenderPassCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkRenderingFlagBits(pub u32);

impl VkRenderingFlagBits {
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT: Self = Self(1);
    pub const SUSPENDING_BIT: Self = Self(2);
    pub const RESUMING_BIT: Self = Self(4);
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR: Self = Self(1);
    pub const SUSPENDING_BIT_KHR: Self = Self(2);
    pub const RESUMING_BIT_KHR: Self = Self(4);
    pub const RESERVED_9_BIT_IMG: Self = Self(512);
    pub const CONTENTS_INLINE_BIT_EXT: Self = Self(16);
    pub const ENABLE_LEGACY_DITHERING_BIT_EXT: Self = Self(8);
    pub const CONTENTS_INLINE_BIT_KHR: Self = Self(16);
    pub const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE: Self = Self(32);
    pub const FRAGMENT_REGION_BIT_EXT: Self = Self(64);
    pub const CUSTOM_RESOLVE_BIT_EXT: Self = Self(128);
    pub const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_BIT_KHR: Self = Self(256);
    pub const RESERVED_10_BIT_VALVE: Self = Self(1024);
    pub const RESERVED_11_BIT_VALVE: Self = Self(2048);
    pub const RESERVED_12_BIT_VALVE: Self = Self(4096);
}

impl core::ops::BitOr for VkRenderingFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkResolveModeFlagBits(pub u32);

impl VkResolveModeFlagBits {
    pub const NONE: Self = Self(0);
    pub const SAMPLE_ZERO_BIT: Self = Self(1);
    pub const AVERAGE_BIT: Self = Self(2);
    pub const MIN_BIT: Self = Self(4);
    pub const MAX_BIT: Self = Self(8);
    pub const NONE_KHR: Self = Self(0);
    pub const SAMPLE_ZERO_BIT_KHR: Self = Self(1);
    pub const AVERAGE_BIT_KHR: Self = Self(2);
    pub const MIN_BIT_KHR: Self = Self(4);
    pub const MAX_BIT_KHR: Self = Self(8);
    pub const EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID: Self = Self(16);
    pub const EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID: Self = Self(16);
    pub const CUSTOM_BIT_EXT: Self = Self(32);
}

impl core::ops::BitOr for VkResolveModeFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkResult(pub i32);

impl VkResult {
    pub const VK_SUCCESS: Self = Self(0);
    pub const VK_NOT_READY: Self = Self(1);
    pub const VK_TIMEOUT: Self = Self(2);
    pub const VK_EVENT_SET: Self = Self(3);
    pub const VK_EVENT_RESET: Self = Self(4);
    pub const VK_INCOMPLETE: Self = Self(5);
    pub const VK_ERROR_OUT_OF_HOST_MEMORY: Self = Self(-1);
    pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: Self = Self(-2);
    pub const VK_ERROR_INITIALIZATION_FAILED: Self = Self(-3);
    pub const VK_ERROR_DEVICE_LOST: Self = Self(-4);
    pub const VK_ERROR_MEMORY_MAP_FAILED: Self = Self(-5);
    pub const VK_ERROR_LAYER_NOT_PRESENT: Self = Self(-6);
    pub const VK_ERROR_EXTENSION_NOT_PRESENT: Self = Self(-7);
    pub const VK_ERROR_FEATURE_NOT_PRESENT: Self = Self(-8);
    pub const VK_ERROR_INCOMPATIBLE_DRIVER: Self = Self(-9);
    pub const VK_ERROR_TOO_MANY_OBJECTS: Self = Self(-10);
    pub const VK_ERROR_FORMAT_NOT_SUPPORTED: Self = Self(-11);
    pub const VK_ERROR_FRAGMENTED_POOL: Self = Self(-12);
    pub const VK_ERROR_UNKNOWN: Self = Self(-13);
    pub const VK_ERROR_VALIDATION_FAILED: Self = Self(-1000011001);
    pub const VK_ERROR_OUT_OF_POOL_MEMORY: Self = Self(-1000069000);
    pub const VK_ERROR_INVALID_EXTERNAL_HANDLE: Self = Self(-1000072003);
    pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: Self = Self(-1000257000);
    pub const VK_ERROR_FRAGMENTATION: Self = Self(-1000161000);
    pub const VK_PIPELINE_COMPILE_REQUIRED: Self = Self(1000297000);
    pub const VK_ERROR_NOT_PERMITTED: Self = Self(-1000174001);
    pub const VK_ERROR_SURFACE_LOST_KHR: Self = Self(-1000000000);
    pub const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR: Self = Self(-1000000001);
    pub const VK_SUBOPTIMAL_KHR: Self = Self(1000001003);
    pub const VK_ERROR_OUT_OF_DATE_KHR: Self = Self(-1000001004);
    pub const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR: Self = Self(-1000003001);
    pub const VK_ERROR_VALIDATION_FAILED_EXT: Self = Self(-1000011001);
    pub const VK_ERROR_INVALID_SHADER_NV: Self = Self(-1000012000);
    pub const VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR: Self = Self(-1000023000);
    pub const VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR: Self = Self(-1000023001);
    pub const VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR: Self = Self(-1000023002);
    pub const VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR: Self = Self(-1000023003);
    pub const VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR: Self = Self(-1000023004);
    pub const VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR: Self = Self(-1000023005);
    pub const VK_ERROR_OUT_OF_POOL_MEMORY_KHR: Self = Self(-1000069000);
    pub const VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR: Self = Self(-1000072003);
    pub const VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: Self = Self(-1000158000);
    pub const VK_ERROR_FRAGMENTATION_EXT: Self = Self(-1000161000);
    pub const VK_ERROR_NOT_PERMITTED_EXT: Self = Self(-1000174001);
    pub const VK_ERROR_NOT_PERMITTED_KHR: Self = Self(-1000174001);
    pub const VK_ERROR_PRESENT_TIMING_QUEUE_FULL_EXT: Self = Self(-1000208000);
    pub const VK_ERROR_INVALID_DEVICE_ADDRESS_EXT: Self = Self(-1000257000);
    pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: Self = Self(-1000255000);
    pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: Self = Self(-1000257000);
    pub const VK_THREAD_IDLE_KHR: Self = Self(1000268000);
    pub const VK_THREAD_DONE_KHR: Self = Self(1000268001);
    pub const VK_OPERATION_DEFERRED_KHR: Self = Self(1000268002);
    pub const VK_OPERATION_NOT_DEFERRED_KHR: Self = Self(1000268003);
    pub const VK_PIPELINE_COMPILE_REQUIRED_EXT: Self = Self(1000297000);
    pub const VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT: Self = Self(1000297000);
    pub const VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR: Self = Self(-1000299000);
    pub const VK_ERROR_COMPRESSION_EXHAUSTED_EXT: Self = Self(-1000338000);
    pub const VK_INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self(1000482000);
    pub const VK_ERROR_INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self(1000482000);
    pub const VK_PIPELINE_BINARY_MISSING_KHR: Self = Self(1000483000);
    pub const VK_ERROR_NOT_ENOUGH_SPACE_KHR: Self = Self(-1000483000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSampleCountFlagBits(pub u32);

impl VkSampleCountFlagBits {
    pub const VK_SAMPLE_COUNT_1_BIT: Self = Self(1);
    pub const VK_SAMPLE_COUNT_2_BIT: Self = Self(2);
    pub const VK_SAMPLE_COUNT_4_BIT: Self = Self(4);
    pub const VK_SAMPLE_COUNT_8_BIT: Self = Self(8);
    pub const VK_SAMPLE_COUNT_16_BIT: Self = Self(16);
    pub const VK_SAMPLE_COUNT_32_BIT: Self = Self(32);
    pub const VK_SAMPLE_COUNT_64_BIT: Self = Self(64);
}

impl core::ops::BitOr for VkSampleCountFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSamplerAddressMode(pub i32);

impl VkSamplerAddressMode {
    pub const REPEAT: Self = Self(0);
    pub const MIRRORED_REPEAT: Self = Self(1);
    pub const CLAMP_TO_EDGE: Self = Self(2);
    pub const CLAMP_TO_BORDER: Self = Self(3);
    pub const MIRROR_CLAMP_TO_EDGE: Self = Self(4);
    pub const MIRROR_CLAMP_TO_EDGE_KHR: Self = Self(4);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSamplerCreateFlagBits(pub u32);

impl VkSamplerCreateFlagBits {
    pub const SUBSAMPLED_BIT_EXT: Self = Self(1);
    pub const SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT: Self = Self(2);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(8);
    pub const NON_SEAMLESS_CUBE_MAP_BIT_EXT: Self = Self(4);
    pub const IMAGE_PROCESSING_BIT_QCOM: Self = Self(16);
}

impl core::ops::BitOr for VkSamplerCreateFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSamplerMipmapMode(pub i32);

impl VkSamplerMipmapMode {
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSamplerReductionMode(pub i32);

impl VkSamplerReductionMode {
    pub const WEIGHTED_AVERAGE: Self = Self(0);
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(2);
    pub const WEIGHTED_AVERAGE_EXT: Self = Self(0);
    pub const MIN_EXT: Self = Self(1);
    pub const MAX_EXT: Self = Self(2);
    pub const WEIGHTED_AVERAGE_RANGECLAMP_QCOM: Self = Self(1000521000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSamplerYcbcrModelConversion(pub i32);

impl VkSamplerYcbcrModelConversion {
    pub const RGB_IDENTITY: Self = Self(0);
    pub const YCBCR_IDENTITY: Self = Self(1);
    pub const YCBCR_709: Self = Self(2);
    pub const YCBCR_601: Self = Self(3);
    pub const YCBCR_2020: Self = Self(4);
    pub const RGB_IDENTITY_KHR: Self = Self(0);
    pub const YCBCR_IDENTITY_KHR: Self = Self(1);
    pub const YCBCR_709_KHR: Self = Self(2);
    pub const YCBCR_601_KHR: Self = Self(3);
    pub const YCBCR_2020_KHR: Self = Self(4);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSamplerYcbcrRange(pub i32);

impl VkSamplerYcbcrRange {
    pub const ITU_FULL: Self = Self(0);
    pub const ITU_NARROW: Self = Self(1);
    pub const ITU_FULL_KHR: Self = Self(0);
    pub const ITU_NARROW_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSemaphoreImportFlagBits(pub u32);

impl VkSemaphoreImportFlagBits {
    pub const TEMPORARY_BIT: Self = Self(1);
    pub const TEMPORARY_BIT_KHR: Self = Self(1);
}

impl core::ops::BitOr for VkSemaphoreImportFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSemaphoreType(pub i32);

impl VkSemaphoreType {
    pub const BINARY: Self = Self(0);
    pub const TIMELINE: Self = Self(1);
    pub const BINARY_KHR: Self = Self(0);
    pub const TIMELINE_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSemaphoreWaitFlagBits(pub u32);

impl VkSemaphoreWaitFlagBits {
    pub const ANY_BIT: Self = Self(1);
    pub const ANY_BIT_KHR: Self = Self(1);
}

impl core::ops::BitOr for VkSemaphoreWaitFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkShaderFloatControlsIndependence(pub i32);

impl VkShaderFloatControlsIndependence {
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY: Self = Self(0);
    pub const ALL: Self = Self(1);
    pub const NONE: Self = Self(2);
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY_KHR: Self = Self(0);
    pub const ALL_KHR: Self = Self(1);
    pub const NONE_KHR: Self = Self(2);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkShaderStageFlagBits(pub u32);

impl VkShaderStageFlagBits {
    pub const VERTEX_BIT: Self = Self(1);
    pub const TESSELLATION_CONTROL_BIT: Self = Self(2);
    pub const TESSELLATION_EVALUATION_BIT: Self = Self(4);
    pub const GEOMETRY_BIT: Self = Self(8);
    pub const FRAGMENT_BIT: Self = Self(16);
    pub const COMPUTE_BIT: Self = Self(32);
    pub const ALL_GRAPHICS: Self = Self(31);
    pub const ALL: Self = Self(2147483647);
    pub const RAYGEN_BIT_KHR: Self = Self(256);
    pub const ANY_HIT_BIT_KHR: Self = Self(512);
    pub const CLOSEST_HIT_BIT_KHR: Self = Self(1024);
    pub const MISS_BIT_KHR: Self = Self(2048);
    pub const INTERSECTION_BIT_KHR: Self = Self(4096);
    pub const CALLABLE_BIT_KHR: Self = Self(8192);
    pub const RAYGEN_BIT_NV: Self = Self(256);
    pub const ANY_HIT_BIT_NV: Self = Self(512);
    pub const CLOSEST_HIT_BIT_NV: Self = Self(1024);
    pub const MISS_BIT_NV: Self = Self(2048);
    pub const INTERSECTION_BIT_NV: Self = Self(4096);
    pub const CALLABLE_BIT_NV: Self = Self(8192);
    pub const TASK_BIT_NV: Self = Self(64);
    pub const MESH_BIT_NV: Self = Self(128);
    pub const TASK_BIT_EXT: Self = Self(64);
    pub const MESH_BIT_EXT: Self = Self(128);
    pub const SUBPASS_SHADING_BIT_HUAWEI: Self = Self(16384);
    pub const CLUSTER_CULLING_BIT_HUAWEI: Self = Self(524288);
    pub const RESERVED_15_BIT_NV: Self = Self(32768);
    pub const RESERVED_16_BIT_HUAWEI: Self = Self(65536);
}

impl core::ops::BitOr for VkShaderStageFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSharingMode(pub i32);

impl VkSharingMode {
    pub const EXCLUSIVE: Self = Self(0);
    pub const CONCURRENT: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSparseImageFormatFlagBits(pub u32);

impl VkSparseImageFormatFlagBits {
    pub const SINGLE_MIPTAIL_BIT: Self = Self(1);
    pub const ALIGNED_MIP_SIZE_BIT: Self = Self(2);
    pub const NONSTANDARD_BLOCK_SIZE_BIT: Self = Self(4);
}

impl core::ops::BitOr for VkSparseImageFormatFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSparseMemoryBindFlagBits(pub u32);

impl VkSparseMemoryBindFlagBits {
    pub const METADATA_BIT: Self = Self(1);
}

impl core::ops::BitOr for VkSparseMemoryBindFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkStencilFaceFlagBits(pub u32);

impl VkStencilFaceFlagBits {
    pub const FRONT_BIT: Self = Self(1);
    pub const BACK_BIT: Self = Self(2);
    pub const FRONT_AND_BACK: Self = Self(3);
    pub const VK_STENCIL_FRONT_AND_BACK: Self = Self(3);
}

impl core::ops::BitOr for VkStencilFaceFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkStencilOp(pub i32);

impl VkStencilOp {
    pub const KEEP: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const REPLACE: Self = Self(2);
    pub const INCREMENT_AND_CLAMP: Self = Self(3);
    pub const DECREMENT_AND_CLAMP: Self = Self(4);
    pub const INVERT: Self = Self(5);
    pub const INCREMENT_AND_WRAP: Self = Self(6);
    pub const DECREMENT_AND_WRAP: Self = Self(7);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkStructureType(pub i32);

impl VkStructureType {
    pub const APPLICATION_INFO: Self = Self(0);
    pub const INSTANCE_CREATE_INFO: Self = Self(1);
    pub const DEVICE_QUEUE_CREATE_INFO: Self = Self(2);
    pub const DEVICE_CREATE_INFO: Self = Self(3);
    pub const SUBMIT_INFO: Self = Self(4);
    pub const MEMORY_ALLOCATE_INFO: Self = Self(5);
    pub const MAPPED_MEMORY_RANGE: Self = Self(6);
    pub const BIND_SPARSE_INFO: Self = Self(7);
    pub const FENCE_CREATE_INFO: Self = Self(8);
    pub const SEMAPHORE_CREATE_INFO: Self = Self(9);
    pub const EVENT_CREATE_INFO: Self = Self(10);
    pub const QUERY_POOL_CREATE_INFO: Self = Self(11);
    pub const BUFFER_CREATE_INFO: Self = Self(12);
    pub const BUFFER_VIEW_CREATE_INFO: Self = Self(13);
    pub const IMAGE_CREATE_INFO: Self = Self(14);
    pub const IMAGE_VIEW_CREATE_INFO: Self = Self(15);
    pub const SHADER_MODULE_CREATE_INFO: Self = Self(16);
    pub const PIPELINE_CACHE_CREATE_INFO: Self = Self(17);
    pub const PIPELINE_SHADER_STAGE_CREATE_INFO: Self = Self(18);
    pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = Self(19);
    pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = Self(20);
    pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = Self(21);
    pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = Self(22);
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = Self(23);
    pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = Self(24);
    pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = Self(25);
    pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = Self(26);
    pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = Self(27);
    pub const GRAPHICS_PIPELINE_CREATE_INFO: Self = Self(28);
    pub const COMPUTE_PIPELINE_CREATE_INFO: Self = Self(29);
    pub const PIPELINE_LAYOUT_CREATE_INFO: Self = Self(30);
    pub const SAMPLER_CREATE_INFO: Self = Self(31);
    pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = Self(32);
    pub const DESCRIPTOR_POOL_CREATE_INFO: Self = Self(33);
    pub const DESCRIPTOR_SET_ALLOCATE_INFO: Self = Self(34);
    pub const WRITE_DESCRIPTOR_SET: Self = Self(35);
    pub const COPY_DESCRIPTOR_SET: Self = Self(36);
    pub const FRAMEBUFFER_CREATE_INFO: Self = Self(37);
    pub const RENDER_PASS_CREATE_INFO: Self = Self(38);
    pub const COMMAND_POOL_CREATE_INFO: Self = Self(39);
    pub const COMMAND_BUFFER_ALLOCATE_INFO: Self = Self(40);
    pub const COMMAND_BUFFER_INHERITANCE_INFO: Self = Self(41);
    pub const COMMAND_BUFFER_BEGIN_INFO: Self = Self(42);
    pub const RENDER_PASS_BEGIN_INFO: Self = Self(43);
    pub const BUFFER_MEMORY_BARRIER: Self = Self(44);
    pub const IMAGE_MEMORY_BARRIER: Self = Self(45);
    pub const MEMORY_BARRIER: Self = Self(46);
    pub const LOADER_INSTANCE_CREATE_INFO: Self = Self(47);
    pub const LOADER_DEVICE_CREATE_INFO: Self = Self(48);
    pub const BIND_BUFFER_MEMORY_INFO: Self = Self(1000157000);
    pub const BIND_IMAGE_MEMORY_INFO: Self = Self(1000157001);
    pub const MEMORY_DEDICATED_REQUIREMENTS: Self = Self(1000127000);
    pub const MEMORY_DEDICATED_ALLOCATE_INFO: Self = Self(1000127001);
    pub const MEMORY_ALLOCATE_FLAGS_INFO: Self = Self(1000060000);
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: Self = Self(1000060004);
    pub const DEVICE_GROUP_SUBMIT_INFO: Self = Self(1000060005);
    pub const DEVICE_GROUP_BIND_SPARSE_INFO: Self = Self(1000060006);
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060013);
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060014);
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES: Self = Self(1000070000);
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO: Self = Self(1000070001);
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146000);
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146001);
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146002);
    pub const MEMORY_REQUIREMENTS_2: Self = Self(1000146003);
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: Self = Self(1000146004);
    pub const PHYSICAL_DEVICE_FEATURES_2: Self = Self(1000059000);
    pub const PHYSICAL_DEVICE_PROPERTIES_2: Self = Self(1000059001);
    pub const FORMAT_PROPERTIES_2: Self = Self(1000059002);
    pub const IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059003);
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: Self = Self(1000059004);
    pub const QUEUE_FAMILY_PROPERTIES_2: Self = Self(1000059005);
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: Self = Self(1000059006);
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059007);
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: Self = Self(1000059008);
    pub const IMAGE_VIEW_USAGE_CREATE_INFO: Self = Self(1000117002);
    pub const PROTECTED_SUBMIT_INFO: Self = Self(1000145000);
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: Self = Self(1000145001);
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: Self = Self(1000145002);
    pub const DEVICE_QUEUE_INFO_2: Self = Self(1000145003);
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: Self = Self(1000071000);
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES: Self = Self(1000071001);
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: Self = Self(1000071002);
    pub const EXTERNAL_BUFFER_PROPERTIES: Self = Self(1000071003);
    pub const PHYSICAL_DEVICE_ID_PROPERTIES: Self = Self(1000071004);
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO: Self = Self(1000072000);
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO: Self = Self(1000072001);
    pub const EXPORT_MEMORY_ALLOCATE_INFO: Self = Self(1000072002);
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: Self = Self(1000112000);
    pub const EXTERNAL_FENCE_PROPERTIES: Self = Self(1000112001);
    pub const EXPORT_FENCE_CREATE_INFO: Self = Self(1000113000);
    pub const EXPORT_SEMAPHORE_CREATE_INFO: Self = Self(1000077000);
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: Self = Self(1000076000);
    pub const EXTERNAL_SEMAPHORE_PROPERTIES: Self = Self(1000076001);
    pub const PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: Self = Self(1000094000);
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: Self = Self(1000083000);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: Self = Self(1000120000);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: Self = Self(1000120000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: Self = Self(1000085000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: Self = Self(1000168000);
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT: Self = Self(1000168001);
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO: Self = Self(1000156000);
    pub const SAMPLER_YCBCR_CONVERSION_INFO: Self = Self(1000156001);
    pub const BIND_IMAGE_PLANE_MEMORY_INFO: Self = Self(1000156002);
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: Self = Self(1000156003);
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: Self = Self(1000156004);
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: Self = Self(1000156005);
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: Self = Self(1000060003);
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: Self = Self(1000117000);
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: Self = Self(1000117001);
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: Self = Self(1000117003);
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO: Self = Self(1000053000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES: Self = Self(1000053001);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: Self = Self(1000053002);
    pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES: Self = Self(1000063000);
    pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES: Self = Self(1000063000);
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES: Self = Self(1000196000);
    pub const PHYSICAL_DEVICE_VULKAN_1_1_FEATURES: Self = Self(49);
    pub const PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES: Self = Self(50);
    pub const PHYSICAL_DEVICE_VULKAN_1_2_FEATURES: Self = Self(51);
    pub const PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES: Self = Self(52);
    pub const IMAGE_FORMAT_LIST_CREATE_INFO: Self = Self(1000147000);
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES: Self = Self(1000211000);
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: Self = Self(1000261000);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: Self = Self(1000207000);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: Self = Self(1000207001);
    pub const SEMAPHORE_TYPE_CREATE_INFO: Self = Self(1000207002);
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO: Self = Self(1000207003);
    pub const SEMAPHORE_WAIT_INFO: Self = Self(1000207004);
    pub const SEMAPHORE_SIGNAL_INFO: Self = Self(1000207005);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: Self = Self(1000257000);
    pub const BUFFER_DEVICE_ADDRESS_INFO: Self = Self(1000244001);
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: Self = Self(1000257002);
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: Self = Self(1000257003);
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: Self = Self(1000257004);
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: Self = Self(1000177000);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES: Self = Self(1000180000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES: Self = Self(1000082000);
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: Self = Self(1000197000);
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO: Self = Self(1000161000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES: Self = Self(1000161001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES: Self = Self(1000161002);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO: Self = Self(1000161003);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT: Self = Self(1000161004);
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES: Self = Self(1000221000);
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES: Self = Self(1000130000);
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO: Self = Self(1000130001);
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES: Self = Self(1000253000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES: Self = Self(1000175000);
    pub const ATTACHMENT_DESCRIPTION_2: Self = Self(1000109000);
    pub const ATTACHMENT_REFERENCE_2: Self = Self(1000109001);
    pub const SUBPASS_DESCRIPTION_2: Self = Self(1000109002);
    pub const SUBPASS_DEPENDENCY_2: Self = Self(1000109003);
    pub const RENDER_PASS_CREATE_INFO_2: Self = Self(1000109004);
    pub const SUBPASS_BEGIN_INFO: Self = Self(1000109005);
    pub const SUBPASS_END_INFO: Self = Self(1000109006);
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES: Self = Self(1000199000);
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: Self = Self(1000199001);
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO: Self = Self(1000246000);
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES: Self = Self(1000108000);
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: Self = Self(1000108001);
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: Self = Self(1000108002);
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO: Self = Self(1000108003);
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES: Self = Self(1000241000);
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT: Self = Self(1000241001);
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: Self = Self(1000241002);
    pub const PHYSICAL_DEVICE_VULKAN_1_3_FEATURES: Self = Self(53);
    pub const PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES: Self = Self(54);
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES: Self = Self(1000245000);
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES: Self = Self(1000295000);
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO: Self = Self(1000295001);
    pub const PRIVATE_DATA_SLOT_CREATE_INFO: Self = Self(1000295002);
    pub const MEMORY_BARRIER_2: Self = Self(1000314000);
    pub const BUFFER_MEMORY_BARRIER_2: Self = Self(1000314001);
    pub const IMAGE_MEMORY_BARRIER_2: Self = Self(1000314002);
    pub const DEPENDENCY_INFO: Self = Self(1000314003);
    pub const SUBMIT_INFO_2: Self = Self(1000314004);
    pub const SEMAPHORE_SUBMIT_INFO: Self = Self(1000314005);
    pub const COMMAND_BUFFER_SUBMIT_INFO: Self = Self(1000314006);
    pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES: Self = Self(1000314007);
    pub const COPY_BUFFER_INFO_2: Self = Self(1000337000);
    pub const COPY_IMAGE_INFO_2: Self = Self(1000337001);
    pub const COPY_BUFFER_TO_IMAGE_INFO_2: Self = Self(1000337002);
    pub const COPY_IMAGE_TO_BUFFER_INFO_2: Self = Self(1000337003);
    pub const BUFFER_COPY_2: Self = Self(1000337006);
    pub const IMAGE_COPY_2: Self = Self(1000337007);
    pub const BUFFER_IMAGE_COPY_2: Self = Self(1000337009);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES: Self = Self(1000066000);
    pub const FORMAT_PROPERTIES_3: Self = Self(1000360000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES: Self = Self(1000413000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES: Self = Self(1000413001);
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS: Self = Self(1000413002);
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS: Self = Self(1000413003);
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO: Self = Self(1000192000);
    pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES: Self = Self(1000215000);
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES: Self = Self(1000276000);
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES: Self = Self(1000297000);
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES: Self = Self(1000325000);
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES: Self = Self(1000335000);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES: Self = Self(1000225000);
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO: Self = Self(1000225001);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES: Self = Self(1000225002);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES: Self = Self(1000138000);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES: Self = Self(1000138001);
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK: Self = Self(1000138002);
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO: Self = Self(1000138003);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES: Self = Self(1000280000);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES: Self = Self(1000280001);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES: Self = Self(1000281001);
    pub const BLIT_IMAGE_INFO_2: Self = Self(1000337004);
    pub const RESOLVE_IMAGE_INFO_2: Self = Self(1000337005);
    pub const IMAGE_BLIT_2: Self = Self(1000337008);
    pub const IMAGE_RESOLVE_2: Self = Self(1000337010);
    pub const RENDERING_INFO: Self = Self(1000044000);
    pub const RENDERING_ATTACHMENT_INFO: Self = Self(1000044001);
    pub const PIPELINE_RENDERING_CREATE_INFO: Self = Self(1000044002);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES: Self = Self(1000044003);
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO: Self = Self(1000044004);
    pub const PHYSICAL_DEVICE_VULKAN_1_4_FEATURES: Self = Self(55);
    pub const PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES: Self = Self(56);
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO: Self = Self(1000174000);
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES: Self = Self(1000388000);
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES: Self = Self(1000388001);
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES: Self = Self(1000265000);
    pub const MEMORY_MAP_INFO: Self = Self(1000271000);
    pub const MEMORY_UNMAP_INFO: Self = Self(1000271001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES: Self = Self(1000470000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES: Self = Self(1000470001);
    pub const DEVICE_IMAGE_SUBRESOURCE_INFO: Self = Self(1000470004);
    pub const SUBRESOURCE_LAYOUT_2: Self = Self(1000338002);
    pub const IMAGE_SUBRESOURCE_2: Self = Self(1000338003);
    pub const BUFFER_USAGE_FLAGS_2_CREATE_INFO: Self = Self(1000470006);
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES: Self = Self(1000545000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES: Self = Self(1000545001);
    pub const BIND_MEMORY_STATUS: Self = Self(1000545002);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES: Self = Self(1000270000);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES: Self = Self(1000270001);
    pub const MEMORY_TO_IMAGE_COPY: Self = Self(1000270002);
    pub const IMAGE_TO_MEMORY_COPY: Self = Self(1000270003);
    pub const COPY_IMAGE_TO_MEMORY_INFO: Self = Self(1000270004);
    pub const COPY_MEMORY_TO_IMAGE_INFO: Self = Self(1000270005);
    pub const HOST_IMAGE_LAYOUT_TRANSITION_INFO: Self = Self(1000270006);
    pub const COPY_IMAGE_TO_IMAGE_INFO: Self = Self(1000270007);
    pub const SUBRESOURCE_HOST_MEMCPY_SIZE: Self = Self(1000270008);
    pub const HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY: Self = Self(1000270009);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES: Self = Self(1000416000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES: Self = Self(1000528000);
    pub const PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES: Self = Self(1000544000);
    pub const PIPELINE_CREATE_FLAGS_2_CREATE_INFO: Self = Self(1000470005);
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES: Self = Self(1000080000);
    pub const BIND_DESCRIPTOR_SETS_INFO: Self = Self(1000545003);
    pub const PUSH_CONSTANTS_INFO: Self = Self(1000545004);
    pub const PUSH_DESCRIPTOR_SET_INFO: Self = Self(1000545005);
    pub const PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO: Self = Self(1000545006);
    pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES: Self = Self(1000466000);
    pub const PIPELINE_ROBUSTNESS_CREATE_INFO: Self = Self(1000068000);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES: Self = Self(1000068001);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES: Self = Self(1000068002);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES: Self = Self(1000259000);
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO: Self = Self(1000259001);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES: Self = Self(1000259002);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES: Self = Self(1000525000);
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO: Self = Self(1000190001);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES: Self = Self(1000190002);
    pub const RENDERING_AREA_INFO: Self = Self(1000470003);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES: Self = Self(1000232000);
    pub const RENDERING_ATTACHMENT_LOCATION_INFO: Self = Self(1000232001);
    pub const RENDERING_INPUT_ATTACHMENT_INDEX_INFO: Self = Self(1000232002);
    pub const SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000001000);
    pub const PRESENT_INFO_KHR: Self = Self(1000001001);
    pub const DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: Self = Self(1000060007);
    pub const IMAGE_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060008);
    pub const BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: Self = Self(1000060009);
    pub const ACQUIRE_NEXT_IMAGE_INFO_KHR: Self = Self(1000060010);
    pub const DEVICE_GROUP_PRESENT_INFO_KHR: Self = Self(1000060011);
    pub const DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060012);
    pub const DISPLAY_MODE_CREATE_INFO_KHR: Self = Self(1000002000);
    pub const DISPLAY_SURFACE_CREATE_INFO_KHR: Self = Self(1000002001);
    pub const DISPLAY_PRESENT_INFO_KHR: Self = Self(1000003000);
    pub const XLIB_SURFACE_CREATE_INFO_KHR: Self = Self(1000004000);
    pub const XCB_SURFACE_CREATE_INFO_KHR: Self = Self(1000005000);
    pub const WAYLAND_SURFACE_CREATE_INFO_KHR: Self = Self(1000006000);
    pub const ANDROID_SURFACE_CREATE_INFO_KHR: Self = Self(1000008000);
    pub const WIN32_SURFACE_CREATE_INFO_KHR: Self = Self(1000009000);
    pub const NATIVE_BUFFER_ANDROID: Self = Self(1000010000);
    pub const SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID: Self = Self(1000010001);
    pub const PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID: Self = Self(1000010002);
    pub const DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: Self = Self(1000011000);
    pub const DEBUG_REPORT_CREATE_INFO_EXT: Self = Self(1000011000);
    pub const PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: Self = Self(1000018000);
    pub const DEBUG_MARKER_OBJECT_NAME_INFO_EXT: Self = Self(1000022000);
    pub const DEBUG_MARKER_OBJECT_TAG_INFO_EXT: Self = Self(1000022001);
    pub const DEBUG_MARKER_MARKER_INFO_EXT: Self = Self(1000022002);
    pub const VIDEO_PROFILE_INFO_KHR: Self = Self(1000023000);
    pub const VIDEO_CAPABILITIES_KHR: Self = Self(1000023001);
    pub const VIDEO_PICTURE_RESOURCE_INFO_KHR: Self = Self(1000023002);
    pub const VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR: Self = Self(1000023003);
    pub const BIND_VIDEO_SESSION_MEMORY_INFO_KHR: Self = Self(1000023004);
    pub const VIDEO_SESSION_CREATE_INFO_KHR: Self = Self(1000023005);
    pub const VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000023006);
    pub const VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR: Self = Self(1000023007);
    pub const VIDEO_BEGIN_CODING_INFO_KHR: Self = Self(1000023008);
    pub const VIDEO_END_CODING_INFO_KHR: Self = Self(1000023009);
    pub const VIDEO_CODING_CONTROL_INFO_KHR: Self = Self(1000023010);
    pub const VIDEO_REFERENCE_SLOT_INFO_KHR: Self = Self(1000023011);
    pub const QUEUE_FAMILY_VIDEO_PROPERTIES_KHR: Self = Self(1000023012);
    pub const VIDEO_PROFILE_LIST_INFO_KHR: Self = Self(1000023013);
    pub const PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR: Self = Self(1000023014);
    pub const VIDEO_FORMAT_PROPERTIES_KHR: Self = Self(1000023015);
    pub const QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR: Self = Self(1000023016);
    pub const VIDEO_DECODE_INFO_KHR: Self = Self(1000024000);
    pub const VIDEO_DECODE_CAPABILITIES_KHR: Self = Self(1000024001);
    pub const VIDEO_DECODE_USAGE_INFO_KHR: Self = Self(1000024002);
    pub const DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: Self = Self(1000026000);
    pub const DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: Self = Self(1000026001);
    pub const DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000026002);
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT: Self = Self(1000028000);
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT: Self = Self(1000028001);
    pub const PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT: Self = Self(1000028002);
    pub const CU_MODULE_CREATE_INFO_NVX: Self = Self(1000029000);
    pub const CU_FUNCTION_CREATE_INFO_NVX: Self = Self(1000029001);
    pub const CU_LAUNCH_INFO_NVX: Self = Self(1000029002);
    pub const CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX: Self = Self(1000029004);
    pub const IMAGE_VIEW_HANDLE_INFO_NVX: Self = Self(1000030000);
    pub const IMAGE_VIEW_ADDRESS_PROPERTIES_NVX: Self = Self(1000030001);
    pub const VIDEO_ENCODE_H264_CAPABILITIES_KHR: Self = Self(1000038000);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000038001);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000038002);
    pub const VIDEO_ENCODE_H264_PICTURE_INFO_KHR: Self = Self(1000038003);
    pub const VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1000038004);
    pub const VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR: Self = Self(1000038005);
    pub const VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000038006);
    pub const VIDEO_ENCODE_H264_PROFILE_INFO_KHR: Self = Self(1000038007);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR: Self = Self(1000038008);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000038009);
    pub const VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR: Self = Self(1000038010);
    pub const VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000038011);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000038012);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000038013);
    pub const VIDEO_ENCODE_H265_CAPABILITIES_KHR: Self = Self(1000039000);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000039001);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000039002);
    pub const VIDEO_ENCODE_H265_PICTURE_INFO_KHR: Self = Self(1000039003);
    pub const VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1000039004);
    pub const VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR: Self = Self(1000039005);
    pub const VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000039006);
    pub const VIDEO_ENCODE_H265_PROFILE_INFO_KHR: Self = Self(1000039007);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR: Self = Self(1000039009);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000039010);
    pub const VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR: Self = Self(1000039011);
    pub const VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000039012);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000039013);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000039014);
    pub const VIDEO_DECODE_H264_CAPABILITIES_KHR: Self = Self(1000040000);
    pub const VIDEO_DECODE_H264_PICTURE_INFO_KHR: Self = Self(1000040001);
    pub const VIDEO_DECODE_H264_PROFILE_INFO_KHR: Self = Self(1000040003);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000040004);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000040005);
    pub const VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1000040006);
    pub const TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: Self = Self(1000041000);
    pub const RENDERING_INFO_KHR: Self = Self(1000044000);
    pub const RENDERING_ATTACHMENT_INFO_KHR: Self = Self(1000044001);
    pub const PIPELINE_RENDERING_CREATE_INFO_KHR: Self = Self(1000044002);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR: Self = Self(1000044003);
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR: Self = Self(1000044004);
    pub const STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: Self = Self(1000049000);
    pub const PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV: Self = Self(1000050000);
    pub const PRIVATE_VENDOR_INFO_PLACEHOLDER_OFFSET_0_NV: Self = Self(1000051000);
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: Self = Self(1000053000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: Self = Self(1000053001);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: Self = Self(1000053002);
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: Self = Self(1000056000);
    pub const EXPORT_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000056001);
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057001);
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: Self = Self(1000058000);
    pub const PHYSICAL_DEVICE_FEATURES_2_KHR: Self = Self(1000059000);
    pub const PHYSICAL_DEVICE_PROPERTIES_2_KHR: Self = Self(1000059001);
    pub const FORMAT_PROPERTIES_2_KHR: Self = Self(1000059002);
    pub const IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self(1000059003);
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: Self = Self(1000059004);
    pub const QUEUE_FAMILY_PROPERTIES_2_KHR: Self = Self(1000059005);
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: Self = Self(1000059006);
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self(1000059007);
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: Self = Self(1000059008);
    pub const MEMORY_ALLOCATE_FLAGS_INFO_KHR: Self = Self(1000060000);
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: Self = Self(1000060003);
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: Self = Self(1000060004);
    pub const DEVICE_GROUP_SUBMIT_INFO_KHR: Self = Self(1000060005);
    pub const DEVICE_GROUP_BIND_SPARSE_INFO_KHR: Self = Self(1000060006);
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: Self = Self(1000060013);
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: Self = Self(1000060014);
    pub const VALIDATION_FLAGS_EXT: Self = Self(1000061000);
    pub const VI_SURFACE_CREATE_INFO_NN: Self = Self(1000062000);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT: Self = Self(1000066000);
    pub const IMAGE_VIEW_ASTC_DECODE_MODE_EXT: Self = Self(1000067000);
    pub const PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT: Self = Self(1000067001);
    pub const PIPELINE_ROBUSTNESS_CREATE_INFO_EXT: Self = Self(1000068000);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000068001);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT: Self = Self(1000068002);
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: Self = Self(1000070000);
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: Self = Self(1000070001);
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR: Self = Self(1000071000);
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR: Self = Self(1000071001);
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: Self = Self(1000071002);
    pub const EXTERNAL_BUFFER_PROPERTIES_KHR: Self = Self(1000071003);
    pub const PHYSICAL_DEVICE_ID_PROPERTIES_KHR: Self = Self(1000071004);
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: Self = Self(1000072000);
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: Self = Self(1000072001);
    pub const EXPORT_MEMORY_ALLOCATE_INFO_KHR: Self = Self(1000072002);
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073001);
    pub const MEMORY_WIN32_HANDLE_PROPERTIES_KHR: Self = Self(1000073002);
    pub const MEMORY_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000073003);
    pub const IMPORT_MEMORY_FD_INFO_KHR: Self = Self(1000074000);
    pub const MEMORY_FD_PROPERTIES_KHR: Self = Self(1000074001);
    pub const MEMORY_GET_FD_INFO_KHR: Self = Self(1000074002);
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: Self = Self(1000075000);
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: Self = Self(1000076000);
    pub const EXTERNAL_SEMAPHORE_PROPERTIES_KHR: Self = Self(1000076001);
    pub const EXPORT_SEMAPHORE_CREATE_INFO_KHR: Self = Self(1000077000);
    pub const IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078000);
    pub const EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078001);
    pub const D3D12_FENCE_SUBMIT_INFO_KHR: Self = Self(1000078002);
    pub const SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000078003);
    pub const IMPORT_SEMAPHORE_FD_INFO_KHR: Self = Self(1000079000);
    pub const SEMAPHORE_GET_FD_INFO_KHR: Self = Self(1000079001);
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: Self = Self(1000080000);
    pub const COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT: Self = Self(1000081000);
    pub const PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT: Self = Self(1000081001);
    pub const CONDITIONAL_RENDERING_BEGIN_INFO_EXT: Self = Self(1000081002);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR: Self = Self(1000082000);
    pub const PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR: Self = Self(1000082000);
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: Self = Self(1000083000);
    pub const PRESENT_REGIONS_KHR: Self = Self(1000084000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: Self = Self(1000085000);
    pub const PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV: Self = Self(1000087000);
    pub const SURFACE_CAPABILITIES_2_EXT: Self = Self(1000090000);
    pub const SURFACE_CAPABILITIES2_EXT: Self = Self(1000090000);
    pub const DISPLAY_POWER_INFO_EXT: Self = Self(1000091000);
    pub const DEVICE_EVENT_INFO_EXT: Self = Self(1000091001);
    pub const DISPLAY_EVENT_INFO_EXT: Self = Self(1000091002);
    pub const SWAPCHAIN_COUNTER_CREATE_INFO_EXT: Self = Self(1000091003);
    pub const PRESENT_TIMES_INFO_GOOGLE: Self = Self(1000092000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: Self = Self(1000097000);
    pub const MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX: Self = Self(1000044009);
    pub const PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: Self = Self(1000098000);
    pub const PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT: Self = Self(1000099000);
    pub const PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT: Self = Self(1000099001);
    pub const PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT: Self = Self(1000101000);
    pub const PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT: Self = Self(1000101001);
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT: Self = Self(1000102000);
    pub const PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT: Self = Self(1000102001);
    pub const HDR_METADATA_EXT: Self = Self(1000105000);
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR: Self = Self(1000108000);
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR: Self = Self(1000108001);
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR: Self = Self(1000108002);
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR: Self = Self(1000108003);
    pub const ATTACHMENT_DESCRIPTION_2_KHR: Self = Self(1000109000);
    pub const ATTACHMENT_REFERENCE_2_KHR: Self = Self(1000109001);
    pub const SUBPASS_DESCRIPTION_2_KHR: Self = Self(1000109002);
    pub const SUBPASS_DEPENDENCY_2_KHR: Self = Self(1000109003);
    pub const RENDER_PASS_CREATE_INFO_2_KHR: Self = Self(1000109004);
    pub const SUBPASS_BEGIN_INFO_KHR: Self = Self(1000109005);
    pub const SUBPASS_END_INFO_KHR: Self = Self(1000109006);
    pub const PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG: Self = Self(1000110000);
    pub const SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: Self = Self(1000111000);
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: Self = Self(1000112000);
    pub const EXTERNAL_FENCE_PROPERTIES_KHR: Self = Self(1000112001);
    pub const EXPORT_FENCE_CREATE_INFO_KHR: Self = Self(1000113000);
    pub const IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114000);
    pub const EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114001);
    pub const FENCE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000114002);
    pub const IMPORT_FENCE_FD_INFO_KHR: Self = Self(1000115000);
    pub const FENCE_GET_FD_INFO_KHR: Self = Self(1000115001);
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR: Self = Self(1000116000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR: Self = Self(1000116001);
    pub const QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR: Self = Self(1000116002);
    pub const PERFORMANCE_QUERY_SUBMIT_INFO_KHR: Self = Self(1000116003);
    pub const ACQUIRE_PROFILING_LOCK_INFO_KHR: Self = Self(1000116004);
    pub const PERFORMANCE_COUNTER_KHR: Self = Self(1000116005);
    pub const PERFORMANCE_COUNTER_DESCRIPTION_KHR: Self = Self(1000116006);
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR: Self = Self(1000117000);
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR: Self = Self(1000117001);
    pub const IMAGE_VIEW_USAGE_CREATE_INFO_KHR: Self = Self(1000117002);
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR: Self = Self(1000117003);
    pub const PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: Self = Self(1000119000);
    pub const SURFACE_CAPABILITIES_2_KHR: Self = Self(1000119001);
    pub const SURFACE_FORMAT_2_KHR: Self = Self(1000119002);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: Self = Self(1000120000);
    pub const DISPLAY_PROPERTIES_2_KHR: Self = Self(1000121000);
    pub const DISPLAY_PLANE_PROPERTIES_2_KHR: Self = Self(1000121001);
    pub const DISPLAY_MODE_PROPERTIES_2_KHR: Self = Self(1000121002);
    pub const DISPLAY_PLANE_INFO_2_KHR: Self = Self(1000121003);
    pub const DISPLAY_PLANE_CAPABILITIES_2_KHR: Self = Self(1000121004);
    pub const IOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000122000);
    pub const MACOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000123000);
    pub const MEMORY_DEDICATED_REQUIREMENTS_KHR: Self = Self(1000127000);
    pub const MEMORY_DEDICATED_ALLOCATE_INFO_KHR: Self = Self(1000127001);
    pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: Self = Self(1000128000);
    pub const DEBUG_UTILS_OBJECT_TAG_INFO_EXT: Self = Self(1000128001);
    pub const DEBUG_UTILS_LABEL_EXT: Self = Self(1000128002);
    pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: Self = Self(1000128003);
    pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: Self = Self(1000128004);
    pub const ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: Self = Self(1000129000);
    pub const ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: Self = Self(1000129001);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID: Self = Self(1000129002);
    pub const IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129003);
    pub const MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129004);
    pub const EXTERNAL_FORMAT_ANDROID: Self = Self(1000129005);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID: Self = Self(1000129006);
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: Self = Self(1000130000);
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: Self = Self(1000130001);
    pub const PHYSICAL_DEVICE_GPA_FEATURES_AMD: Self = Self(1000133000);
    pub const PHYSICAL_DEVICE_GPA_PROPERTIES_AMD: Self = Self(1000133001);
    pub const GPA_SAMPLE_BEGIN_INFO_AMD: Self = Self(1000133002);
    pub const GPA_SESSION_CREATE_INFO_AMD: Self = Self(1000133003);
    pub const GPA_DEVICE_CLOCK_MODE_INFO_AMD: Self = Self(1000133004);
    pub const PHYSICAL_DEVICE_GPA_PROPERTIES_2_AMD: Self = Self(1000133005);
    pub const GPA_DEVICE_GET_CLOCK_INFO_AMD: Self = Self(1000133006);
    pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX: Self = Self(1000134000);
    pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX: Self = Self(1000134001);
    pub const EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX: Self = Self(1000134002);
    pub const EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX: Self = Self(1000134003);
    pub const PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX: Self = Self(1000134004);
    pub const TEXEL_BUFFER_DESCRIPTOR_INFO_EXT: Self = Self(1000135000);
    pub const IMAGE_DESCRIPTOR_INFO_EXT: Self = Self(1000135001);
    pub const RESOURCE_DESCRIPTOR_INFO_EXT: Self = Self(1000135002);
    pub const BIND_HEAP_INFO_EXT: Self = Self(1000135003);
    pub const PUSH_DATA_INFO_EXT: Self = Self(1000135004);
    pub const DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT: Self = Self(1000135005);
    pub const SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT: Self = Self(1000135006);
    pub const OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT: Self = Self(1000135007);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT: Self = Self(1000135008);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT: Self = Self(1000135009);
    pub const COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT: Self = Self(1000135010);
    pub const SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT: Self = Self(1000135011);
    pub const INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV: Self = Self(1000135012);
    pub const SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1000135013);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM: Self = Self(1000135014);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_AMD: Self = Self(1000044008);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT: Self = Self(1000138000);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT: Self = Self(1000138001);
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT: Self = Self(1000138002);
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT: Self = Self(1000138003);
    pub const PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR: Self = Self(1000141000);
    pub const SAMPLE_LOCATIONS_INFO_EXT: Self = Self(1000143000);
    pub const RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: Self = Self(1000143001);
    pub const PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: Self = Self(1000143002);
    pub const PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: Self = Self(1000143003);
    pub const MULTISAMPLE_PROPERTIES_EXT: Self = Self(1000143004);
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self(1000146000);
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self(1000146001);
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self(1000146002);
    pub const MEMORY_REQUIREMENTS_2_KHR: Self = Self(1000146003);
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: Self = Self(1000146004);
    pub const IMAGE_FORMAT_LIST_CREATE_INFO_KHR: Self = Self(1000147000);
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: Self = Self(1000148000);
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: Self = Self(1000148001);
    pub const PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: Self = Self(1000148002);
    pub const PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: Self = Self(1000149000);
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR: Self = Self(1000150007);
    pub const ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR: Self = Self(1000150002);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR: Self = Self(1000150003);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR: Self = Self(1000150004);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR: Self = Self(1000150005);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_KHR: Self = Self(1000150006);
    pub const ACCELERATION_STRUCTURE_VERSION_INFO_KHR: Self = Self(1000150009);
    pub const COPY_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150010);
    pub const COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR: Self = Self(1000150011);
    pub const COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150012);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR: Self = Self(1000150013);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR: Self = Self(1000150014);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_KHR: Self = Self(1000150017);
    pub const ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR: Self = Self(1000150020);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR: Self = Self(1000347000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR: Self = Self(1000347001);
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_KHR: Self = Self(1000150015);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR: Self = Self(1000150016);
    pub const RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR: Self = Self(1000150018);
    pub const PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR: Self = Self(1000348013);
    pub const PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: Self = Self(1000152000);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_NV: Self = Self(1000044008);
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV: Self = Self(1000154000);
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV: Self = Self(1000154001);
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: Self = Self(1000156000);
    pub const SAMPLER_YCBCR_CONVERSION_INFO_KHR: Self = Self(1000156001);
    pub const BIND_IMAGE_PLANE_MEMORY_INFO_KHR: Self = Self(1000156002);
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: Self = Self(1000156003);
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR: Self = Self(1000156004);
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR: Self = Self(1000156005);
    pub const BIND_BUFFER_MEMORY_INFO_KHR: Self = Self(1000157000);
    pub const BIND_IMAGE_MEMORY_INFO_KHR: Self = Self(1000157001);
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: Self = Self(1000158000);
    pub const PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT: Self = Self(1000158002);
    pub const IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT: Self = Self(1000158003);
    pub const IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT: Self = Self(1000158004);
    pub const IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: Self = Self(1000158005);
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT: Self = Self(1000158006);
    pub const VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160000);
    pub const SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160001);
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: Self = Self(1000161000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: Self = Self(1000161001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: Self = Self(1000161002);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: Self = Self(1000161003);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: Self = Self(1000161004);
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR: Self = Self(1000163000);
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR: Self = Self(1000163001);
    pub const PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV: Self = Self(1000164000);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV: Self = Self(1000164001);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV: Self = Self(1000164002);
    pub const PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV: Self = Self(1000164005);
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_NV: Self = Self(1000165000);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self = Self(1000165001);
    pub const GEOMETRY_NV: Self = Self(1000165003);
    pub const GEOMETRY_TRIANGLES_NV: Self = Self(1000165004);
    pub const GEOMETRY_AABB_NV: Self = Self(1000165005);
    pub const BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV: Self = Self(1000165006);
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV: Self = Self(1000165007);
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1000165008);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV: Self = Self(1000165009);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000165011);
    pub const ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1000165012);
    pub const PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV: Self = Self(1000166000);
    pub const PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV: Self = Self(1000166001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: Self = Self(1000168000);
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: Self = Self(1000168001);
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT: Self = Self(1000170000);
    pub const FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1000170001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_CONVERSION_FEATURES_QCOM: Self = Self(1000172000);
    pub const PHYSICAL_DEVICE_ELAPSED_TIMER_QUERY_FEATURES_QCOM: Self = Self(1000173000);
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: Self = Self(1000174000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR: Self = Self(1000175000);
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR: Self = Self(1000177000);
    pub const IMPORT_MEMORY_HOST_POINTER_INFO_EXT: Self = Self(1000178000);
    pub const MEMORY_HOST_POINTER_PROPERTIES_EXT: Self = Self(1000178001);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: Self = Self(1000178002);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR: Self = Self(1000180000);
    pub const PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR: Self = Self(1000181000);
    pub const PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD: Self = Self(1000183000);
    pub const CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self(1000184000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD: Self = Self(1000185000);
    pub const VIDEO_DECODE_H265_CAPABILITIES_KHR: Self = Self(1000187000);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000187001);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000187002);
    pub const VIDEO_DECODE_H265_PROFILE_INFO_KHR: Self = Self(1000187003);
    pub const VIDEO_DECODE_H265_PICTURE_INFO_KHR: Self = Self(1000187004);
    pub const VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1000187005);
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR: Self = Self(1000174000);
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR: Self = Self(1000388000);
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR: Self = Self(1000388001);
    pub const DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD: Self = Self(1000189000);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: Self = Self(1000190000);
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: Self = Self(1000190001);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT: Self = Self(1000190002);
    pub const PRESENT_FRAME_TOKEN_GGP: Self = Self(1000191000);
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000192000);
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR: Self = Self(1000196000);
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR: Self = Self(1000197000);
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR: Self = Self(1000199000);
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR: Self = Self(1000199001);
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV: Self = Self(1000201000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV: Self = Self(1000202000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV: Self = Self(1000202001);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV: Self = Self(1000203000);
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV: Self = Self(1000204000);
    pub const PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV: Self = Self(1000205000);
    pub const PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV: Self = Self(1000205002);
    pub const CHECKPOINT_DATA_NV: Self = Self(1000206000);
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV: Self = Self(1000206001);
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV: Self = Self(1000314008);
    pub const CHECKPOINT_DATA_2_NV: Self = Self(1000314009);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR: Self = Self(1000207000);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR: Self = Self(1000207001);
    pub const SEMAPHORE_TYPE_CREATE_INFO_KHR: Self = Self(1000207002);
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR: Self = Self(1000207003);
    pub const SEMAPHORE_WAIT_INFO_KHR: Self = Self(1000207004);
    pub const SEMAPHORE_SIGNAL_INFO_KHR: Self = Self(1000207005);
    pub const PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT: Self = Self(1000208000);
    pub const SWAPCHAIN_TIMING_PROPERTIES_EXT: Self = Self(1000208001);
    pub const SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT: Self = Self(1000208002);
    pub const PRESENT_TIMINGS_INFO_EXT: Self = Self(1000208003);
    pub const PRESENT_TIMING_INFO_EXT: Self = Self(1000208004);
    pub const PAST_PRESENTATION_TIMING_INFO_EXT: Self = Self(1000208005);
    pub const PAST_PRESENTATION_TIMING_PROPERTIES_EXT: Self = Self(1000208006);
    pub const PAST_PRESENTATION_TIMING_EXT: Self = Self(1000208007);
    pub const PRESENT_TIMING_SURFACE_CAPABILITIES_EXT: Self = Self(1000208008);
    pub const SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self(1000208009);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL: Self = Self(1000209000);
    pub const QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL: Self = Self(1000210000);
    pub const QUERY_POOL_CREATE_INFO_INTEL: Self = Self(1000210000);
    pub const INITIALIZE_PERFORMANCE_API_INFO_INTEL: Self = Self(1000210001);
    pub const PERFORMANCE_MARKER_INFO_INTEL: Self = Self(1000210002);
    pub const PERFORMANCE_STREAM_MARKER_INFO_INTEL: Self = Self(1000210003);
    pub const PERFORMANCE_OVERRIDE_INFO_INTEL: Self = Self(1000210004);
    pub const PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL: Self = Self(1000210005);
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR: Self = Self(1000211000);
    pub const PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT: Self = Self(1000212000);
    pub const DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD: Self = Self(1000213000);
    pub const SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD: Self = Self(1000213001);
    pub const IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA: Self = Self(1000214000);
    pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR: Self = Self(1000215000);
    pub const METAL_SURFACE_CREATE_INFO_EXT: Self = Self(1000217000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT: Self = Self(1000218000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT: Self = Self(1000218001);
    pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT: Self = Self(1000218002);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT: Self = Self(1000044007);
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT: Self = Self(1000221000);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT: Self = Self(1000225000);
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self = Self(1000225001);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT: Self = Self(1000225002);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000226000);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR: Self = Self(1000226001);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR: Self = Self(1000226002);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR: Self = Self(1000226003);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226004);
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000044006);
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD: Self = Self(1000227000);
    pub const PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD: Self = Self(1000229000);
    pub const PHYSICAL_DEVICE_SHADER_CONSTANT_DATA_FEATURES_KHR: Self = Self(1000231000);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR: Self = Self(1000232000);
    pub const RENDERING_ATTACHMENT_LOCATION_INFO_KHR: Self = Self(1000232001);
    pub const RENDERING_INPUT_ATTACHMENT_INDEX_INFO_KHR: Self = Self(1000232002);
    pub const PHYSICAL_DEVICE_SHADER_ABORT_FEATURES_KHR: Self = Self(1000233000);
    pub const DEVICE_FAULT_SHADER_ABORT_MESSAGE_INFO_KHR: Self = Self(1000233001);
    pub const PHYSICAL_DEVICE_SHADER_ABORT_PROPERTIES_KHR: Self = Self(1000233002);
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT: Self = Self(1000234000);
    pub const PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR: Self = Self(1000235000);
    pub const PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT: Self = Self(1000237000);
    pub const PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT: Self = Self(1000238000);
    pub const MEMORY_PRIORITY_ALLOCATE_INFO_EXT: Self = Self(1000238001);
    pub const SURFACE_PROTECTED_CAPABILITIES_KHR: Self = Self(1000239000);
    pub const PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV: Self = Self(1000240000);
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR: Self = Self(1000241000);
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR: Self = Self(1000241001);
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR: Self = Self(1000241002);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT: Self = Self(1000244000);
    pub const PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT: Self = Self(1000244000);
    pub const BUFFER_DEVICE_ADDRESS_INFO_EXT: Self = Self(1000244001);
    pub const BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT: Self = Self(1000244002);
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: Self = Self(1000245000);
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO_EXT: Self = Self(1000246000);
    pub const VALIDATION_FEATURES_EXT: Self = Self(1000247000);
    pub const PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR: Self = Self(1000248000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV: Self = Self(1000249000);
    pub const COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249002);
    pub const PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV: Self = Self(1000250000);
    pub const PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV: Self = Self(1000250001);
    pub const FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV: Self = Self(1000250002);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT: Self = Self(1000251000);
    pub const PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT: Self = Self(1000252000);
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR: Self = Self(1000253000);
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT: Self = Self(1000254000);
    pub const PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT: Self = Self(1000254001);
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT: Self = Self(1000254002);
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: Self = Self(1000255000);
    pub const SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: Self = Self(1000255002);
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: Self = Self(1000255001);
    pub const HEADLESS_SURFACE_CREATE_INFO_EXT: Self = Self(1000256000);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR: Self = Self(1000257000);
    pub const BUFFER_DEVICE_ADDRESS_INFO_KHR: Self = Self(1000244001);
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR: Self = Self(1000257002);
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR: Self = Self(1000257003);
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR: Self = Self(1000257004);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT: Self = Self(1000259000);
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT: Self = Self(1000259001);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT: Self = Self(1000259002);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT: Self = Self(1000260000);
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT: Self = Self(1000261000);
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT: Self = Self(1000265000);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000267000);
    pub const PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR: Self = Self(1000269000);
    pub const PIPELINE_INFO_KHR: Self = Self(1000269001);
    pub const PIPELINE_EXECUTABLE_PROPERTIES_KHR: Self = Self(1000269002);
    pub const PIPELINE_EXECUTABLE_INFO_KHR: Self = Self(1000269003);
    pub const PIPELINE_EXECUTABLE_STATISTIC_KHR: Self = Self(1000269004);
    pub const PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR: Self = Self(1000269005);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT: Self = Self(1000270000);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES_EXT: Self = Self(1000270001);
    pub const MEMORY_TO_IMAGE_COPY_EXT: Self = Self(1000270002);
    pub const IMAGE_TO_MEMORY_COPY_EXT: Self = Self(1000270003);
    pub const COPY_IMAGE_TO_MEMORY_INFO_EXT: Self = Self(1000270004);
    pub const COPY_MEMORY_TO_IMAGE_INFO_EXT: Self = Self(1000270005);
    pub const HOST_IMAGE_LAYOUT_TRANSITION_INFO_EXT: Self = Self(1000270006);
    pub const COPY_IMAGE_TO_IMAGE_INFO_EXT: Self = Self(1000270007);
    pub const SUBRESOURCE_HOST_MEMCPY_SIZE_EXT: Self = Self(1000270008);
    pub const HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY_EXT: Self = Self(1000270009);
    pub const MEMORY_MAP_INFO_KHR: Self = Self(1000271000);
    pub const MEMORY_UNMAP_INFO_KHR: Self = Self(1000271001);
    pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT: Self = Self(1000272000);
    pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT: Self = Self(1000272001);
    pub const MEMORY_MAP_PLACED_INFO_EXT: Self = Self(1000272002);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT: Self = Self(1000273000);
    pub const SURFACE_PRESENT_MODE_EXT: Self = Self(1000274000);
    pub const SURFACE_PRESENT_SCALING_CAPABILITIES_EXT: Self = Self(1000274001);
    pub const SURFACE_PRESENT_MODE_COMPATIBILITY_EXT: Self = Self(1000274002);
    pub const PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT: Self = Self(1000275000);
    pub const SWAPCHAIN_PRESENT_FENCE_INFO_EXT: Self = Self(1000275001);
    pub const SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT: Self = Self(1000275002);
    pub const SWAPCHAIN_PRESENT_MODE_INFO_EXT: Self = Self(1000275003);
    pub const SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT: Self = Self(1000275004);
    pub const RELEASE_SWAPCHAIN_IMAGES_INFO_EXT: Self = Self(1000275005);
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT: Self = Self(1000276000);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV: Self = Self(1000277000);
    pub const GRAPHICS_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000277001);
    pub const GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV: Self = Self(1000277002);
    pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_NV: Self = Self(1000277003);
    pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV: Self = Self(1000277004);
    pub const GENERATED_COMMANDS_INFO_NV: Self = Self(1000277005);
    pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1000277006);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV: Self = Self(1000277007);
    pub const PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV: Self = Self(1000278000);
    pub const COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV: Self = Self(1000278001);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR: Self = Self(1000280000);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR: Self = Self(1000280001);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT: Self = Self(1000281000);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT: Self = Self(1000281001);
    pub const COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM: Self = Self(1000282000);
    pub const RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM: Self = Self(1000282001);
    pub const PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT: Self = Self(1000283000);
    pub const DEPTH_BIAS_INFO_EXT: Self = Self(1000283001);
    pub const DEPTH_BIAS_REPRESENTATION_INFO_EXT: Self = Self(1000283002);
    pub const PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT: Self = Self(1000284000);
    pub const DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT: Self = Self(1000284001);
    pub const DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT: Self = Self(1000284002);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT: Self = Self(1000286000);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT: Self = Self(1000286001);
    pub const SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT: Self = Self(1000287000);
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT: Self = Self(1000287001);
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT: Self = Self(1000287002);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT: Self = Self(1000288000);
    pub const PIPELINE_LIBRARY_CREATE_INFO_KHR: Self = Self(1000290000);
    pub const PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV: Self = Self(1000292000);
    pub const SURFACE_CAPABILITIES_PRESENT_BARRIER_NV: Self = Self(1000292001);
    pub const SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV: Self = Self(1000292002);
    pub const PRESENT_ID_KHR: Self = Self(1000294000);
    pub const PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR: Self = Self(1000294001);
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT: Self = Self(1000295000);
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO_EXT: Self = Self(1000295001);
    pub const PRIVATE_DATA_SLOT_CREATE_INFO_EXT: Self = Self(1000295002);
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT: Self = Self(1000297000);
    pub const VIDEO_ENCODE_INFO_KHR: Self = Self(1000299000);
    pub const VIDEO_ENCODE_RATE_CONTROL_INFO_KHR: Self = Self(1000299001);
    pub const VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000299002);
    pub const VIDEO_ENCODE_CAPABILITIES_KHR: Self = Self(1000299003);
    pub const VIDEO_ENCODE_USAGE_INFO_KHR: Self = Self(1000299004);
    pub const QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR: Self = Self(1000299005);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1000299006);
    pub const VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000299007);
    pub const VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1000299008);
    pub const VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000299009);
    pub const VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000299010);
    pub const PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV: Self = Self(1000300000);
    pub const DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV: Self = Self(1000300001);
    pub const PERF_HINT_INFO_QCOM: Self = Self(1000302000);
    pub const PHYSICAL_DEVICE_QUEUE_PERF_HINT_FEATURES_QCOM: Self = Self(1000302001);
    pub const PHYSICAL_DEVICE_QUEUE_PERF_HINT_PROPERTIES_QCOM: Self = Self(1000302002);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_3_FEATURES_QCOM: Self = Self(1000303000);
    pub const PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_FEATURES_QCOM: Self = Self(1000304000);
    pub const PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_PROPERTIES_QCOM: Self = Self(1000304001);
    pub const PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_FEATURES_EXT: Self = Self(1000305000);
    pub const PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_PROPERTIES_EXT: Self = Self(1000305001);
    pub const CUDA_MODULE_CREATE_INFO_NV: Self = Self(1000307000);
    pub const CUDA_FUNCTION_CREATE_INFO_NV: Self = Self(1000307001);
    pub const CUDA_LAUNCH_INFO_NV: Self = Self(1000307002);
    pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV: Self = Self(1000307003);
    pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV: Self = Self(1000307004);
    pub const REFRESH_OBJECT_LIST_KHR: Self = Self(1000308000);
    pub const PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM: Self = Self(1000309000);
    pub const PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM: Self = Self(1000309001);
    pub const RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM: Self = Self(1000309002);
    pub const PER_TILE_BEGIN_INFO_QCOM: Self = Self(1000309003);
    pub const PER_TILE_END_INFO_QCOM: Self = Self(1000309004);
    pub const DISPATCH_TILE_INFO_QCOM: Self = Self(1000309005);
    pub const QUERY_LOW_LATENCY_SUPPORT_NV: Self = Self(1000310000);
    pub const EXPORT_METAL_OBJECT_CREATE_INFO_EXT: Self = Self(1000311000);
    pub const EXPORT_METAL_OBJECTS_INFO_EXT: Self = Self(1000311001);
    pub const EXPORT_METAL_DEVICE_INFO_EXT: Self = Self(1000311002);
    pub const EXPORT_METAL_COMMAND_QUEUE_INFO_EXT: Self = Self(1000311003);
    pub const EXPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311004);
    pub const IMPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311005);
    pub const EXPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311006);
    pub const IMPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311007);
    pub const EXPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311008);
    pub const IMPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311009);
    pub const EXPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311010);
    pub const IMPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311011);
    pub const MEMORY_BARRIER_2_KHR: Self = Self(1000314000);
    pub const BUFFER_MEMORY_BARRIER_2_KHR: Self = Self(1000314001);
    pub const IMAGE_MEMORY_BARRIER_2_KHR: Self = Self(1000314002);
    pub const DEPENDENCY_INFO_KHR: Self = Self(1000314003);
    pub const SUBMIT_INFO_2_KHR: Self = Self(1000314004);
    pub const SEMAPHORE_SUBMIT_INFO_KHR: Self = Self(1000314005);
    pub const COMMAND_BUFFER_SUBMIT_INFO_KHR: Self = Self(1000314006);
    pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR: Self = Self(1000314007);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT: Self = Self(1000316000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT: Self = Self(1000316001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT: Self = Self(1000316002);
    pub const DESCRIPTOR_ADDRESS_INFO_EXT: Self = Self(1000316003);
    pub const DESCRIPTOR_GET_INFO_EXT: Self = Self(1000316004);
    pub const BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316005);
    pub const IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316006);
    pub const IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316007);
    pub const SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316008);
    pub const OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT: Self = Self(1000316010);
    pub const DESCRIPTOR_BUFFER_BINDING_INFO_EXT: Self = Self(1000316011);
    pub const DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT: Self = Self(1000316012);
    pub const ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316009);
    pub const DEVICE_MEMORY_COPY_KHR: Self = Self(1000318000);
    pub const COPY_DEVICE_MEMORY_INFO_KHR: Self = Self(1000318001);
    pub const DEVICE_MEMORY_IMAGE_COPY_KHR: Self = Self(1000318002);
    pub const COPY_DEVICE_MEMORY_IMAGE_INFO_KHR: Self = Self(1000318003);
    pub const MEMORY_RANGE_BARRIERS_INFO_KHR: Self = Self(1000318004);
    pub const MEMORY_RANGE_BARRIER_KHR: Self = Self(1000318005);
    pub const PHYSICAL_DEVICE_DEVICE_ADDRESS_COMMANDS_FEATURES_KHR: Self = Self(1000318006);
    pub const BIND_INDEX_BUFFER_3_INFO_KHR: Self = Self(1000318007);
    pub const BIND_VERTEX_BUFFER_3_INFO_KHR: Self = Self(1000318008);
    pub const DRAW_INDIRECT_2_INFO_KHR: Self = Self(1000318009);
    pub const DRAW_INDIRECT_COUNT_2_INFO_KHR: Self = Self(1000318010);
    pub const DISPATCH_INDIRECT_2_INFO_KHR: Self = Self(1000318011);
    pub const CONDITIONAL_RENDERING_BEGIN_INFO_2_EXT: Self = Self(1000318012);
    pub const BIND_TRANSFORM_FEEDBACK_BUFFER_2_INFO_EXT: Self = Self(1000318013);
    pub const MEMORY_MARKER_INFO_AMD: Self = Self(1000318014);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_2_KHR: Self = Self(1000318015);
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT: Self = Self(1000320000);
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT: Self = Self(1000320001);
    pub const GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT: Self = Self(1000320002);
    pub const PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD: Self = Self(1000321000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR: Self = Self(1000203000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR: Self = Self(1000322000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR: Self = Self(1000323000);
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR: Self = Self(1000325000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV: Self = Self(1000326000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV: Self = Self(1000326001);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV: Self = Self(1000326002);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV: Self = Self(1000327000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV: Self = Self(1000327001);
    pub const ACCELERATION_STRUCTURE_MOTION_INFO_NV: Self = Self(1000327002);
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT: Self = Self(1000328000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT: Self = Self(1000328001);
    pub const PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT: Self = Self(1000330000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT: Self = Self(1000332000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT: Self = Self(1000332001);
    pub const COPY_COMMAND_TRANSFORM_INFO_QCOM: Self = Self(1000333000);
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000335000);
    pub const PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR: Self = Self(1000336000);
    pub const COPY_BUFFER_INFO_2_KHR: Self = Self(1000337000);
    pub const COPY_IMAGE_INFO_2_KHR: Self = Self(1000337001);
    pub const COPY_BUFFER_TO_IMAGE_INFO_2_KHR: Self = Self(1000337002);
    pub const COPY_IMAGE_TO_BUFFER_INFO_2_KHR: Self = Self(1000337003);
    pub const BLIT_IMAGE_INFO_2_KHR: Self = Self(1000337004);
    pub const RESOLVE_IMAGE_INFO_2_KHR: Self = Self(1000337005);
    pub const BUFFER_COPY_2_KHR: Self = Self(1000337006);
    pub const IMAGE_COPY_2_KHR: Self = Self(1000337007);
    pub const IMAGE_BLIT_2_KHR: Self = Self(1000337008);
    pub const BUFFER_IMAGE_COPY_2_KHR: Self = Self(1000337009);
    pub const IMAGE_RESOLVE_2_KHR: Self = Self(1000337010);
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT: Self = Self(1000338000);
    pub const IMAGE_COMPRESSION_CONTROL_EXT: Self = Self(1000338001);
    pub const SUBRESOURCE_LAYOUT_2_EXT: Self = Self(1000338002);
    pub const IMAGE_SUBRESOURCE_2_EXT: Self = Self(1000338003);
    pub const IMAGE_COMPRESSION_PROPERTIES_EXT: Self = Self(1000338004);
    pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT: Self = Self(1000339000);
    pub const PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT: Self = Self(1000340000);
    pub const PHYSICAL_DEVICE_FAULT_FEATURES_EXT: Self = Self(1000341000);
    pub const DEVICE_FAULT_COUNTS_EXT: Self = Self(1000341001);
    pub const DEVICE_FAULT_INFO_EXT: Self = Self(1000341002);
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM: Self = Self(1000342000);
    pub const PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT: Self = Self(1000344000);
    pub const DIRECTFB_SURFACE_CREATE_INFO_EXT: Self = Self(1000346000);
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE: Self = Self(1000351000);
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE: Self = Self(1000351002);
    pub const PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000352000);
    pub const VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT: Self = Self(1000352001);
    pub const VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT: Self = Self(1000352002);
    pub const PHYSICAL_DEVICE_DRM_PROPERTIES_EXT: Self = Self(1000353000);
    pub const PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT: Self = Self(1000354000);
    pub const DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT: Self = Self(1000354001);
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT: Self = Self(1000355000);
    pub const PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT: Self = Self(1000355001);
    pub const PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT: Self = Self(1000356000);
    pub const FORMAT_PROPERTIES_3_KHR: Self = Self(1000360000);
    pub const PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_EXT: Self = Self(1000361000);
    pub const IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364000);
    pub const MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: Self = Self(1000364001);
    pub const MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364002);
    pub const IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365000);
    pub const SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365001);
    pub const BUFFER_COLLECTION_CREATE_INFO_FUCHSIA: Self = Self(1000366000);
    pub const IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366001);
    pub const BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA: Self = Self(1000366002);
    pub const BUFFER_COLLECTION_PROPERTIES_FUCHSIA: Self = Self(1000366003);
    pub const BUFFER_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366004);
    pub const BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA: Self = Self(1000366005);
    pub const IMAGE_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366006);
    pub const IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366007);
    pub const SYSMEM_COLOR_SPACE_FUCHSIA: Self = Self(1000366008);
    pub const BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366009);
    pub const SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI: Self = Self(1000369000);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI: Self = Self(1000369001);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI: Self = Self(1000369002);
    pub const PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI: Self = Self(1000370000);
    pub const MEMORY_GET_REMOTE_ADDRESS_INFO_NV: Self = Self(1000371000);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV: Self = Self(1000371001);
    pub const PIPELINE_PROPERTIES_IDENTIFIER_EXT: Self = Self(1000372000);
    pub const PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT: Self = Self(1000372001);
    pub const PIPELINE_INFO_EXT: Self = Self(1000269001);
    pub const IMPORT_FENCE_SCI_SYNC_INFO_NV: Self = Self(1000373000);
    pub const EXPORT_FENCE_SCI_SYNC_INFO_NV: Self = Self(1000373001);
    pub const FENCE_GET_SCI_SYNC_INFO_NV: Self = Self(1000373002);
    pub const SCI_SYNC_ATTRIBUTES_INFO_NV: Self = Self(1000373003);
    pub const IMPORT_SEMAPHORE_SCI_SYNC_INFO_NV: Self = Self(1000373004);
    pub const EXPORT_SEMAPHORE_SCI_SYNC_INFO_NV: Self = Self(1000373005);
    pub const SEMAPHORE_GET_SCI_SYNC_INFO_NV: Self = Self(1000373006);
    pub const PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_FEATURES_NV: Self = Self(1000373007);
    pub const IMPORT_MEMORY_SCI_BUF_INFO_NV: Self = Self(1000374000);
    pub const EXPORT_MEMORY_SCI_BUF_INFO_NV: Self = Self(1000374001);
    pub const MEMORY_GET_SCI_BUF_INFO_NV: Self = Self(1000374002);
    pub const MEMORY_SCI_BUF_PROPERTIES_NV: Self = Self(1000374003);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCI_BUF_FEATURES_NV: Self = Self(1000374004);
    pub const PHYSICAL_DEVICE_EXTERNAL_SCI_BUF_FEATURES_NV: Self = Self(1000374004);
    pub const PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT: Self = Self(1000375000);
    pub const FRAME_BOUNDARY_EXT: Self = Self(1000375001);
    pub const PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT: Self = Self(1000376000);
    pub const SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT: Self = Self(1000376001);
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT: Self = Self(1000376002);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT: Self = Self(1000377000);
    pub const SCREEN_SURFACE_CREATE_INFO_QNX: Self = Self(1000378000);
    pub const PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT: Self = Self(1000381000);
    pub const PIPELINE_COLOR_WRITE_CREATE_INFO_EXT: Self = Self(1000381001);
    pub const PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT: Self = Self(1000382000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000386000);
    pub const PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR: Self = Self(1000387000);
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT: Self = Self(1000388000);
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT: Self = Self(1000388001);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE: Self = Self(1000390000);
    pub const VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE: Self = Self(1000390001);
    pub const VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE: Self = Self(1000390002);
    pub const VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE: Self = Self(1000390003);
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT: Self = Self(1000391000);
    pub const IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT: Self = Self(1000391001);
    pub const PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT: Self = Self(1000392000);
    pub const PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT: Self = Self(1000392001);
    pub const PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT: Self = Self(1000393000);
    pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT: Self = Self(1000395000);
    pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT: Self = Self(1000395001);
    pub const MICROMAP_BUILD_INFO_EXT: Self = Self(1000396000);
    pub const MICROMAP_VERSION_INFO_EXT: Self = Self(1000396001);
    pub const COPY_MICROMAP_INFO_EXT: Self = Self(1000396002);
    pub const COPY_MICROMAP_TO_MEMORY_INFO_EXT: Self = Self(1000396003);
    pub const COPY_MEMORY_TO_MICROMAP_INFO_EXT: Self = Self(1000396004);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT: Self = Self(1000396005);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT: Self = Self(1000396006);
    pub const MICROMAP_CREATE_INFO_EXT: Self = Self(1000396007);
    pub const MICROMAP_BUILD_SIZES_INFO_EXT: Self = Self(1000396008);
    pub const ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT: Self = Self(1000396009);
    pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV: Self = Self(1000397000);
    pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV: Self = Self(1000397001);
    pub const ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV: Self = Self(1000397002);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI: Self = Self(1000404000);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI: Self = Self(1000404001);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI: Self = Self(1000404002);
    pub const PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT: Self = Self(1000411000);
    pub const SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT: Self = Self(1000411001);
    pub const PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT: Self = Self(1000412000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR: Self = Self(1000413000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR: Self = Self(1000413001);
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: Self = Self(1000413002);
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: Self = Self(1000413003);
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM: Self = Self(1000415000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR: Self = Self(1000416000);
    pub const DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM: Self = Self(1000417000);
    pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM: Self = Self(1000417001);
    pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM: Self = Self(1000417002);
    pub const DISPATCH_PARAMETERS_ARM: Self = Self(1000417003);
    pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_PROPERTIES_ARM: Self = Self(1000417004);
    pub const PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT: Self = Self(1000418000);
    pub const IMAGE_VIEW_SLICED_CREATE_INFO_EXT: Self = Self(1000418001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE: Self = Self(1000420000);
    pub const DESCRIPTOR_SET_BINDING_REFERENCE_VALVE: Self = Self(1000420001);
    pub const DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE: Self = Self(1000420002);
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT: Self = Self(1000421000);
    pub const PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT: Self = Self(1000422000);
    pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM: Self = Self(1000424000);
    pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM: Self = Self(1000424001);
    pub const RENDER_PASS_STRIPE_BEGIN_INFO_ARM: Self = Self(1000424002);
    pub const RENDER_PASS_STRIPE_INFO_ARM: Self = Self(1000424003);
    pub const RENDER_PASS_STRIPE_SUBMIT_INFO_ARM: Self = Self(1000424004);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM: Self = Self(1000425000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM: Self = Self(1000425001);
    pub const SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM: Self = Self(1000425002);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV: Self = Self(1000426000);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV: Self = Self(1000426001);
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV: Self = Self(1000427000);
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV: Self = Self(1000427001);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV: Self = Self(1000428000);
    pub const COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV: Self = Self(1000428001);
    pub const PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV: Self = Self(1000428002);
    pub const PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV: Self = Self(1000429008);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV: Self = Self(1000429009);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV: Self = Self(1000429010);
    pub const PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV: Self = Self(1000430000);
    pub const PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR: Self = Self(1000434000);
    pub const APPLICATION_PARAMETERS_EXT: Self = Self(1000435000);
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT: Self = Self(1000437000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM: Self = Self(1000440000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM: Self = Self(1000440001);
    pub const IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM: Self = Self(1000440002);
    pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT: Self = Self(1000451000);
    pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT: Self = Self(1000451001);
    pub const NATIVE_BUFFER_USAGE_OHOS: Self = Self(1000452000);
    pub const NATIVE_BUFFER_PROPERTIES_OHOS: Self = Self(1000452001);
    pub const NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS: Self = Self(1000452002);
    pub const IMPORT_NATIVE_BUFFER_INFO_OHOS: Self = Self(1000452003);
    pub const MEMORY_GET_NATIVE_BUFFER_INFO_OHOS: Self = Self(1000452004);
    pub const EXTERNAL_FORMAT_OHOS: Self = Self(1000452005);
    pub const EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT: Self = Self(1000453000);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT: Self = Self(1000455000);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT: Self = Self(1000455001);
    pub const PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT: Self = Self(1000458000);
    pub const RENDER_PASS_CREATION_CONTROL_EXT: Self = Self(1000458001);
    pub const RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458002);
    pub const RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458003);
    pub const DIRECT_DRIVER_LOADING_INFO_LUNARG: Self = Self(1000459000);
    pub const DIRECT_DRIVER_LOADING_LIST_LUNARG: Self = Self(1000459001);
    pub const TENSOR_CREATE_INFO_ARM: Self = Self(1000460000);
    pub const TENSOR_VIEW_CREATE_INFO_ARM: Self = Self(1000460001);
    pub const BIND_TENSOR_MEMORY_INFO_ARM: Self = Self(1000460002);
    pub const WRITE_DESCRIPTOR_SET_TENSOR_ARM: Self = Self(1000460003);
    pub const PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM: Self = Self(1000460004);
    pub const TENSOR_FORMAT_PROPERTIES_ARM: Self = Self(1000460005);
    pub const TENSOR_DESCRIPTION_ARM: Self = Self(1000460006);
    pub const TENSOR_MEMORY_REQUIREMENTS_INFO_ARM: Self = Self(1000460007);
    pub const TENSOR_MEMORY_BARRIER_ARM: Self = Self(1000460008);
    pub const PHYSICAL_DEVICE_TENSOR_FEATURES_ARM: Self = Self(1000460009);
    pub const DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM: Self = Self(1000460010);
    pub const COPY_TENSOR_INFO_ARM: Self = Self(1000460011);
    pub const TENSOR_COPY_ARM: Self = Self(1000460012);
    pub const TENSOR_DEPENDENCY_INFO_ARM: Self = Self(1000460013);
    pub const MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM: Self = Self(1000460014);
    pub const PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM: Self = Self(1000460015);
    pub const EXTERNAL_TENSOR_PROPERTIES_ARM: Self = Self(1000460016);
    pub const EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM: Self = Self(1000460017);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM: Self = Self(1000460018);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM: Self = Self(1000460019);
    pub const DESCRIPTOR_GET_TENSOR_INFO_ARM: Self = Self(1000460020);
    pub const TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM: Self = Self(1000460021);
    pub const TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM: Self = Self(1000460022);
    pub const FRAME_BOUNDARY_TENSORS_ARM: Self = Self(1000460023);
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT: Self = Self(1000462000);
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT: Self = Self(1000462001);
    pub const PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT: Self = Self(1000462002);
    pub const SHADER_MODULE_IDENTIFIER_EXT: Self = Self(1000462003);
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT: Self = Self(1000342000);
    pub const PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV: Self = Self(1000464000);
    pub const PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV: Self = Self(1000464001);
    pub const OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV: Self = Self(1000464002);
    pub const OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV: Self = Self(1000464003);
    pub const OPTICAL_FLOW_SESSION_CREATE_INFO_NV: Self = Self(1000464004);
    pub const OPTICAL_FLOW_EXECUTE_INFO_NV: Self = Self(1000464005);
    pub const OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV: Self = Self(1000464010);
    pub const PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT: Self = Self(1000465000);
    pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT: Self = Self(1000466000);
    pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID: Self = Self(1000468000);
    pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self = Self(1000468001);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self = Self(1000468002);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR: Self = Self(1000470000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES_KHR: Self = Self(1000470001);
    pub const RENDERING_AREA_INFO_KHR: Self = Self(1000470003);
    pub const DEVICE_IMAGE_SUBRESOURCE_INFO_KHR: Self = Self(1000470004);
    pub const SUBRESOURCE_LAYOUT_2_KHR: Self = Self(1000338002);
    pub const IMAGE_SUBRESOURCE_2_KHR: Self = Self(1000338003);
    pub const PIPELINE_CREATE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1000470005);
    pub const BUFFER_USAGE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1000470006);
    pub const PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD: Self = Self(1000476000);
    pub const ANTI_LAG_DATA_AMD: Self = Self(1000476001);
    pub const ANTI_LAG_PRESENTATION_INFO_AMD: Self = Self(1000476002);
    pub const PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX: Self = Self(1000478000);
    pub const ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX: Self = Self(1000478001);
    pub const SURFACE_CAPABILITIES_PRESENT_ID_2_KHR: Self = Self(1000479000);
    pub const PRESENT_ID_2_KHR: Self = Self(1000479001);
    pub const PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR: Self = Self(1000479002);
    pub const SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR: Self = Self(1000480000);
    pub const PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR: Self = Self(1000480001);
    pub const PRESENT_WAIT_2_INFO_KHR: Self = Self(1000480002);
    pub const PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR: Self = Self(1000481000);
    pub const PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT: Self = Self(1000482000);
    pub const PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT: Self = Self(1000482001);
    pub const SHADER_CREATE_INFO_EXT: Self = Self(1000482002);
    pub const SHADER_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self = Self(1000225001);
    pub const PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR: Self = Self(1000483000);
    pub const PIPELINE_BINARY_CREATE_INFO_KHR: Self = Self(1000483001);
    pub const PIPELINE_BINARY_INFO_KHR: Self = Self(1000483002);
    pub const PIPELINE_BINARY_KEY_KHR: Self = Self(1000483003);
    pub const PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR: Self = Self(1000483004);
    pub const RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR: Self = Self(1000483005);
    pub const PIPELINE_BINARY_DATA_INFO_KHR: Self = Self(1000483006);
    pub const PIPELINE_CREATE_INFO_KHR: Self = Self(1000483007);
    pub const DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR: Self = Self(1000483008);
    pub const PIPELINE_BINARY_HANDLES_INFO_KHR: Self = Self(1000483009);
    pub const PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM: Self = Self(1000484000);
    pub const TILE_PROPERTIES_QCOM: Self = Self(1000484001);
    pub const PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC: Self = Self(1000485000);
    pub const AMIGO_PROFILING_SUBMIT_INFO_SEC: Self = Self(1000485001);
    pub const SURFACE_PRESENT_MODE_KHR: Self = Self(1000274000);
    pub const SURFACE_PRESENT_SCALING_CAPABILITIES_KHR: Self = Self(1000274001);
    pub const SURFACE_PRESENT_MODE_COMPATIBILITY_KHR: Self = Self(1000274002);
    pub const PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000275000);
    pub const SWAPCHAIN_PRESENT_FENCE_INFO_KHR: Self = Self(1000275001);
    pub const SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR: Self = Self(1000275002);
    pub const SWAPCHAIN_PRESENT_MODE_INFO_KHR: Self = Self(1000275003);
    pub const SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR: Self = Self(1000275004);
    pub const RELEASE_SWAPCHAIN_IMAGES_INFO_KHR: Self = Self(1000275005);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM: Self = Self(1000488000);
    pub const SEMAPHORE_SCI_SYNC_POOL_CREATE_INFO_NV: Self = Self(1000489000);
    pub const SEMAPHORE_SCI_SYNC_CREATE_INFO_NV: Self = Self(1000489001);
    pub const PHYSICAL_DEVICE_EXTERNAL_SCI_SYNC_2_FEATURES_NV: Self = Self(1000489002);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV: Self = Self(1000490000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV: Self = Self(1000490001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV: Self = Self(1000491000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV: Self = Self(1000491001);
    pub const COOPERATIVE_VECTOR_PROPERTIES_NV: Self = Self(1000491002);
    pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV: Self = Self(1000491004);
    pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV: Self = Self(1000492000);
    pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV: Self = Self(1000492001);
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT: Self = Self(1000351000);
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT: Self = Self(1000351002);
    pub const PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT: Self = Self(1000495000);
    pub const PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT: Self = Self(1000495001);
    pub const LAYER_SETTINGS_CREATE_INFO_EXT: Self = Self(1000496000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM: Self = Self(1000497000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM: Self = Self(1000497001);
    pub const PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT: Self = Self(1000498000);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT: Self = Self(1000499000);
    pub const PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR: Self = Self(1000504000);
    pub const LATENCY_SLEEP_MODE_INFO_NV: Self = Self(1000505000);
    pub const LATENCY_SLEEP_INFO_NV: Self = Self(1000505001);
    pub const SET_LATENCY_MARKER_INFO_NV: Self = Self(1000505002);
    pub const GET_LATENCY_MARKER_INFO_NV: Self = Self(1000505003);
    pub const LATENCY_TIMINGS_FRAME_REPORT_NV: Self = Self(1000505004);
    pub const LATENCY_SUBMISSION_PRESENT_ID_NV: Self = Self(1000505005);
    pub const OUT_OF_BAND_QUEUE_TYPE_INFO_NV: Self = Self(1000505006);
    pub const SWAPCHAIN_LATENCY_CREATE_INFO_NV: Self = Self(1000505007);
    pub const LATENCY_SURFACE_CAPABILITIES_NV: Self = Self(1000505008);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR: Self = Self(1000506000);
    pub const COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1000506001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1000506002);
    pub const DATA_GRAPH_PIPELINE_CREATE_INFO_ARM: Self = Self(1000507000);
    pub const DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM: Self = Self(1000507001);
    pub const DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM: Self = Self(1000507002);
    pub const DATA_GRAPH_PIPELINE_CONSTANT_ARM: Self = Self(1000507003);
    pub const DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM: Self = Self(1000507004);
    pub const BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM: Self = Self(1000507005);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM: Self = Self(1000507006);
    pub const DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM: Self = Self(1000507007);
    pub const DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM: Self = Self(1000507008);
    pub const DATA_GRAPH_PIPELINE_INFO_ARM: Self = Self(1000507009);
    pub const DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM: Self = Self(1000507010);
    pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM: Self = Self(1000507011);
    pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM: Self = Self(1000507012);
    pub const DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM: Self = Self(1000507013);
    pub const DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM: Self = Self(1000507014);
    pub const DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM: Self = Self(1000507016);
    pub const QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM: Self = Self(1000507017);
    pub const QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM: Self = Self(1000507018);
    pub const PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM: Self = Self(1000507019);
    pub const DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM: Self = Self(1000507015);
    pub const QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM: Self = Self(1000508000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM: Self = Self(1000510000);
    pub const MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM: Self = Self(1000510001);
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR: Self = Self(1000201000);
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR: Self = Self(1000511000);
    pub const VIDEO_DECODE_AV1_CAPABILITIES_KHR: Self = Self(1000512000);
    pub const VIDEO_DECODE_AV1_PICTURE_INFO_KHR: Self = Self(1000512001);
    pub const VIDEO_DECODE_AV1_PROFILE_INFO_KHR: Self = Self(1000512003);
    pub const VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000512004);
    pub const VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR: Self = Self(1000512005);
    pub const VIDEO_ENCODE_AV1_CAPABILITIES_KHR: Self = Self(1000513000);
    pub const VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000513001);
    pub const VIDEO_ENCODE_AV1_PICTURE_INFO_KHR: Self = Self(1000513002);
    pub const VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR: Self = Self(1000513003);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR: Self = Self(1000513004);
    pub const VIDEO_ENCODE_AV1_PROFILE_INFO_KHR: Self = Self(1000513005);
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR: Self = Self(1000513006);
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000513007);
    pub const VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000513008);
    pub const VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR: Self = Self(1000513009);
    pub const VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000513010);
    pub const PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR: Self = Self(1000514000);
    pub const VIDEO_DECODE_VP9_CAPABILITIES_KHR: Self = Self(1000514001);
    pub const VIDEO_DECODE_VP9_PICTURE_INFO_KHR: Self = Self(1000514002);
    pub const VIDEO_DECODE_VP9_PROFILE_INFO_KHR: Self = Self(1000514003);
    pub const PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000515000);
    pub const VIDEO_INLINE_QUERY_INFO_KHR: Self = Self(1000515001);
    pub const PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV: Self = Self(1000516000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM: Self = Self(1000518000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM: Self = Self(1000518001);
    pub const SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM: Self = Self(1000518002);
    pub const SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM: Self = Self(1000519000);
    pub const PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM: Self = Self(1000519001);
    pub const BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM: Self = Self(1000519002);
    pub const PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM: Self = Self(1000520000);
    pub const SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM: Self = Self(1000520001);
    pub const PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM: Self = Self(1000521000);
    pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000524000);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR: Self = Self(1000525000);
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR: Self = Self(1000190001);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR: Self = Self(1000190002);
    pub const PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR: Self = Self(1000527000);
    pub const ATTACHMENT_FEEDBACK_LOOP_INFO_EXT: Self = Self(1000527001);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR: Self = Self(1000528000);
    pub const SCREEN_BUFFER_PROPERTIES_QNX: Self = Self(1000529000);
    pub const SCREEN_BUFFER_FORMAT_PROPERTIES_QNX: Self = Self(1000529001);
    pub const IMPORT_SCREEN_BUFFER_INFO_QNX: Self = Self(1000529002);
    pub const EXTERNAL_FORMAT_QNX: Self = Self(1000529003);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX: Self = Self(1000529004);
    pub const PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT: Self = Self(1000530000);
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR: Self = Self(1000265000);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR: Self = Self(1000259000);
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR: Self = Self(1000259001);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR: Self = Self(1000259002);
    pub const CALIBRATED_TIMESTAMP_INFO_KHR: Self = Self(1000184000);
    pub const PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR: Self = Self(1000544000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR: Self = Self(1000545000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES_KHR: Self = Self(1000545001);
    pub const BIND_MEMORY_STATUS_KHR: Self = Self(1000545002);
    pub const BIND_DESCRIPTOR_SETS_INFO_KHR: Self = Self(1000545003);
    pub const PUSH_CONSTANTS_INFO_KHR: Self = Self(1000545004);
    pub const PUSH_DESCRIPTOR_SET_INFO_KHR: Self = Self(1000545005);
    pub const PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO_KHR: Self = Self(1000545006);
    pub const SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT: Self = Self(1000545007);
    pub const BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT: Self = Self(1000545008);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV: Self = Self(1000546000);
    pub const PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM: Self = Self(1000547000);
    pub const PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM: Self = Self(1000547001);
    pub const TILE_MEMORY_REQUIREMENTS_QCOM: Self = Self(1000547002);
    pub const TILE_MEMORY_BIND_INFO_QCOM: Self = Self(1000547003);
    pub const TILE_MEMORY_SIZE_INFO_QCOM: Self = Self(1000547004);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR: Self = Self(1000549000);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR: Self = Self(1000426001);
    pub const COPY_MEMORY_INDIRECT_INFO_KHR: Self = Self(1000549002);
    pub const COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR: Self = Self(1000549003);
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT: Self = Self(1000427000);
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT: Self = Self(1000427001);
    pub const DECOMPRESS_MEMORY_INFO_EXT: Self = Self(1000550002);
    pub const DISPLAY_SURFACE_STEREO_CREATE_INFO_NV: Self = Self(1000551000);
    pub const DISPLAY_MODE_STEREO_PROPERTIES_NV: Self = Self(1000551001);
    pub const VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR: Self = Self(1000552000);
    pub const VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR: Self = Self(1000552001);
    pub const VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR: Self = Self(1000552002);
    pub const VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR: Self = Self(1000552003);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR: Self = Self(1000552004);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553000);
    pub const VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553001);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR: Self = Self(1000553002);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000553005);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR: Self = Self(1000553009);
    pub const VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553003);
    pub const VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553004);
    pub const VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553006);
    pub const VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553007);
    pub const VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553008);
    pub const PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV: Self = Self(1000555000);
    pub const EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV: Self = Self(1000556000);
    pub const EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV: Self = Self(1000556001);
    pub const EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV: Self = Self(1000556002);
    pub const PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV: Self = Self(1000556003);
    pub const PHYSICAL_DEVICE_SHADER_RELAXED_EXTENDED_INSTRUCTION_FEATURES_KHR: Self = Self(1000558000);
    pub const PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV: Self = Self(1000559000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR: Self = Self(1000562000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR: Self = Self(1000562001);
    pub const PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR: Self = Self(1000562002);
    pub const PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR: Self = Self(1000562003);
    pub const PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR: Self = Self(1000562004);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV: Self = Self(1000563000);
    pub const PHYSICAL_DEVICE_SHADER_REPLICATED_COMPOSITES_FEATURES_EXT: Self = Self(1000564000);
    pub const TENSOR_EXPLICIT_TILING_FORMAT_PROPERTIES_ARM: Self = Self(1000565000);
    pub const TENSOR_ROLLING_BACKING_CREATE_INFO_ARM: Self = Self(1000565001);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT: Self = Self(1000567000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV: Self = Self(1000568000);
    pub const PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV: Self = Self(1000569000);
    pub const PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV: Self = Self(1000569001);
    pub const CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV: Self = Self(1000569002);
    pub const CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV: Self = Self(1000569003);
    pub const CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV: Self = Self(1000569004);
    pub const CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV: Self = Self(1000569005);
    pub const CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV: Self = Self(1000569006);
    pub const RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self = Self(1000569007);
    pub const PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV: Self = Self(1000570000);
    pub const PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV: Self = Self(1000570001);
    pub const WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV: Self = Self(1000570002);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV: Self = Self(1000570003);
    pub const BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1000570004);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV: Self = Self(1000570005);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT: Self = Self(1000572000);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT: Self = Self(1000572001);
    pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT: Self = Self(1000572002);
    pub const INDIRECT_EXECUTION_SET_CREATE_INFO_EXT: Self = Self(1000572003);
    pub const GENERATED_COMMANDS_INFO_EXT: Self = Self(1000572004);
    pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT: Self = Self(1000572006);
    pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT: Self = Self(1000572007);
    pub const WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT: Self = Self(1000572008);
    pub const WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT: Self = Self(1000572009);
    pub const INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT: Self = Self(1000572010);
    pub const INDIRECT_EXECUTION_SET_SHADER_INFO_EXT: Self = Self(1000572011);
    pub const INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT: Self = Self(1000572012);
    pub const GENERATED_COMMANDS_PIPELINE_INFO_EXT: Self = Self(1000572013);
    pub const GENERATED_COMMANDS_SHADER_INFO_EXT: Self = Self(1000572014);
    pub const PHYSICAL_DEVICE_FAULT_FEATURES_KHR: Self = Self(1000573000);
    pub const PHYSICAL_DEVICE_FAULT_PROPERTIES_KHR: Self = Self(1000573001);
    pub const DEVICE_FAULT_INFO_KHR: Self = Self(1000573002);
    pub const DEVICE_FAULT_DEBUG_INFO_KHR: Self = Self(1000573003);
    pub const PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR: Self = Self(1000574000);
    pub const MEMORY_BARRIER_ACCESS_FLAGS_3_KHR: Self = Self(1000574002);
    pub const PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA: Self = Self(1000575000);
    pub const PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA: Self = Self(1000575001);
    pub const IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA: Self = Self(1000575002);
    pub const PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR: Self = Self(1000579000);
    pub const PUSH_CONSTANT_BANK_INFO_NV: Self = Self(1000580000);
    pub const PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV: Self = Self(1000580001);
    pub const PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV: Self = Self(1000580002);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT: Self = Self(1000581000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT: Self = Self(1000581001);
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT: Self = Self(1000582000);
    pub const PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT: Self = Self(1000582001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR: Self = Self(1000584000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR: Self = Self(1000584001);
    pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR: Self = Self(1000584002);
    pub const PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR: Self = Self(1000586000);
    pub const VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586001);
    pub const VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586002);
    pub const VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586003);
    pub const SURFACE_CREATE_INFO_OHOS: Self = Self(1000685000);
    pub const NATIVE_BUFFER_OHOS: Self = Self(1000453001);
    pub const SWAPCHAIN_IMAGE_CREATE_INFO_OHOS: Self = Self(1000453002);
    pub const PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_OHOS: Self = Self(1000453003);
    pub const PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI: Self = Self(1000590000);
    pub const HDR_VIVID_DYNAMIC_METADATA_HUAWEI: Self = Self(1000590001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV: Self = Self(1000593000);
    pub const COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV: Self = Self(1000593001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV: Self = Self(1000593002);
    pub const PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM: Self = Self(1000596000);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_FEEDBACK_2_FEATURES_KHR: Self = Self(1000598000);
    pub const VIDEO_ENCODE_FEEDBACK_2_CAPABILITIES_KHR: Self = Self(1000598001);
    pub const QUERY_POOL_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_CREATE_INFO_KHR: Self = Self(1000598002);
    pub const IMPORT_MEMORY_METAL_HANDLE_INFO_EXT: Self = Self(1000602000);
    pub const MEMORY_METAL_HANDLE_PROPERTIES_EXT: Self = Self(1000602001);
    pub const MEMORY_GET_METAL_HANDLE_INFO_EXT: Self = Self(1000602002);
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR: Self = Self(1000421000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM: Self = Self(1000605000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM: Self = Self(1000605001);
    pub const PERFORMANCE_COUNTER_ARM: Self = Self(1000605002);
    pub const PERFORMANCE_COUNTER_DESCRIPTION_ARM: Self = Self(1000605003);
    pub const RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM: Self = Self(1000605004);
    pub const PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_FEATURES_ARM: Self = Self(1000607000);
    pub const PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_PROPERTIES_ARM: Self = Self(1000607001);
    pub const SHADER_INSTRUMENTATION_CREATE_INFO_ARM: Self = Self(1000607002);
    pub const SHADER_INSTRUMENTATION_METRIC_DESCRIPTION_ARM: Self = Self(1000607003);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000608000);
    pub const PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM: Self = Self(1000609000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE: Self = Self(1000611000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE: Self = Self(1000611001);
    pub const PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE: Self = Self(1000611002);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR: Self = Self(1000286000);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR: Self = Self(1000286001);
    pub const SET_PRESENT_CONFIG_NV: Self = Self(1000613000);
    pub const PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV: Self = Self(1000613001);
    pub const PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SWAPCHAIN_FEATURES_EXT: Self = Self(1000616000);
    pub const SWAPCHAIN_FLAGS_SURFACE_CAPABILITIES_EXT: Self = Self(1000616001);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT: Self = Self(1000425000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT: Self = Self(1000425001);
    pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT: Self = Self(1000425002);
    pub const RENDERING_END_INFO_EXT: Self = Self(1000619003);
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT: Self = Self(1000620000);
    pub const PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR: Self = Self(1000361000);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_KHR: Self = Self(1000623000);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_KHR: Self = Self(1000623001);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_MICROMAP_DATA_KHR: Self = Self(1000623002);
    pub const ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_KHR: Self = Self(1000623003);
    pub const PHYSICAL_DEVICE_SHADER_64_BIT_INDEXING_FEATURES_EXT: Self = Self(1000627000);
    pub const PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT: Self = Self(1000628000);
    pub const BEGIN_CUSTOM_RESOLVE_INFO_EXT: Self = Self(1000628001);
    pub const CUSTOM_RESOLVE_CREATE_INFO_EXT: Self = Self(1000628002);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM: Self = Self(1000629000);
    pub const DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM: Self = Self(1000629001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR: Self = Self(1000630000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR: Self = Self(1000630001);
    pub const RENDERING_ATTACHMENT_FLAGS_INFO_KHR: Self = Self(1000630002);
    pub const RENDERING_END_INFO_KHR: Self = Self(1000619003);
    pub const RESOLVE_IMAGE_MODE_INFO_KHR: Self = Self(1000630004);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_OPTICAL_FLOW_FEATURES_ARM: Self = Self(1000631000);
    pub const QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_PROPERTIES_ARM: Self = Self(1000631001);
    pub const DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_INFO_ARM: Self = Self(1000631003);
    pub const DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_ARM: Self = Self(1000631004);
    pub const DATA_GRAPH_PIPELINE_OPTICAL_FLOW_DISPATCH_INFO_ARM: Self = Self(1000631005);
    pub const DATA_GRAPH_PIPELINE_OPTICAL_FLOW_CREATE_INFO_ARM: Self = Self(1000631002);
    pub const DATA_GRAPH_PIPELINE_RESOURCE_INFO_IMAGE_LAYOUT_ARM: Self = Self(1000631006);
    pub const DATA_GRAPH_PIPELINE_SINGLE_NODE_CREATE_INFO_ARM: Self = Self(1000631007);
    pub const DATA_GRAPH_PIPELINE_SINGLE_NODE_CONNECTION_ARM: Self = Self(1000631008);
    pub const PHYSICAL_DEVICE_SHADER_LONG_VECTOR_FEATURES_EXT: Self = Self(1000635000);
    pub const PHYSICAL_DEVICE_SHADER_LONG_VECTOR_PROPERTIES_EXT: Self = Self(1000635001);
    pub const PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC: Self = Self(1000637000);
    pub const PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT: Self = Self(1000642000);
    pub const COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV: Self = Self(1000645000);
    pub const PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV: Self = Self(1000645001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_11_FEATURES_KHR: Self = Self(1000657000);
    pub const QUEUE_FAMILY_OPTIMAL_IMAGE_TRANSFER_GRANULARITY_PROPERTIES_KHR: Self = Self(1000657001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_MAINTENANCE_1_FEATURES_EXT: Self = Self(1000659000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_INFO_2_EXT: Self = Self(1000659001);
    pub const COOPERATIVE_MATRIX_PROPERTIES_2_EXT: Self = Self(1000659002);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT: Self = Self(1000662000);
    pub const UBM_SURFACE_CREATE_INFO_SEC: Self = Self(1000664000);
    pub const FORMAT_PROPERTIES_4_KHR: Self = Self(1000668000);
    pub const IMAGE_CREATE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1000668001);
    pub const IMAGE_USAGE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1000668002);
    pub const IMAGE_VIEW_USAGE_2_CREATE_INFO_KHR: Self = Self(1000668003);
    pub const PHYSICAL_DEVICE_EXTENDED_FLAGS_FEATURES_KHR: Self = Self(1000668004);
    pub const IMAGE_STENCIL_USAGE_2_CREATE_INFO_KHR: Self = Self(1000668005);
    pub const SHARED_PRESENT_SURFACE_CAPABILITIES_2_KHR: Self = Self(1000668006);
    pub const PHYSICAL_DEVICE_SHADER_OCP_MICROSCALING_TYPES_FEATURES_EXT: Self = Self(1000672000);
    pub const PHYSICAL_DEVICE_SHADER_MIXED_FLOAT_DOT_PRODUCT_FEATURES_VALVE: Self = Self(1000673000);
    pub const PHYSICAL_DEVICE_THROTTLE_HINT_FEATURES_SEC: Self = Self(1000674000);
    pub const THROTTLE_HINT_SUBMIT_INFO_SEC: Self = Self(1000674001);
    pub const DATA_GRAPH_PIPELINE_NEURAL_STATISTICS_CREATE_INFO_ARM: Self = Self(1000676000);
    pub const DATA_GRAPH_PIPELINE_SESSION_NEURAL_STATISTICS_CREATE_INFO_ARM: Self = Self(1000676001);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_FEATURES_ARM: Self = Self(1000676002);
    pub const PHYSICAL_DEVICE_PRIMITIVE_RESTART_INDEX_FEATURES_EXT: Self = Self(1000678000);
    pub const PHYSICAL_DEVICE_IMAGE_TILING_CONTROL_FEATURES_EXT: Self = Self(1000687000);
    pub const IMAGE_TILING_CONTROL_CREATE_INFO_EXT: Self = Self(1000687001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_DECODE_VECTOR_FEATURES_NV: Self = Self(1000689000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSubgroupFeatureFlagBits(pub u32);

impl VkSubgroupFeatureFlagBits {
    pub const BASIC_BIT: Self = Self(1);
    pub const VOTE_BIT: Self = Self(2);
    pub const ARITHMETIC_BIT: Self = Self(4);
    pub const BALLOT_BIT: Self = Self(8);
    pub const SHUFFLE_BIT: Self = Self(16);
    pub const SHUFFLE_RELATIVE_BIT: Self = Self(32);
    pub const CLUSTERED_BIT: Self = Self(64);
    pub const QUAD_BIT: Self = Self(128);
    pub const ROTATE_BIT: Self = Self(512);
    pub const ROTATE_CLUSTERED_BIT: Self = Self(1024);
    pub const PARTITIONED_BIT_NV: Self = Self(256);
    pub const ROTATE_BIT_KHR: Self = Self(512);
    pub const ROTATE_CLUSTERED_BIT_KHR: Self = Self(1024);
    pub const PARTITIONED_BIT_EXT: Self = Self(256);
}

impl core::ops::BitOr for VkSubgroupFeatureFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSubmitFlagBits(pub u32);

impl VkSubmitFlagBits {
    pub const PROTECTED_BIT: Self = Self(1);
    pub const PROTECTED_BIT_KHR: Self = Self(1);
}

impl core::ops::BitOr for VkSubmitFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSubpassContents(pub i32);

impl VkSubpassContents {
    pub const INLINE: Self = Self(0);
    pub const SECONDARY_COMMAND_BUFFERS: Self = Self(1);
    pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS_EXT: Self = Self(1000451000);
    pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR: Self = Self(1000451000);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSubpassDescriptionFlagBits(pub u32);

impl VkSubpassDescriptionFlagBits {
    pub const PER_VIEW_ATTRIBUTES_BIT_NVX: Self = Self(1);
    pub const PER_VIEW_POSITION_X_ONLY_BIT_NVX: Self = Self(2);
    pub const FRAGMENT_REGION_BIT_QCOM: Self = Self(4);
    pub const SHADER_RESOLVE_BIT_QCOM: Self = Self(8);
    pub const TILE_SHADING_APRON_BIT_QCOM: Self = Self(256);
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM: Self = Self(16);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM: Self = Self(32);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM: Self = Self(64);
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT: Self = Self(16);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT: Self = Self(32);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT: Self = Self(64);
    pub const ENABLE_LEGACY_DITHERING_BIT_EXT: Self = Self(128);
    pub const FRAGMENT_REGION_BIT_EXT: Self = Self(4);
    pub const CUSTOM_RESOLVE_BIT_EXT: Self = Self(8);
}

impl core::ops::BitOr for VkSubpassDescriptionFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSurfaceTransformFlagBitsKHR(pub u32);

impl VkSurfaceTransformFlagBitsKHR {
    pub const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR: Self = Self(1);
    pub const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: Self = Self(2);
    pub const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: Self = Self(4);
    pub const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: Self = Self(8);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: Self = Self(16);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: Self = Self(32);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: Self = Self(64);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: Self = Self(128);
    pub const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR: Self = Self(256);
}

impl core::ops::BitOr for VkSurfaceTransformFlagBitsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSwapchainCreateFlagBitsKHR(pub u32);

impl VkSwapchainCreateFlagBitsKHR {
    pub const VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: Self = Self(1);
    pub const VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR: Self = Self(2);
    pub const VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR: Self = Self(4);
    pub const VK_SWAPCHAIN_CREATE_PRESENT_TIMING_BIT_EXT: Self = Self(512);
    pub const VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_EXT: Self = Self(8);
    pub const VK_SWAPCHAIN_CREATE_RESERVED_5_BIT_EXT: Self = Self(32);
    pub const VK_SWAPCHAIN_CREATE_RESERVED_4_BIT_EXT: Self = Self(16);
    pub const VK_SWAPCHAIN_CREATE_PRESENT_ID_2_BIT_KHR: Self = Self(64);
    pub const VK_SWAPCHAIN_CREATE_PRESENT_WAIT_2_BIT_KHR: Self = Self(128);
    pub const VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_KHR: Self = Self(8);
    pub const VK_SWAPCHAIN_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT: Self = Self(256);
    pub const VK_SWAPCHAIN_CREATE_RESERVED_10_BIT_HUAWEI: Self = Self(1024);
}

impl core::ops::BitOr for VkSwapchainCreateFlagBitsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkSystemAllocationScope(pub i32);

impl VkSystemAllocationScope {
    pub const COMMAND: Self = Self(0);
    pub const OBJECT: Self = Self(1);
    pub const CACHE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const INSTANCE: Self = Self(4);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkTessellationDomainOrigin(pub i32);

impl VkTessellationDomainOrigin {
    pub const UPPER_LEFT: Self = Self(0);
    pub const LOWER_LEFT: Self = Self(1);
    pub const UPPER_LEFT_KHR: Self = Self(0);
    pub const LOWER_LEFT_KHR: Self = Self(1);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkToolPurposeFlagBits(pub u32);

impl VkToolPurposeFlagBits {
    pub const VALIDATION_BIT: Self = Self(1);
    pub const PROFILING_BIT: Self = Self(2);
    pub const TRACING_BIT: Self = Self(4);
    pub const ADDITIONAL_FEATURES_BIT: Self = Self(8);
    pub const MODIFYING_FEATURES_BIT: Self = Self(16);
    pub const VALIDATION_BIT_EXT: Self = Self(1);
    pub const PROFILING_BIT_EXT: Self = Self(2);
    pub const TRACING_BIT_EXT: Self = Self(4);
    pub const ADDITIONAL_FEATURES_BIT_EXT: Self = Self(8);
    pub const MODIFYING_FEATURES_BIT_EXT: Self = Self(16);
    pub const DEBUG_REPORTING_BIT_EXT: Self = Self(32);
    pub const DEBUG_MARKERS_BIT_EXT: Self = Self(64);
}

impl core::ops::BitOr for VkToolPurposeFlagBits {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkVendorId(pub i32);

impl VkVendorId {
    pub const KHRONOS: Self = Self(65536);
    pub const VIV: Self = Self(65537);
    pub const VSI: Self = Self(65538);
    pub const KAZAN: Self = Self(65539);
    pub const CODEPLAY: Self = Self(65540);
    pub const MESA: Self = Self(65541);
    pub const POCL: Self = Self(65542);
    pub const MOBILEYE: Self = Self(65543);
    pub const APE: Self = Self(65544);
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VkVertexInputRate(pub i32);

impl VkVertexInputRate {
    pub const VERTEX: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
}

// ---------------------------------------------------------------
// Bitmasks
// ---------------------------------------------------------------

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkFramebufferCreateFlags(pub u32);

impl VkFramebufferCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const IMAGELESS_BIT: Self = Self(1);
    pub const IMAGELESS_BIT_KHR: Self = Self(1);
}

impl From<VkFramebufferCreateFlagBits> for VkFramebufferCreateFlags {
    #[inline]
    fn from(bits: VkFramebufferCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkFramebufferCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkQueryPoolCreateFlags(pub u32);

impl VkQueryPoolCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const RESET_BIT_KHR: Self = Self(1);
}

impl From<VkQueryPoolCreateFlagBits> for VkQueryPoolCreateFlags {
    #[inline]
    fn from(bits: VkQueryPoolCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkQueryPoolCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkRenderPassCreateFlags(pub u32);

impl VkRenderPassCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const RESERVED_3_BIT_IMG: Self = Self(8);
    pub const RESERVED_0_BIT_KHR: Self = Self(1);
    pub const TRANSFORM_BIT_QCOM: Self = Self(2);
    pub const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE: Self = Self(4);
}

impl From<VkRenderPassCreateFlagBits> for VkRenderPassCreateFlags {
    #[inline]
    fn from(bits: VkRenderPassCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkRenderPassCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkSamplerCreateFlags(pub u32);

impl VkSamplerCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const SUBSAMPLED_BIT_EXT: Self = Self(1);
    pub const SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT: Self = Self(2);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(8);
    pub const NON_SEAMLESS_CUBE_MAP_BIT_EXT: Self = Self(4);
    pub const IMAGE_PROCESSING_BIT_QCOM: Self = Self(16);
}

impl From<VkSamplerCreateFlagBits> for VkSamplerCreateFlags {
    #[inline]
    fn from(bits: VkSamplerCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkSamplerCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineLayoutCreateFlags(pub u32);

impl VkPipelineLayoutCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const INDEPENDENT_SETS_BIT_EXT: Self = Self(2);
    pub const NO_TASK_SHADER_BIT_KHR: Self = Self(4);
}

impl From<VkPipelineLayoutCreateFlagBits> for VkPipelineLayoutCreateFlags {
    #[inline]
    fn from(bits: VkPipelineLayoutCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkPipelineLayoutCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineCacheCreateFlags(pub u32);

impl VkPipelineCacheCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const EXTERNALLY_SYNCHRONIZED_BIT: Self = Self(1);
    pub const EXTERNALLY_SYNCHRONIZED_BIT_EXT: Self = Self(1);
    pub const INTERNALLY_SYNCHRONIZED_MERGE_BIT_KHR: Self = Self(8);
}

impl From<VkPipelineCacheCreateFlagBits> for VkPipelineCacheCreateFlags {
    #[inline]
    fn from(bits: VkPipelineCacheCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkPipelineCacheCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineDepthStencilStateCreateFlags(pub u32);

impl VkPipelineDepthStencilStateCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM: Self = Self(1);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM: Self = Self(2);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT: Self = Self(1);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT: Self = Self(2);
}

impl From<VkPipelineDepthStencilStateCreateFlagBits> for VkPipelineDepthStencilStateCreateFlags {
    #[inline]
    fn from(bits: VkPipelineDepthStencilStateCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkPipelineDepthStencilStateCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineDynamicStateCreateFlags(pub u32);

impl VkPipelineDynamicStateCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkPipelineDynamicStateCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineColorBlendStateCreateFlags(pub u32);

impl VkPipelineColorBlendStateCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM: Self = Self(1);
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT: Self = Self(1);
}

impl From<VkPipelineColorBlendStateCreateFlagBits> for VkPipelineColorBlendStateCreateFlags {
    #[inline]
    fn from(bits: VkPipelineColorBlendStateCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkPipelineColorBlendStateCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineMultisampleStateCreateFlags(pub u32);

impl VkPipelineMultisampleStateCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkPipelineMultisampleStateCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineRasterizationStateCreateFlags(pub u32);

impl VkPipelineRasterizationStateCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkPipelineRasterizationStateCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineViewportStateCreateFlags(pub u32);

impl VkPipelineViewportStateCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkPipelineViewportStateCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineTessellationStateCreateFlags(pub u32);

impl VkPipelineTessellationStateCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkPipelineTessellationStateCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineInputAssemblyStateCreateFlags(pub u32);

impl VkPipelineInputAssemblyStateCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkPipelineInputAssemblyStateCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineVertexInputStateCreateFlags(pub u32);

impl VkPipelineVertexInputStateCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkPipelineVertexInputStateCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineShaderStageCreateFlags(pub u32);

impl VkPipelineShaderStageCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const ALLOW_VARYING_SUBGROUP_SIZE_BIT: Self = Self(1);
    pub const REQUIRE_FULL_SUBGROUPS_BIT: Self = Self(2);
    pub const ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT: Self = Self(1);
    pub const REQUIRE_FULL_SUBGROUPS_BIT_EXT: Self = Self(2);
    pub const RESERVED_3_BIT_KHR: Self = Self(8);
}

impl From<VkPipelineShaderStageCreateFlagBits> for VkPipelineShaderStageCreateFlags {
    #[inline]
    fn from(bits: VkPipelineShaderStageCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkPipelineShaderStageCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkDescriptorSetLayoutCreateFlags(pub u32);

impl VkDescriptorSetLayoutCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const UPDATE_AFTER_BIND_POOL_BIT: Self = Self(2);
    pub const PUSH_DESCRIPTOR_BIT: Self = Self(1);
    pub const PUSH_DESCRIPTOR_BIT_KHR: Self = Self(1);
    pub const UPDATE_AFTER_BIND_POOL_BIT_EXT: Self = Self(2);
    pub const DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(16);
    pub const EMBEDDED_IMMUTABLE_SAMPLERS_BIT_EXT: Self = Self(32);
    pub const HOST_ONLY_POOL_BIT_VALVE: Self = Self(4);
    pub const INDIRECT_BINDABLE_BIT_NV: Self = Self(128);
    pub const HOST_ONLY_POOL_BIT_EXT: Self = Self(4);
    pub const PER_STAGE_BIT_NV: Self = Self(64);
}

impl From<VkDescriptorSetLayoutCreateFlagBits> for VkDescriptorSetLayoutCreateFlags {
    #[inline]
    fn from(bits: VkDescriptorSetLayoutCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkDescriptorSetLayoutCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkBufferViewCreateFlags(pub u32);

impl VkBufferViewCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkBufferViewCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkInstanceCreateFlags(pub u32);

impl VkInstanceCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const ENUMERATE_PORTABILITY_BIT_KHR: Self = Self(1);
    pub const RESERVED_616_BIT_EXT: Self = Self(2);
}

impl From<VkInstanceCreateFlagBits> for VkInstanceCreateFlags {
    #[inline]
    fn from(bits: VkInstanceCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkInstanceCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkDeviceCreateFlags(pub u32);

impl VkDeviceCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkDeviceCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkDeviceQueueCreateFlags(pub u32);

impl VkDeviceQueueCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const PROTECTED_BIT: Self = Self(1);
    pub const RESERVED_1_BIT_QCOM: Self = Self(2);
    pub const INTERNALLY_SYNCHRONIZED_BIT_KHR: Self = Self(4);
}

impl From<VkDeviceQueueCreateFlagBits> for VkDeviceQueueCreateFlags {
    #[inline]
    fn from(bits: VkDeviceQueueCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkDeviceQueueCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkQueueFlags(pub u32);

impl VkQueueFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const GRAPHICS_BIT: Self = Self(1);
    pub const COMPUTE_BIT: Self = Self(2);
    pub const TRANSFER_BIT: Self = Self(4);
    pub const SPARSE_BINDING_BIT: Self = Self(8);
    pub const PROTECTED_BIT: Self = Self(16);
    pub const VIDEO_DECODE_BIT_KHR: Self = Self(32);
    pub const VIDEO_ENCODE_BIT_KHR: Self = Self(64);
    pub const RESERVED_7_BIT_QCOM: Self = Self(128);
    pub const OPTICAL_FLOW_BIT_NV: Self = Self(256);
    pub const DATA_GRAPH_BIT_ARM: Self = Self(1024);
    pub const RESERVED_12_BIT_EXT: Self = Self(4096);
    pub const RESERVED_9_BIT_EXT: Self = Self(512);
    pub const RESERVED_13_BIT_EXT: Self = Self(8192);
    pub const RESERVED_11_BIT_ARM: Self = Self(2048);
    pub const RESERVED_14_BIT_EXT: Self = Self(16384);
}

impl From<VkQueueFlagBits> for VkQueueFlags {
    #[inline]
    fn from(bits: VkQueueFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkQueueFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkMemoryPropertyFlags(pub u32);

impl VkMemoryPropertyFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const DEVICE_LOCAL_BIT: Self = Self(1);
    pub const HOST_VISIBLE_BIT: Self = Self(2);
    pub const HOST_COHERENT_BIT: Self = Self(4);
    pub const HOST_CACHED_BIT: Self = Self(8);
    pub const LAZILY_ALLOCATED_BIT: Self = Self(16);
    pub const PROTECTED_BIT: Self = Self(32);
    pub const DEVICE_COHERENT_BIT_AMD: Self = Self(64);
    pub const DEVICE_UNCACHED_BIT_AMD: Self = Self(128);
    pub const RDMA_CAPABLE_BIT_NV: Self = Self(256);
}

impl From<VkMemoryPropertyFlagBits> for VkMemoryPropertyFlags {
    #[inline]
    fn from(bits: VkMemoryPropertyFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkMemoryPropertyFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkMemoryHeapFlags(pub u32);

impl VkMemoryHeapFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const DEVICE_LOCAL_BIT: Self = Self(1);
    pub const MULTI_INSTANCE_BIT: Self = Self(2);
    pub const MULTI_INSTANCE_BIT_KHR: Self = Self(2);
    pub const TILE_MEMORY_BIT_QCOM: Self = Self(8);
}

impl From<VkMemoryHeapFlagBits> for VkMemoryHeapFlags {
    #[inline]
    fn from(bits: VkMemoryHeapFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkMemoryHeapFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkAccessFlags(pub u32);

impl VkAccessFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const INDIRECT_COMMAND_READ_BIT: Self = Self(1);
    pub const INDEX_READ_BIT: Self = Self(2);
    pub const VERTEX_ATTRIBUTE_READ_BIT: Self = Self(4);
    pub const UNIFORM_READ_BIT: Self = Self(8);
    pub const INPUT_ATTACHMENT_READ_BIT: Self = Self(16);
    pub const SHADER_READ_BIT: Self = Self(32);
    pub const SHADER_WRITE_BIT: Self = Self(64);
    pub const COLOR_ATTACHMENT_READ_BIT: Self = Self(128);
    pub const COLOR_ATTACHMENT_WRITE_BIT: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_READ_BIT: Self = Self(512);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: Self = Self(1024);
    pub const TRANSFER_READ_BIT: Self = Self(2048);
    pub const TRANSFER_WRITE_BIT: Self = Self(4096);
    pub const HOST_READ_BIT: Self = Self(8192);
    pub const HOST_WRITE_BIT: Self = Self(16384);
    pub const MEMORY_READ_BIT: Self = Self(32768);
    pub const MEMORY_WRITE_BIT: Self = Self(65536);
    pub const NONE: Self = Self(0);
    pub const TRANSFORM_FEEDBACK_WRITE_BIT_EXT: Self = Self(33554432);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT: Self = Self(67108864);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT: Self = Self(134217728);
    pub const CONDITIONAL_RENDERING_READ_BIT_EXT: Self = Self(1048576);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: Self = Self(524288);
    pub const ACCELERATION_STRUCTURE_READ_BIT_KHR: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_WRITE_BIT_KHR: Self = Self(4194304);
    pub const SHADING_RATE_IMAGE_READ_BIT_NV: Self = Self(8388608);
    pub const ACCELERATION_STRUCTURE_READ_BIT_NV: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_WRITE_BIT_NV: Self = Self(4194304);
    pub const FRAGMENT_DENSITY_MAP_READ_BIT_EXT: Self = Self(16777216);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: Self = Self(8388608);
    pub const COMMAND_PREPROCESS_READ_BIT_NV: Self = Self(131072);
    pub const COMMAND_PREPROCESS_WRITE_BIT_NV: Self = Self(262144);
    pub const NONE_KHR: Self = Self(0);
    pub const COMMAND_PREPROCESS_READ_BIT_EXT: Self = Self(131072);
    pub const COMMAND_PREPROCESS_WRITE_BIT_EXT: Self = Self(262144);
}

impl From<VkAccessFlagBits> for VkAccessFlags {
    #[inline]
    fn from(bits: VkAccessFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkAccessFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkBufferUsageFlags(pub u32);

impl VkBufferUsageFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const TRANSFER_SRC_BIT: Self = Self(1);
    pub const TRANSFER_DST_BIT: Self = Self(2);
    pub const UNIFORM_TEXEL_BUFFER_BIT: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER_BIT: Self = Self(8);
    pub const UNIFORM_BUFFER_BIT: Self = Self(16);
    pub const STORAGE_BUFFER_BIT: Self = Self(32);
    pub const INDEX_BUFFER_BIT: Self = Self(64);
    pub const VERTEX_BUFFER_BIT: Self = Self(128);
    pub const INDIRECT_BUFFER_BIT: Self = Self(256);
    pub const SHADER_DEVICE_ADDRESS_BIT: Self = Self(131072);
    pub const VIDEO_DECODE_SRC_BIT_KHR: Self = Self(8192);
    pub const VIDEO_DECODE_DST_BIT_KHR: Self = Self(16384);
    pub const TRANSFORM_FEEDBACK_BUFFER_BIT_EXT: Self = Self(2048);
    pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT: Self = Self(4096);
    pub const CONDITIONAL_RENDERING_BIT_EXT: Self = Self(512);
    pub const EXECUTION_GRAPH_SCRATCH_BIT_AMDX: Self = Self(33554432);
    pub const DESCRIPTOR_HEAP_BIT_EXT: Self = Self(268435456);
    pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR: Self = Self(524288);
    pub const ACCELERATION_STRUCTURE_STORAGE_BIT_KHR: Self = Self(1048576);
    pub const SHADER_BINDING_TABLE_BIT_KHR: Self = Self(1024);
    pub const RAY_TRACING_BIT_NV: Self = Self(1024);
    pub const SHADER_DEVICE_ADDRESS_BIT_EXT: Self = Self(131072);
    pub const SHADER_DEVICE_ADDRESS_BIT_KHR: Self = Self(131072);
    pub const VIDEO_ENCODE_DST_BIT_KHR: Self = Self(32768);
    pub const VIDEO_ENCODE_SRC_BIT_KHR: Self = Self(65536);
    pub const SAMPLER_DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(2097152);
    pub const RESOURCE_DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(4194304);
    pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(67108864);
    pub const MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT: Self = Self(8388608);
    pub const MICROMAP_STORAGE_BIT_EXT: Self = Self(16777216);
    pub const TILE_MEMORY_BIT_QCOM: Self = Self(134217728);
}

impl From<VkBufferUsageFlagBits> for VkBufferUsageFlags {
    #[inline]
    fn from(bits: VkBufferUsageFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkBufferUsageFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkBufferCreateFlags(pub u32);

impl VkBufferCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const SPARSE_BINDING_BIT: Self = Self(1);
    pub const SPARSE_RESIDENCY_BIT: Self = Self(2);
    pub const SPARSE_ALIASED_BIT: Self = Self(4);
    pub const PROTECTED_BIT: Self = Self(8);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: Self = Self(16);
    pub const RESERVED_7_BIT_IMG: Self = Self(128);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT: Self = Self(16);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: Self = Self(16);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(32);
    pub const VIDEO_PROFILE_INDEPENDENT_BIT_KHR: Self = Self(64);
}

impl From<VkBufferCreateFlagBits> for VkBufferCreateFlags {
    #[inline]
    fn from(bits: VkBufferCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkBufferCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkShaderStageFlags(pub u32);

impl VkShaderStageFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const VERTEX_BIT: Self = Self(1);
    pub const TESSELLATION_CONTROL_BIT: Self = Self(2);
    pub const TESSELLATION_EVALUATION_BIT: Self = Self(4);
    pub const GEOMETRY_BIT: Self = Self(8);
    pub const FRAGMENT_BIT: Self = Self(16);
    pub const COMPUTE_BIT: Self = Self(32);
    pub const ALL_GRAPHICS: Self = Self(31);
    pub const ALL: Self = Self(2147483647);
    pub const RAYGEN_BIT_KHR: Self = Self(256);
    pub const ANY_HIT_BIT_KHR: Self = Self(512);
    pub const CLOSEST_HIT_BIT_KHR: Self = Self(1024);
    pub const MISS_BIT_KHR: Self = Self(2048);
    pub const INTERSECTION_BIT_KHR: Self = Self(4096);
    pub const CALLABLE_BIT_KHR: Self = Self(8192);
    pub const RAYGEN_BIT_NV: Self = Self(256);
    pub const ANY_HIT_BIT_NV: Self = Self(512);
    pub const CLOSEST_HIT_BIT_NV: Self = Self(1024);
    pub const MISS_BIT_NV: Self = Self(2048);
    pub const INTERSECTION_BIT_NV: Self = Self(4096);
    pub const CALLABLE_BIT_NV: Self = Self(8192);
    pub const TASK_BIT_NV: Self = Self(64);
    pub const MESH_BIT_NV: Self = Self(128);
    pub const TASK_BIT_EXT: Self = Self(64);
    pub const MESH_BIT_EXT: Self = Self(128);
    pub const SUBPASS_SHADING_BIT_HUAWEI: Self = Self(16384);
    pub const CLUSTER_CULLING_BIT_HUAWEI: Self = Self(524288);
    pub const RESERVED_15_BIT_NV: Self = Self(32768);
    pub const RESERVED_16_BIT_HUAWEI: Self = Self(65536);
}

impl From<VkShaderStageFlagBits> for VkShaderStageFlags {
    #[inline]
    fn from(bits: VkShaderStageFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkShaderStageFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkImageUsageFlags(pub u32);

impl VkImageUsageFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const TRANSFER_SRC_BIT: Self = Self(1);
    pub const TRANSFER_DST_BIT: Self = Self(2);
    pub const SAMPLED_BIT: Self = Self(4);
    pub const STORAGE_BIT: Self = Self(8);
    pub const COLOR_ATTACHMENT_BIT: Self = Self(16);
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: Self = Self(32);
    pub const TRANSIENT_ATTACHMENT_BIT: Self = Self(64);
    pub const INPUT_ATTACHMENT_BIT: Self = Self(128);
    pub const HOST_TRANSFER_BIT: Self = Self(4194304);
    pub const VIDEO_DECODE_DST_BIT_KHR: Self = Self(1024);
    pub const VIDEO_DECODE_SRC_BIT_KHR: Self = Self(2048);
    pub const VIDEO_DECODE_DPB_BIT_KHR: Self = Self(4096);
    pub const SHADING_RATE_IMAGE_BIT_NV: Self = Self(256);
    pub const FRAGMENT_DENSITY_MAP_BIT_EXT: Self = Self(512);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(256);
    pub const HOST_TRANSFER_BIT_EXT: Self = Self(4194304);
    pub const VIDEO_ENCODE_DST_BIT_KHR: Self = Self(8192);
    pub const VIDEO_ENCODE_SRC_BIT_KHR: Self = Self(16384);
    pub const VIDEO_ENCODE_DPB_BIT_KHR: Self = Self(32768);
    pub const ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: Self = Self(524288);
    pub const INVOCATION_MASK_BIT_HUAWEI: Self = Self(262144);
    pub const SAMPLE_WEIGHT_BIT_QCOM: Self = Self(1048576);
    pub const SAMPLE_BLOCK_MATCH_BIT_QCOM: Self = Self(2097152);
    pub const RESERVED_24_BIT_COREAVI: Self = Self(16777216);
    pub const TENSOR_ALIASING_BIT_ARM: Self = Self(8388608);
    pub const RESERVED_28_BIT_EXT: Self = Self(268435456);
    pub const TILE_MEMORY_BIT_QCOM: Self = Self(134217728);
    pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR: Self = Self(33554432);
    pub const VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR: Self = Self(67108864);
    pub const RESERVED_29_BIT_KHR: Self = Self(536870912);
    pub const RESERVED_30_BIT_KHR: Self = Self(1073741824);
    pub const RESERVED_16_BIT_HUAWEI: Self = Self(65536);
    pub const RESERVED_17_BIT_HUAWEI: Self = Self(131072);
}

impl From<VkImageUsageFlagBits> for VkImageUsageFlags {
    #[inline]
    fn from(bits: VkImageUsageFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkImageUsageFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkImageCreateFlags(pub u32);

impl VkImageCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const SPARSE_BINDING_BIT: Self = Self(1);
    pub const SPARSE_RESIDENCY_BIT: Self = Self(2);
    pub const SPARSE_ALIASED_BIT: Self = Self(4);
    pub const MUTABLE_FORMAT_BIT: Self = Self(8);
    pub const CUBE_COMPATIBLE_BIT: Self = Self(16);
    pub const ALIAS_BIT: Self = Self(1024);
    pub const SPLIT_INSTANCE_BIND_REGIONS_BIT: Self = Self(64);
    pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT: Self = Self(32);
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE_BIT: Self = Self(128);
    pub const EXTENDED_USAGE_BIT: Self = Self(256);
    pub const PROTECTED_BIT: Self = Self(2048);
    pub const DISJOINT_BIT: Self = Self(512);
    pub const CORNER_SAMPLED_BIT_NV: Self = Self(8192);
    pub const SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: Self = Self(64);
    pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR: Self = Self(32);
    pub const RESERVED_21_BIT_IMG: Self = Self(2097152);
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR: Self = Self(128);
    pub const EXTENDED_USAGE_BIT_KHR: Self = Self(256);
    pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_EXT: Self = Self(65536);
    pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT: Self = Self(4096);
    pub const DISJOINT_BIT_KHR: Self = Self(512);
    pub const ALIAS_BIT_KHR: Self = Self(1024);
    pub const SUBSAMPLED_BIT_EXT: Self = Self(16384);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(65536);
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT: Self = Self(262144);
    pub const VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT: Self = Self(131072);
    pub const FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM: Self = Self(32768);
    pub const VIDEO_PROFILE_INDEPENDENT_BIT_KHR: Self = Self(1048576);
    pub const FRAGMENT_DENSITY_MAP_OFFSET_BIT_EXT: Self = Self(32768);
    pub const ALIAS_SINGLE_LAYER_DESCRIPTOR_BIT_KHR: Self = Self(4194304);
    pub const RESERVED_19_BIT_NV: Self = Self(524288);
}

impl From<VkImageCreateFlagBits> for VkImageCreateFlags {
    #[inline]
    fn from(bits: VkImageCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkImageCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkImageViewCreateFlags(pub u32);

impl VkImageViewCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT: Self = Self(1);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT: Self = Self(4);
    pub const FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT: Self = Self(2);
}

impl From<VkImageViewCreateFlagBits> for VkImageViewCreateFlags {
    #[inline]
    fn from(bits: VkImageViewCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkImageViewCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineCreateFlags(pub u32);

impl VkPipelineCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const DISABLE_OPTIMIZATION_BIT: Self = Self(1);
    pub const ALLOW_DERIVATIVES_BIT: Self = Self(2);
    pub const DERIVATIVE_BIT: Self = Self(4);
    pub const DISPATCH_BASE_BIT: Self = Self(16);
    pub const DISPATCH_BASE: Self = Self(16);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX_BIT: Self = Self(8);
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT: Self = Self(256);
    pub const EARLY_RETURN_ON_FAILURE_BIT: Self = Self(512);
    pub const NO_PROTECTED_ACCESS_BIT: Self = Self(134217728);
    pub const PROTECTED_ACCESS_ONLY_BIT: Self = Self(1073741824);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR: Self = Self(8);
    pub const DISPATCH_BASE_BIT_KHR: Self = Self(16);
    pub const DISPATCH_BASE_KHR: Self = Self(16);
    pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR: Self = Self(16384);
    pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR: Self = Self(32768);
    pub const RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR: Self = Self(65536);
    pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR: Self = Self(131072);
    pub const RAY_TRACING_SKIP_TRIANGLES_BIT_KHR: Self = Self(4096);
    pub const RAY_TRACING_SKIP_AABBS_BIT_KHR: Self = Self(8192);
    pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR: Self = Self(524288);
    pub const DEFER_COMPILE_BIT_NV: Self = Self(32);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: Self = Self(4194304);
    pub const VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: Self = Self(4194304);
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(2097152);
    pub const VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(2097152);
    pub const CAPTURE_STATISTICS_BIT_KHR: Self = Self(64);
    pub const CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR: Self = Self(128);
    pub const INDIRECT_BINDABLE_BIT_NV: Self = Self(262144);
    pub const LIBRARY_BIT_KHR: Self = Self(2048);
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT: Self = Self(256);
    pub const EARLY_RETURN_ON_FAILURE_BIT_EXT: Self = Self(512);
    pub const DESCRIPTOR_BUFFER_BIT_EXT: Self = Self(536870912);
    pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT: Self = Self(8388608);
    pub const LINK_TIME_OPTIMIZATION_BIT_EXT: Self = Self(1024);
    pub const RAY_TRACING_ALLOW_MOTION_BIT_NV: Self = Self(1048576);
    pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: Self = Self(33554432);
    pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT: Self = Self(67108864);
    pub const RAY_TRACING_OPACITY_MICROMAP_BIT_EXT: Self = Self(16777216);
    pub const RAY_TRACING_DISPLACEMENT_MICROMAP_BIT_NV: Self = Self(268435456);
    pub const NO_PROTECTED_ACCESS_BIT_EXT: Self = Self(134217728);
    pub const PROTECTED_ACCESS_ONLY_BIT_EXT: Self = Self(1073741824);
    pub const RAY_TRACING_OPACITY_MICROMAP_BIT_KHR: Self = Self(16777216);
}

impl From<VkPipelineCreateFlagBits> for VkPipelineCreateFlags {
    #[inline]
    fn from(bits: VkPipelineCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkPipelineCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkColorComponentFlags(pub u32);

impl VkColorComponentFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const R_BIT: Self = Self(1);
    pub const G_BIT: Self = Self(2);
    pub const B_BIT: Self = Self(4);
    pub const A_BIT: Self = Self(8);
}

impl From<VkColorComponentFlagBits> for VkColorComponentFlags {
    #[inline]
    fn from(bits: VkColorComponentFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkColorComponentFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkFenceCreateFlags(pub u32);

impl VkFenceCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const SIGNALED_BIT: Self = Self(1);
}

impl From<VkFenceCreateFlagBits> for VkFenceCreateFlags {
    #[inline]
    fn from(bits: VkFenceCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkFenceCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkSemaphoreCreateFlags(pub u32);

impl VkSemaphoreCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkSemaphoreCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkFormatFeatureFlags(pub u32);

impl VkFormatFeatureFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const SAMPLED_IMAGE_BIT: Self = Self(1);
    pub const STORAGE_IMAGE_BIT: Self = Self(2);
    pub const STORAGE_IMAGE_ATOMIC_BIT: Self = Self(4);
    pub const UNIFORM_TEXEL_BUFFER_BIT: Self = Self(8);
    pub const STORAGE_TEXEL_BUFFER_BIT: Self = Self(16);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_BIT: Self = Self(32);
    pub const VERTEX_BUFFER_BIT: Self = Self(64);
    pub const COLOR_ATTACHMENT_BIT: Self = Self(128);
    pub const COLOR_ATTACHMENT_BLEND_BIT: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: Self = Self(512);
    pub const BLIT_SRC_BIT: Self = Self(1024);
    pub const BLIT_DST_BIT: Self = Self(2048);
    pub const SAMPLED_IMAGE_FILTER_LINEAR_BIT: Self = Self(4096);
    pub const TRANSFER_SRC_BIT: Self = Self(16384);
    pub const TRANSFER_DST_BIT: Self = Self(32768);
    pub const MIDPOINT_CHROMA_SAMPLES_BIT: Self = Self(131072);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: Self = Self(262144);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: Self = Self(524288);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: Self = Self(1048576);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: Self = Self(2097152);
    pub const DISJOINT_BIT: Self = Self(4194304);
    pub const COSITED_CHROMA_SAMPLES_BIT: Self = Self(8388608);
    pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT: Self = Self(65536);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG: Self = Self(8192);
    pub const VIDEO_DECODE_OUTPUT_BIT_KHR: Self = Self(33554432);
    pub const VIDEO_DECODE_DPB_BIT_KHR: Self = Self(67108864);
    pub const TRANSFER_SRC_BIT_KHR: Self = Self(16384);
    pub const TRANSFER_DST_BIT_KHR: Self = Self(32768);
    pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT: Self = Self(65536);
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR: Self = Self(536870912);
    pub const MIDPOINT_CHROMA_SAMPLES_BIT_KHR: Self = Self(131072);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: Self = Self(262144);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: Self = Self(524288);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: Self = Self(1048576);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: Self = Self(2097152);
    pub const DISJOINT_BIT_KHR: Self = Self(4194304);
    pub const COSITED_CHROMA_SAMPLES_BIT_KHR: Self = Self(8388608);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: Self = Self(8192);
    pub const FRAGMENT_DENSITY_MAP_BIT_EXT: Self = Self(16777216);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(1073741824);
    pub const VIDEO_ENCODE_INPUT_BIT_KHR: Self = Self(134217728);
    pub const VIDEO_ENCODE_DPB_BIT_KHR: Self = Self(268435456);
}

impl From<VkFormatFeatureFlagBits> for VkFormatFeatureFlags {
    #[inline]
    fn from(bits: VkFormatFeatureFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkFormatFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkQueryControlFlags(pub u32);

impl VkQueryControlFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const PRECISE_BIT: Self = Self(1);
}

impl From<VkQueryControlFlagBits> for VkQueryControlFlags {
    #[inline]
    fn from(bits: VkQueryControlFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkQueryControlFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkQueryResultFlags(pub u32);

impl VkQueryResultFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const VK_QUERY_RESULT_64_BIT: Self = Self(1);
    pub const WAIT_BIT: Self = Self(2);
    pub const WITH_AVAILABILITY_BIT: Self = Self(4);
    pub const PARTIAL_BIT: Self = Self(8);
    pub const WITH_STATUS_BIT_KHR: Self = Self(16);
}

impl From<VkQueryResultFlagBits> for VkQueryResultFlags {
    #[inline]
    fn from(bits: VkQueryResultFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkQueryResultFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkShaderModuleCreateFlags(pub u32);

impl VkShaderModuleCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkShaderModuleCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkEventCreateFlags(pub u32);

impl VkEventCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const DEVICE_ONLY_BIT: Self = Self(1);
    pub const DEVICE_ONLY_BIT_KHR: Self = Self(1);
}

impl From<VkEventCreateFlagBits> for VkEventCreateFlags {
    #[inline]
    fn from(bits: VkEventCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkEventCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkCommandPoolCreateFlags(pub u32);

impl VkCommandPoolCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const TRANSIENT_BIT: Self = Self(1);
    pub const RESET_COMMAND_BUFFER_BIT: Self = Self(2);
    pub const PROTECTED_BIT: Self = Self(4);
}

impl From<VkCommandPoolCreateFlagBits> for VkCommandPoolCreateFlags {
    #[inline]
    fn from(bits: VkCommandPoolCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkCommandPoolCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkCommandPoolResetFlags(pub u32);

impl VkCommandPoolResetFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const RELEASE_RESOURCES_BIT: Self = Self(1);
    pub const RESERVED_1_BIT_COREAVI: Self = Self(2);
}

impl From<VkCommandPoolResetFlagBits> for VkCommandPoolResetFlags {
    #[inline]
    fn from(bits: VkCommandPoolResetFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkCommandPoolResetFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkCommandBufferResetFlags(pub u32);

impl VkCommandBufferResetFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const RELEASE_RESOURCES_BIT: Self = Self(1);
}

impl From<VkCommandBufferResetFlagBits> for VkCommandBufferResetFlags {
    #[inline]
    fn from(bits: VkCommandBufferResetFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkCommandBufferResetFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkCommandBufferUsageFlags(pub u32);

impl VkCommandBufferUsageFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const ONE_TIME_SUBMIT_BIT: Self = Self(1);
    pub const RENDER_PASS_CONTINUE_BIT: Self = Self(2);
    pub const SIMULTANEOUS_USE_BIT: Self = Self(4);
    pub const RESERVED_3_BIT_HUAWEI: Self = Self(8);
    pub const RESERVED_4_BIT_HUAWEI: Self = Self(16);
}

impl From<VkCommandBufferUsageFlagBits> for VkCommandBufferUsageFlags {
    #[inline]
    fn from(bits: VkCommandBufferUsageFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkCommandBufferUsageFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkQueryPipelineStatisticFlags(pub u32);

impl VkQueryPipelineStatisticFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const INPUT_ASSEMBLY_VERTICES_BIT: Self = Self(1);
    pub const INPUT_ASSEMBLY_PRIMITIVES_BIT: Self = Self(2);
    pub const VERTEX_SHADER_INVOCATIONS_BIT: Self = Self(4);
    pub const GEOMETRY_SHADER_INVOCATIONS_BIT: Self = Self(8);
    pub const GEOMETRY_SHADER_PRIMITIVES_BIT: Self = Self(16);
    pub const CLIPPING_INVOCATIONS_BIT: Self = Self(32);
    pub const CLIPPING_PRIMITIVES_BIT: Self = Self(64);
    pub const FRAGMENT_SHADER_INVOCATIONS_BIT: Self = Self(128);
    pub const TESSELLATION_CONTROL_SHADER_PATCHES_BIT: Self = Self(256);
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: Self = Self(512);
    pub const COMPUTE_SHADER_INVOCATIONS_BIT: Self = Self(1024);
    pub const TASK_SHADER_INVOCATIONS_BIT_EXT: Self = Self(2048);
    pub const MESH_SHADER_INVOCATIONS_BIT_EXT: Self = Self(4096);
    pub const CLUSTER_CULLING_SHADER_INVOCATIONS_BIT_HUAWEI: Self = Self(8192);
}

impl From<VkQueryPipelineStatisticFlagBits> for VkQueryPipelineStatisticFlags {
    #[inline]
    fn from(bits: VkQueryPipelineStatisticFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkQueryPipelineStatisticFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkMemoryMapFlags(pub u32);

impl VkMemoryMapFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const PLACED_BIT_EXT: Self = Self(1);
}

impl From<VkMemoryMapFlagBits> for VkMemoryMapFlags {
    #[inline]
    fn from(bits: VkMemoryMapFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkMemoryMapFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkImageAspectFlags(pub u32);

impl VkImageAspectFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const COLOR_BIT: Self = Self(1);
    pub const DEPTH_BIT: Self = Self(2);
    pub const STENCIL_BIT: Self = Self(4);
    pub const METADATA_BIT: Self = Self(8);
    pub const PLANE_0_BIT: Self = Self(16);
    pub const PLANE_1_BIT: Self = Self(32);
    pub const PLANE_2_BIT: Self = Self(64);
    pub const NONE: Self = Self(0);
    pub const PLANE_0_BIT_KHR: Self = Self(16);
    pub const PLANE_1_BIT_KHR: Self = Self(32);
    pub const PLANE_2_BIT_KHR: Self = Self(64);
    pub const MEMORY_PLANE_0_BIT_EXT: Self = Self(128);
    pub const MEMORY_PLANE_1_BIT_EXT: Self = Self(256);
    pub const MEMORY_PLANE_2_BIT_EXT: Self = Self(512);
    pub const MEMORY_PLANE_3_BIT_EXT: Self = Self(1024);
    pub const NONE_KHR: Self = Self(0);
    pub const RESERVED_11_BIT_HUAWEI: Self = Self(2048);
}

impl From<VkImageAspectFlagBits> for VkImageAspectFlags {
    #[inline]
    fn from(bits: VkImageAspectFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkImageAspectFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkSparseMemoryBindFlags(pub u32);

impl VkSparseMemoryBindFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const METADATA_BIT: Self = Self(1);
}

impl From<VkSparseMemoryBindFlagBits> for VkSparseMemoryBindFlags {
    #[inline]
    fn from(bits: VkSparseMemoryBindFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkSparseMemoryBindFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkSparseImageFormatFlags(pub u32);

impl VkSparseImageFormatFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const SINGLE_MIPTAIL_BIT: Self = Self(1);
    pub const ALIGNED_MIP_SIZE_BIT: Self = Self(2);
    pub const NONSTANDARD_BLOCK_SIZE_BIT: Self = Self(4);
}

impl From<VkSparseImageFormatFlagBits> for VkSparseImageFormatFlags {
    #[inline]
    fn from(bits: VkSparseImageFormatFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkSparseImageFormatFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkSubpassDescriptionFlags(pub u32);

impl VkSubpassDescriptionFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const PER_VIEW_ATTRIBUTES_BIT_NVX: Self = Self(1);
    pub const PER_VIEW_POSITION_X_ONLY_BIT_NVX: Self = Self(2);
    pub const FRAGMENT_REGION_BIT_QCOM: Self = Self(4);
    pub const SHADER_RESOLVE_BIT_QCOM: Self = Self(8);
    pub const TILE_SHADING_APRON_BIT_QCOM: Self = Self(256);
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM: Self = Self(16);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM: Self = Self(32);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM: Self = Self(64);
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT: Self = Self(16);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT: Self = Self(32);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT: Self = Self(64);
    pub const ENABLE_LEGACY_DITHERING_BIT_EXT: Self = Self(128);
    pub const FRAGMENT_REGION_BIT_EXT: Self = Self(4);
    pub const CUSTOM_RESOLVE_BIT_EXT: Self = Self(8);
}

impl From<VkSubpassDescriptionFlagBits> for VkSubpassDescriptionFlags {
    #[inline]
    fn from(bits: VkSubpassDescriptionFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkSubpassDescriptionFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineStageFlags(pub u32);

impl VkPipelineStageFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const TOP_OF_PIPE_BIT: Self = Self(1);
    pub const DRAW_INDIRECT_BIT: Self = Self(2);
    pub const VERTEX_INPUT_BIT: Self = Self(4);
    pub const VERTEX_SHADER_BIT: Self = Self(8);
    pub const TESSELLATION_CONTROL_SHADER_BIT: Self = Self(16);
    pub const TESSELLATION_EVALUATION_SHADER_BIT: Self = Self(32);
    pub const GEOMETRY_SHADER_BIT: Self = Self(64);
    pub const FRAGMENT_SHADER_BIT: Self = Self(128);
    pub const EARLY_FRAGMENT_TESTS_BIT: Self = Self(256);
    pub const LATE_FRAGMENT_TESTS_BIT: Self = Self(512);
    pub const COLOR_ATTACHMENT_OUTPUT_BIT: Self = Self(1024);
    pub const COMPUTE_SHADER_BIT: Self = Self(2048);
    pub const TRANSFER_BIT: Self = Self(4096);
    pub const BOTTOM_OF_PIPE_BIT: Self = Self(8192);
    pub const HOST_BIT: Self = Self(16384);
    pub const ALL_GRAPHICS_BIT: Self = Self(32768);
    pub const ALL_COMMANDS_BIT: Self = Self(65536);
    pub const NONE: Self = Self(0);
    pub const TRANSFORM_FEEDBACK_BIT_EXT: Self = Self(16777216);
    pub const CONDITIONAL_RENDERING_BIT_EXT: Self = Self(262144);
    pub const ACCELERATION_STRUCTURE_BUILD_BIT_KHR: Self = Self(33554432);
    pub const RAY_TRACING_SHADER_BIT_KHR: Self = Self(2097152);
    pub const SHADING_RATE_IMAGE_BIT_NV: Self = Self(4194304);
    pub const RAY_TRACING_SHADER_BIT_NV: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_BUILD_BIT_NV: Self = Self(33554432);
    pub const TASK_SHADER_BIT_NV: Self = Self(524288);
    pub const MESH_SHADER_BIT_NV: Self = Self(1048576);
    pub const FRAGMENT_DENSITY_PROCESS_BIT_EXT: Self = Self(8388608);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(4194304);
    pub const COMMAND_PREPROCESS_BIT_NV: Self = Self(131072);
    pub const NONE_KHR: Self = Self(0);
    pub const TASK_SHADER_BIT_EXT: Self = Self(524288);
    pub const MESH_SHADER_BIT_EXT: Self = Self(1048576);
    pub const COMMAND_PREPROCESS_BIT_EXT: Self = Self(131072);
}

impl From<VkPipelineStageFlagBits> for VkPipelineStageFlags {
    #[inline]
    fn from(bits: VkPipelineStageFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkPipelineStageFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkSampleCountFlags(pub u32);

impl VkSampleCountFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const VK_SAMPLE_COUNT_1_BIT: Self = Self(1);
    pub const VK_SAMPLE_COUNT_2_BIT: Self = Self(2);
    pub const VK_SAMPLE_COUNT_4_BIT: Self = Self(4);
    pub const VK_SAMPLE_COUNT_8_BIT: Self = Self(8);
    pub const VK_SAMPLE_COUNT_16_BIT: Self = Self(16);
    pub const VK_SAMPLE_COUNT_32_BIT: Self = Self(32);
    pub const VK_SAMPLE_COUNT_64_BIT: Self = Self(64);
}

impl From<VkSampleCountFlagBits> for VkSampleCountFlags {
    #[inline]
    fn from(bits: VkSampleCountFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkSampleCountFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkAttachmentDescriptionFlags(pub u32);

impl VkAttachmentDescriptionFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const MAY_ALIAS_BIT: Self = Self(1);
    pub const RESOLVE_SKIP_TRANSFER_FUNCTION_BIT_KHR: Self = Self(2);
    pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_BIT_KHR: Self = Self(4);
}

impl From<VkAttachmentDescriptionFlagBits> for VkAttachmentDescriptionFlags {
    #[inline]
    fn from(bits: VkAttachmentDescriptionFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkAttachmentDescriptionFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkStencilFaceFlags(pub u32);

impl VkStencilFaceFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const FRONT_BIT: Self = Self(1);
    pub const BACK_BIT: Self = Self(2);
    pub const FRONT_AND_BACK: Self = Self(3);
    pub const VK_STENCIL_FRONT_AND_BACK: Self = Self(3);
}

impl From<VkStencilFaceFlagBits> for VkStencilFaceFlags {
    #[inline]
    fn from(bits: VkStencilFaceFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkStencilFaceFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkCullModeFlags(pub u32);

impl VkCullModeFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const NONE: Self = Self(0);
    pub const FRONT_BIT: Self = Self(1);
    pub const BACK_BIT: Self = Self(2);
    pub const FRONT_AND_BACK: Self = Self(3);
}

impl From<VkCullModeFlagBits> for VkCullModeFlags {
    #[inline]
    fn from(bits: VkCullModeFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkCullModeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkDescriptorPoolCreateFlags(pub u32);

impl VkDescriptorPoolCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const FREE_DESCRIPTOR_SET_BIT: Self = Self(1);
    pub const UPDATE_AFTER_BIND_BIT: Self = Self(2);
    pub const UPDATE_AFTER_BIND_BIT_EXT: Self = Self(2);
    pub const HOST_ONLY_BIT_VALVE: Self = Self(4);
    pub const HOST_ONLY_BIT_EXT: Self = Self(4);
    pub const ALLOW_OVERALLOCATION_SETS_BIT_NV: Self = Self(8);
    pub const ALLOW_OVERALLOCATION_POOLS_BIT_NV: Self = Self(16);
}

impl From<VkDescriptorPoolCreateFlagBits> for VkDescriptorPoolCreateFlags {
    #[inline]
    fn from(bits: VkDescriptorPoolCreateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkDescriptorPoolCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkDescriptorPoolResetFlags(pub u32);

impl VkDescriptorPoolResetFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkDescriptorPoolResetFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkDependencyFlags(pub u32);

impl VkDependencyFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const BY_REGION_BIT: Self = Self(1);
    pub const DEVICE_GROUP_BIT: Self = Self(4);
    pub const VIEW_LOCAL_BIT: Self = Self(2);
    pub const VIEW_LOCAL_BIT_KHR: Self = Self(2);
    pub const DEVICE_GROUP_BIT_KHR: Self = Self(4);
    pub const FEEDBACK_LOOP_BIT_EXT: Self = Self(8);
    pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_BIT_KHR: Self = Self(32);
    pub const ASYMMETRIC_EVENT_BIT_KHR: Self = Self(64);
    pub const EXTENSION_586_BIT_IMG: Self = Self(16);
}

impl From<VkDependencyFlagBits> for VkDependencyFlags {
    #[inline]
    fn from(bits: VkDependencyFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkDependencyFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkSubgroupFeatureFlags(pub u32);

impl VkSubgroupFeatureFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const BASIC_BIT: Self = Self(1);
    pub const VOTE_BIT: Self = Self(2);
    pub const ARITHMETIC_BIT: Self = Self(4);
    pub const BALLOT_BIT: Self = Self(8);
    pub const SHUFFLE_BIT: Self = Self(16);
    pub const SHUFFLE_RELATIVE_BIT: Self = Self(32);
    pub const CLUSTERED_BIT: Self = Self(64);
    pub const QUAD_BIT: Self = Self(128);
    pub const ROTATE_BIT: Self = Self(512);
    pub const ROTATE_CLUSTERED_BIT: Self = Self(1024);
    pub const PARTITIONED_BIT_NV: Self = Self(256);
    pub const ROTATE_BIT_KHR: Self = Self(512);
    pub const ROTATE_CLUSTERED_BIT_KHR: Self = Self(1024);
    pub const PARTITIONED_BIT_EXT: Self = Self(256);
}

impl From<VkSubgroupFeatureFlagBits> for VkSubgroupFeatureFlags {
    #[inline]
    fn from(bits: VkSubgroupFeatureFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkSubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPrivateDataSlotCreateFlags(pub u32);

impl VkPrivateDataSlotCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkPrivateDataSlotCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkDescriptorUpdateTemplateCreateFlags(pub u32);

impl VkDescriptorUpdateTemplateCreateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkDescriptorUpdateTemplateCreateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineCreationFeedbackFlags(pub u32);

impl VkPipelineCreationFeedbackFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const VALID_BIT: Self = Self(1);
    pub const APPLICATION_PIPELINE_CACHE_HIT_BIT: Self = Self(2);
    pub const BASE_PIPELINE_ACCELERATION_BIT: Self = Self(4);
    pub const VALID_BIT_EXT: Self = Self(1);
    pub const APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT: Self = Self(2);
    pub const BASE_PIPELINE_ACCELERATION_BIT_EXT: Self = Self(4);
}

impl From<VkPipelineCreationFeedbackFlagBits> for VkPipelineCreationFeedbackFlags {
    #[inline]
    fn from(bits: VkPipelineCreationFeedbackFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkPipelineCreationFeedbackFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkSemaphoreWaitFlags(pub u32);

impl VkSemaphoreWaitFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const ANY_BIT: Self = Self(1);
    pub const ANY_BIT_KHR: Self = Self(1);
}

impl From<VkSemaphoreWaitFlagBits> for VkSemaphoreWaitFlags {
    #[inline]
    fn from(bits: VkSemaphoreWaitFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkSemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkAccessFlags2(pub u64);

impl VkAccessFlags2 {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const NONE: Self = Self(0);
    pub const INDIRECT_COMMAND_READ_BIT: Self = Self(1);
    pub const INDEX_READ_BIT: Self = Self(2);
    pub const VERTEX_ATTRIBUTE_READ_BIT: Self = Self(4);
    pub const UNIFORM_READ_BIT: Self = Self(8);
    pub const INPUT_ATTACHMENT_READ_BIT: Self = Self(16);
    pub const SHADER_READ_BIT: Self = Self(32);
    pub const SHADER_WRITE_BIT: Self = Self(64);
    pub const COLOR_ATTACHMENT_READ_BIT: Self = Self(128);
    pub const COLOR_ATTACHMENT_WRITE_BIT: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_READ_BIT: Self = Self(512);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: Self = Self(1024);
    pub const TRANSFER_READ_BIT: Self = Self(2048);
    pub const TRANSFER_WRITE_BIT: Self = Self(4096);
    pub const HOST_READ_BIT: Self = Self(8192);
    pub const HOST_WRITE_BIT: Self = Self(16384);
    pub const MEMORY_READ_BIT: Self = Self(32768);
    pub const MEMORY_WRITE_BIT: Self = Self(65536);
    pub const SHADER_SAMPLED_READ_BIT: Self = Self(4294967296);
    pub const SHADER_STORAGE_READ_BIT: Self = Self(8589934592);
    pub const SHADER_STORAGE_WRITE_BIT: Self = Self(17179869184);
    pub const VIDEO_DECODE_READ_BIT_KHR: Self = Self(34359738368);
    pub const VIDEO_DECODE_WRITE_BIT_KHR: Self = Self(68719476736);
    pub const SAMPLER_HEAP_READ_BIT_EXT: Self = Self(144115188075855872);
    pub const RESOURCE_HEAP_READ_BIT_EXT: Self = Self(288230376151711744);
    pub const RESERVED_46_BIT_INTEL: Self = Self(70368744177664);
    pub const VIDEO_ENCODE_READ_BIT_KHR: Self = Self(137438953472);
    pub const VIDEO_ENCODE_WRITE_BIT_KHR: Self = Self(274877906944);
    pub const RESERVED_53_BIT_KHR: Self = Self(9007199254740992);
    pub const RESERVED_54_BIT_KHR: Self = Self(18014398509481984);
    pub const SHADER_TILE_ATTACHMENT_READ_BIT_QCOM: Self = Self(2251799813685248);
    pub const SHADER_TILE_ATTACHMENT_WRITE_BIT_QCOM: Self = Self(4503599627370496);
    pub const NONE_KHR: Self = Self(0);
    pub const INDIRECT_COMMAND_READ_BIT_KHR: Self = Self(1);
    pub const INDEX_READ_BIT_KHR: Self = Self(2);
    pub const VERTEX_ATTRIBUTE_READ_BIT_KHR: Self = Self(4);
    pub const UNIFORM_READ_BIT_KHR: Self = Self(8);
    pub const INPUT_ATTACHMENT_READ_BIT_KHR: Self = Self(16);
    pub const SHADER_READ_BIT_KHR: Self = Self(32);
    pub const SHADER_WRITE_BIT_KHR: Self = Self(64);
    pub const COLOR_ATTACHMENT_READ_BIT_KHR: Self = Self(128);
    pub const COLOR_ATTACHMENT_WRITE_BIT_KHR: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR: Self = Self(512);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR: Self = Self(1024);
    pub const TRANSFER_READ_BIT_KHR: Self = Self(2048);
    pub const TRANSFER_WRITE_BIT_KHR: Self = Self(4096);
    pub const HOST_READ_BIT_KHR: Self = Self(8192);
    pub const HOST_WRITE_BIT_KHR: Self = Self(16384);
    pub const MEMORY_READ_BIT_KHR: Self = Self(32768);
    pub const MEMORY_WRITE_BIT_KHR: Self = Self(65536);
    pub const SHADER_SAMPLED_READ_BIT_KHR: Self = Self(4294967296);
    pub const SHADER_STORAGE_READ_BIT_KHR: Self = Self(8589934592);
    pub const SHADER_STORAGE_WRITE_BIT_KHR: Self = Self(17179869184);
    pub const TRANSFORM_FEEDBACK_WRITE_BIT_EXT: Self = Self(33554432);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT: Self = Self(67108864);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT: Self = Self(134217728);
    pub const CONDITIONAL_RENDERING_READ_BIT_EXT: Self = Self(1048576);
    pub const COMMAND_PREPROCESS_READ_BIT_NV: Self = Self(131072);
    pub const COMMAND_PREPROCESS_WRITE_BIT_NV: Self = Self(262144);
    pub const COMMAND_PREPROCESS_READ_BIT_EXT: Self = Self(131072);
    pub const COMMAND_PREPROCESS_WRITE_BIT_EXT: Self = Self(262144);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: Self = Self(8388608);
    pub const SHADING_RATE_IMAGE_READ_BIT_NV: Self = Self(8388608);
    pub const ACCELERATION_STRUCTURE_READ_BIT_KHR: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_WRITE_BIT_KHR: Self = Self(4194304);
    pub const ACCELERATION_STRUCTURE_READ_BIT_NV: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_WRITE_BIT_NV: Self = Self(4194304);
    pub const FRAGMENT_DENSITY_MAP_READ_BIT_EXT: Self = Self(16777216);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: Self = Self(524288);
    pub const DESCRIPTOR_BUFFER_READ_BIT_EXT: Self = Self(2199023255552);
    pub const INVOCATION_MASK_READ_BIT_HUAWEI: Self = Self(549755813888);
    pub const SHADER_BINDING_TABLE_READ_BIT_KHR: Self = Self(1099511627776);
    pub const MICROMAP_READ_BIT_EXT: Self = Self(17592186044416);
    pub const MICROMAP_WRITE_BIT_EXT: Self = Self(35184372088832);
    pub const OPTICAL_FLOW_READ_BIT_NV: Self = Self(4398046511104);
    pub const OPTICAL_FLOW_WRITE_BIT_NV: Self = Self(8796093022208);
    pub const DATA_GRAPH_READ_BIT_ARM: Self = Self(140737488355328);
    pub const DATA_GRAPH_WRITE_BIT_ARM: Self = Self(281474976710656);
    pub const MEMORY_DECOMPRESSION_READ_BIT_EXT: Self = Self(36028797018963968);
    pub const MEMORY_DECOMPRESSION_WRITE_BIT_EXT: Self = Self(72057594037927936);
    pub const RESERVED_62_BIT_EXT: Self = Self(4611686018427387904);
    pub const RESERVED_63_BIT_EXT: Self = Self(9223372036854775808);
    pub const RESERVED_60_BIT_KHR: Self = Self(1152921504606846976);
    pub const RESERVED_61_BIT_KHR: Self = Self(2305843009213693952);
    pub const RESERVED_28_BIT_AMD: Self = Self(268435456);
    pub const RESERVED_29_BIT_AMD: Self = Self(536870912);
    pub const RESERVED_49_BIT_ARM: Self = Self(562949953421312);
    pub const RESERVED_50_BIT_ARM: Self = Self(1125899906842624);
}

impl From<VkAccessFlagBits2> for VkAccessFlags2 {
    #[inline]
    fn from(bits: VkAccessFlagBits2) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkAccessFlags2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPipelineStageFlags2(pub u64);

impl VkPipelineStageFlags2 {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const NONE: Self = Self(0);
    pub const TOP_OF_PIPE_BIT: Self = Self(1);
    pub const DRAW_INDIRECT_BIT: Self = Self(2);
    pub const VERTEX_INPUT_BIT: Self = Self(4);
    pub const VERTEX_SHADER_BIT: Self = Self(8);
    pub const TESSELLATION_CONTROL_SHADER_BIT: Self = Self(16);
    pub const TESSELLATION_EVALUATION_SHADER_BIT: Self = Self(32);
    pub const GEOMETRY_SHADER_BIT: Self = Self(64);
    pub const FRAGMENT_SHADER_BIT: Self = Self(128);
    pub const EARLY_FRAGMENT_TESTS_BIT: Self = Self(256);
    pub const LATE_FRAGMENT_TESTS_BIT: Self = Self(512);
    pub const COLOR_ATTACHMENT_OUTPUT_BIT: Self = Self(1024);
    pub const COMPUTE_SHADER_BIT: Self = Self(2048);
    pub const ALL_TRANSFER_BIT: Self = Self(4096);
    pub const TRANSFER_BIT: Self = Self(4096);
    pub const BOTTOM_OF_PIPE_BIT: Self = Self(8192);
    pub const HOST_BIT: Self = Self(16384);
    pub const ALL_GRAPHICS_BIT: Self = Self(32768);
    pub const ALL_COMMANDS_BIT: Self = Self(65536);
    pub const COPY_BIT: Self = Self(4294967296);
    pub const RESOLVE_BIT: Self = Self(8589934592);
    pub const BLIT_BIT: Self = Self(17179869184);
    pub const CLEAR_BIT: Self = Self(34359738368);
    pub const INDEX_INPUT_BIT: Self = Self(68719476736);
    pub const VERTEX_ATTRIBUTE_INPUT_BIT: Self = Self(137438953472);
    pub const PRE_RASTERIZATION_SHADERS_BIT: Self = Self(274877906944);
    pub const VIDEO_DECODE_BIT_KHR: Self = Self(67108864);
    pub const VIDEO_ENCODE_BIT_KHR: Self = Self(134217728);
    pub const RESERVED_50_BIT_KHR: Self = Self(1125899906842624);
    pub const NONE_KHR: Self = Self(0);
    pub const TOP_OF_PIPE_BIT_KHR: Self = Self(1);
    pub const DRAW_INDIRECT_BIT_KHR: Self = Self(2);
    pub const VERTEX_INPUT_BIT_KHR: Self = Self(4);
    pub const VERTEX_SHADER_BIT_KHR: Self = Self(8);
    pub const TESSELLATION_CONTROL_SHADER_BIT_KHR: Self = Self(16);
    pub const TESSELLATION_EVALUATION_SHADER_BIT_KHR: Self = Self(32);
    pub const GEOMETRY_SHADER_BIT_KHR: Self = Self(64);
    pub const FRAGMENT_SHADER_BIT_KHR: Self = Self(128);
    pub const EARLY_FRAGMENT_TESTS_BIT_KHR: Self = Self(256);
    pub const LATE_FRAGMENT_TESTS_BIT_KHR: Self = Self(512);
    pub const COLOR_ATTACHMENT_OUTPUT_BIT_KHR: Self = Self(1024);
    pub const COMPUTE_SHADER_BIT_KHR: Self = Self(2048);
    pub const ALL_TRANSFER_BIT_KHR: Self = Self(4096);
    pub const TRANSFER_BIT_KHR: Self = Self(4096);
    pub const BOTTOM_OF_PIPE_BIT_KHR: Self = Self(8192);
    pub const HOST_BIT_KHR: Self = Self(16384);
    pub const ALL_GRAPHICS_BIT_KHR: Self = Self(32768);
    pub const ALL_COMMANDS_BIT_KHR: Self = Self(65536);
    pub const COPY_BIT_KHR: Self = Self(4294967296);
    pub const RESOLVE_BIT_KHR: Self = Self(8589934592);
    pub const BLIT_BIT_KHR: Self = Self(17179869184);
    pub const CLEAR_BIT_KHR: Self = Self(34359738368);
    pub const INDEX_INPUT_BIT_KHR: Self = Self(68719476736);
    pub const VERTEX_ATTRIBUTE_INPUT_BIT_KHR: Self = Self(137438953472);
    pub const PRE_RASTERIZATION_SHADERS_BIT_KHR: Self = Self(274877906944);
    pub const TRANSFORM_FEEDBACK_BIT_EXT: Self = Self(16777216);
    pub const CONDITIONAL_RENDERING_BIT_EXT: Self = Self(262144);
    pub const COMMAND_PREPROCESS_BIT_NV: Self = Self(131072);
    pub const COMMAND_PREPROCESS_BIT_EXT: Self = Self(131072);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(4194304);
    pub const SHADING_RATE_IMAGE_BIT_NV: Self = Self(4194304);
    pub const ACCELERATION_STRUCTURE_BUILD_BIT_KHR: Self = Self(33554432);
    pub const RAY_TRACING_SHADER_BIT_KHR: Self = Self(2097152);
    pub const RAY_TRACING_SHADER_BIT_NV: Self = Self(2097152);
    pub const ACCELERATION_STRUCTURE_BUILD_BIT_NV: Self = Self(33554432);
    pub const FRAGMENT_DENSITY_PROCESS_BIT_EXT: Self = Self(8388608);
    pub const TASK_SHADER_BIT_NV: Self = Self(524288);
    pub const MESH_SHADER_BIT_NV: Self = Self(1048576);
    pub const TASK_SHADER_BIT_EXT: Self = Self(524288);
    pub const MESH_SHADER_BIT_EXT: Self = Self(1048576);
    pub const SUBPASS_SHADER_BIT_HUAWEI: Self = Self(549755813888);
    pub const SUBPASS_SHADING_BIT_HUAWEI: Self = Self(549755813888);
    pub const INVOCATION_MASK_BIT_HUAWEI: Self = Self(1099511627776);
    pub const ACCELERATION_STRUCTURE_COPY_BIT_KHR: Self = Self(268435456);
    pub const MICROMAP_BUILD_BIT_EXT: Self = Self(1073741824);
    pub const CLUSTER_CULLING_SHADER_BIT_HUAWEI: Self = Self(2199023255552);
    pub const OPTICAL_FLOW_BIT_NV: Self = Self(536870912);
    pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_BIT_NV: Self = Self(17592186044416);
    pub const DATA_GRAPH_BIT_ARM: Self = Self(4398046511104);
    pub const COPY_INDIRECT_BIT_KHR: Self = Self(70368744177664);
    pub const MEMORY_DECOMPRESSION_BIT_EXT: Self = Self(35184372088832);
    pub const RESERVED_49_BIT_EXT: Self = Self(562949953421312);
    pub const RESERVED_47_BIT_KHR: Self = Self(140737488355328);
    pub const RESERVED_31_BIT_AMD: Self = Self(2147483648);
    pub const RESERVED_43_BIT_ARM: Self = Self(8796093022208);
    pub const RESERVED_48_BIT_HUAWEI: Self = Self(281474976710656);
}

impl From<VkPipelineStageFlagBits2> for VkPipelineStageFlags2 {
    #[inline]
    fn from(bits: VkPipelineStageFlagBits2) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkPipelineStageFlags2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkFormatFeatureFlags2(pub u64);

impl VkFormatFeatureFlags2 {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const SAMPLED_IMAGE_BIT: Self = Self(1);
    pub const STORAGE_IMAGE_BIT: Self = Self(2);
    pub const STORAGE_IMAGE_ATOMIC_BIT: Self = Self(4);
    pub const UNIFORM_TEXEL_BUFFER_BIT: Self = Self(8);
    pub const STORAGE_TEXEL_BUFFER_BIT: Self = Self(16);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_BIT: Self = Self(32);
    pub const VERTEX_BUFFER_BIT: Self = Self(64);
    pub const COLOR_ATTACHMENT_BIT: Self = Self(128);
    pub const COLOR_ATTACHMENT_BLEND_BIT: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: Self = Self(512);
    pub const BLIT_SRC_BIT: Self = Self(1024);
    pub const BLIT_DST_BIT: Self = Self(2048);
    pub const SAMPLED_IMAGE_FILTER_LINEAR_BIT: Self = Self(4096);
    pub const TRANSFER_SRC_BIT: Self = Self(16384);
    pub const TRANSFER_DST_BIT: Self = Self(32768);
    pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT: Self = Self(65536);
    pub const MIDPOINT_CHROMA_SAMPLES_BIT: Self = Self(131072);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: Self = Self(262144);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: Self = Self(524288);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: Self = Self(1048576);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: Self = Self(2097152);
    pub const DISJOINT_BIT: Self = Self(4194304);
    pub const COSITED_CHROMA_SAMPLES_BIT: Self = Self(8388608);
    pub const STORAGE_READ_WITHOUT_FORMAT_BIT: Self = Self(2147483648);
    pub const STORAGE_WRITE_WITHOUT_FORMAT_BIT: Self = Self(4294967296);
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON_BIT: Self = Self(8589934592);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT: Self = Self(8192);
    pub const HOST_IMAGE_TRANSFER_BIT: Self = Self(70368744177664);
    pub const VIDEO_DECODE_OUTPUT_BIT_KHR: Self = Self(33554432);
    pub const VIDEO_DECODE_DPB_BIT_KHR: Self = Self(67108864);
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR: Self = Self(536870912);
    pub const FRAGMENT_DENSITY_MAP_BIT_EXT: Self = Self(16777216);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: Self = Self(1073741824);
    pub const HOST_IMAGE_TRANSFER_BIT_EXT: Self = Self(70368744177664);
    pub const VIDEO_ENCODE_INPUT_BIT_KHR: Self = Self(134217728);
    pub const VIDEO_ENCODE_DPB_BIT_KHR: Self = Self(268435456);
    pub const BLOCK_MATCHING_SXD_BIT_QCOM: Self = Self(17592186044416);
    pub const SAMPLED_IMAGE_BIT_KHR: Self = Self(1);
    pub const STORAGE_IMAGE_BIT_KHR: Self = Self(2);
    pub const STORAGE_IMAGE_ATOMIC_BIT_KHR: Self = Self(4);
    pub const UNIFORM_TEXEL_BUFFER_BIT_KHR: Self = Self(8);
    pub const STORAGE_TEXEL_BUFFER_BIT_KHR: Self = Self(16);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR: Self = Self(32);
    pub const VERTEX_BUFFER_BIT_KHR: Self = Self(64);
    pub const COLOR_ATTACHMENT_BIT_KHR: Self = Self(128);
    pub const COLOR_ATTACHMENT_BLEND_BIT_KHR: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_BIT_KHR: Self = Self(512);
    pub const BLIT_SRC_BIT_KHR: Self = Self(1024);
    pub const BLIT_DST_BIT_KHR: Self = Self(2048);
    pub const SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR: Self = Self(4096);
    pub const TRANSFER_SRC_BIT_KHR: Self = Self(16384);
    pub const TRANSFER_DST_BIT_KHR: Self = Self(32768);
    pub const MIDPOINT_CHROMA_SAMPLES_BIT_KHR: Self = Self(131072);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: Self = Self(262144);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: Self = Self(524288);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: Self = Self(1048576);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: Self = Self(2097152);
    pub const DISJOINT_BIT_KHR: Self = Self(4194304);
    pub const COSITED_CHROMA_SAMPLES_BIT_KHR: Self = Self(8388608);
    pub const STORAGE_READ_WITHOUT_FORMAT_BIT_KHR: Self = Self(2147483648);
    pub const STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR: Self = Self(4294967296);
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR: Self = Self(8589934592);
    pub const SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR: Self = Self(65536);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: Self = Self(8192);
    pub const ACCELERATION_STRUCTURE_RADIUS_BUFFER_BIT_NV: Self = Self(2251799813685248);
    pub const LINEAR_COLOR_ATTACHMENT_BIT_NV: Self = Self(274877906944);
    pub const WEIGHT_IMAGE_BIT_QCOM: Self = Self(17179869184);
    pub const WEIGHT_SAMPLED_IMAGE_BIT_QCOM: Self = Self(34359738368);
    pub const BLOCK_MATCHING_BIT_QCOM: Self = Self(68719476736);
    pub const BOX_FILTER_SAMPLED_BIT_QCOM: Self = Self(137438953472);
    pub const TENSOR_SHADER_BIT_ARM: Self = Self(549755813888);
    pub const TENSOR_IMAGE_ALIASING_BIT_ARM: Self = Self(8796093022208);
    pub const OPTICAL_FLOW_IMAGE_BIT_NV: Self = Self(1099511627776);
    pub const OPTICAL_FLOW_VECTOR_BIT_NV: Self = Self(2199023255552);
    pub const OPTICAL_FLOW_COST_BIT_NV: Self = Self(4398046511104);
    pub const TENSOR_DATA_GRAPH_BIT_ARM: Self = Self(281474976710656);
    pub const RESERVED_60_BIT_EXT: Self = Self(1152921504606846976);
    pub const COPY_IMAGE_INDIRECT_DST_BIT_KHR: Self = Self(576460752303423488);
    pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_BIT_KHR: Self = Self(562949953421312);
    pub const VIDEO_ENCODE_EMPHASIS_MAP_BIT_KHR: Self = Self(1125899906842624);
    pub const SAMPLED_IMAGE_FILTER_LINEAR_2D_BIT_IMG: Self = Self(35184372088832);
    pub const DEPTH_COPY_ON_COMPUTE_QUEUE_BIT_KHR: Self = Self(4503599627370496);
    pub const DEPTH_COPY_ON_TRANSFER_QUEUE_BIT_KHR: Self = Self(9007199254740992);
    pub const STENCIL_COPY_ON_COMPUTE_QUEUE_BIT_KHR: Self = Self(18014398509481984);
    pub const STENCIL_COPY_ON_TRANSFER_QUEUE_BIT_KHR: Self = Self(36028797018963968);
    pub const DATA_GRAPH_OPTICAL_FLOW_IMAGE_BIT_ARM: Self = Self(72057594037927936);
    pub const DATA_GRAPH_OPTICAL_FLOW_VECTOR_BIT_ARM: Self = Self(144115188075855872);
    pub const DATA_GRAPH_OPTICAL_FLOW_COST_BIT_ARM: Self = Self(288230376151711744);
    pub const RESERVED_47_BIT_ARM: Self = Self(140737488355328);
    pub const RESERVED_61_BIT_HUAWEI: Self = Self(2305843009213693952);
}

impl From<VkFormatFeatureFlagBits2> for VkFormatFeatureFlags2 {
    #[inline]
    fn from(bits: VkFormatFeatureFlagBits2) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkFormatFeatureFlags2 {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkRenderingFlags(pub u32);

impl VkRenderingFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT: Self = Self(1);
    pub const SUSPENDING_BIT: Self = Self(2);
    pub const RESUMING_BIT: Self = Self(4);
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR: Self = Self(1);
    pub const SUSPENDING_BIT_KHR: Self = Self(2);
    pub const RESUMING_BIT_KHR: Self = Self(4);
    pub const RESERVED_9_BIT_IMG: Self = Self(512);
    pub const CONTENTS_INLINE_BIT_EXT: Self = Self(16);
    pub const ENABLE_LEGACY_DITHERING_BIT_EXT: Self = Self(8);
    pub const CONTENTS_INLINE_BIT_KHR: Self = Self(16);
    pub const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE: Self = Self(32);
    pub const FRAGMENT_REGION_BIT_EXT: Self = Self(64);
    pub const CUSTOM_RESOLVE_BIT_EXT: Self = Self(128);
    pub const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_BIT_KHR: Self = Self(256);
    pub const RESERVED_10_BIT_VALVE: Self = Self(1024);
    pub const RESERVED_11_BIT_VALVE: Self = Self(2048);
    pub const RESERVED_12_BIT_VALVE: Self = Self(4096);
}

impl From<VkRenderingFlagBits> for VkRenderingFlags {
    #[inline]
    fn from(bits: VkRenderingFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkRenderingFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkCompositeAlphaFlagsKHR(pub u32);

impl VkCompositeAlphaFlagsKHR {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: Self = Self(1);
    pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: Self = Self(2);
    pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: Self = Self(4);
    pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR: Self = Self(8);
}

impl From<VkCompositeAlphaFlagBitsKHR> for VkCompositeAlphaFlagsKHR {
    #[inline]
    fn from(bits: VkCompositeAlphaFlagBitsKHR) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkCompositeAlphaFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkSurfaceTransformFlagsKHR(pub u32);

impl VkSurfaceTransformFlagsKHR {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR: Self = Self(1);
    pub const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: Self = Self(2);
    pub const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: Self = Self(4);
    pub const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: Self = Self(8);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: Self = Self(16);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: Self = Self(32);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: Self = Self(64);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: Self = Self(128);
    pub const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR: Self = Self(256);
}

impl From<VkSurfaceTransformFlagBitsKHR> for VkSurfaceTransformFlagsKHR {
    #[inline]
    fn from(bits: VkSurfaceTransformFlagBitsKHR) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkSurfaceTransformFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkSwapchainCreateFlagsKHR(pub u32);

impl VkSwapchainCreateFlagsKHR {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: Self = Self(1);
    pub const VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR: Self = Self(2);
    pub const VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR: Self = Self(4);
    pub const VK_SWAPCHAIN_CREATE_PRESENT_TIMING_BIT_EXT: Self = Self(512);
    pub const VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_EXT: Self = Self(8);
    pub const VK_SWAPCHAIN_CREATE_RESERVED_5_BIT_EXT: Self = Self(32);
    pub const VK_SWAPCHAIN_CREATE_RESERVED_4_BIT_EXT: Self = Self(16);
    pub const VK_SWAPCHAIN_CREATE_PRESENT_ID_2_BIT_KHR: Self = Self(64);
    pub const VK_SWAPCHAIN_CREATE_PRESENT_WAIT_2_BIT_KHR: Self = Self(128);
    pub const VK_SWAPCHAIN_CREATE_DEFERRED_MEMORY_ALLOCATION_BIT_KHR: Self = Self(8);
    pub const VK_SWAPCHAIN_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT: Self = Self(256);
    pub const VK_SWAPCHAIN_CREATE_RESERVED_10_BIT_HUAWEI: Self = Self(1024);
}

impl From<VkSwapchainCreateFlagBitsKHR> for VkSwapchainCreateFlagsKHR {
    #[inline]
    fn from(bits: VkSwapchainCreateFlagBitsKHR) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkSwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkPeerMemoryFeatureFlags(pub u32);

impl VkPeerMemoryFeatureFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const COPY_SRC_BIT: Self = Self(1);
    pub const COPY_DST_BIT: Self = Self(2);
    pub const GENERIC_SRC_BIT: Self = Self(4);
    pub const GENERIC_DST_BIT: Self = Self(8);
    pub const COPY_SRC_BIT_KHR: Self = Self(1);
    pub const COPY_DST_BIT_KHR: Self = Self(2);
    pub const GENERIC_SRC_BIT_KHR: Self = Self(4);
    pub const GENERIC_DST_BIT_KHR: Self = Self(8);
}

impl From<VkPeerMemoryFeatureFlagBits> for VkPeerMemoryFeatureFlags {
    #[inline]
    fn from(bits: VkPeerMemoryFeatureFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkPeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkMemoryAllocateFlags(pub u32);

impl VkMemoryAllocateFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const DEVICE_MASK_BIT: Self = Self(1);
    pub const DEVICE_ADDRESS_BIT: Self = Self(2);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: Self = Self(4);
    pub const DEVICE_MASK_BIT_KHR: Self = Self(1);
    pub const DEVICE_ADDRESS_BIT_KHR: Self = Self(2);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: Self = Self(4);
    pub const ZERO_INITIALIZE_BIT_EXT: Self = Self(8);
}

impl From<VkMemoryAllocateFlagBits> for VkMemoryAllocateFlags {
    #[inline]
    fn from(bits: VkMemoryAllocateFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkMemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkDeviceGroupPresentModeFlagsKHR(pub u32);

impl VkDeviceGroupPresentModeFlagsKHR {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR: Self = Self(1);
    pub const VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR: Self = Self(2);
    pub const VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR: Self = Self(4);
    pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR: Self = Self(8);
}

impl From<VkDeviceGroupPresentModeFlagBitsKHR> for VkDeviceGroupPresentModeFlagsKHR {
    #[inline]
    fn from(bits: VkDeviceGroupPresentModeFlagBitsKHR) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkDeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkCommandPoolTrimFlags(pub u32);

impl VkCommandPoolTrimFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl core::ops::BitOr for VkCommandPoolTrimFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkExternalMemoryHandleTypeFlags(pub u32);

impl VkExternalMemoryHandleTypeFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const OPAQUE_FD_BIT: Self = Self(1);
    pub const OPAQUE_WIN32_BIT: Self = Self(2);
    pub const OPAQUE_WIN32_KMT_BIT: Self = Self(4);
    pub const D3D11_TEXTURE_BIT: Self = Self(8);
    pub const D3D11_TEXTURE_KMT_BIT: Self = Self(16);
    pub const D3D12_HEAP_BIT: Self = Self(32);
    pub const D3D12_RESOURCE_BIT: Self = Self(64);
    pub const OPAQUE_FD_BIT_KHR: Self = Self(1);
    pub const OPAQUE_WIN32_BIT_KHR: Self = Self(2);
    pub const OPAQUE_WIN32_KMT_BIT_KHR: Self = Self(4);
    pub const D3D11_TEXTURE_BIT_KHR: Self = Self(8);
    pub const D3D11_TEXTURE_KMT_BIT_KHR: Self = Self(16);
    pub const D3D12_HEAP_BIT_KHR: Self = Self(32);
    pub const D3D12_RESOURCE_BIT_KHR: Self = Self(64);
    pub const DMA_BUF_BIT_EXT: Self = Self(512);
    pub const ANDROID_HARDWARE_BUFFER_BIT_ANDROID: Self = Self(1024);
    pub const HOST_ALLOCATION_BIT_EXT: Self = Self(128);
    pub const HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT: Self = Self(256);
    pub const ZIRCON_VMO_BIT_FUCHSIA: Self = Self(2048);
    pub const RDMA_ADDRESS_BIT_NV: Self = Self(4096);
    pub const SCI_BUF_BIT_NV: Self = Self(8192);
    pub const OH_NATIVE_BUFFER_BIT_OHOS: Self = Self(32768);
    pub const SCREEN_BUFFER_BIT_QNX: Self = Self(16384);
    pub const MTLBUFFER_BIT_EXT: Self = Self(65536);
    pub const MTLTEXTURE_BIT_EXT: Self = Self(131072);
    pub const MTLHEAP_BIT_EXT: Self = Self(262144);
}

impl From<VkExternalMemoryHandleTypeFlagBits> for VkExternalMemoryHandleTypeFlags {
    #[inline]
    fn from(bits: VkExternalMemoryHandleTypeFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkExternalMemoryFeatureFlags(pub u32);

impl VkExternalMemoryFeatureFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const DEDICATED_ONLY_BIT: Self = Self(1);
    pub const EXPORTABLE_BIT: Self = Self(2);
    pub const IMPORTABLE_BIT: Self = Self(4);
    pub const DEDICATED_ONLY_BIT_KHR: Self = Self(1);
    pub const EXPORTABLE_BIT_KHR: Self = Self(2);
    pub const IMPORTABLE_BIT_KHR: Self = Self(4);
}

impl From<VkExternalMemoryFeatureFlagBits> for VkExternalMemoryFeatureFlags {
    #[inline]
    fn from(bits: VkExternalMemoryFeatureFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkExternalSemaphoreHandleTypeFlags(pub u32);

impl VkExternalSemaphoreHandleTypeFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const OPAQUE_FD_BIT: Self = Self(1);
    pub const OPAQUE_WIN32_BIT: Self = Self(2);
    pub const OPAQUE_WIN32_KMT_BIT: Self = Self(4);
    pub const D3D12_FENCE_BIT: Self = Self(8);
    pub const D3D11_FENCE_BIT: Self = Self(8);
    pub const SYNC_FD_BIT: Self = Self(16);
    pub const OPAQUE_FD_BIT_KHR: Self = Self(1);
    pub const OPAQUE_WIN32_BIT_KHR: Self = Self(2);
    pub const OPAQUE_WIN32_KMT_BIT_KHR: Self = Self(4);
    pub const D3D12_FENCE_BIT_KHR: Self = Self(8);
    pub const SYNC_FD_BIT_KHR: Self = Self(16);
    pub const ZIRCON_EVENT_BIT_FUCHSIA: Self = Self(128);
    pub const SCI_SYNC_OBJ_BIT_NV: Self = Self(32);
}

impl From<VkExternalSemaphoreHandleTypeFlagBits> for VkExternalSemaphoreHandleTypeFlags {
    #[inline]
    fn from(bits: VkExternalSemaphoreHandleTypeFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkExternalSemaphoreFeatureFlags(pub u32);

impl VkExternalSemaphoreFeatureFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const EXPORTABLE_BIT: Self = Self(1);
    pub const IMPORTABLE_BIT: Self = Self(2);
    pub const EXPORTABLE_BIT_KHR: Self = Self(1);
    pub const IMPORTABLE_BIT_KHR: Self = Self(2);
}

impl From<VkExternalSemaphoreFeatureFlagBits> for VkExternalSemaphoreFeatureFlags {
    #[inline]
    fn from(bits: VkExternalSemaphoreFeatureFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkSemaphoreImportFlags(pub u32);

impl VkSemaphoreImportFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const TEMPORARY_BIT: Self = Self(1);
    pub const TEMPORARY_BIT_KHR: Self = Self(1);
}

impl From<VkSemaphoreImportFlagBits> for VkSemaphoreImportFlags {
    #[inline]
    fn from(bits: VkSemaphoreImportFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkSemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkExternalFenceHandleTypeFlags(pub u32);

impl VkExternalFenceHandleTypeFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const OPAQUE_FD_BIT: Self = Self(1);
    pub const OPAQUE_WIN32_BIT: Self = Self(2);
    pub const OPAQUE_WIN32_KMT_BIT: Self = Self(4);
    pub const SYNC_FD_BIT: Self = Self(8);
    pub const OPAQUE_FD_BIT_KHR: Self = Self(1);
    pub const OPAQUE_WIN32_BIT_KHR: Self = Self(2);
    pub const OPAQUE_WIN32_KMT_BIT_KHR: Self = Self(4);
    pub const SYNC_FD_BIT_KHR: Self = Self(8);
    pub const SCI_SYNC_OBJ_BIT_NV: Self = Self(16);
    pub const SCI_SYNC_FENCE_BIT_NV: Self = Self(32);
}

impl From<VkExternalFenceHandleTypeFlagBits> for VkExternalFenceHandleTypeFlags {
    #[inline]
    fn from(bits: VkExternalFenceHandleTypeFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkExternalFenceFeatureFlags(pub u32);

impl VkExternalFenceFeatureFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const EXPORTABLE_BIT: Self = Self(1);
    pub const IMPORTABLE_BIT: Self = Self(2);
    pub const EXPORTABLE_BIT_KHR: Self = Self(1);
    pub const IMPORTABLE_BIT_KHR: Self = Self(2);
}

impl From<VkExternalFenceFeatureFlagBits> for VkExternalFenceFeatureFlags {
    #[inline]
    fn from(bits: VkExternalFenceFeatureFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkFenceImportFlags(pub u32);

impl VkFenceImportFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const TEMPORARY_BIT: Self = Self(1);
    pub const TEMPORARY_BIT_KHR: Self = Self(1);
}

impl From<VkFenceImportFlagBits> for VkFenceImportFlags {
    #[inline]
    fn from(bits: VkFenceImportFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkFenceImportFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkDescriptorBindingFlags(pub u32);

impl VkDescriptorBindingFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const UPDATE_AFTER_BIND_BIT: Self = Self(1);
    pub const UPDATE_UNUSED_WHILE_PENDING_BIT: Self = Self(2);
    pub const PARTIALLY_BOUND_BIT: Self = Self(4);
    pub const VARIABLE_DESCRIPTOR_COUNT_BIT: Self = Self(8);
    pub const UPDATE_AFTER_BIND_BIT_EXT: Self = Self(1);
    pub const UPDATE_UNUSED_WHILE_PENDING_BIT_EXT: Self = Self(2);
    pub const PARTIALLY_BOUND_BIT_EXT: Self = Self(4);
    pub const VARIABLE_DESCRIPTOR_COUNT_BIT_EXT: Self = Self(8);
    pub const RESERVED_4_BIT_QCOM: Self = Self(16);
}

impl From<VkDescriptorBindingFlagBits> for VkDescriptorBindingFlags {
    #[inline]
    fn from(bits: VkDescriptorBindingFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkDescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkResolveModeFlags(pub u32);

impl VkResolveModeFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const NONE: Self = Self(0);
    pub const SAMPLE_ZERO_BIT: Self = Self(1);
    pub const AVERAGE_BIT: Self = Self(2);
    pub const MIN_BIT: Self = Self(4);
    pub const MAX_BIT: Self = Self(8);
    pub const NONE_KHR: Self = Self(0);
    pub const SAMPLE_ZERO_BIT_KHR: Self = Self(1);
    pub const AVERAGE_BIT_KHR: Self = Self(2);
    pub const MIN_BIT_KHR: Self = Self(4);
    pub const MAX_BIT_KHR: Self = Self(8);
    pub const EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID: Self = Self(16);
    pub const EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID: Self = Self(16);
    pub const CUSTOM_BIT_EXT: Self = Self(32);
}

impl From<VkResolveModeFlagBits> for VkResolveModeFlags {
    #[inline]
    fn from(bits: VkResolveModeFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkResolveModeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkToolPurposeFlags(pub u32);

impl VkToolPurposeFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const VALIDATION_BIT: Self = Self(1);
    pub const PROFILING_BIT: Self = Self(2);
    pub const TRACING_BIT: Self = Self(4);
    pub const ADDITIONAL_FEATURES_BIT: Self = Self(8);
    pub const MODIFYING_FEATURES_BIT: Self = Self(16);
    pub const VALIDATION_BIT_EXT: Self = Self(1);
    pub const PROFILING_BIT_EXT: Self = Self(2);
    pub const TRACING_BIT_EXT: Self = Self(4);
    pub const ADDITIONAL_FEATURES_BIT_EXT: Self = Self(8);
    pub const MODIFYING_FEATURES_BIT_EXT: Self = Self(16);
    pub const DEBUG_REPORTING_BIT_EXT: Self = Self(32);
    pub const DEBUG_MARKERS_BIT_EXT: Self = Self(64);
}

impl From<VkToolPurposeFlagBits> for VkToolPurposeFlags {
    #[inline]
    fn from(bits: VkToolPurposeFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkToolPurposeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VkSubmitFlags(pub u32);

impl VkSubmitFlags {
    pub const EMPTY: Self = Self(0);
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const PROTECTED_BIT: Self = Self(1);
    pub const PROTECTED_BIT_KHR: Self = Self(1);
}

impl From<VkSubmitFlagBits> for VkSubmitFlags {
    #[inline]
    fn from(bits: VkSubmitFlagBits) -> Self { Self(bits.0) }
}

impl core::ops::BitOr for VkSubmitFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

// ---------------------------------------------------------------
// Callback types
//
// Wrapped in Option so that a null pointer is representable. A bare
// `fn` is a non-null type in Rust: zeroing a struct that holds one
// is undefined behaviour, and VkAllocationCallbacks is routinely
// zeroed.
// ---------------------------------------------------------------

pub type PFN_vkInternalAllocationNotification = Option<unsafe extern "system" fn(pUserData: *mut core::ffi::c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope)>;
pub type PFN_vkInternalFreeNotification = Option<unsafe extern "system" fn(pUserData: *mut core::ffi::c_void, size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope)>;
pub type PFN_vkReallocationFunction = Option<unsafe extern "system" fn(pUserData: *mut core::ffi::c_void, pOriginal: *mut core::ffi::c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut core::ffi::c_void>;
pub type PFN_vkAllocationFunction = Option<unsafe extern "system" fn(pUserData: *mut core::ffi::c_void, size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut core::ffi::c_void>;
pub type PFN_vkFreeFunction = Option<unsafe extern "system" fn(pUserData: *mut core::ffi::c_void, pMemory: *mut core::ffi::c_void)>;
pub type PFN_vkVoidFunction = Option<unsafe extern "system" fn()>;

// ---------------------------------------------------------------
// Structures
//
// No Debug or PartialEq derive: several of these embed unions, for
// which neither can be generated, and a surface that is derived on
// some types and not others is worse than one that is uniform.
//
// Default zeroes the struct and then writes the sType the registry
// fixes for it. A zeroed sType is VK_STRUCTURE_TYPE_APPLICATION_INFO,
// which is a valid value and therefore not a mistake the validation
// layers can reliably catch.
// ---------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBaseOutStructure {
    pub sType: VkStructureType,
    pub pNext: *mut VkBaseOutStructure,
}

impl Default for VkBaseOutStructure {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBaseInStructure {
    pub sType: VkStructureType,
    pub pNext: *const VkBaseInStructure,
}

impl Default for VkBaseInStructure {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkOffset2D {
    pub x: i32,
    pub y: i32,
}

impl Default for VkOffset2D {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkOffset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Default for VkOffset3D {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExtent2D {
    pub width: u32,
    pub height: u32,
}

impl Default for VkExtent2D {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl Default for VkExtent3D {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkViewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub minDepth: f32,
    pub maxDepth: f32,
}

impl Default for VkViewport {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}

impl Default for VkRect2D {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkClearRect {
    pub rect: VkRect2D,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

impl Default for VkClearRect {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkComponentMapping {
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}

impl Default for VkComponentMapping {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: VkPhysicalDeviceType,
    pub deviceName: [core::ffi::c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}

impl Default for VkPhysicalDeviceProperties {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExtensionProperties {
    pub extensionName: [core::ffi::c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub specVersion: u32,
}

impl Default for VkExtensionProperties {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkLayerProperties {
    pub layerName: [core::ffi::c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub specVersion: u32,
    pub implementationVersion: u32,
    pub description: [core::ffi::c_char; VK_MAX_DESCRIPTION_SIZE],
}

impl Default for VkLayerProperties {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkApplicationInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pApplicationName: *const core::ffi::c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const core::ffi::c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

impl Default for VkApplicationInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::APPLICATION_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAllocationCallbacks {
    pub pUserData: *mut core::ffi::c_void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    pub pfnInternalFree: PFN_vkInternalFreeNotification,
}

impl Default for VkAllocationCallbacks {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceQueueCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDeviceQueueCreateFlags,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const f32,
}

impl Default for VkDeviceQueueCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_QUEUE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDeviceCreateFlags,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const core::ffi::c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const core::ffi::c_char,
    pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}

impl Default for VkDeviceCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkInstanceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkInstanceCreateFlags,
    pub pApplicationInfo: *const VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const core::ffi::c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const core::ffi::c_char,
}

impl Default for VkInstanceCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::INSTANCE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkQueueFamilyProperties {
    pub queueFlags: VkQueueFlags,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: VkExtent3D,
}

impl Default for VkQueueFamilyProperties {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_TYPES],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

impl Default for VkPhysicalDeviceMemoryProperties {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub allocationSize: VkDeviceSize,
    pub memoryTypeIndex: u32,
}

impl Default for VkMemoryAllocateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::MEMORY_ALLOCATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memoryTypeBits: u32,
}

impl Default for VkMemoryRequirements {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSparseImageFormatProperties {
    pub aspectMask: VkImageAspectFlags,
    pub imageGranularity: VkExtent3D,
    pub flags: VkSparseImageFormatFlags,
}

impl Default for VkSparseImageFormatProperties {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSparseImageMemoryRequirements {
    pub formatProperties: VkSparseImageFormatProperties,
    pub imageMipTailFirstLod: u32,
    pub imageMipTailSize: VkDeviceSize,
    pub imageMipTailOffset: VkDeviceSize,
    pub imageMipTailStride: VkDeviceSize,
}

impl Default for VkSparseImageMemoryRequirements {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryType {
    pub propertyFlags: VkMemoryPropertyFlags,
    pub heapIndex: u32,
}

impl Default for VkMemoryType {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}

impl Default for VkMemoryHeap {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMappedMemoryRange {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

impl Default for VkMappedMemoryRange {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::MAPPED_MEMORY_RANGE;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkFormatProperties {
    pub linearTilingFeatures: VkFormatFeatureFlags,
    pub optimalTilingFeatures: VkFormatFeatureFlags,
    pub bufferFeatures: VkFormatFeatureFlags,
}

impl Default for VkFormatProperties {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageFormatProperties {
    pub maxExtent: VkExtent3D,
    pub maxMipLevels: u32,
    pub maxArrayLayers: u32,
    pub sampleCounts: VkSampleCountFlags,
    pub maxResourceSize: VkDeviceSize,
}

impl Default for VkImageFormatProperties {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorBufferInfo {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

impl Default for VkDescriptorBufferInfo {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorImageInfo {
    pub sampler: VkSampler,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
}

impl Default for VkDescriptorImageInfo {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkWriteDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub pImageInfo: *const VkDescriptorImageInfo,
    pub pBufferInfo: *const VkDescriptorBufferInfo,
    pub pTexelBufferView: *const VkBufferView,
}

impl Default for VkWriteDescriptorSet {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::WRITE_DESCRIPTOR_SET;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCopyDescriptorSet {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcSet: VkDescriptorSet,
    pub srcBinding: u32,
    pub srcArrayElement: u32,
    pub dstSet: VkDescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
}

impl Default for VkCopyDescriptorSet {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::COPY_DESCRIPTOR_SET;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkBufferCreateFlags,
    pub size: VkDeviceSize,
    pub usage: VkBufferUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}

impl Default for VkBufferCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BUFFER_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkBufferViewCreateFlags,
    pub buffer: VkBuffer,
    pub format: VkFormat,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

impl Default for VkBufferViewCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BUFFER_VIEW_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageSubresource {
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub arrayLayer: u32,
}

impl Default for VkImageSubresource {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageSubresourceLayers {
    pub aspectMask: VkImageAspectFlags,
    pub mipLevel: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

impl Default for VkImageSubresourceLayers {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageSubresourceRange {
    pub aspectMask: VkImageAspectFlags,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

impl Default for VkImageSubresourceRange {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
}

impl Default for VkMemoryBarrier {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::MEMORY_BARRIER;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

impl Default for VkBufferMemoryBarrier {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BUFFER_MEMORY_BARRIER;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageMemoryBarrier {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: VkImage,
    pub subresourceRange: VkImageSubresourceRange,
}

impl Default for VkImageMemoryBarrier {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_MEMORY_BARRIER;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkImageCreateFlags,
    pub imageType: VkImageType,
    pub format: VkFormat,
    pub extent: VkExtent3D,
    pub mipLevels: u32,
    pub arrayLayers: u32,
    pub samples: VkSampleCountFlagBits,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub sharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub initialLayout: VkImageLayout,
}

impl Default for VkImageCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubresourceLayout {
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub rowPitch: VkDeviceSize,
    pub arrayPitch: VkDeviceSize,
    pub depthPitch: VkDeviceSize,
}

impl Default for VkSubresourceLayout {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageViewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkImageViewCreateFlags,
    pub image: VkImage,
    pub viewType: VkImageViewType,
    pub format: VkFormat,
    pub components: VkComponentMapping,
    pub subresourceRange: VkImageSubresourceRange,
}

impl Default for VkImageViewCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_VIEW_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferCopy {
    pub srcOffset: VkDeviceSize,
    pub dstOffset: VkDeviceSize,
    pub size: VkDeviceSize,
}

impl Default for VkBufferCopy {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSparseMemoryBind {
    pub resourceOffset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

impl Default for VkSparseMemoryBind {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSparseImageMemoryBind {
    pub subresource: VkImageSubresource,
    pub offset: VkOffset3D,
    pub extent: VkExtent3D,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

impl Default for VkSparseImageMemoryBind {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSparseBufferMemoryBindInfo {
    pub buffer: VkBuffer,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

impl Default for VkSparseBufferMemoryBindInfo {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseMemoryBind,
}

impl Default for VkSparseImageOpaqueMemoryBindInfo {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSparseImageMemoryBindInfo {
    pub image: VkImage,
    pub bindCount: u32,
    pub pBinds: *const VkSparseImageMemoryBind,
}

impl Default for VkSparseImageMemoryBindInfo {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBindSparseInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub bufferBindCount: u32,
    pub pBufferBinds: *const VkSparseBufferMemoryBindInfo,
    pub imageOpaqueBindCount: u32,
    pub pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo,
    pub imageBindCount: u32,
    pub pImageBinds: *const VkSparseImageMemoryBindInfo,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}

impl Default for VkBindSparseInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BIND_SPARSE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageCopy {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

impl Default for VkImageCopy {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageBlit {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffsets: [VkOffset3D; 2],
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffsets: [VkOffset3D; 2],
}

impl Default for VkImageBlit {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferImageCopy {
    pub bufferOffset: VkDeviceSize,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

impl Default for VkBufferImageCopy {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageResolve {
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

impl Default for VkImageResolve {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkShaderModuleCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkShaderModuleCreateFlags,
    pub codeSize: usize,
    pub pCode: *const u32,
}

impl Default for VkShaderModuleCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SHADER_MODULE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: VkDescriptorType,
    pub descriptorCount: u32,
    pub stageFlags: VkShaderStageFlags,
    pub pImmutableSamplers: *const VkSampler,
}

impl Default for VkDescriptorSetLayoutBinding {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorSetLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDescriptorSetLayoutCreateFlags,
    pub bindingCount: u32,
    pub pBindings: *const VkDescriptorSetLayoutBinding,
}

impl Default for VkDescriptorSetLayoutCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorPoolSize {
    pub r#type: VkDescriptorType,
    pub descriptorCount: u32,
}

impl Default for VkDescriptorPoolSize {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDescriptorPoolCreateFlags,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: *const VkDescriptorPoolSize,
}

impl Default for VkDescriptorPoolCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DESCRIPTOR_POOL_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorSetAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub descriptorPool: VkDescriptorPool,
    pub descriptorSetCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
}

impl Default for VkDescriptorSetAllocateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DESCRIPTOR_SET_ALLOCATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSpecializationMapEntry {
    pub constantID: u32,
    pub offset: u32,
    pub size: usize,
}

impl Default for VkSpecializationMapEntry {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const VkSpecializationMapEntry,
    pub dataSize: usize,
    pub pData: *const core::ffi::c_void,
}

impl Default for VkSpecializationInfo {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineShaderStageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineShaderStageCreateFlags,
    pub stage: VkShaderStageFlagBits,
    pub module: VkShaderModule,
    pub pName: *const core::ffi::c_char,
    pub pSpecializationInfo: *const VkSpecializationInfo,
}

impl Default for VkPipelineShaderStageCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_SHADER_STAGE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkComputePipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineCreateFlags,
    pub stage: VkPipelineShaderStageCreateInfo,
    pub layout: VkPipelineLayout,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

impl Default for VkComputePipelineCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::COMPUTE_PIPELINE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkVertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub inputRate: VkVertexInputRate,
}

impl Default for VkVertexInputBindingDescription {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkVertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: VkFormat,
    pub offset: u32,
}

impl Default for VkVertexInputAttributeDescription {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineVertexInputStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineVertexInputStateCreateFlags,
    pub vertexBindingDescriptionCount: u32,
    pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
    pub vertexAttributeDescriptionCount: u32,
    pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
}

impl Default for VkPipelineVertexInputStateCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineInputAssemblyStateCreateFlags,
    pub topology: VkPrimitiveTopology,
    pub primitiveRestartEnable: VkBool32,
}

impl Default for VkPipelineInputAssemblyStateCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineTessellationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineTessellationStateCreateFlags,
    pub patchControlPoints: u32,
}

impl Default for VkPipelineTessellationStateCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineViewportStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineViewportStateCreateFlags,
    pub viewportCount: u32,
    pub pViewports: *const VkViewport,
    pub scissorCount: u32,
    pub pScissors: *const VkRect2D,
}

impl Default for VkPipelineViewportStateCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineRasterizationStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineRasterizationStateCreateFlags,
    pub depthClampEnable: VkBool32,
    pub rasterizerDiscardEnable: VkBool32,
    pub polygonMode: VkPolygonMode,
    pub cullMode: VkCullModeFlags,
    pub frontFace: VkFrontFace,
    pub depthBiasEnable: VkBool32,
    pub depthBiasConstantFactor: f32,
    pub depthBiasClamp: f32,
    pub depthBiasSlopeFactor: f32,
    pub lineWidth: f32,
}

impl Default for VkPipelineRasterizationStateCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineMultisampleStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineMultisampleStateCreateFlags,
    pub rasterizationSamples: VkSampleCountFlagBits,
    pub sampleShadingEnable: VkBool32,
    pub minSampleShading: f32,
    pub pSampleMask: *const VkSampleMask,
    pub alphaToCoverageEnable: VkBool32,
    pub alphaToOneEnable: VkBool32,
}

impl Default for VkPipelineMultisampleStateCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineColorBlendAttachmentState {
    pub blendEnable: VkBool32,
    pub srcColorBlendFactor: VkBlendFactor,
    pub dstColorBlendFactor: VkBlendFactor,
    pub colorBlendOp: VkBlendOp,
    pub srcAlphaBlendFactor: VkBlendFactor,
    pub dstAlphaBlendFactor: VkBlendFactor,
    pub alphaBlendOp: VkBlendOp,
    pub colorWriteMask: VkColorComponentFlags,
}

impl Default for VkPipelineColorBlendAttachmentState {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineColorBlendStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineColorBlendStateCreateFlags,
    pub logicOpEnable: VkBool32,
    pub logicOp: VkLogicOp,
    pub attachmentCount: u32,
    pub pAttachments: *const VkPipelineColorBlendAttachmentState,
    pub blendConstants: [f32; 4],
}

impl Default for VkPipelineColorBlendStateCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineDynamicStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineDynamicStateCreateFlags,
    pub dynamicStateCount: u32,
    pub pDynamicStates: *const VkDynamicState,
}

impl Default for VkPipelineDynamicStateCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkStencilOpState {
    pub failOp: VkStencilOp,
    pub passOp: VkStencilOp,
    pub depthFailOp: VkStencilOp,
    pub compareOp: VkCompareOp,
    pub compareMask: u32,
    pub writeMask: u32,
    pub reference: u32,
}

impl Default for VkStencilOpState {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineDepthStencilStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineDepthStencilStateCreateFlags,
    pub depthTestEnable: VkBool32,
    pub depthWriteEnable: VkBool32,
    pub depthCompareOp: VkCompareOp,
    pub depthBoundsTestEnable: VkBool32,
    pub stencilTestEnable: VkBool32,
    pub front: VkStencilOpState,
    pub back: VkStencilOpState,
    pub minDepthBounds: f32,
    pub maxDepthBounds: f32,
}

impl Default for VkPipelineDepthStencilStateCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkGraphicsPipelineCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineCreateFlags,
    pub stageCount: u32,
    pub pStages: *const VkPipelineShaderStageCreateInfo,
    pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
    pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
    pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
    pub pViewportState: *const VkPipelineViewportStateCreateInfo,
    pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
    pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
    pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
    pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
    pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
    pub layout: VkPipelineLayout,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub basePipelineHandle: VkPipeline,
    pub basePipelineIndex: i32,
}

impl Default for VkGraphicsPipelineCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::GRAPHICS_PIPELINE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineCacheCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineCacheCreateFlags,
    pub initialDataSize: usize,
    pub pInitialData: *const core::ffi::c_void,
}

impl Default for VkPipelineCacheCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_CACHE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineCacheHeaderVersionOne {
    pub headerSize: u32,
    pub headerVersion: VkPipelineCacheHeaderVersion,
    pub vendorID: u32,
    pub deviceID: u32,
    pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
}

impl Default for VkPipelineCacheHeaderVersionOne {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPushConstantRange {
    pub stageFlags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

impl Default for VkPushConstantRange {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineLayoutCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPipelineLayoutCreateFlags,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const VkDescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const VkPushConstantRange,
}

impl Default for VkPipelineLayoutCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_LAYOUT_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSamplerCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkSamplerCreateFlags,
    pub magFilter: VkFilter,
    pub minFilter: VkFilter,
    pub mipmapMode: VkSamplerMipmapMode,
    pub addressModeU: VkSamplerAddressMode,
    pub addressModeV: VkSamplerAddressMode,
    pub addressModeW: VkSamplerAddressMode,
    pub mipLodBias: f32,
    pub anisotropyEnable: VkBool32,
    pub maxAnisotropy: f32,
    pub compareEnable: VkBool32,
    pub compareOp: VkCompareOp,
    pub minLod: f32,
    pub maxLod: f32,
    pub borderColor: VkBorderColor,
    pub unnormalizedCoordinates: VkBool32,
}

impl Default for VkSamplerCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SAMPLER_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCommandPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkCommandPoolCreateFlags,
    pub queueFamilyIndex: u32,
}

impl Default for VkCommandPoolCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::COMMAND_POOL_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCommandBufferAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub commandPool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub commandBufferCount: u32,
}

impl Default for VkCommandBufferAllocateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::COMMAND_BUFFER_ALLOCATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCommandBufferInheritanceInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub renderPass: VkRenderPass,
    pub subpass: u32,
    pub framebuffer: VkFramebuffer,
    pub occlusionQueryEnable: VkBool32,
    pub queryFlags: VkQueryControlFlags,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

impl Default for VkCommandBufferInheritanceInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::COMMAND_BUFFER_INHERITANCE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCommandBufferBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkCommandBufferUsageFlags,
    pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}

impl Default for VkCommandBufferBeginInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::COMMAND_BUFFER_BEGIN_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRenderPassBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub renderPass: VkRenderPass,
    pub framebuffer: VkFramebuffer,
    pub renderArea: VkRect2D,
    pub clearValueCount: u32,
    pub pClearValues: *const VkClearValue,
}

impl Default for VkRenderPassBeginInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::RENDER_PASS_BEGIN_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union VkClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

impl Default for VkClearColorValue {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

impl Default for VkClearDepthStencilValue {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union VkClearValue {
    pub color: VkClearColorValue,
    pub depthStencil: VkClearDepthStencilValue,
}

impl Default for VkClearValue {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkClearAttachment {
    pub aspectMask: VkImageAspectFlags,
    pub colorAttachment: u32,
    pub clearValue: VkClearValue,
}

impl Default for VkClearAttachment {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAttachmentDescription {
    pub flags: VkAttachmentDescriptionFlags,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub loadOp: VkAttachmentLoadOp,
    pub storeOp: VkAttachmentStoreOp,
    pub stencilLoadOp: VkAttachmentLoadOp,
    pub stencilStoreOp: VkAttachmentStoreOp,
    pub initialLayout: VkImageLayout,
    pub finalLayout: VkImageLayout,
}

impl Default for VkAttachmentDescription {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: VkImageLayout,
}

impl Default for VkAttachmentReference {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubpassDescription {
    pub flags: VkSubpassDescriptionFlags,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const VkAttachmentReference,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkAttachmentReference,
    pub pResolveAttachments: *const VkAttachmentReference,
    pub pDepthStencilAttachment: *const VkAttachmentReference,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}

impl Default for VkSubpassDescription {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubpassDependency {
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlags,
    pub dstStageMask: VkPipelineStageFlags,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub dependencyFlags: VkDependencyFlags,
}

impl Default for VkSubpassDependency {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRenderPassCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkRenderPassCreateFlags,
    pub attachmentCount: u32,
    pub pAttachments: *const VkAttachmentDescription,
    pub subpassCount: u32,
    pub pSubpasses: *const VkSubpassDescription,
    pub dependencyCount: u32,
    pub pDependencies: *const VkSubpassDependency,
}

impl Default for VkRenderPassCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::RENDER_PASS_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkEventCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkEventCreateFlags,
}

impl Default for VkEventCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::EVENT_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkFenceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkFenceCreateFlags,
}

impl Default for VkFenceCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::FENCE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceFeatures {
    pub robustBufferAccess: VkBool32,
    pub fullDrawIndexUint32: VkBool32,
    pub imageCubeArray: VkBool32,
    pub independentBlend: VkBool32,
    pub geometryShader: VkBool32,
    pub tessellationShader: VkBool32,
    pub sampleRateShading: VkBool32,
    pub dualSrcBlend: VkBool32,
    pub logicOp: VkBool32,
    pub multiDrawIndirect: VkBool32,
    pub drawIndirectFirstInstance: VkBool32,
    pub depthClamp: VkBool32,
    pub depthBiasClamp: VkBool32,
    pub fillModeNonSolid: VkBool32,
    pub depthBounds: VkBool32,
    pub wideLines: VkBool32,
    pub largePoints: VkBool32,
    pub alphaToOne: VkBool32,
    pub multiViewport: VkBool32,
    pub samplerAnisotropy: VkBool32,
    pub textureCompressionETC2: VkBool32,
    pub textureCompressionASTC_LDR: VkBool32,
    pub textureCompressionBC: VkBool32,
    pub occlusionQueryPrecise: VkBool32,
    pub pipelineStatisticsQuery: VkBool32,
    pub vertexPipelineStoresAndAtomics: VkBool32,
    pub fragmentStoresAndAtomics: VkBool32,
    pub shaderTessellationAndGeometryPointSize: VkBool32,
    pub shaderImageGatherExtended: VkBool32,
    pub shaderStorageImageExtendedFormats: VkBool32,
    pub shaderStorageImageMultisample: VkBool32,
    pub shaderStorageImageReadWithoutFormat: VkBool32,
    pub shaderStorageImageWriteWithoutFormat: VkBool32,
    pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
    pub shaderSampledImageArrayDynamicIndexing: VkBool32,
    pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageImageArrayDynamicIndexing: VkBool32,
    pub shaderClipDistance: VkBool32,
    pub shaderCullDistance: VkBool32,
    pub shaderFloat64: VkBool32,
    pub shaderInt64: VkBool32,
    pub shaderInt16: VkBool32,
    pub shaderResourceResidency: VkBool32,
    pub shaderResourceMinLod: VkBool32,
    pub sparseBinding: VkBool32,
    pub sparseResidencyBuffer: VkBool32,
    pub sparseResidencyImage2D: VkBool32,
    pub sparseResidencyImage3D: VkBool32,
    pub sparseResidency2Samples: VkBool32,
    pub sparseResidency4Samples: VkBool32,
    pub sparseResidency8Samples: VkBool32,
    pub sparseResidency16Samples: VkBool32,
    pub sparseResidencyAliased: VkBool32,
    pub variableMultisampleRate: VkBool32,
    pub inheritedQueries: VkBool32,
}

impl Default for VkPhysicalDeviceFeatures {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: VkBool32,
    pub residencyStandard2DMultisampleBlockShape: VkBool32,
    pub residencyStandard3DBlockShape: VkBool32,
    pub residencyAlignedMipSize: VkBool32,
    pub residencyNonResidentStrict: VkBool32,
}

impl Default for VkPhysicalDeviceSparseProperties {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceLimits {
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: VkDeviceSize,
    pub sparseAddressSpaceSize: VkDeviceSize,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: f32,
    pub maxSamplerAnisotropy: f32,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2],
    pub viewportBoundsRange: [f32; 2],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment: usize,
    pub minTexelBufferOffsetAlignment: VkDeviceSize,
    pub minUniformBufferOffsetAlignment: VkDeviceSize,
    pub minStorageBufferOffsetAlignment: VkDeviceSize,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: f32,
    pub maxInterpolationOffset: f32,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: VkSampleCountFlags,
    pub framebufferDepthSampleCounts: VkSampleCountFlags,
    pub framebufferStencilSampleCounts: VkSampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: VkSampleCountFlags,
    pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
    pub sampledImageDepthSampleCounts: VkSampleCountFlags,
    pub sampledImageStencilSampleCounts: VkSampleCountFlags,
    pub storageImageSampleCounts: VkSampleCountFlags,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: VkBool32,
    pub timestampPeriod: f32,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [f32; 2],
    pub lineWidthRange: [f32; 2],
    pub pointSizeGranularity: f32,
    pub lineWidthGranularity: f32,
    pub strictLines: VkBool32,
    pub standardSampleLocations: VkBool32,
    pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
    pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
    pub nonCoherentAtomSize: VkDeviceSize,
}

impl Default for VkPhysicalDeviceLimits {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSemaphoreCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkSemaphoreCreateFlags,
}

impl Default for VkSemaphoreCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SEMAPHORE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkQueryPoolCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkQueryPoolCreateFlags,
    pub queryType: VkQueryType,
    pub queryCount: u32,
    pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

impl Default for VkQueryPoolCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::QUERY_POOL_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkFramebufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkFramebufferCreateFlags,
    pub renderPass: VkRenderPass,
    pub attachmentCount: u32,
    pub pAttachments: *const VkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

impl Default for VkFramebufferCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::FRAMEBUFFER_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDrawIndirectCommand {
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub firstVertex: u32,
    pub firstInstance: u32,
}

impl Default for VkDrawIndirectCommand {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDrawIndexedIndirectCommand {
    pub indexCount: u32,
    pub instanceCount: u32,
    pub firstIndex: u32,
    pub vertexOffset: i32,
    pub firstInstance: u32,
}

impl Default for VkDrawIndexedIndirectCommand {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Default for VkDispatchIndirectCommand {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub pWaitDstStageMask: *const VkPipelineStageFlags,
    pub commandBufferCount: u32,
    pub pCommandBuffers: *const VkCommandBuffer,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const VkSemaphore,
}

impl Default for VkSubmitInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SUBMIT_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSurfaceCapabilitiesKHR {
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: VkExtent2D,
    pub minImageExtent: VkExtent2D,
    pub maxImageExtent: VkExtent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: VkSurfaceTransformFlagsKHR,
    pub currentTransform: VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags: VkImageUsageFlags,
}

impl Default for VkSurfaceCapabilitiesKHR {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSurfaceFormatKHR {
    pub format: VkFormat,
    pub colorSpace: VkColorSpaceKHR,
}

impl Default for VkSurfaceFormatKHR {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkSwapchainCreateFlagsKHR,
    pub surface: VkSurfaceKHR,
    pub minImageCount: u32,
    pub imageFormat: VkFormat,
    pub imageColorSpace: VkColorSpaceKHR,
    pub imageExtent: VkExtent2D,
    pub imageArrayLayers: u32,
    pub imageUsage: VkImageUsageFlags,
    pub imageSharingMode: VkSharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub preTransform: VkSurfaceTransformFlagBitsKHR,
    pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
    pub presentMode: VkPresentModeKHR,
    pub clipped: VkBool32,
    pub oldSwapchain: VkSwapchainKHR,
}

impl Default for VkSwapchainCreateInfoKHR {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SWAPCHAIN_CREATE_INFO_KHR;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const VkSemaphore,
    pub swapchainCount: u32,
    pub pSwapchains: *const VkSwapchainKHR,
    pub pImageIndices: *const u32,
    pub pResults: *mut VkResult,
}

impl Default for VkPresentInfoKHR {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PRESENT_INFO_KHR;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDevicePrivateDataCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub privateDataSlotRequestCount: u32,
}

impl Default for VkDevicePrivateDataCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_PRIVATE_DATA_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPrivateDataSlotCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkPrivateDataSlotCreateFlags,
}

impl Default for VkPrivateDataSlotCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PRIVATE_DATA_SLOT_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDevicePrivateDataFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub privateData: VkBool32,
}

impl Default for VkPhysicalDevicePrivateDataFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceFeatures2 {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub features: VkPhysicalDeviceFeatures,
}

impl Default for VkPhysicalDeviceFeatures2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_FEATURES_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub properties: VkPhysicalDeviceProperties,
}

impl Default for VkPhysicalDeviceProperties2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_PROPERTIES_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkFormatProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub formatProperties: VkFormatProperties,
}

impl Default for VkFormatProperties2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::FORMAT_PROPERTIES_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageFormatProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub imageFormatProperties: VkImageFormatProperties,
}

impl Default for VkImageFormatProperties2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_FORMAT_PROPERTIES_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceImageFormatInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub format: VkFormat,
    pub r#type: VkImageType,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub flags: VkImageCreateFlags,
}

impl Default for VkPhysicalDeviceImageFormatInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkQueueFamilyProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub queueFamilyProperties: VkQueueFamilyProperties,
}

impl Default for VkQueueFamilyProperties2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::QUEUE_FAMILY_PROPERTIES_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceMemoryProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub memoryProperties: VkPhysicalDeviceMemoryProperties,
}

impl Default for VkPhysicalDeviceMemoryProperties2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSparseImageFormatProperties2 {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub properties: VkSparseImageFormatProperties,
}

impl Default for VkSparseImageFormatProperties2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SPARSE_IMAGE_FORMAT_PROPERTIES_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub format: VkFormat,
    pub r#type: VkImageType,
    pub samples: VkSampleCountFlagBits,
    pub usage: VkImageUsageFlags,
    pub tiling: VkImageTiling,
}

impl Default for VkPhysicalDeviceSparseImageFormatInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkConformanceVersion {
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
    pub patch: u8,
}

impl Default for VkConformanceVersion {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceDriverProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub driverID: VkDriverId,
    pub driverName: [core::ffi::c_char; VK_MAX_DRIVER_NAME_SIZE],
    pub driverInfo: [core::ffi::c_char; VK_MAX_DRIVER_INFO_SIZE],
    pub conformanceVersion: VkConformanceVersion,
}

impl Default for VkPhysicalDeviceDriverProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_DRIVER_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceVariablePointersFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub variablePointersStorageBuffer: VkBool32,
    pub variablePointers: VkBool32,
}

impl Default for VkPhysicalDeviceVariablePointersFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceVariablePointerFeatures {
}

impl Default for VkPhysicalDeviceVariablePointerFeatures {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExternalMemoryProperties {
    pub externalMemoryFeatures: VkExternalMemoryFeatureFlags,
    pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlags,
    pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlags,
}

impl Default for VkExternalMemoryProperties {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceExternalImageFormatInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

impl Default for VkPhysicalDeviceExternalImageFormatInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExternalImageFormatProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub externalMemoryProperties: VkExternalMemoryProperties,
}

impl Default for VkExternalImageFormatProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::EXTERNAL_IMAGE_FORMAT_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceExternalBufferInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkBufferCreateFlags,
    pub usage: VkBufferUsageFlags,
    pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

impl Default for VkPhysicalDeviceExternalBufferInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExternalBufferProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub externalMemoryProperties: VkExternalMemoryProperties,
}

impl Default for VkExternalBufferProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::EXTERNAL_BUFFER_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceIDProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub deviceUUID: [u8; VK_UUID_SIZE],
    pub driverUUID: [u8; VK_UUID_SIZE],
    pub deviceLUID: [u8; VK_LUID_SIZE],
    pub deviceNodeMask: u32,
    pub deviceLUIDValid: VkBool32,
}

impl Default for VkPhysicalDeviceIDProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_ID_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExternalMemoryImageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlags,
}

impl Default for VkExternalMemoryImageCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExternalMemoryBufferCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlags,
}

impl Default for VkExternalMemoryBufferCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExportMemoryAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleTypes: VkExternalMemoryHandleTypeFlags,
}

impl Default for VkExportMemoryAllocateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::EXPORT_MEMORY_ALLOCATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceExternalSemaphoreInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}

impl Default for VkPhysicalDeviceExternalSemaphoreInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExternalSemaphoreProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlags,
    pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlags,
    pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlags,
}

impl Default for VkExternalSemaphoreProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::EXTERNAL_SEMAPHORE_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExportSemaphoreCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleTypes: VkExternalSemaphoreHandleTypeFlags,
}

impl Default for VkExportSemaphoreCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::EXPORT_SEMAPHORE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceExternalFenceInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleType: VkExternalFenceHandleTypeFlagBits,
}

impl Default for VkPhysicalDeviceExternalFenceInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExternalFenceProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlags,
    pub compatibleHandleTypes: VkExternalFenceHandleTypeFlags,
    pub externalFenceFeatures: VkExternalFenceFeatureFlags,
}

impl Default for VkExternalFenceProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::EXTERNAL_FENCE_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkExportFenceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub handleTypes: VkExternalFenceHandleTypeFlags,
}

impl Default for VkExportFenceCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::EXPORT_FENCE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceMultiviewFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub multiview: VkBool32,
    pub multiviewGeometryShader: VkBool32,
    pub multiviewTessellationShader: VkBool32,
}

impl Default for VkPhysicalDeviceMultiviewFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceMultiviewProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxMultiviewViewCount: u32,
    pub maxMultiviewInstanceIndex: u32,
}

impl Default for VkPhysicalDeviceMultiviewProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRenderPassMultiviewCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub subpassCount: u32,
    pub pViewMasks: *const u32,
    pub dependencyCount: u32,
    pub pViewOffsets: *const i32,
    pub correlationMaskCount: u32,
    pub pCorrelationMasks: *const u32,
}

impl Default for VkRenderPassMultiviewCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::RENDER_PASS_MULTIVIEW_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceGroupProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub physicalDeviceCount: u32,
    pub physicalDevices: [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE],
    pub subsetAllocation: VkBool32,
}

impl Default for VkPhysicalDeviceGroupProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_GROUP_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryAllocateFlagsInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkMemoryAllocateFlags,
    pub deviceMask: u32,
}

impl Default for VkMemoryAllocateFlagsInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::MEMORY_ALLOCATE_FLAGS_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBindBufferMemoryInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub buffer: VkBuffer,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
}

impl Default for VkBindBufferMemoryInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BIND_BUFFER_MEMORY_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBindBufferMemoryDeviceGroupInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
}

impl Default for VkBindBufferMemoryDeviceGroupInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBindImageMemoryInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub image: VkImage,
    pub memory: VkDeviceMemory,
    pub memoryOffset: VkDeviceSize,
}

impl Default for VkBindImageMemoryInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BIND_IMAGE_MEMORY_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBindImageMemoryDeviceGroupInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub deviceIndexCount: u32,
    pub pDeviceIndices: *const u32,
    pub splitInstanceBindRegionCount: u32,
    pub pSplitInstanceBindRegions: *const VkRect2D,
}

impl Default for VkBindImageMemoryDeviceGroupInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceGroupRenderPassBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub deviceMask: u32,
    pub deviceRenderAreaCount: u32,
    pub pDeviceRenderAreas: *const VkRect2D,
}

impl Default for VkDeviceGroupRenderPassBeginInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceGroupCommandBufferBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub deviceMask: u32,
}

impl Default for VkDeviceGroupCommandBufferBeginInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceGroupSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphoreDeviceIndices: *const u32,
    pub commandBufferCount: u32,
    pub pCommandBufferDeviceMasks: *const u32,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphoreDeviceIndices: *const u32,
}

impl Default for VkDeviceGroupSubmitInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_GROUP_SUBMIT_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceGroupBindSparseInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub resourceDeviceIndex: u32,
    pub memoryDeviceIndex: u32,
}

impl Default for VkDeviceGroupBindSparseInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_GROUP_BIND_SPARSE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceGroupPresentCapabilitiesKHR {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub presentMask: [u32; VK_MAX_DEVICE_GROUP_SIZE],
    pub modes: VkDeviceGroupPresentModeFlagsKHR,
}

impl Default for VkDeviceGroupPresentCapabilitiesKHR {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub swapchain: VkSwapchainKHR,
}

impl Default for VkImageSwapchainCreateInfoKHR {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHR;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBindImageMemorySwapchainInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub swapchain: VkSwapchainKHR,
    pub imageIndex: u32,
}

impl Default for VkBindImageMemorySwapchainInfoKHR {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAcquireNextImageInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub swapchain: VkSwapchainKHR,
    pub timeout: u64,
    pub semaphore: VkSemaphore,
    pub fence: VkFence,
    pub deviceMask: u32,
}

impl Default for VkAcquireNextImageInfoKHR {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::ACQUIRE_NEXT_IMAGE_INFO_KHR;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceGroupPresentInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub swapchainCount: u32,
    pub pDeviceMasks: *const u32,
    pub mode: VkDeviceGroupPresentModeFlagBitsKHR,
}

impl Default for VkDeviceGroupPresentInfoKHR {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_GROUP_PRESENT_INFO_KHR;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceGroupDeviceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub physicalDeviceCount: u32,
    pub pPhysicalDevices: *const VkPhysicalDevice,
}

impl Default for VkDeviceGroupDeviceCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_GROUP_DEVICE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceGroupSwapchainCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub modes: VkDeviceGroupPresentModeFlagsKHR,
}

impl Default for VkDeviceGroupSwapchainCreateInfoKHR {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorUpdateTemplateEntry {
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: VkDescriptorType,
    pub offset: usize,
    pub stride: usize,
}

impl Default for VkDescriptorUpdateTemplateEntry {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorUpdateTemplateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDescriptorUpdateTemplateCreateFlags,
    pub descriptorUpdateEntryCount: u32,
    pub pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntry,
    pub templateType: VkDescriptorUpdateTemplateType,
    pub descriptorSetLayout: VkDescriptorSetLayout,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub pipelineLayout: VkPipelineLayout,
    pub set: u32,
}

impl Default for VkDescriptorUpdateTemplateCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkInputAttachmentAspectReference {
    pub subpass: u32,
    pub inputAttachmentIndex: u32,
    pub aspectMask: VkImageAspectFlags,
}

impl Default for VkInputAttachmentAspectReference {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRenderPassInputAttachmentAspectCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub aspectReferenceCount: u32,
    pub pAspectReferences: *const VkInputAttachmentAspectReference,
}

impl Default for VkRenderPassInputAttachmentAspectCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDevice16BitStorageFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub storageBuffer16BitAccess: VkBool32,
    pub uniformAndStorageBuffer16BitAccess: VkBool32,
    pub storagePushConstant16: VkBool32,
    pub storageInputOutput16: VkBool32,
}

impl Default for VkPhysicalDevice16BitStorageFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceSubgroupProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub subgroupSize: u32,
    pub supportedStages: VkShaderStageFlags,
    pub supportedOperations: VkSubgroupFeatureFlags,
    pub quadOperationsInAllStages: VkBool32,
}

impl Default for VkPhysicalDeviceSubgroupProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub shaderSubgroupExtendedTypes: VkBool32,
}

impl Default for VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferMemoryRequirementsInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub buffer: VkBuffer,
}

impl Default for VkBufferMemoryRequirementsInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceBufferMemoryRequirements {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pCreateInfo: *const VkBufferCreateInfo,
}

impl Default for VkDeviceBufferMemoryRequirements {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_BUFFER_MEMORY_REQUIREMENTS;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageMemoryRequirementsInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub image: VkImage,
}

impl Default for VkImageMemoryRequirementsInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageSparseMemoryRequirementsInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub image: VkImage,
}

impl Default for VkImageSparseMemoryRequirementsInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceImageMemoryRequirements {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pCreateInfo: *const VkImageCreateInfo,
    pub planeAspect: VkImageAspectFlagBits,
}

impl Default for VkDeviceImageMemoryRequirements {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_IMAGE_MEMORY_REQUIREMENTS;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryRequirements2 {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub memoryRequirements: VkMemoryRequirements,
}

impl Default for VkMemoryRequirements2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::MEMORY_REQUIREMENTS_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSparseImageMemoryRequirements2 {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub memoryRequirements: VkSparseImageMemoryRequirements,
}

impl Default for VkSparseImageMemoryRequirements2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDevicePointClippingProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub pointClippingBehavior: VkPointClippingBehavior,
}

impl Default for VkPhysicalDevicePointClippingProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryDedicatedRequirements {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub prefersDedicatedAllocation: VkBool32,
    pub requiresDedicatedAllocation: VkBool32,
}

impl Default for VkMemoryDedicatedRequirements {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::MEMORY_DEDICATED_REQUIREMENTS;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryDedicatedAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub image: VkImage,
    pub buffer: VkBuffer,
}

impl Default for VkMemoryDedicatedAllocateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::MEMORY_DEDICATED_ALLOCATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageViewUsageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub usage: VkImageUsageFlags,
}

impl Default for VkImageViewUsageCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_VIEW_USAGE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub domainOrigin: VkTessellationDomainOrigin,
}

impl Default for VkPipelineTessellationDomainOriginStateCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSamplerYcbcrConversionInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub conversion: VkSamplerYcbcrConversion,
}

impl Default for VkSamplerYcbcrConversionInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SAMPLER_YCBCR_CONVERSION_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSamplerYcbcrConversionCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub format: VkFormat,
    pub ycbcrModel: VkSamplerYcbcrModelConversion,
    pub ycbcrRange: VkSamplerYcbcrRange,
    pub components: VkComponentMapping,
    pub xChromaOffset: VkChromaLocation,
    pub yChromaOffset: VkChromaLocation,
    pub chromaFilter: VkFilter,
    pub forceExplicitReconstruction: VkBool32,
}

impl Default for VkSamplerYcbcrConversionCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SAMPLER_YCBCR_CONVERSION_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBindImagePlaneMemoryInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub planeAspect: VkImageAspectFlagBits,
}

impl Default for VkBindImagePlaneMemoryInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BIND_IMAGE_PLANE_MEMORY_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImagePlaneMemoryRequirementsInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub planeAspect: VkImageAspectFlagBits,
}

impl Default for VkImagePlaneMemoryRequirementsInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub samplerYcbcrConversion: VkBool32,
}

impl Default for VkPhysicalDeviceSamplerYcbcrConversionFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSamplerYcbcrConversionImageFormatProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub combinedImageSamplerDescriptorCount: u32,
}

impl Default for VkSamplerYcbcrConversionImageFormatProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkProtectedSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub protectedSubmit: VkBool32,
}

impl Default for VkProtectedSubmitInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PROTECTED_SUBMIT_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceProtectedMemoryFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub protectedMemory: VkBool32,
}

impl Default for VkPhysicalDeviceProtectedMemoryFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceProtectedMemoryProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub protectedNoFault: VkBool32,
}

impl Default for VkPhysicalDeviceProtectedMemoryProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceQueueInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkDeviceQueueCreateFlags,
    pub queueFamilyIndex: u32,
    pub queueIndex: u32,
}

impl Default for VkDeviceQueueInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_QUEUE_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub filterMinmaxSingleComponentFormats: VkBool32,
    pub filterMinmaxImageComponentMapping: VkBool32,
}

impl Default for VkPhysicalDeviceSamplerFilterMinmaxProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSamplerReductionModeCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub reductionMode: VkSamplerReductionMode,
}

impl Default for VkSamplerReductionModeCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SAMPLER_REDUCTION_MODE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceInlineUniformBlockFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub inlineUniformBlock: VkBool32,
    pub descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32,
}

impl Default for VkPhysicalDeviceInlineUniformBlockFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceInlineUniformBlockProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxInlineUniformBlockSize: u32,
    pub maxPerStageDescriptorInlineUniformBlocks: u32,
    pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: u32,
    pub maxDescriptorSetInlineUniformBlocks: u32,
    pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks: u32,
}

impl Default for VkPhysicalDeviceInlineUniformBlockProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkWriteDescriptorSetInlineUniformBlock {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub dataSize: u32,
    pub pData: *const core::ffi::c_void,
}

impl Default for VkWriteDescriptorSetInlineUniformBlock {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorPoolInlineUniformBlockCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub maxInlineUniformBlockBindings: u32,
}

impl Default for VkDescriptorPoolInlineUniformBlockCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageFormatListCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub viewFormatCount: u32,
    pub pViewFormats: *const VkFormat,
}

impl Default for VkImageFormatListCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_FORMAT_LIST_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceMaintenance3Properties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxPerSetDescriptors: u32,
    pub maxMemoryAllocationSize: VkDeviceSize,
}

impl Default for VkPhysicalDeviceMaintenance3Properties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceMaintenance4Features {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maintenance4: VkBool32,
}

impl Default for VkPhysicalDeviceMaintenance4Features {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceMaintenance4Properties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxBufferSize: VkDeviceSize,
}

impl Default for VkPhysicalDeviceMaintenance4Properties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorSetLayoutSupport {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub supported: VkBool32,
}

impl Default for VkDescriptorSetLayoutSupport {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DESCRIPTOR_SET_LAYOUT_SUPPORT;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceShaderDrawParametersFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub shaderDrawParameters: VkBool32,
}

impl Default for VkPhysicalDeviceShaderDrawParametersFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceShaderDrawParameterFeatures {
}

impl Default for VkPhysicalDeviceShaderDrawParameterFeatures {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceShaderFloat16Int8Features {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub shaderFloat16: VkBool32,
    pub shaderInt8: VkBool32,
}

impl Default for VkPhysicalDeviceShaderFloat16Int8Features {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceFloatControlsProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
    pub roundingModeIndependence: VkShaderFloatControlsIndependence,
    pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
    pub shaderDenormPreserveFloat16: VkBool32,
    pub shaderDenormPreserveFloat32: VkBool32,
    pub shaderDenormPreserveFloat64: VkBool32,
    pub shaderDenormFlushToZeroFloat16: VkBool32,
    pub shaderDenormFlushToZeroFloat32: VkBool32,
    pub shaderDenormFlushToZeroFloat64: VkBool32,
    pub shaderRoundingModeRTEFloat16: VkBool32,
    pub shaderRoundingModeRTEFloat32: VkBool32,
    pub shaderRoundingModeRTEFloat64: VkBool32,
    pub shaderRoundingModeRTZFloat16: VkBool32,
    pub shaderRoundingModeRTZFloat32: VkBool32,
    pub shaderRoundingModeRTZFloat64: VkBool32,
}

impl Default for VkPhysicalDeviceFloatControlsProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceHostQueryResetFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub hostQueryReset: VkBool32,
}

impl Default for VkPhysicalDeviceHostQueryResetFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceDescriptorIndexingFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
    pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
    pub descriptorBindingPartiallyBound: VkBool32,
    pub descriptorBindingVariableDescriptorCount: VkBool32,
    pub runtimeDescriptorArray: VkBool32,
}

impl Default for VkPhysicalDeviceDescriptorIndexingFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceDescriptorIndexingProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxUpdateAfterBindDescriptorsInAllPools: u32,
    pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
    pub robustBufferAccessUpdateAfterBind: VkBool32,
    pub quadDivergentImplicitLod: VkBool32,
    pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
    pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
    pub maxPerStageUpdateAfterBindResources: u32,
    pub maxDescriptorSetUpdateAfterBindSamplers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
    pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
    pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
}

impl Default for VkPhysicalDeviceDescriptorIndexingProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub bindingCount: u32,
    pub pBindingFlags: *const VkDescriptorBindingFlags,
}

impl Default for VkDescriptorSetLayoutBindingFlagsCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub descriptorSetCount: u32,
    pub pDescriptorCounts: *const u32,
}

impl Default for VkDescriptorSetVariableDescriptorCountAllocateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxVariableDescriptorCount: u32,
}

impl Default for VkDescriptorSetVariableDescriptorCountLayoutSupport {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAttachmentDescription2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkAttachmentDescriptionFlags,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub loadOp: VkAttachmentLoadOp,
    pub storeOp: VkAttachmentStoreOp,
    pub stencilLoadOp: VkAttachmentLoadOp,
    pub stencilStoreOp: VkAttachmentStoreOp,
    pub initialLayout: VkImageLayout,
    pub finalLayout: VkImageLayout,
}

impl Default for VkAttachmentDescription2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::ATTACHMENT_DESCRIPTION_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAttachmentReference2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub attachment: u32,
    pub layout: VkImageLayout,
    pub aspectMask: VkImageAspectFlags,
}

impl Default for VkAttachmentReference2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::ATTACHMENT_REFERENCE_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubpassDescription2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkSubpassDescriptionFlags,
    pub pipelineBindPoint: VkPipelineBindPoint,
    pub viewMask: u32,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const VkAttachmentReference2,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkAttachmentReference2,
    pub pResolveAttachments: *const VkAttachmentReference2,
    pub pDepthStencilAttachment: *const VkAttachmentReference2,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}

impl Default for VkSubpassDescription2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SUBPASS_DESCRIPTION_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubpassDependency2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: VkPipelineStageFlags,
    pub dstStageMask: VkPipelineStageFlags,
    pub srcAccessMask: VkAccessFlags,
    pub dstAccessMask: VkAccessFlags,
    pub dependencyFlags: VkDependencyFlags,
    pub viewOffset: i32,
}

impl Default for VkSubpassDependency2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SUBPASS_DEPENDENCY_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRenderPassCreateInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkRenderPassCreateFlags,
    pub attachmentCount: u32,
    pub pAttachments: *const VkAttachmentDescription2,
    pub subpassCount: u32,
    pub pSubpasses: *const VkSubpassDescription2,
    pub dependencyCount: u32,
    pub pDependencies: *const VkSubpassDependency2,
    pub correlatedViewMaskCount: u32,
    pub pCorrelatedViewMasks: *const u32,
}

impl Default for VkRenderPassCreateInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::RENDER_PASS_CREATE_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubpassBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub contents: VkSubpassContents,
}

impl Default for VkSubpassBeginInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SUBPASS_BEGIN_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubpassEndInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
}

impl Default for VkSubpassEndInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SUBPASS_END_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub timelineSemaphore: VkBool32,
}

impl Default for VkPhysicalDeviceTimelineSemaphoreFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceTimelineSemaphoreProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub maxTimelineSemaphoreValueDifference: u64,
}

impl Default for VkPhysicalDeviceTimelineSemaphoreProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSemaphoreTypeCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub semaphoreType: VkSemaphoreType,
    pub initialValue: u64,
}

impl Default for VkSemaphoreTypeCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SEMAPHORE_TYPE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkTimelineSemaphoreSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub waitSemaphoreValueCount: u32,
    pub pWaitSemaphoreValues: *const u64,
    pub signalSemaphoreValueCount: u32,
    pub pSignalSemaphoreValues: *const u64,
}

impl Default for VkTimelineSemaphoreSubmitInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::TIMELINE_SEMAPHORE_SUBMIT_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSemaphoreWaitInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkSemaphoreWaitFlags,
    pub semaphoreCount: u32,
    pub pSemaphores: *const VkSemaphore,
    pub pValues: *const u64,
}

impl Default for VkSemaphoreWaitInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SEMAPHORE_WAIT_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSemaphoreSignalInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub semaphore: VkSemaphore,
    pub value: u64,
}

impl Default for VkSemaphoreSignalInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SEMAPHORE_SIGNAL_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDevice8BitStorageFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub storageBuffer8BitAccess: VkBool32,
    pub uniformAndStorageBuffer8BitAccess: VkBool32,
    pub storagePushConstant8: VkBool32,
}

impl Default for VkPhysicalDevice8BitStorageFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub vulkanMemoryModel: VkBool32,
    pub vulkanMemoryModelDeviceScope: VkBool32,
    pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
}

impl Default for VkPhysicalDeviceVulkanMemoryModelFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceShaderAtomicInt64Features {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub shaderBufferInt64Atomics: VkBool32,
    pub shaderSharedInt64Atomics: VkBool32,
}

impl Default for VkPhysicalDeviceShaderAtomicInt64Features {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceDepthStencilResolveProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub supportedDepthResolveModes: VkResolveModeFlags,
    pub supportedStencilResolveModes: VkResolveModeFlags,
    pub independentResolveNone: VkBool32,
    pub independentResolve: VkBool32,
}

impl Default for VkPhysicalDeviceDepthStencilResolveProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubpassDescriptionDepthStencilResolve {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub depthResolveMode: VkResolveModeFlagBits,
    pub stencilResolveMode: VkResolveModeFlagBits,
    pub pDepthStencilResolveAttachment: *const VkAttachmentReference2,
}

impl Default for VkSubpassDescriptionDepthStencilResolve {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageStencilUsageCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub stencilUsage: VkImageUsageFlags,
}

impl Default for VkImageStencilUsageCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_STENCIL_USAGE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub scalarBlockLayout: VkBool32,
}

impl Default for VkPhysicalDeviceScalarBlockLayoutFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub uniformBufferStandardLayout: VkBool32,
}

impl Default for VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub bufferDeviceAddress: VkBool32,
    pub bufferDeviceAddressCaptureReplay: VkBool32,
    pub bufferDeviceAddressMultiDevice: VkBool32,
}

impl Default for VkPhysicalDeviceBufferDeviceAddressFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferDeviceAddressInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub buffer: VkBuffer,
}

impl Default for VkBufferDeviceAddressInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BUFFER_DEVICE_ADDRESS_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferOpaqueCaptureAddressCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub opaqueCaptureAddress: u64,
}

impl Default for VkBufferOpaqueCaptureAddressCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceImagelessFramebufferFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub imagelessFramebuffer: VkBool32,
}

impl Default for VkPhysicalDeviceImagelessFramebufferFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkFramebufferAttachmentsCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub attachmentImageInfoCount: u32,
    pub pAttachmentImageInfos: *const VkFramebufferAttachmentImageInfo,
}

impl Default for VkFramebufferAttachmentsCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkFramebufferAttachmentImageInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkImageCreateFlags,
    pub usage: VkImageUsageFlags,
    pub width: u32,
    pub height: u32,
    pub layerCount: u32,
    pub viewFormatCount: u32,
    pub pViewFormats: *const VkFormat,
}

impl Default for VkFramebufferAttachmentImageInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRenderPassAttachmentBeginInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub attachmentCount: u32,
    pub pAttachments: *const VkImageView,
}

impl Default for VkRenderPassAttachmentBeginInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::RENDER_PASS_ATTACHMENT_BEGIN_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub textureCompressionASTC_HDR: VkBool32,
}

impl Default for VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineCreationFeedback {
    pub flags: VkPipelineCreationFeedbackFlags,
    pub duration: u64,
}

impl Default for VkPipelineCreationFeedback {
    #[inline]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineCreationFeedbackCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub pPipelineCreationFeedback: *mut VkPipelineCreationFeedback,
    pub pipelineStageCreationFeedbackCount: u32,
    pub pPipelineStageCreationFeedbacks: *mut VkPipelineCreationFeedback,
}

impl Default for VkPipelineCreationFeedbackCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_CREATION_FEEDBACK_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub separateDepthStencilLayouts: VkBool32,
}

impl Default for VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAttachmentReferenceStencilLayout {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub stencilLayout: VkImageLayout,
}

impl Default for VkAttachmentReferenceStencilLayout {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::ATTACHMENT_REFERENCE_STENCIL_LAYOUT;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkAttachmentDescriptionStencilLayout {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub stencilInitialLayout: VkImageLayout,
    pub stencilFinalLayout: VkImageLayout,
}

impl Default for VkAttachmentDescriptionStencilLayout {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub shaderDemoteToHelperInvocation: VkBool32,
}

impl Default for VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceTexelBufferAlignmentProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
    pub storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
    pub uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
    pub uniformTexelBufferOffsetSingleTexelAlignment: VkBool32,
}

impl Default for VkPhysicalDeviceTexelBufferAlignmentProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceSubgroupSizeControlFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub subgroupSizeControl: VkBool32,
    pub computeFullSubgroups: VkBool32,
}

impl Default for VkPhysicalDeviceSubgroupSizeControlFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceSubgroupSizeControlProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub minSubgroupSize: u32,
    pub maxSubgroupSize: u32,
    pub maxComputeWorkgroupSubgroups: u32,
    pub requiredSubgroupSizeStages: VkShaderStageFlags,
}

impl Default for VkPhysicalDeviceSubgroupSizeControlProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub requiredSubgroupSize: u32,
}

impl Default for VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub opaqueCaptureAddress: u64,
}

impl Default for VkMemoryOpaqueCaptureAddressAllocateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub memory: VkDeviceMemory,
}

impl Default for VkDeviceMemoryOpaqueCaptureAddressInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDevicePipelineCreationCacheControlFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub pipelineCreationCacheControl: VkBool32,
}

impl Default for VkPhysicalDevicePipelineCreationCacheControlFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceVulkan11Features {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub storageBuffer16BitAccess: VkBool32,
    pub uniformAndStorageBuffer16BitAccess: VkBool32,
    pub storagePushConstant16: VkBool32,
    pub storageInputOutput16: VkBool32,
    pub multiview: VkBool32,
    pub multiviewGeometryShader: VkBool32,
    pub multiviewTessellationShader: VkBool32,
    pub variablePointersStorageBuffer: VkBool32,
    pub variablePointers: VkBool32,
    pub protectedMemory: VkBool32,
    pub samplerYcbcrConversion: VkBool32,
    pub shaderDrawParameters: VkBool32,
}

impl Default for VkPhysicalDeviceVulkan11Features {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceVulkan11Properties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub deviceUUID: [u8; VK_UUID_SIZE],
    pub driverUUID: [u8; VK_UUID_SIZE],
    pub deviceLUID: [u8; VK_LUID_SIZE],
    pub deviceNodeMask: u32,
    pub deviceLUIDValid: VkBool32,
    pub subgroupSize: u32,
    pub subgroupSupportedStages: VkShaderStageFlags,
    pub subgroupSupportedOperations: VkSubgroupFeatureFlags,
    pub subgroupQuadOperationsInAllStages: VkBool32,
    pub pointClippingBehavior: VkPointClippingBehavior,
    pub maxMultiviewViewCount: u32,
    pub maxMultiviewInstanceIndex: u32,
    pub protectedNoFault: VkBool32,
    pub maxPerSetDescriptors: u32,
    pub maxMemoryAllocationSize: VkDeviceSize,
}

impl Default for VkPhysicalDeviceVulkan11Properties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceVulkan12Features {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub samplerMirrorClampToEdge: VkBool32,
    pub drawIndirectCount: VkBool32,
    pub storageBuffer8BitAccess: VkBool32,
    pub uniformAndStorageBuffer8BitAccess: VkBool32,
    pub storagePushConstant8: VkBool32,
    pub shaderBufferInt64Atomics: VkBool32,
    pub shaderSharedInt64Atomics: VkBool32,
    pub shaderFloat16: VkBool32,
    pub shaderInt8: VkBool32,
    pub descriptorIndexing: VkBool32,
    pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
    pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
    pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
    pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
    pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
    pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
    pub descriptorBindingPartiallyBound: VkBool32,
    pub descriptorBindingVariableDescriptorCount: VkBool32,
    pub runtimeDescriptorArray: VkBool32,
    pub samplerFilterMinmax: VkBool32,
    pub scalarBlockLayout: VkBool32,
    pub imagelessFramebuffer: VkBool32,
    pub uniformBufferStandardLayout: VkBool32,
    pub shaderSubgroupExtendedTypes: VkBool32,
    pub separateDepthStencilLayouts: VkBool32,
    pub hostQueryReset: VkBool32,
    pub timelineSemaphore: VkBool32,
    pub bufferDeviceAddress: VkBool32,
    pub bufferDeviceAddressCaptureReplay: VkBool32,
    pub bufferDeviceAddressMultiDevice: VkBool32,
    pub vulkanMemoryModel: VkBool32,
    pub vulkanMemoryModelDeviceScope: VkBool32,
    pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
    pub shaderOutputViewportIndex: VkBool32,
    pub shaderOutputLayer: VkBool32,
    pub subgroupBroadcastDynamicId: VkBool32,
}

impl Default for VkPhysicalDeviceVulkan12Features {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceVulkan12Properties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub driverID: VkDriverId,
    pub driverName: [core::ffi::c_char; VK_MAX_DRIVER_NAME_SIZE],
    pub driverInfo: [core::ffi::c_char; VK_MAX_DRIVER_INFO_SIZE],
    pub conformanceVersion: VkConformanceVersion,
    pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
    pub roundingModeIndependence: VkShaderFloatControlsIndependence,
    pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
    pub shaderDenormPreserveFloat16: VkBool32,
    pub shaderDenormPreserveFloat32: VkBool32,
    pub shaderDenormPreserveFloat64: VkBool32,
    pub shaderDenormFlushToZeroFloat16: VkBool32,
    pub shaderDenormFlushToZeroFloat32: VkBool32,
    pub shaderDenormFlushToZeroFloat64: VkBool32,
    pub shaderRoundingModeRTEFloat16: VkBool32,
    pub shaderRoundingModeRTEFloat32: VkBool32,
    pub shaderRoundingModeRTEFloat64: VkBool32,
    pub shaderRoundingModeRTZFloat16: VkBool32,
    pub shaderRoundingModeRTZFloat32: VkBool32,
    pub shaderRoundingModeRTZFloat64: VkBool32,
    pub maxUpdateAfterBindDescriptorsInAllPools: u32,
    pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
    pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
    pub robustBufferAccessUpdateAfterBind: VkBool32,
    pub quadDivergentImplicitLod: VkBool32,
    pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
    pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
    pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
    pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
    pub maxPerStageUpdateAfterBindResources: u32,
    pub maxDescriptorSetUpdateAfterBindSamplers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
    pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
    pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
    pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
    pub supportedDepthResolveModes: VkResolveModeFlags,
    pub supportedStencilResolveModes: VkResolveModeFlags,
    pub independentResolveNone: VkBool32,
    pub independentResolve: VkBool32,
    pub filterMinmaxSingleComponentFormats: VkBool32,
    pub filterMinmaxImageComponentMapping: VkBool32,
    pub maxTimelineSemaphoreValueDifference: u64,
    pub framebufferIntegerColorSampleCounts: VkSampleCountFlags,
}

impl Default for VkPhysicalDeviceVulkan12Properties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceVulkan13Features {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub robustImageAccess: VkBool32,
    pub inlineUniformBlock: VkBool32,
    pub descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32,
    pub pipelineCreationCacheControl: VkBool32,
    pub privateData: VkBool32,
    pub shaderDemoteToHelperInvocation: VkBool32,
    pub shaderTerminateInvocation: VkBool32,
    pub subgroupSizeControl: VkBool32,
    pub computeFullSubgroups: VkBool32,
    pub synchronization2: VkBool32,
    pub textureCompressionASTC_HDR: VkBool32,
    pub shaderZeroInitializeWorkgroupMemory: VkBool32,
    pub dynamicRendering: VkBool32,
    pub shaderIntegerDotProduct: VkBool32,
    pub maintenance4: VkBool32,
}

impl Default for VkPhysicalDeviceVulkan13Features {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceVulkan13Properties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub minSubgroupSize: u32,
    pub maxSubgroupSize: u32,
    pub maxComputeWorkgroupSubgroups: u32,
    pub requiredSubgroupSizeStages: VkShaderStageFlags,
    pub maxInlineUniformBlockSize: u32,
    pub maxPerStageDescriptorInlineUniformBlocks: u32,
    pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: u32,
    pub maxDescriptorSetInlineUniformBlocks: u32,
    pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks: u32,
    pub maxInlineUniformTotalSize: u32,
    pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct8BitSignedAccelerated: VkBool32,
    pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
    pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
    pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct16BitSignedAccelerated: VkBool32,
    pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct32BitSignedAccelerated: VkBool32,
    pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct64BitSignedAccelerated: VkBool32,
    pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32,
    pub storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
    pub storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
    pub uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
    pub uniformTexelBufferOffsetSingleTexelAlignment: VkBool32,
    pub maxBufferSize: VkDeviceSize,
}

impl Default for VkPhysicalDeviceVulkan13Properties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceToolProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub name: [core::ffi::c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub version: [core::ffi::c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub purposes: VkToolPurposeFlags,
    pub description: [core::ffi::c_char; VK_MAX_DESCRIPTION_SIZE],
    pub layer: [core::ffi::c_char; VK_MAX_EXTENSION_NAME_SIZE],
}

impl Default for VkPhysicalDeviceToolProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_TOOL_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub shaderZeroInitializeWorkgroupMemory: VkBool32,
}

impl Default for VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceImageRobustnessFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub robustImageAccess: VkBool32,
}

impl Default for VkPhysicalDeviceImageRobustnessFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferCopy2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcOffset: VkDeviceSize,
    pub dstOffset: VkDeviceSize,
    pub size: VkDeviceSize,
}

impl Default for VkBufferCopy2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BUFFER_COPY_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageCopy2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

impl Default for VkImageCopy2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_COPY_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageBlit2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffsets: [VkOffset3D; 2],
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffsets: [VkOffset3D; 2],
}

impl Default for VkImageBlit2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_BLIT_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferImageCopy2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub bufferOffset: VkDeviceSize,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: VkImageSubresourceLayers,
    pub imageOffset: VkOffset3D,
    pub imageExtent: VkExtent3D,
}

impl Default for VkBufferImageCopy2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BUFFER_IMAGE_COPY_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageResolve2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcSubresource: VkImageSubresourceLayers,
    pub srcOffset: VkOffset3D,
    pub dstSubresource: VkImageSubresourceLayers,
    pub dstOffset: VkOffset3D,
    pub extent: VkExtent3D,
}

impl Default for VkImageResolve2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_RESOLVE_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCopyBufferInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcBuffer: VkBuffer,
    pub dstBuffer: VkBuffer,
    pub regionCount: u32,
    pub pRegions: *const VkBufferCopy2,
}

impl Default for VkCopyBufferInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::COPY_BUFFER_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCopyImageInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkImageCopy2,
}

impl Default for VkCopyImageInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::COPY_IMAGE_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBlitImageInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkImageBlit2,
    pub filter: VkFilter,
}

impl Default for VkBlitImageInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BLIT_IMAGE_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCopyBufferToImageInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcBuffer: VkBuffer,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkBufferImageCopy2,
}

impl Default for VkCopyBufferToImageInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::COPY_BUFFER_TO_IMAGE_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCopyImageToBufferInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstBuffer: VkBuffer,
    pub regionCount: u32,
    pub pRegions: *const VkBufferImageCopy2,
}

impl Default for VkCopyImageToBufferInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::COPY_IMAGE_TO_BUFFER_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkResolveImageInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcImage: VkImage,
    pub srcImageLayout: VkImageLayout,
    pub dstImage: VkImage,
    pub dstImageLayout: VkImageLayout,
    pub regionCount: u32,
    pub pRegions: *const VkImageResolve2,
}

impl Default for VkResolveImageInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::RESOLVE_IMAGE_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceShaderTerminateInvocationFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub shaderTerminateInvocation: VkBool32,
}

impl Default for VkPhysicalDeviceShaderTerminateInvocationFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkMemoryBarrier2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcStageMask: VkPipelineStageFlags2,
    pub srcAccessMask: VkAccessFlags2,
    pub dstStageMask: VkPipelineStageFlags2,
    pub dstAccessMask: VkAccessFlags2,
}

impl Default for VkMemoryBarrier2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::MEMORY_BARRIER_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkImageMemoryBarrier2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcStageMask: VkPipelineStageFlags2,
    pub srcAccessMask: VkAccessFlags2,
    pub dstStageMask: VkPipelineStageFlags2,
    pub dstAccessMask: VkAccessFlags2,
    pub oldLayout: VkImageLayout,
    pub newLayout: VkImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: VkImage,
    pub subresourceRange: VkImageSubresourceRange,
}

impl Default for VkImageMemoryBarrier2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::IMAGE_MEMORY_BARRIER_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkBufferMemoryBarrier2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub srcStageMask: VkPipelineStageFlags2,
    pub srcAccessMask: VkAccessFlags2,
    pub dstStageMask: VkPipelineStageFlags2,
    pub dstAccessMask: VkAccessFlags2,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

impl Default for VkBufferMemoryBarrier2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::BUFFER_MEMORY_BARRIER_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkDependencyInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub dependencyFlags: VkDependencyFlags,
    pub memoryBarrierCount: u32,
    pub pMemoryBarriers: *const VkMemoryBarrier2,
    pub bufferMemoryBarrierCount: u32,
    pub pBufferMemoryBarriers: *const VkBufferMemoryBarrier2,
    pub imageMemoryBarrierCount: u32,
    pub pImageMemoryBarriers: *const VkImageMemoryBarrier2,
}

impl Default for VkDependencyInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::DEPENDENCY_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSemaphoreSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub semaphore: VkSemaphore,
    pub value: u64,
    pub stageMask: VkPipelineStageFlags2,
    pub deviceIndex: u32,
}

impl Default for VkSemaphoreSubmitInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SEMAPHORE_SUBMIT_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCommandBufferSubmitInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub commandBuffer: VkCommandBuffer,
    pub deviceMask: u32,
}

impl Default for VkCommandBufferSubmitInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::COMMAND_BUFFER_SUBMIT_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkSubmitInfo2 {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkSubmitFlags,
    pub waitSemaphoreInfoCount: u32,
    pub pWaitSemaphoreInfos: *const VkSemaphoreSubmitInfo,
    pub commandBufferInfoCount: u32,
    pub pCommandBufferInfos: *const VkCommandBufferSubmitInfo,
    pub signalSemaphoreInfoCount: u32,
    pub pSignalSemaphoreInfos: *const VkSemaphoreSubmitInfo,
}

impl Default for VkSubmitInfo2 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::SUBMIT_INFO_2;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceSynchronization2Features {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub synchronization2: VkBool32,
}

impl Default for VkPhysicalDeviceSynchronization2Features {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceShaderIntegerDotProductFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub shaderIntegerDotProduct: VkBool32,
}

impl Default for VkPhysicalDeviceShaderIntegerDotProductFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceShaderIntegerDotProductProperties {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct8BitSignedAccelerated: VkBool32,
    pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
    pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
    pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct16BitSignedAccelerated: VkBool32,
    pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct32BitSignedAccelerated: VkBool32,
    pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
    pub integerDotProduct64BitSignedAccelerated: VkBool32,
    pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32,
    pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32,
}

impl Default for VkPhysicalDeviceShaderIntegerDotProductProperties {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkFormatProperties3 {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub linearTilingFeatures: VkFormatFeatureFlags2,
    pub optimalTilingFeatures: VkFormatFeatureFlags2,
    pub bufferFeatures: VkFormatFeatureFlags2,
}

impl Default for VkFormatProperties3 {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::FORMAT_PROPERTIES_3;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPipelineRenderingCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub viewMask: u32,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentFormats: *const VkFormat,
    pub depthAttachmentFormat: VkFormat,
    pub stencilAttachmentFormat: VkFormat,
}

impl Default for VkPipelineRenderingCreateInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PIPELINE_RENDERING_CREATE_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRenderingInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkRenderingFlags,
    pub renderArea: VkRect2D,
    pub layerCount: u32,
    pub viewMask: u32,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const VkRenderingAttachmentInfo,
    pub pDepthAttachment: *const VkRenderingAttachmentInfo,
    pub pStencilAttachment: *const VkRenderingAttachmentInfo,
}

impl Default for VkRenderingInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::RENDERING_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkRenderingAttachmentInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub imageView: VkImageView,
    pub imageLayout: VkImageLayout,
    pub resolveMode: VkResolveModeFlagBits,
    pub resolveImageView: VkImageView,
    pub resolveImageLayout: VkImageLayout,
    pub loadOp: VkAttachmentLoadOp,
    pub storeOp: VkAttachmentStoreOp,
    pub clearValue: VkClearValue,
}

impl Default for VkRenderingAttachmentInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::RENDERING_ATTACHMENT_INFO;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkPhysicalDeviceDynamicRenderingFeatures {
    pub sType: VkStructureType,
    pub pNext: *mut core::ffi::c_void,
    pub dynamicRendering: VkBool32,
}

impl Default for VkPhysicalDeviceDynamicRenderingFeatures {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES;
        value
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkCommandBufferInheritanceRenderingInfo {
    pub sType: VkStructureType,
    pub pNext: *const core::ffi::c_void,
    pub flags: VkRenderingFlags,
    pub viewMask: u32,
    pub colorAttachmentCount: u32,
    pub pColorAttachmentFormats: *const VkFormat,
    pub depthAttachmentFormat: VkFormat,
    pub stencilAttachmentFormat: VkFormat,
    pub rasterizationSamples: VkSampleCountFlagBits,
}

impl Default for VkCommandBufferInheritanceRenderingInfo {
    #[inline]
    fn default() -> Self {
        let mut value: Self = unsafe { core::mem::zeroed() };
        value.sType = VkStructureType::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO;
        value
    }
}

// ---------------------------------------------------------------
// Command signatures
//
// `extern "system"` matches VKAPI_PTR: stdcall on 32-bit Windows,
// the platform C ABI everywhere else. `extern "C"` would be a
// stack-corrupting mismatch on exactly one supported target.
// ---------------------------------------------------------------

pub type PFN_vkCreateInstance = unsafe extern "system" fn(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;
pub type PFN_vkDestroyInstance = unsafe extern "system" fn(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;
pub type PFN_vkGetDeviceProcAddr = unsafe extern "system" fn(device: VkDevice, pName: *const core::ffi::c_char) -> PFN_vkVoidFunction;
pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn(instance: VkInstance, pName: *const core::ffi::c_char) -> PFN_vkVoidFunction;
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);
pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);
pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);
pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, r#type: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult;
pub type PFN_vkCreateDevice = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;
pub type PFN_vkDestroyDevice = unsafe extern "system" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkEnumerateInstanceVersion = unsafe extern "system" fn(pApiVersion: *mut u32) -> VkResult;
pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(pLayerName: *const core::ffi::c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
pub type PFN_vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pLayerName: *const core::ffi::c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
pub type PFN_vkGetDeviceQueue = unsafe extern "system" fn(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);
pub type PFN_vkQueueSubmit = unsafe extern "system" fn(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;
pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(queue: VkQueue) -> VkResult;
pub type PFN_vkDeviceWaitIdle = unsafe extern "system" fn(device: VkDevice) -> VkResult;
pub type PFN_vkAllocateMemory = unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult;
pub type PFN_vkFreeMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkMapMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut core::ffi::c_void) -> VkResult;
pub type PFN_vkUnmapMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory);
pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize);
pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements);
pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);
pub type PFN_vkBindImageMemory = unsafe extern "system" fn(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
pub type PFN_vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, r#type: VkImageType, samples: VkSampleCountFlagBits, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties);
pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult;
pub type PFN_vkCreateFence = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
pub type PFN_vkDestroyFence = unsafe extern "system" fn(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkResetFences = unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;
pub type PFN_vkGetFenceStatus = unsafe extern "system" fn(device: VkDevice, fence: VkFence) -> VkResult;
pub type PFN_vkWaitForFences = unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult;
pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult;
pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateEvent = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult;
pub type PFN_vkDestroyEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetEventStatus = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
pub type PFN_vkSetEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
pub type PFN_vkResetEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult;
pub type PFN_vkDestroyQueryPool = unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetQueryPoolResults = unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut core::ffi::c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult;
pub type PFN_vkResetQueryPool = unsafe extern "system" fn(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
pub type PFN_vkCreateBuffer = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult;
pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateBufferView = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult;
pub type PFN_vkDestroyBufferView = unsafe extern "system" fn(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateImage = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult;
pub type PFN_vkDestroyImage = unsafe extern "system" fn(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);
pub type PFN_vkCreateImageView = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult;
pub type PFN_vkDestroyImageView = unsafe extern "system" fn(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult;
pub type PFN_vkDestroyShaderModule = unsafe extern "system" fn(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult;
pub type PFN_vkDestroyPipelineCache = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetPipelineCacheData = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut usize, pData: *mut core::ffi::c_void) -> VkResult;
pub type PFN_vkMergePipelineCaches = unsafe extern "system" fn(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult;
pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
pub type PFN_vkDestroyPipeline = unsafe extern "system" fn(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult;
pub type PFN_vkDestroyPipelineLayout = unsafe extern "system" fn(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateSampler = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult;
pub type PFN_vkDestroySampler = unsafe extern "system" fn(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;
pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult;
pub type PFN_vkDestroyDescriptorPool = unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkResetDescriptorPool = unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;
pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult;
pub type PFN_vkFreeDescriptorSets = unsafe extern "system" fn(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult;
pub type PFN_vkUpdateDescriptorSets = unsafe extern "system" fn(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);
pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult;
pub type PFN_vkDestroyFramebuffer = unsafe extern "system" fn(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;
pub type PFN_vkDestroyRenderPass = unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);
pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult;
pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkResetCommandPool = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;
pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;
pub type PFN_vkFreeCommandBuffers = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
pub type PFN_vkBeginCommandBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;
pub type PFN_vkEndCommandBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer) -> VkResult;
pub type PFN_vkResetCommandBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;
pub type PFN_vkCmdBindPipeline = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);
pub type PFN_vkCmdSetViewport = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);
pub type PFN_vkCmdSetScissor = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);
pub type PFN_vkCmdSetLineWidth = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, lineWidth: f32);
pub type PFN_vkCmdSetDepthBias = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32);
pub type PFN_vkCmdSetBlendConstants = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, blendConstants: [f32; 4]);
pub type PFN_vkCmdSetDepthBounds = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32);
pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);
pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);
pub type PFN_vkCmdSetStencilReference = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);
pub type PFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32);
pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);
pub type PFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);
pub type PFN_vkCmdDraw = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);
pub type PFN_vkCmdDrawIndexed = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);
pub type PFN_vkCmdDrawIndirect = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
pub type PFN_vkCmdDispatch = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
pub type PFN_vkCmdDispatchIndirect = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);
pub type PFN_vkCmdCopyImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy);
pub type PFN_vkCmdBlitImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter);
pub type PFN_vkCmdCopyBufferToImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);
pub type PFN_vkCmdCopyImageToBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);
pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const core::ffi::c_void);
pub type PFN_vkCmdFillBuffer = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32);
pub type PFN_vkCmdClearColorImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
pub type PFN_vkCmdClearAttachments = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);
pub type PFN_vkCmdResolveImage = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve);
pub type PFN_vkCmdSetEvent = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
pub type PFN_vkCmdResetEvent = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
pub type PFN_vkCmdWaitEvents = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
pub type PFN_vkCmdPipelineBarrier = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
pub type PFN_vkCmdBeginQuery = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);
pub type PFN_vkCmdEndQuery = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
pub type PFN_vkCmdResetQueryPool = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
pub type PFN_vkCmdWriteTimestamp = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: u32);
pub type PFN_vkCmdCopyQueryPoolResults = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags);
pub type PFN_vkCmdPushConstants = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const core::ffi::c_void);
pub type PFN_vkCmdBeginRenderPass = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);
pub type PFN_vkCmdNextSubpass = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
pub type PFN_vkCmdEndRenderPass = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);
pub type PFN_vkCmdExecuteCommands = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
pub type PFN_vkDestroySurfaceKHR = unsafe extern "system" fn(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;
pub type PFN_vkCreateSwapchainKHR = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR) -> VkResult;
pub type PFN_vkDestroySwapchainKHR = unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut VkImage) -> VkResult;
pub type PFN_vkAcquireNextImageKHR = unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, pImageIndex: *mut u32) -> VkResult;
pub type PFN_vkQueuePresentKHR = unsafe extern "system" fn(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult;
pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2);
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2);
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2, pImageFormatProperties: *mut VkImageFormatProperties2) -> VkResult;
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2);
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2);
pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlags);
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo, pExternalBufferProperties: *mut VkExternalBufferProperties);
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo, pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties);
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo, pExternalFenceProperties: *mut VkExternalFenceProperties);
pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(instance: VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties) -> VkResult;
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags);
pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfo) -> VkResult;
pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfo) -> VkResult;
pub type PFN_vkCmdSetDeviceMask = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, deviceMask: u32);
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "system" fn(device: VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR) -> VkResult;
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = unsafe extern "system" fn(device: VkDevice, surface: VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR) -> VkResult;
pub type PFN_vkAcquireNextImage2KHR = unsafe extern "system" fn(device: VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHR, pImageIndex: *mut u32) -> VkResult;
pub type PFN_vkCmdDispatchBase = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut VkRect2D) -> VkResult;
pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate) -> VkResult;
pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pData: *const core::ffi::c_void);
pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);
pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);
pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageSparseMemoryRequirementsInfo2, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2);
pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkDeviceBufferMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);
pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);
pub type PFN_vkGetDeviceImageSparseMemoryRequirements = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirements, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2);
pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo, pAllocator: *const VkAllocationCallbacks, pYcbcrConversion: *mut VkSamplerYcbcrConversion) -> VkResult;
pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversion, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(device: VkDevice, pQueueInfo: *const VkDeviceQueueInfo2, pQueue: *mut VkQueue);
pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pSupport: *mut VkDescriptorSetLayoutSupport);
pub type PFN_vkCreateRenderPass2 = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo2, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;
pub type PFN_vkCmdBeginRenderPass2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, pSubpassBeginInfo: *const VkSubpassBeginInfo);
pub type PFN_vkCmdNextSubpass2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pSubpassBeginInfo: *const VkSubpassBeginInfo, pSubpassEndInfo: *const VkSubpassEndInfo);
pub type PFN_vkCmdEndRenderPass2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfo);
pub type PFN_vkGetSemaphoreCounterValue = unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult;
pub type PFN_vkWaitSemaphores = unsafe extern "system" fn(device: VkDevice, pWaitInfo: *const VkSemaphoreWaitInfo, timeout: u64) -> VkResult;
pub type PFN_vkSignalSemaphore = unsafe extern "system" fn(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfo) -> VkResult;
pub type PFN_vkCmdDrawIndirectCount = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);
pub type PFN_vkCmdDrawIndexedIndirectCount = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);
pub type PFN_vkGetBufferOpaqueCaptureAddress = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> u64;
pub type PFN_vkGetBufferDeviceAddress = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> VkDeviceAddress;
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "system" fn(device: VkDevice, pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo) -> u64;
pub type PFN_vkGetPhysicalDeviceToolProperties = unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, pToolCount: *mut u32, pToolProperties: *mut VkPhysicalDeviceToolProperties) -> VkResult;
pub type PFN_vkCmdSetCullMode = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags);
pub type PFN_vkCmdSetFrontFace = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace);
pub type PFN_vkCmdSetPrimitiveTopology = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, primitiveTopology: VkPrimitiveTopology);
pub type PFN_vkCmdSetViewportWithCount = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, viewportCount: u32, pViewports: *const VkViewport);
pub type PFN_vkCmdSetScissorWithCount = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, scissorCount: u32, pScissors: *const VkRect2D);
pub type PFN_vkCmdBindVertexBuffers2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize, pSizes: *const VkDeviceSize, pStrides: *const VkDeviceSize);
pub type PFN_vkCmdSetDepthTestEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32);
pub type PFN_vkCmdSetDepthWriteEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32);
pub type PFN_vkCmdSetDepthCompareOp = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp);
pub type PFN_vkCmdSetDepthBoundsTestEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBoundsTestEnable: VkBool32);
pub type PFN_vkCmdSetStencilTestEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, stencilTestEnable: VkBool32);
pub type PFN_vkCmdSetStencilOp = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, failOp: VkStencilOp, passOp: VkStencilOp, depthFailOp: VkStencilOp, compareOp: VkCompareOp);
pub type PFN_vkCmdSetRasterizerDiscardEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, rasterizerDiscardEnable: VkBool32);
pub type PFN_vkCmdSetDepthBiasEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBiasEnable: VkBool32);
pub type PFN_vkCmdSetPrimitiveRestartEnable = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, primitiveRestartEnable: VkBool32);
pub type PFN_vkCreatePrivateDataSlot = unsafe extern "system" fn(device: VkDevice, pCreateInfo: *const VkPrivateDataSlotCreateInfo, pAllocator: *const VkAllocationCallbacks, pPrivateDataSlot: *mut VkPrivateDataSlot) -> VkResult;
pub type PFN_vkDestroyPrivateDataSlot = unsafe extern "system" fn(device: VkDevice, privateDataSlot: VkPrivateDataSlot, pAllocator: *const VkAllocationCallbacks);
pub type PFN_vkSetPrivateData = unsafe extern "system" fn(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, data: u64) -> VkResult;
pub type PFN_vkGetPrivateData = unsafe extern "system" fn(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, pData: *mut u64);
pub type PFN_vkCmdCopyBuffer2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pCopyBufferInfo: *const VkCopyBufferInfo2);
pub type PFN_vkCmdCopyImage2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pCopyImageInfo: *const VkCopyImageInfo2);
pub type PFN_vkCmdBlitImage2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pBlitImageInfo: *const VkBlitImageInfo2);
pub type PFN_vkCmdCopyBufferToImage2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2);
pub type PFN_vkCmdCopyImageToBuffer2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2);
pub type PFN_vkCmdResolveImage2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pResolveImageInfo: *const VkResolveImageInfo2);
pub type PFN_vkCmdSetEvent2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, pDependencyInfo: *const VkDependencyInfo);
pub type PFN_vkCmdResetEvent2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags2);
pub type PFN_vkCmdWaitEvents2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, pDependencyInfos: *const VkDependencyInfo);
pub type PFN_vkCmdPipelineBarrier2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfo);
pub type PFN_vkQueueSubmit2 = unsafe extern "system" fn(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo2, fence: VkFence) -> VkResult;
pub type PFN_vkCmdWriteTimestamp2 = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, stage: VkPipelineStageFlags2, queryPool: VkQueryPool, query: u32);
pub type PFN_vkCmdBeginRendering = unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pRenderingInfo: *const VkRenderingInfo);
pub type PFN_vkCmdEndRendering = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);

// ---------------------------------------------------------------
// Dispatch tables
//
// Vulkan has no link-time symbols worth using: the loader resolves
// everything through vkGetInstanceProcAddr and vkGetDeviceProcAddr.
// Device-level commands are deliberately fetched from the device
// getter, which returns a pointer straight into the driver instead
// of one into the loader's dispatch trampoline.
//
// A missing entry is None rather than a panic. Every one of these
// is legitimately absent on some driver, and a table that refuses
// to load because an unused command is missing is unusable.
// ---------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct EntryFns {
    pub create_instance: Option<PFN_vkCreateInstance>,
    pub get_device_proc_addr: Option<PFN_vkGetDeviceProcAddr>,
    pub get_instance_proc_addr: Option<PFN_vkGetInstanceProcAddr>,
    pub enumerate_instance_version: Option<PFN_vkEnumerateInstanceVersion>,
    pub enumerate_instance_layer_properties: Option<PFN_vkEnumerateInstanceLayerProperties>,
    pub enumerate_instance_extension_properties: Option<PFN_vkEnumerateInstanceExtensionProperties>,
}

impl EntryFns {
    /// Resolves every command in this table.
    ///
    /// # Safety
    ///
    /// `getter` must be the loader's real entry point, and `handle`
    /// must be a live VkInstance obtained from it (or
    /// VkInstance::NULL where the specification allows it).
    /// The returned pointers are only valid while `handle` is.
    #[allow(unused_variables)]
    pub unsafe fn load(getter: PFN_vkGetInstanceProcAddr, handle: VkInstance) -> Self {
        Self {
            create_instance: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateInstance>>(
                getter(handle, b"vkCreateInstance\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_device_proc_addr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDeviceProcAddr>>(
                getter(handle, b"vkGetDeviceProcAddr\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_instance_proc_addr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetInstanceProcAddr>>(
                getter(handle, b"vkGetInstanceProcAddr\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            enumerate_instance_version: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkEnumerateInstanceVersion>>(
                getter(handle, b"vkEnumerateInstanceVersion\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            enumerate_instance_layer_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkEnumerateInstanceLayerProperties>>(
                getter(handle, b"vkEnumerateInstanceLayerProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            enumerate_instance_extension_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkEnumerateInstanceExtensionProperties>>(
                getter(handle, b"vkEnumerateInstanceExtensionProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
        }
    }
}

#[derive(Clone, Copy)]
pub struct InstanceFns {
    pub destroy_instance: Option<PFN_vkDestroyInstance>,
    pub enumerate_physical_devices: Option<PFN_vkEnumeratePhysicalDevices>,
    pub get_physical_device_properties: Option<PFN_vkGetPhysicalDeviceProperties>,
    pub get_physical_device_queue_family_properties: Option<PFN_vkGetPhysicalDeviceQueueFamilyProperties>,
    pub get_physical_device_memory_properties: Option<PFN_vkGetPhysicalDeviceMemoryProperties>,
    pub get_physical_device_features: Option<PFN_vkGetPhysicalDeviceFeatures>,
    pub get_physical_device_format_properties: Option<PFN_vkGetPhysicalDeviceFormatProperties>,
    pub get_physical_device_image_format_properties: Option<PFN_vkGetPhysicalDeviceImageFormatProperties>,
    pub create_device: Option<PFN_vkCreateDevice>,
    pub enumerate_device_layer_properties: Option<PFN_vkEnumerateDeviceLayerProperties>,
    pub enumerate_device_extension_properties: Option<PFN_vkEnumerateDeviceExtensionProperties>,
    pub get_physical_device_sparse_image_format_properties: Option<PFN_vkGetPhysicalDeviceSparseImageFormatProperties>,
    pub destroy_surface_khr: Option<PFN_vkDestroySurfaceKHR>,
    pub get_physical_device_surface_support_khr: Option<PFN_vkGetPhysicalDeviceSurfaceSupportKHR>,
    pub get_physical_device_surface_capabilities_khr: Option<PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>,
    pub get_physical_device_surface_formats_khr: Option<PFN_vkGetPhysicalDeviceSurfaceFormatsKHR>,
    pub get_physical_device_surface_present_modes_khr: Option<PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>,
    pub get_physical_device_features2: Option<PFN_vkGetPhysicalDeviceFeatures2>,
    pub get_physical_device_properties2: Option<PFN_vkGetPhysicalDeviceProperties2>,
    pub get_physical_device_format_properties2: Option<PFN_vkGetPhysicalDeviceFormatProperties2>,
    pub get_physical_device_image_format_properties2: Option<PFN_vkGetPhysicalDeviceImageFormatProperties2>,
    pub get_physical_device_queue_family_properties2: Option<PFN_vkGetPhysicalDeviceQueueFamilyProperties2>,
    pub get_physical_device_memory_properties2: Option<PFN_vkGetPhysicalDeviceMemoryProperties2>,
    pub get_physical_device_sparse_image_format_properties2: Option<PFN_vkGetPhysicalDeviceSparseImageFormatProperties2>,
    pub get_physical_device_external_buffer_properties: Option<PFN_vkGetPhysicalDeviceExternalBufferProperties>,
    pub get_physical_device_external_semaphore_properties: Option<PFN_vkGetPhysicalDeviceExternalSemaphoreProperties>,
    pub get_physical_device_external_fence_properties: Option<PFN_vkGetPhysicalDeviceExternalFenceProperties>,
    pub enumerate_physical_device_groups: Option<PFN_vkEnumeratePhysicalDeviceGroups>,
    pub get_physical_device_present_rectangles_khr: Option<PFN_vkGetPhysicalDevicePresentRectanglesKHR>,
    pub get_physical_device_tool_properties: Option<PFN_vkGetPhysicalDeviceToolProperties>,
}

impl InstanceFns {
    /// Resolves every command in this table.
    ///
    /// # Safety
    ///
    /// `getter` must be the loader's real entry point, and `handle`
    /// must be a live VkInstance obtained from it (or
    /// VkInstance::NULL where the specification allows it).
    /// The returned pointers are only valid while `handle` is.
    #[allow(unused_variables)]
    pub unsafe fn load(getter: PFN_vkGetInstanceProcAddr, handle: VkInstance) -> Self {
        Self {
            destroy_instance: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyInstance>>(
                getter(handle, b"vkDestroyInstance\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            enumerate_physical_devices: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkEnumeratePhysicalDevices>>(
                getter(handle, b"vkEnumeratePhysicalDevices\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceProperties>>(
                getter(handle, b"vkGetPhysicalDeviceProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_queue_family_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceQueueFamilyProperties>>(
                getter(handle, b"vkGetPhysicalDeviceQueueFamilyProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_memory_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceMemoryProperties>>(
                getter(handle, b"vkGetPhysicalDeviceMemoryProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_features: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceFeatures>>(
                getter(handle, b"vkGetPhysicalDeviceFeatures\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_format_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceFormatProperties>>(
                getter(handle, b"vkGetPhysicalDeviceFormatProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_image_format_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceImageFormatProperties>>(
                getter(handle, b"vkGetPhysicalDeviceImageFormatProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_device: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateDevice>>(
                getter(handle, b"vkCreateDevice\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            enumerate_device_layer_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkEnumerateDeviceLayerProperties>>(
                getter(handle, b"vkEnumerateDeviceLayerProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            enumerate_device_extension_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkEnumerateDeviceExtensionProperties>>(
                getter(handle, b"vkEnumerateDeviceExtensionProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_sparse_image_format_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceSparseImageFormatProperties>>(
                getter(handle, b"vkGetPhysicalDeviceSparseImageFormatProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_surface_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroySurfaceKHR>>(
                getter(handle, b"vkDestroySurfaceKHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_surface_support_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceSurfaceSupportKHR>>(
                getter(handle, b"vkGetPhysicalDeviceSurfaceSupportKHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_surface_capabilities_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>>(
                getter(handle, b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_surface_formats_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceSurfaceFormatsKHR>>(
                getter(handle, b"vkGetPhysicalDeviceSurfaceFormatsKHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_surface_present_modes_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>>(
                getter(handle, b"vkGetPhysicalDeviceSurfacePresentModesKHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_features2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceFeatures2>>(
                getter(handle, b"vkGetPhysicalDeviceFeatures2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_properties2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceProperties2>>(
                getter(handle, b"vkGetPhysicalDeviceProperties2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_format_properties2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceFormatProperties2>>(
                getter(handle, b"vkGetPhysicalDeviceFormatProperties2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_image_format_properties2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceImageFormatProperties2>>(
                getter(handle, b"vkGetPhysicalDeviceImageFormatProperties2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_queue_family_properties2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceQueueFamilyProperties2>>(
                getter(handle, b"vkGetPhysicalDeviceQueueFamilyProperties2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_memory_properties2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceMemoryProperties2>>(
                getter(handle, b"vkGetPhysicalDeviceMemoryProperties2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_sparse_image_format_properties2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceSparseImageFormatProperties2>>(
                getter(handle, b"vkGetPhysicalDeviceSparseImageFormatProperties2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_external_buffer_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceExternalBufferProperties>>(
                getter(handle, b"vkGetPhysicalDeviceExternalBufferProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_external_semaphore_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceExternalSemaphoreProperties>>(
                getter(handle, b"vkGetPhysicalDeviceExternalSemaphoreProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_external_fence_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceExternalFenceProperties>>(
                getter(handle, b"vkGetPhysicalDeviceExternalFenceProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            enumerate_physical_device_groups: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkEnumeratePhysicalDeviceGroups>>(
                getter(handle, b"vkEnumeratePhysicalDeviceGroups\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_present_rectangles_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDevicePresentRectanglesKHR>>(
                getter(handle, b"vkGetPhysicalDevicePresentRectanglesKHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_physical_device_tool_properties: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPhysicalDeviceToolProperties>>(
                getter(handle, b"vkGetPhysicalDeviceToolProperties\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
        }
    }
}

#[derive(Clone, Copy)]
pub struct DeviceFns {
    pub destroy_device: Option<PFN_vkDestroyDevice>,
    pub get_device_queue: Option<PFN_vkGetDeviceQueue>,
    pub queue_submit: Option<PFN_vkQueueSubmit>,
    pub queue_wait_idle: Option<PFN_vkQueueWaitIdle>,
    pub device_wait_idle: Option<PFN_vkDeviceWaitIdle>,
    pub allocate_memory: Option<PFN_vkAllocateMemory>,
    pub free_memory: Option<PFN_vkFreeMemory>,
    pub map_memory: Option<PFN_vkMapMemory>,
    pub unmap_memory: Option<PFN_vkUnmapMemory>,
    pub flush_mapped_memory_ranges: Option<PFN_vkFlushMappedMemoryRanges>,
    pub invalidate_mapped_memory_ranges: Option<PFN_vkInvalidateMappedMemoryRanges>,
    pub get_device_memory_commitment: Option<PFN_vkGetDeviceMemoryCommitment>,
    pub get_buffer_memory_requirements: Option<PFN_vkGetBufferMemoryRequirements>,
    pub bind_buffer_memory: Option<PFN_vkBindBufferMemory>,
    pub get_image_memory_requirements: Option<PFN_vkGetImageMemoryRequirements>,
    pub bind_image_memory: Option<PFN_vkBindImageMemory>,
    pub get_image_sparse_memory_requirements: Option<PFN_vkGetImageSparseMemoryRequirements>,
    pub queue_bind_sparse: Option<PFN_vkQueueBindSparse>,
    pub create_fence: Option<PFN_vkCreateFence>,
    pub destroy_fence: Option<PFN_vkDestroyFence>,
    pub reset_fences: Option<PFN_vkResetFences>,
    pub get_fence_status: Option<PFN_vkGetFenceStatus>,
    pub wait_for_fences: Option<PFN_vkWaitForFences>,
    pub create_semaphore: Option<PFN_vkCreateSemaphore>,
    pub destroy_semaphore: Option<PFN_vkDestroySemaphore>,
    pub create_event: Option<PFN_vkCreateEvent>,
    pub destroy_event: Option<PFN_vkDestroyEvent>,
    pub get_event_status: Option<PFN_vkGetEventStatus>,
    pub set_event: Option<PFN_vkSetEvent>,
    pub reset_event: Option<PFN_vkResetEvent>,
    pub create_query_pool: Option<PFN_vkCreateQueryPool>,
    pub destroy_query_pool: Option<PFN_vkDestroyQueryPool>,
    pub get_query_pool_results: Option<PFN_vkGetQueryPoolResults>,
    pub reset_query_pool: Option<PFN_vkResetQueryPool>,
    pub create_buffer: Option<PFN_vkCreateBuffer>,
    pub destroy_buffer: Option<PFN_vkDestroyBuffer>,
    pub create_buffer_view: Option<PFN_vkCreateBufferView>,
    pub destroy_buffer_view: Option<PFN_vkDestroyBufferView>,
    pub create_image: Option<PFN_vkCreateImage>,
    pub destroy_image: Option<PFN_vkDestroyImage>,
    pub get_image_subresource_layout: Option<PFN_vkGetImageSubresourceLayout>,
    pub create_image_view: Option<PFN_vkCreateImageView>,
    pub destroy_image_view: Option<PFN_vkDestroyImageView>,
    pub create_shader_module: Option<PFN_vkCreateShaderModule>,
    pub destroy_shader_module: Option<PFN_vkDestroyShaderModule>,
    pub create_pipeline_cache: Option<PFN_vkCreatePipelineCache>,
    pub destroy_pipeline_cache: Option<PFN_vkDestroyPipelineCache>,
    pub get_pipeline_cache_data: Option<PFN_vkGetPipelineCacheData>,
    pub merge_pipeline_caches: Option<PFN_vkMergePipelineCaches>,
    pub create_graphics_pipelines: Option<PFN_vkCreateGraphicsPipelines>,
    pub create_compute_pipelines: Option<PFN_vkCreateComputePipelines>,
    pub destroy_pipeline: Option<PFN_vkDestroyPipeline>,
    pub create_pipeline_layout: Option<PFN_vkCreatePipelineLayout>,
    pub destroy_pipeline_layout: Option<PFN_vkDestroyPipelineLayout>,
    pub create_sampler: Option<PFN_vkCreateSampler>,
    pub destroy_sampler: Option<PFN_vkDestroySampler>,
    pub create_descriptor_set_layout: Option<PFN_vkCreateDescriptorSetLayout>,
    pub destroy_descriptor_set_layout: Option<PFN_vkDestroyDescriptorSetLayout>,
    pub create_descriptor_pool: Option<PFN_vkCreateDescriptorPool>,
    pub destroy_descriptor_pool: Option<PFN_vkDestroyDescriptorPool>,
    pub reset_descriptor_pool: Option<PFN_vkResetDescriptorPool>,
    pub allocate_descriptor_sets: Option<PFN_vkAllocateDescriptorSets>,
    pub free_descriptor_sets: Option<PFN_vkFreeDescriptorSets>,
    pub update_descriptor_sets: Option<PFN_vkUpdateDescriptorSets>,
    pub create_framebuffer: Option<PFN_vkCreateFramebuffer>,
    pub destroy_framebuffer: Option<PFN_vkDestroyFramebuffer>,
    pub create_render_pass: Option<PFN_vkCreateRenderPass>,
    pub destroy_render_pass: Option<PFN_vkDestroyRenderPass>,
    pub get_render_area_granularity: Option<PFN_vkGetRenderAreaGranularity>,
    pub create_command_pool: Option<PFN_vkCreateCommandPool>,
    pub destroy_command_pool: Option<PFN_vkDestroyCommandPool>,
    pub reset_command_pool: Option<PFN_vkResetCommandPool>,
    pub allocate_command_buffers: Option<PFN_vkAllocateCommandBuffers>,
    pub free_command_buffers: Option<PFN_vkFreeCommandBuffers>,
    pub begin_command_buffer: Option<PFN_vkBeginCommandBuffer>,
    pub end_command_buffer: Option<PFN_vkEndCommandBuffer>,
    pub reset_command_buffer: Option<PFN_vkResetCommandBuffer>,
    pub cmd_bind_pipeline: Option<PFN_vkCmdBindPipeline>,
    pub cmd_set_viewport: Option<PFN_vkCmdSetViewport>,
    pub cmd_set_scissor: Option<PFN_vkCmdSetScissor>,
    pub cmd_set_line_width: Option<PFN_vkCmdSetLineWidth>,
    pub cmd_set_depth_bias: Option<PFN_vkCmdSetDepthBias>,
    pub cmd_set_blend_constants: Option<PFN_vkCmdSetBlendConstants>,
    pub cmd_set_depth_bounds: Option<PFN_vkCmdSetDepthBounds>,
    pub cmd_set_stencil_compare_mask: Option<PFN_vkCmdSetStencilCompareMask>,
    pub cmd_set_stencil_write_mask: Option<PFN_vkCmdSetStencilWriteMask>,
    pub cmd_set_stencil_reference: Option<PFN_vkCmdSetStencilReference>,
    pub cmd_bind_descriptor_sets: Option<PFN_vkCmdBindDescriptorSets>,
    pub cmd_bind_index_buffer: Option<PFN_vkCmdBindIndexBuffer>,
    pub cmd_bind_vertex_buffers: Option<PFN_vkCmdBindVertexBuffers>,
    pub cmd_draw: Option<PFN_vkCmdDraw>,
    pub cmd_draw_indexed: Option<PFN_vkCmdDrawIndexed>,
    pub cmd_draw_indirect: Option<PFN_vkCmdDrawIndirect>,
    pub cmd_draw_indexed_indirect: Option<PFN_vkCmdDrawIndexedIndirect>,
    pub cmd_dispatch: Option<PFN_vkCmdDispatch>,
    pub cmd_dispatch_indirect: Option<PFN_vkCmdDispatchIndirect>,
    pub cmd_copy_buffer: Option<PFN_vkCmdCopyBuffer>,
    pub cmd_copy_image: Option<PFN_vkCmdCopyImage>,
    pub cmd_blit_image: Option<PFN_vkCmdBlitImage>,
    pub cmd_copy_buffer_to_image: Option<PFN_vkCmdCopyBufferToImage>,
    pub cmd_copy_image_to_buffer: Option<PFN_vkCmdCopyImageToBuffer>,
    pub cmd_update_buffer: Option<PFN_vkCmdUpdateBuffer>,
    pub cmd_fill_buffer: Option<PFN_vkCmdFillBuffer>,
    pub cmd_clear_color_image: Option<PFN_vkCmdClearColorImage>,
    pub cmd_clear_depth_stencil_image: Option<PFN_vkCmdClearDepthStencilImage>,
    pub cmd_clear_attachments: Option<PFN_vkCmdClearAttachments>,
    pub cmd_resolve_image: Option<PFN_vkCmdResolveImage>,
    pub cmd_set_event: Option<PFN_vkCmdSetEvent>,
    pub cmd_reset_event: Option<PFN_vkCmdResetEvent>,
    pub cmd_wait_events: Option<PFN_vkCmdWaitEvents>,
    pub cmd_pipeline_barrier: Option<PFN_vkCmdPipelineBarrier>,
    pub cmd_begin_query: Option<PFN_vkCmdBeginQuery>,
    pub cmd_end_query: Option<PFN_vkCmdEndQuery>,
    pub cmd_reset_query_pool: Option<PFN_vkCmdResetQueryPool>,
    pub cmd_write_timestamp: Option<PFN_vkCmdWriteTimestamp>,
    pub cmd_copy_query_pool_results: Option<PFN_vkCmdCopyQueryPoolResults>,
    pub cmd_push_constants: Option<PFN_vkCmdPushConstants>,
    pub cmd_begin_render_pass: Option<PFN_vkCmdBeginRenderPass>,
    pub cmd_next_subpass: Option<PFN_vkCmdNextSubpass>,
    pub cmd_end_render_pass: Option<PFN_vkCmdEndRenderPass>,
    pub cmd_execute_commands: Option<PFN_vkCmdExecuteCommands>,
    pub create_swapchain_khr: Option<PFN_vkCreateSwapchainKHR>,
    pub destroy_swapchain_khr: Option<PFN_vkDestroySwapchainKHR>,
    pub get_swapchain_images_khr: Option<PFN_vkGetSwapchainImagesKHR>,
    pub acquire_next_image_khr: Option<PFN_vkAcquireNextImageKHR>,
    pub queue_present_khr: Option<PFN_vkQueuePresentKHR>,
    pub trim_command_pool: Option<PFN_vkTrimCommandPool>,
    pub get_device_group_peer_memory_features: Option<PFN_vkGetDeviceGroupPeerMemoryFeatures>,
    pub bind_buffer_memory2: Option<PFN_vkBindBufferMemory2>,
    pub bind_image_memory2: Option<PFN_vkBindImageMemory2>,
    pub cmd_set_device_mask: Option<PFN_vkCmdSetDeviceMask>,
    pub get_device_group_present_capabilities_khr: Option<PFN_vkGetDeviceGroupPresentCapabilitiesKHR>,
    pub get_device_group_surface_present_modes_khr: Option<PFN_vkGetDeviceGroupSurfacePresentModesKHR>,
    pub acquire_next_image2_khr: Option<PFN_vkAcquireNextImage2KHR>,
    pub cmd_dispatch_base: Option<PFN_vkCmdDispatchBase>,
    pub create_descriptor_update_template: Option<PFN_vkCreateDescriptorUpdateTemplate>,
    pub destroy_descriptor_update_template: Option<PFN_vkDestroyDescriptorUpdateTemplate>,
    pub update_descriptor_set_with_template: Option<PFN_vkUpdateDescriptorSetWithTemplate>,
    pub get_buffer_memory_requirements2: Option<PFN_vkGetBufferMemoryRequirements2>,
    pub get_image_memory_requirements2: Option<PFN_vkGetImageMemoryRequirements2>,
    pub get_image_sparse_memory_requirements2: Option<PFN_vkGetImageSparseMemoryRequirements2>,
    pub get_device_buffer_memory_requirements: Option<PFN_vkGetDeviceBufferMemoryRequirements>,
    pub get_device_image_memory_requirements: Option<PFN_vkGetDeviceImageMemoryRequirements>,
    pub get_device_image_sparse_memory_requirements: Option<PFN_vkGetDeviceImageSparseMemoryRequirements>,
    pub create_sampler_ycbcr_conversion: Option<PFN_vkCreateSamplerYcbcrConversion>,
    pub destroy_sampler_ycbcr_conversion: Option<PFN_vkDestroySamplerYcbcrConversion>,
    pub get_device_queue2: Option<PFN_vkGetDeviceQueue2>,
    pub get_descriptor_set_layout_support: Option<PFN_vkGetDescriptorSetLayoutSupport>,
    pub create_render_pass2: Option<PFN_vkCreateRenderPass2>,
    pub cmd_begin_render_pass2: Option<PFN_vkCmdBeginRenderPass2>,
    pub cmd_next_subpass2: Option<PFN_vkCmdNextSubpass2>,
    pub cmd_end_render_pass2: Option<PFN_vkCmdEndRenderPass2>,
    pub get_semaphore_counter_value: Option<PFN_vkGetSemaphoreCounterValue>,
    pub wait_semaphores: Option<PFN_vkWaitSemaphores>,
    pub signal_semaphore: Option<PFN_vkSignalSemaphore>,
    pub cmd_draw_indirect_count: Option<PFN_vkCmdDrawIndirectCount>,
    pub cmd_draw_indexed_indirect_count: Option<PFN_vkCmdDrawIndexedIndirectCount>,
    pub get_buffer_opaque_capture_address: Option<PFN_vkGetBufferOpaqueCaptureAddress>,
    pub get_buffer_device_address: Option<PFN_vkGetBufferDeviceAddress>,
    pub get_device_memory_opaque_capture_address: Option<PFN_vkGetDeviceMemoryOpaqueCaptureAddress>,
    pub cmd_set_cull_mode: Option<PFN_vkCmdSetCullMode>,
    pub cmd_set_front_face: Option<PFN_vkCmdSetFrontFace>,
    pub cmd_set_primitive_topology: Option<PFN_vkCmdSetPrimitiveTopology>,
    pub cmd_set_viewport_with_count: Option<PFN_vkCmdSetViewportWithCount>,
    pub cmd_set_scissor_with_count: Option<PFN_vkCmdSetScissorWithCount>,
    pub cmd_bind_vertex_buffers2: Option<PFN_vkCmdBindVertexBuffers2>,
    pub cmd_set_depth_test_enable: Option<PFN_vkCmdSetDepthTestEnable>,
    pub cmd_set_depth_write_enable: Option<PFN_vkCmdSetDepthWriteEnable>,
    pub cmd_set_depth_compare_op: Option<PFN_vkCmdSetDepthCompareOp>,
    pub cmd_set_depth_bounds_test_enable: Option<PFN_vkCmdSetDepthBoundsTestEnable>,
    pub cmd_set_stencil_test_enable: Option<PFN_vkCmdSetStencilTestEnable>,
    pub cmd_set_stencil_op: Option<PFN_vkCmdSetStencilOp>,
    pub cmd_set_rasterizer_discard_enable: Option<PFN_vkCmdSetRasterizerDiscardEnable>,
    pub cmd_set_depth_bias_enable: Option<PFN_vkCmdSetDepthBiasEnable>,
    pub cmd_set_primitive_restart_enable: Option<PFN_vkCmdSetPrimitiveRestartEnable>,
    pub create_private_data_slot: Option<PFN_vkCreatePrivateDataSlot>,
    pub destroy_private_data_slot: Option<PFN_vkDestroyPrivateDataSlot>,
    pub set_private_data: Option<PFN_vkSetPrivateData>,
    pub get_private_data: Option<PFN_vkGetPrivateData>,
    pub cmd_copy_buffer2: Option<PFN_vkCmdCopyBuffer2>,
    pub cmd_copy_image2: Option<PFN_vkCmdCopyImage2>,
    pub cmd_blit_image2: Option<PFN_vkCmdBlitImage2>,
    pub cmd_copy_buffer_to_image2: Option<PFN_vkCmdCopyBufferToImage2>,
    pub cmd_copy_image_to_buffer2: Option<PFN_vkCmdCopyImageToBuffer2>,
    pub cmd_resolve_image2: Option<PFN_vkCmdResolveImage2>,
    pub cmd_set_event2: Option<PFN_vkCmdSetEvent2>,
    pub cmd_reset_event2: Option<PFN_vkCmdResetEvent2>,
    pub cmd_wait_events2: Option<PFN_vkCmdWaitEvents2>,
    pub cmd_pipeline_barrier2: Option<PFN_vkCmdPipelineBarrier2>,
    pub queue_submit2: Option<PFN_vkQueueSubmit2>,
    pub cmd_write_timestamp2: Option<PFN_vkCmdWriteTimestamp2>,
    pub cmd_begin_rendering: Option<PFN_vkCmdBeginRendering>,
    pub cmd_end_rendering: Option<PFN_vkCmdEndRendering>,
}

impl DeviceFns {
    /// Resolves every command in this table.
    ///
    /// # Safety
    ///
    /// `getter` must be the loader's real entry point, and `handle`
    /// must be a live VkDevice obtained from it (or
    /// VkDevice::NULL where the specification allows it).
    /// The returned pointers are only valid while `handle` is.
    #[allow(unused_variables)]
    pub unsafe fn load(getter: PFN_vkGetDeviceProcAddr, handle: VkDevice) -> Self {
        Self {
            destroy_device: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyDevice>>(
                getter(handle, b"vkDestroyDevice\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_device_queue: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDeviceQueue>>(
                getter(handle, b"vkGetDeviceQueue\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            queue_submit: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkQueueSubmit>>(
                getter(handle, b"vkQueueSubmit\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            queue_wait_idle: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkQueueWaitIdle>>(
                getter(handle, b"vkQueueWaitIdle\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            device_wait_idle: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDeviceWaitIdle>>(
                getter(handle, b"vkDeviceWaitIdle\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            allocate_memory: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkAllocateMemory>>(
                getter(handle, b"vkAllocateMemory\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            free_memory: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkFreeMemory>>(
                getter(handle, b"vkFreeMemory\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            map_memory: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkMapMemory>>(
                getter(handle, b"vkMapMemory\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            unmap_memory: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkUnmapMemory>>(
                getter(handle, b"vkUnmapMemory\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            flush_mapped_memory_ranges: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkFlushMappedMemoryRanges>>(
                getter(handle, b"vkFlushMappedMemoryRanges\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            invalidate_mapped_memory_ranges: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkInvalidateMappedMemoryRanges>>(
                getter(handle, b"vkInvalidateMappedMemoryRanges\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_device_memory_commitment: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDeviceMemoryCommitment>>(
                getter(handle, b"vkGetDeviceMemoryCommitment\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_buffer_memory_requirements: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetBufferMemoryRequirements>>(
                getter(handle, b"vkGetBufferMemoryRequirements\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            bind_buffer_memory: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkBindBufferMemory>>(
                getter(handle, b"vkBindBufferMemory\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_image_memory_requirements: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetImageMemoryRequirements>>(
                getter(handle, b"vkGetImageMemoryRequirements\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            bind_image_memory: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkBindImageMemory>>(
                getter(handle, b"vkBindImageMemory\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_image_sparse_memory_requirements: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetImageSparseMemoryRequirements>>(
                getter(handle, b"vkGetImageSparseMemoryRequirements\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            queue_bind_sparse: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkQueueBindSparse>>(
                getter(handle, b"vkQueueBindSparse\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_fence: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateFence>>(
                getter(handle, b"vkCreateFence\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_fence: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyFence>>(
                getter(handle, b"vkDestroyFence\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            reset_fences: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkResetFences>>(
                getter(handle, b"vkResetFences\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_fence_status: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetFenceStatus>>(
                getter(handle, b"vkGetFenceStatus\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            wait_for_fences: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkWaitForFences>>(
                getter(handle, b"vkWaitForFences\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_semaphore: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateSemaphore>>(
                getter(handle, b"vkCreateSemaphore\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_semaphore: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroySemaphore>>(
                getter(handle, b"vkDestroySemaphore\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_event: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateEvent>>(
                getter(handle, b"vkCreateEvent\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_event: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyEvent>>(
                getter(handle, b"vkDestroyEvent\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_event_status: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetEventStatus>>(
                getter(handle, b"vkGetEventStatus\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            set_event: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkSetEvent>>(
                getter(handle, b"vkSetEvent\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            reset_event: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkResetEvent>>(
                getter(handle, b"vkResetEvent\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_query_pool: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateQueryPool>>(
                getter(handle, b"vkCreateQueryPool\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_query_pool: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyQueryPool>>(
                getter(handle, b"vkDestroyQueryPool\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_query_pool_results: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetQueryPoolResults>>(
                getter(handle, b"vkGetQueryPoolResults\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            reset_query_pool: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkResetQueryPool>>(
                getter(handle, b"vkResetQueryPool\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_buffer: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateBuffer>>(
                getter(handle, b"vkCreateBuffer\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_buffer: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyBuffer>>(
                getter(handle, b"vkDestroyBuffer\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_buffer_view: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateBufferView>>(
                getter(handle, b"vkCreateBufferView\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_buffer_view: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyBufferView>>(
                getter(handle, b"vkDestroyBufferView\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_image: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateImage>>(
                getter(handle, b"vkCreateImage\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_image: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyImage>>(
                getter(handle, b"vkDestroyImage\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_image_subresource_layout: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetImageSubresourceLayout>>(
                getter(handle, b"vkGetImageSubresourceLayout\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_image_view: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateImageView>>(
                getter(handle, b"vkCreateImageView\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_image_view: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyImageView>>(
                getter(handle, b"vkDestroyImageView\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_shader_module: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateShaderModule>>(
                getter(handle, b"vkCreateShaderModule\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_shader_module: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyShaderModule>>(
                getter(handle, b"vkDestroyShaderModule\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_pipeline_cache: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreatePipelineCache>>(
                getter(handle, b"vkCreatePipelineCache\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_pipeline_cache: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyPipelineCache>>(
                getter(handle, b"vkDestroyPipelineCache\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_pipeline_cache_data: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPipelineCacheData>>(
                getter(handle, b"vkGetPipelineCacheData\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            merge_pipeline_caches: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkMergePipelineCaches>>(
                getter(handle, b"vkMergePipelineCaches\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_graphics_pipelines: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateGraphicsPipelines>>(
                getter(handle, b"vkCreateGraphicsPipelines\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_compute_pipelines: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateComputePipelines>>(
                getter(handle, b"vkCreateComputePipelines\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_pipeline: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyPipeline>>(
                getter(handle, b"vkDestroyPipeline\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_pipeline_layout: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreatePipelineLayout>>(
                getter(handle, b"vkCreatePipelineLayout\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_pipeline_layout: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyPipelineLayout>>(
                getter(handle, b"vkDestroyPipelineLayout\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_sampler: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateSampler>>(
                getter(handle, b"vkCreateSampler\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_sampler: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroySampler>>(
                getter(handle, b"vkDestroySampler\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_descriptor_set_layout: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateDescriptorSetLayout>>(
                getter(handle, b"vkCreateDescriptorSetLayout\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_descriptor_set_layout: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyDescriptorSetLayout>>(
                getter(handle, b"vkDestroyDescriptorSetLayout\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_descriptor_pool: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateDescriptorPool>>(
                getter(handle, b"vkCreateDescriptorPool\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_descriptor_pool: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyDescriptorPool>>(
                getter(handle, b"vkDestroyDescriptorPool\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            reset_descriptor_pool: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkResetDescriptorPool>>(
                getter(handle, b"vkResetDescriptorPool\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            allocate_descriptor_sets: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkAllocateDescriptorSets>>(
                getter(handle, b"vkAllocateDescriptorSets\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            free_descriptor_sets: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkFreeDescriptorSets>>(
                getter(handle, b"vkFreeDescriptorSets\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            update_descriptor_sets: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkUpdateDescriptorSets>>(
                getter(handle, b"vkUpdateDescriptorSets\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_framebuffer: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateFramebuffer>>(
                getter(handle, b"vkCreateFramebuffer\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_framebuffer: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyFramebuffer>>(
                getter(handle, b"vkDestroyFramebuffer\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_render_pass: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateRenderPass>>(
                getter(handle, b"vkCreateRenderPass\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_render_pass: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyRenderPass>>(
                getter(handle, b"vkDestroyRenderPass\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_render_area_granularity: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetRenderAreaGranularity>>(
                getter(handle, b"vkGetRenderAreaGranularity\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_command_pool: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateCommandPool>>(
                getter(handle, b"vkCreateCommandPool\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_command_pool: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyCommandPool>>(
                getter(handle, b"vkDestroyCommandPool\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            reset_command_pool: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkResetCommandPool>>(
                getter(handle, b"vkResetCommandPool\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            allocate_command_buffers: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkAllocateCommandBuffers>>(
                getter(handle, b"vkAllocateCommandBuffers\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            free_command_buffers: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkFreeCommandBuffers>>(
                getter(handle, b"vkFreeCommandBuffers\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            begin_command_buffer: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkBeginCommandBuffer>>(
                getter(handle, b"vkBeginCommandBuffer\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            end_command_buffer: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkEndCommandBuffer>>(
                getter(handle, b"vkEndCommandBuffer\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            reset_command_buffer: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkResetCommandBuffer>>(
                getter(handle, b"vkResetCommandBuffer\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_bind_pipeline: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdBindPipeline>>(
                getter(handle, b"vkCmdBindPipeline\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_viewport: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetViewport>>(
                getter(handle, b"vkCmdSetViewport\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_scissor: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetScissor>>(
                getter(handle, b"vkCmdSetScissor\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_line_width: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetLineWidth>>(
                getter(handle, b"vkCmdSetLineWidth\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_depth_bias: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetDepthBias>>(
                getter(handle, b"vkCmdSetDepthBias\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_blend_constants: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetBlendConstants>>(
                getter(handle, b"vkCmdSetBlendConstants\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_depth_bounds: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetDepthBounds>>(
                getter(handle, b"vkCmdSetDepthBounds\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_stencil_compare_mask: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetStencilCompareMask>>(
                getter(handle, b"vkCmdSetStencilCompareMask\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_stencil_write_mask: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetStencilWriteMask>>(
                getter(handle, b"vkCmdSetStencilWriteMask\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_stencil_reference: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetStencilReference>>(
                getter(handle, b"vkCmdSetStencilReference\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_bind_descriptor_sets: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdBindDescriptorSets>>(
                getter(handle, b"vkCmdBindDescriptorSets\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_bind_index_buffer: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdBindIndexBuffer>>(
                getter(handle, b"vkCmdBindIndexBuffer\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_bind_vertex_buffers: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdBindVertexBuffers>>(
                getter(handle, b"vkCmdBindVertexBuffers\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_draw: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdDraw>>(
                getter(handle, b"vkCmdDraw\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_draw_indexed: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdDrawIndexed>>(
                getter(handle, b"vkCmdDrawIndexed\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_draw_indirect: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdDrawIndirect>>(
                getter(handle, b"vkCmdDrawIndirect\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_draw_indexed_indirect: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdDrawIndexedIndirect>>(
                getter(handle, b"vkCmdDrawIndexedIndirect\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_dispatch: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdDispatch>>(
                getter(handle, b"vkCmdDispatch\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_dispatch_indirect: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdDispatchIndirect>>(
                getter(handle, b"vkCmdDispatchIndirect\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_copy_buffer: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdCopyBuffer>>(
                getter(handle, b"vkCmdCopyBuffer\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_copy_image: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdCopyImage>>(
                getter(handle, b"vkCmdCopyImage\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_blit_image: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdBlitImage>>(
                getter(handle, b"vkCmdBlitImage\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_copy_buffer_to_image: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdCopyBufferToImage>>(
                getter(handle, b"vkCmdCopyBufferToImage\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_copy_image_to_buffer: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdCopyImageToBuffer>>(
                getter(handle, b"vkCmdCopyImageToBuffer\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_update_buffer: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdUpdateBuffer>>(
                getter(handle, b"vkCmdUpdateBuffer\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_fill_buffer: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdFillBuffer>>(
                getter(handle, b"vkCmdFillBuffer\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_clear_color_image: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdClearColorImage>>(
                getter(handle, b"vkCmdClearColorImage\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_clear_depth_stencil_image: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdClearDepthStencilImage>>(
                getter(handle, b"vkCmdClearDepthStencilImage\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_clear_attachments: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdClearAttachments>>(
                getter(handle, b"vkCmdClearAttachments\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_resolve_image: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdResolveImage>>(
                getter(handle, b"vkCmdResolveImage\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_event: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetEvent>>(
                getter(handle, b"vkCmdSetEvent\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_reset_event: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdResetEvent>>(
                getter(handle, b"vkCmdResetEvent\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_wait_events: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdWaitEvents>>(
                getter(handle, b"vkCmdWaitEvents\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_pipeline_barrier: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdPipelineBarrier>>(
                getter(handle, b"vkCmdPipelineBarrier\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_begin_query: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdBeginQuery>>(
                getter(handle, b"vkCmdBeginQuery\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_end_query: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdEndQuery>>(
                getter(handle, b"vkCmdEndQuery\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_reset_query_pool: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdResetQueryPool>>(
                getter(handle, b"vkCmdResetQueryPool\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_write_timestamp: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdWriteTimestamp>>(
                getter(handle, b"vkCmdWriteTimestamp\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_copy_query_pool_results: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdCopyQueryPoolResults>>(
                getter(handle, b"vkCmdCopyQueryPoolResults\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_push_constants: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdPushConstants>>(
                getter(handle, b"vkCmdPushConstants\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_begin_render_pass: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdBeginRenderPass>>(
                getter(handle, b"vkCmdBeginRenderPass\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_next_subpass: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdNextSubpass>>(
                getter(handle, b"vkCmdNextSubpass\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_end_render_pass: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdEndRenderPass>>(
                getter(handle, b"vkCmdEndRenderPass\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_execute_commands: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdExecuteCommands>>(
                getter(handle, b"vkCmdExecuteCommands\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_swapchain_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateSwapchainKHR>>(
                getter(handle, b"vkCreateSwapchainKHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_swapchain_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroySwapchainKHR>>(
                getter(handle, b"vkDestroySwapchainKHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_swapchain_images_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetSwapchainImagesKHR>>(
                getter(handle, b"vkGetSwapchainImagesKHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            acquire_next_image_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkAcquireNextImageKHR>>(
                getter(handle, b"vkAcquireNextImageKHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            queue_present_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkQueuePresentKHR>>(
                getter(handle, b"vkQueuePresentKHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            trim_command_pool: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkTrimCommandPool>>(
                getter(handle, b"vkTrimCommandPool\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_device_group_peer_memory_features: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDeviceGroupPeerMemoryFeatures>>(
                getter(handle, b"vkGetDeviceGroupPeerMemoryFeatures\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            bind_buffer_memory2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkBindBufferMemory2>>(
                getter(handle, b"vkBindBufferMemory2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            bind_image_memory2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkBindImageMemory2>>(
                getter(handle, b"vkBindImageMemory2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_device_mask: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetDeviceMask>>(
                getter(handle, b"vkCmdSetDeviceMask\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_device_group_present_capabilities_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDeviceGroupPresentCapabilitiesKHR>>(
                getter(handle, b"vkGetDeviceGroupPresentCapabilitiesKHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_device_group_surface_present_modes_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDeviceGroupSurfacePresentModesKHR>>(
                getter(handle, b"vkGetDeviceGroupSurfacePresentModesKHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            acquire_next_image2_khr: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkAcquireNextImage2KHR>>(
                getter(handle, b"vkAcquireNextImage2KHR\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_dispatch_base: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdDispatchBase>>(
                getter(handle, b"vkCmdDispatchBase\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_descriptor_update_template: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateDescriptorUpdateTemplate>>(
                getter(handle, b"vkCreateDescriptorUpdateTemplate\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_descriptor_update_template: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyDescriptorUpdateTemplate>>(
                getter(handle, b"vkDestroyDescriptorUpdateTemplate\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            update_descriptor_set_with_template: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkUpdateDescriptorSetWithTemplate>>(
                getter(handle, b"vkUpdateDescriptorSetWithTemplate\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_buffer_memory_requirements2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetBufferMemoryRequirements2>>(
                getter(handle, b"vkGetBufferMemoryRequirements2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_image_memory_requirements2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetImageMemoryRequirements2>>(
                getter(handle, b"vkGetImageMemoryRequirements2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_image_sparse_memory_requirements2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetImageSparseMemoryRequirements2>>(
                getter(handle, b"vkGetImageSparseMemoryRequirements2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_device_buffer_memory_requirements: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDeviceBufferMemoryRequirements>>(
                getter(handle, b"vkGetDeviceBufferMemoryRequirements\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_device_image_memory_requirements: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDeviceImageMemoryRequirements>>(
                getter(handle, b"vkGetDeviceImageMemoryRequirements\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_device_image_sparse_memory_requirements: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDeviceImageSparseMemoryRequirements>>(
                getter(handle, b"vkGetDeviceImageSparseMemoryRequirements\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_sampler_ycbcr_conversion: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateSamplerYcbcrConversion>>(
                getter(handle, b"vkCreateSamplerYcbcrConversion\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_sampler_ycbcr_conversion: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroySamplerYcbcrConversion>>(
                getter(handle, b"vkDestroySamplerYcbcrConversion\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_device_queue2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDeviceQueue2>>(
                getter(handle, b"vkGetDeviceQueue2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_descriptor_set_layout_support: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDescriptorSetLayoutSupport>>(
                getter(handle, b"vkGetDescriptorSetLayoutSupport\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_render_pass2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreateRenderPass2>>(
                getter(handle, b"vkCreateRenderPass2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_begin_render_pass2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdBeginRenderPass2>>(
                getter(handle, b"vkCmdBeginRenderPass2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_next_subpass2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdNextSubpass2>>(
                getter(handle, b"vkCmdNextSubpass2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_end_render_pass2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdEndRenderPass2>>(
                getter(handle, b"vkCmdEndRenderPass2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_semaphore_counter_value: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetSemaphoreCounterValue>>(
                getter(handle, b"vkGetSemaphoreCounterValue\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            wait_semaphores: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkWaitSemaphores>>(
                getter(handle, b"vkWaitSemaphores\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            signal_semaphore: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkSignalSemaphore>>(
                getter(handle, b"vkSignalSemaphore\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_draw_indirect_count: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdDrawIndirectCount>>(
                getter(handle, b"vkCmdDrawIndirectCount\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_draw_indexed_indirect_count: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdDrawIndexedIndirectCount>>(
                getter(handle, b"vkCmdDrawIndexedIndirectCount\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_buffer_opaque_capture_address: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetBufferOpaqueCaptureAddress>>(
                getter(handle, b"vkGetBufferOpaqueCaptureAddress\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_buffer_device_address: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetBufferDeviceAddress>>(
                getter(handle, b"vkGetBufferDeviceAddress\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_device_memory_opaque_capture_address: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetDeviceMemoryOpaqueCaptureAddress>>(
                getter(handle, b"vkGetDeviceMemoryOpaqueCaptureAddress\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_cull_mode: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetCullMode>>(
                getter(handle, b"vkCmdSetCullMode\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_front_face: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetFrontFace>>(
                getter(handle, b"vkCmdSetFrontFace\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_primitive_topology: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetPrimitiveTopology>>(
                getter(handle, b"vkCmdSetPrimitiveTopology\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_viewport_with_count: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetViewportWithCount>>(
                getter(handle, b"vkCmdSetViewportWithCount\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_scissor_with_count: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetScissorWithCount>>(
                getter(handle, b"vkCmdSetScissorWithCount\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_bind_vertex_buffers2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdBindVertexBuffers2>>(
                getter(handle, b"vkCmdBindVertexBuffers2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_depth_test_enable: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetDepthTestEnable>>(
                getter(handle, b"vkCmdSetDepthTestEnable\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_depth_write_enable: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetDepthWriteEnable>>(
                getter(handle, b"vkCmdSetDepthWriteEnable\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_depth_compare_op: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetDepthCompareOp>>(
                getter(handle, b"vkCmdSetDepthCompareOp\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_depth_bounds_test_enable: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetDepthBoundsTestEnable>>(
                getter(handle, b"vkCmdSetDepthBoundsTestEnable\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_stencil_test_enable: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetStencilTestEnable>>(
                getter(handle, b"vkCmdSetStencilTestEnable\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_stencil_op: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetStencilOp>>(
                getter(handle, b"vkCmdSetStencilOp\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_rasterizer_discard_enable: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetRasterizerDiscardEnable>>(
                getter(handle, b"vkCmdSetRasterizerDiscardEnable\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_depth_bias_enable: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetDepthBiasEnable>>(
                getter(handle, b"vkCmdSetDepthBiasEnable\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_primitive_restart_enable: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetPrimitiveRestartEnable>>(
                getter(handle, b"vkCmdSetPrimitiveRestartEnable\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            create_private_data_slot: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCreatePrivateDataSlot>>(
                getter(handle, b"vkCreatePrivateDataSlot\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            destroy_private_data_slot: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkDestroyPrivateDataSlot>>(
                getter(handle, b"vkDestroyPrivateDataSlot\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            set_private_data: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkSetPrivateData>>(
                getter(handle, b"vkSetPrivateData\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            get_private_data: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkGetPrivateData>>(
                getter(handle, b"vkGetPrivateData\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_copy_buffer2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdCopyBuffer2>>(
                getter(handle, b"vkCmdCopyBuffer2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_copy_image2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdCopyImage2>>(
                getter(handle, b"vkCmdCopyImage2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_blit_image2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdBlitImage2>>(
                getter(handle, b"vkCmdBlitImage2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_copy_buffer_to_image2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdCopyBufferToImage2>>(
                getter(handle, b"vkCmdCopyBufferToImage2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_copy_image_to_buffer2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdCopyImageToBuffer2>>(
                getter(handle, b"vkCmdCopyImageToBuffer2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_resolve_image2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdResolveImage2>>(
                getter(handle, b"vkCmdResolveImage2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_set_event2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdSetEvent2>>(
                getter(handle, b"vkCmdSetEvent2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_reset_event2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdResetEvent2>>(
                getter(handle, b"vkCmdResetEvent2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_wait_events2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdWaitEvents2>>(
                getter(handle, b"vkCmdWaitEvents2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_pipeline_barrier2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdPipelineBarrier2>>(
                getter(handle, b"vkCmdPipelineBarrier2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            queue_submit2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkQueueSubmit2>>(
                getter(handle, b"vkQueueSubmit2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_write_timestamp2: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdWriteTimestamp2>>(
                getter(handle, b"vkCmdWriteTimestamp2\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_begin_rendering: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdBeginRendering>>(
                getter(handle, b"vkCmdBeginRendering\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
            cmd_end_rendering: core::mem::transmute::<PFN_vkVoidFunction, Option<PFN_vkCmdEndRendering>>(
                getter(handle, b"vkCmdEndRendering\0".as_ptr().cast::<core::ffi::c_char>()),
            ),
        }
    }
}

