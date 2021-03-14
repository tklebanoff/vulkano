// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#![doc(html_logo_url = "https://raw.githubusercontent.com/vulkano-rs/vulkano/master/logo.png")]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate paste;
use paste::paste;

pub mod stype;     pub use crate::stype::*;
pub mod format;    pub use crate::format::*;
pub mod extension; pub use crate::extension::*;

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::os::raw::c_char;
use std::os::raw::c_double;
use std::os::raw::c_ulong;
use std::os::raw::c_void;

pub type Flags = u32;
pub type Bool32 = u32;
pub type DeviceSize = u64;
pub type SampleMask = u32;

pub type Instance = usize;
pub type PhysicalDevice = usize;
pub type Device = usize;
pub type Queue = usize;
pub type CommandBuffer = usize;

pub type Semaphore = u64;
pub type Fence = u64;
pub type DeviceMemory = u64;
pub type Buffer = u64;
pub type Image = u64;
pub type Event = u64;
pub type QueryPool = u64;
pub type BufferView = u64;
pub type ImageView = u64;
pub type ShaderModule = u64;
pub type PipelineCache = u64;
pub type PipelineLayout = u64;
pub type RenderPass = u64;
pub type Pipeline = u64;
pub type DescriptorSetLayout = u64;
pub type Sampler = u64;
pub type DescriptorPool = u64;
pub type DescriptorSet = u64;
pub type Framebuffer = u64;
pub type CommandPool = u64;
pub type SurfaceKHR = u64;
pub type SwapchainKHR = u64;
pub type DisplayKHR = u64;
pub type DisplayModeKHR = u64;
pub type DescriptorUpdateTemplateKHR = u64;
pub type DeviceAddress = u64;

pub const LOD_CLAMP_NONE: f32 = 1000.0;
pub const REMAINING_MIP_LEVELS: u32 = 0xffffffff;
pub const REMAINING_ARRAY_LAYERS: u32 = 0xffffffff;
pub const WHOLE_SIZE: u64 = 0xffffffffffffffff;
pub const ATTACHMENT_UNUSED: u32 = 0xffffffff;
pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const QUEUE_FAMILY_IGNORED: u32 = 0xffffffff;
pub const SUBPASS_EXTERNAL: u32 = 0xffffffff;
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
pub const UUID_SIZE: u32 = 16;
pub const MAX_MEMORY_TYPES: u32 = 32;
pub const MAX_MEMORY_HEAPS: u32 = 16;
pub const MAX_EXTENSION_NAME_SIZE: u32 = 256;
pub const MAX_DESCRIPTION_SIZE: u32 = 256;
pub const NULL_HANDLE: u64 = 0;

pub type PipelineCacheHeaderVersion = u32;
pub const PIPELINE_CACHE_HEADER_VERSION_ONE: u32 = 1;

pub type Result = u32;
pub const SUCCESS: u32 = 0;
pub const NOT_READY: u32 = 1;
pub const TIMEOUT: u32 = 2;
pub const EVENT_SET: u32 = 3;
pub const EVENT_RESET: u32 = 4;
pub const INCOMPLETE: u32 = 5;
pub const ERROR_OUT_OF_HOST_MEMORY: u32 = -1i32 as u32;
pub const ERROR_OUT_OF_DEVICE_MEMORY: u32 = -2i32 as u32;
pub const ERROR_INITIALIZATION_FAILED: u32 = -3i32 as u32;
pub const ERROR_DEVICE_LOST: u32 = -4i32 as u32;
pub const ERROR_MEMORY_MAP_FAILED: u32 = -5i32 as u32;
pub const ERROR_LAYER_NOT_PRESENT: u32 = -6i32 as u32;
pub const ERROR_EXTENSION_NOT_PRESENT: u32 = -7i32 as u32;
pub const ERROR_FEATURE_NOT_PRESENT: u32 = -8i32 as u32;
pub const ERROR_INCOMPATIBLE_DRIVER: u32 = -9i32 as u32;
pub const ERROR_TOO_MANY_OBJECTS: u32 = -10i32 as u32;
pub const ERROR_FORMAT_NOT_SUPPORTED: u32 = -11i32 as u32;
pub const ERROR_SURFACE_LOST_KHR: u32 = -1000000000i32 as u32;
pub const ERROR_NATIVE_WINDOW_IN_USE_KHR: u32 = -1000000001i32 as u32;
pub const SUBOPTIMAL_KHR: u32 = 1000001003;
pub const ERROR_OUT_OF_DATE_KHR: u32 = -1000001004i32 as u32;
pub const ERROR_INCOMPATIBLE_DISPLAY_KHR: u32 = -1000003001i32 as u32;
pub const ERROR_VALIDATION_FAILED_EXT: u32 = -1000011001i32 as u32;
pub const ERROR_INVALID_SHADER_NV: u32 = -1000012000i32 as u32;
pub const ERROR_OUT_OF_POOL_MEMORY_KHR: u32 = -1000069000i32 as u32;
pub const ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: u32 = -1000255000i32 as u32;

pub type SystemAllocationScope = u32;
pub const SYSTEM_ALLOCATION_SCOPE_COMMAND: u32 = 0;
pub const SYSTEM_ALLOCATION_SCOPE_OBJECT: u32 = 1;
pub const SYSTEM_ALLOCATION_SCOPE_CACHE: u32 = 2;
pub const SYSTEM_ALLOCATION_SCOPE_DEVICE: u32 = 3;
pub const SYSTEM_ALLOCATION_SCOPE_INSTANCE: u32 = 4;

pub type InternalAllocationType = u32;
pub const INTERNAL_ALLOCATION_TYPE_EXECUTABLE: u32 = 0;

pub type ImageType = u32;
pub const IMAGE_TYPE_1D: u32 = 0;
pub const IMAGE_TYPE_2D: u32 = 1;
pub const IMAGE_TYPE_3D: u32 = 2;

pub type ImageTiling = u32;
pub const IMAGE_TILING_OPTIMAL: u32 = 0;
pub const IMAGE_TILING_LINEAR: u32 = 1;

pub type PhysicalDeviceType = u32;
pub const PHYSICAL_DEVICE_TYPE_OTHER: u32 = 0;
pub const PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: u32 = 1;
pub const PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: u32 = 2;
pub const PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: u32 = 3;
pub const PHYSICAL_DEVICE_TYPE_CPU: u32 = 4;

pub type QueryType = u32;
pub const QUERY_TYPE_OCCLUSION: u32 = 0;
pub const QUERY_TYPE_PIPELINE_STATISTICS: u32 = 1;
pub const QUERY_TYPE_TIMESTAMP: u32 = 2;

pub type SharingMode = u32;
pub const SHARING_MODE_EXCLUSIVE: u32 = 0;
pub const SHARING_MODE_CONCURRENT: u32 = 1;

pub type ImageLayout = u32;
pub const IMAGE_LAYOUT_UNDEFINED: u32 = 0;
pub const IMAGE_LAYOUT_GENERAL: u32 = 1;
pub const IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: u32 = 2;
pub const IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: u32 = 3;
pub const IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: u32 = 4;
pub const IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: u32 = 5;
pub const IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: u32 = 6;
pub const IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: u32 = 7;
pub const IMAGE_LAYOUT_PREINITIALIZED: u32 = 8;
pub const IMAGE_LAYOUT_PRESENT_SRC_KHR: u32 = 1000001002;

pub type ImageViewType = u32;
pub const IMAGE_VIEW_TYPE_1D: u32 = 0;
pub const IMAGE_VIEW_TYPE_2D: u32 = 1;
pub const IMAGE_VIEW_TYPE_3D: u32 = 2;
pub const IMAGE_VIEW_TYPE_CUBE: u32 = 3;
pub const IMAGE_VIEW_TYPE_1D_ARRAY: u32 = 4;
pub const IMAGE_VIEW_TYPE_2D_ARRAY: u32 = 5;
pub const IMAGE_VIEW_TYPE_CUBE_ARRAY: u32 = 6;

pub type ComponentSwizzle = u32;
pub const COMPONENT_SWIZZLE_IDENTITY: u32 = 0;
pub const COMPONENT_SWIZZLE_ZERO: u32 = 1;
pub const COMPONENT_SWIZZLE_ONE: u32 = 2;
pub const COMPONENT_SWIZZLE_R: u32 = 3;
pub const COMPONENT_SWIZZLE_G: u32 = 4;
pub const COMPONENT_SWIZZLE_B: u32 = 5;
pub const COMPONENT_SWIZZLE_A: u32 = 6;

pub type VertexInputRate = u32;
pub const VERTEX_INPUT_RATE_VERTEX: u32 = 0;
pub const VERTEX_INPUT_RATE_INSTANCE: u32 = 1;

pub type PrimitiveTopology = u32;
pub const PRIMITIVE_TOPOLOGY_POINT_LIST: u32 = 0;
pub const PRIMITIVE_TOPOLOGY_LINE_LIST: u32 = 1;
pub const PRIMITIVE_TOPOLOGY_LINE_STRIP: u32 = 2;
pub const PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: u32 = 3;
pub const PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: u32 = 4;
pub const PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: u32 = 5;
pub const PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: u32 = 6;
pub const PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: u32 = 7;
pub const PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: u32 = 8;
pub const PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: u32 = 9;
pub const PRIMITIVE_TOPOLOGY_PATCH_LIST: u32 = 10;

pub type PolygonMode = u32;
pub const POLYGON_MODE_FILL: u32 = 0;
pub const POLYGON_MODE_LINE: u32 = 1;
pub const POLYGON_MODE_POINT: u32 = 2;

pub type FrontFace = u32;
pub const FRONT_FACE_COUNTER_CLOCKWISE: u32 = 0;
pub const FRONT_FACE_CLOCKWISE: u32 = 1;

pub type CompareOp = u32;
pub const COMPARE_OP_NEVER: u32 = 0;
pub const COMPARE_OP_LESS: u32 = 1;
pub const COMPARE_OP_EQUAL: u32 = 2;
pub const COMPARE_OP_LESS_OR_EQUAL: u32 = 3;
pub const COMPARE_OP_GREATER: u32 = 4;
pub const COMPARE_OP_NOT_EQUAL: u32 = 5;
pub const COMPARE_OP_GREATER_OR_EQUAL: u32 = 6;
pub const COMPARE_OP_ALWAYS: u32 = 7;

pub type StencilOp = u32;
pub const STENCIL_OP_KEEP: u32 = 0;
pub const STENCIL_OP_ZERO: u32 = 1;
pub const STENCIL_OP_REPLACE: u32 = 2;
pub const STENCIL_OP_INCREMENT_AND_CLAMP: u32 = 3;
pub const STENCIL_OP_DECREMENT_AND_CLAMP: u32 = 4;
pub const STENCIL_OP_INVERT: u32 = 5;
pub const STENCIL_OP_INCREMENT_AND_WRAP: u32 = 6;
pub const STENCIL_OP_DECREMENT_AND_WRAP: u32 = 7;

pub type LogicOp = u32;
pub const LOGIC_OP_CLEAR: u32 = 0;
pub const LOGIC_OP_AND: u32 = 1;
pub const LOGIC_OP_AND_REVERSE: u32 = 2;
pub const LOGIC_OP_COPY: u32 = 3;
pub const LOGIC_OP_AND_INVERTED: u32 = 4;
pub const LOGIC_OP_NO_OP: u32 = 5;
pub const LOGIC_OP_XOR: u32 = 6;
pub const LOGIC_OP_OR: u32 = 7;
pub const LOGIC_OP_NOR: u32 = 8;
pub const LOGIC_OP_EQUIVALENT: u32 = 9;
pub const LOGIC_OP_INVERT: u32 = 10;
pub const LOGIC_OP_OR_REVERSE: u32 = 11;
pub const LOGIC_OP_COPY_INVERTED: u32 = 12;
pub const LOGIC_OP_OR_INVERTED: u32 = 13;
pub const LOGIC_OP_NAND: u32 = 14;
pub const LOGIC_OP_SET: u32 = 15;

pub type BlendFactor = u32;
pub const BLEND_FACTOR_ZERO: u32 = 0;
pub const BLEND_FACTOR_ONE: u32 = 1;
pub const BLEND_FACTOR_SRC_COLOR: u32 = 2;
pub const BLEND_FACTOR_ONE_MINUS_SRC_COLOR: u32 = 3;
pub const BLEND_FACTOR_DST_COLOR: u32 = 4;
pub const BLEND_FACTOR_ONE_MINUS_DST_COLOR: u32 = 5;
pub const BLEND_FACTOR_SRC_ALPHA: u32 = 6;
pub const BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: u32 = 7;
pub const BLEND_FACTOR_DST_ALPHA: u32 = 8;
pub const BLEND_FACTOR_ONE_MINUS_DST_ALPHA: u32 = 9;
pub const BLEND_FACTOR_CONSTANT_COLOR: u32 = 10;
pub const BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: u32 = 11;
pub const BLEND_FACTOR_CONSTANT_ALPHA: u32 = 12;
pub const BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: u32 = 13;
pub const BLEND_FACTOR_SRC_ALPHA_SATURATE: u32 = 14;
pub const BLEND_FACTOR_SRC1_COLOR: u32 = 15;
pub const BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: u32 = 16;
pub const BLEND_FACTOR_SRC1_ALPHA: u32 = 17;
pub const BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: u32 = 18;

pub type BlendOp = u32;
pub const BLEND_OP_ADD: u32 = 0;
pub const BLEND_OP_SUBTRACT: u32 = 1;
pub const BLEND_OP_REVERSE_SUBTRACT: u32 = 2;
pub const BLEND_OP_MIN: u32 = 3;
pub const BLEND_OP_MAX: u32 = 4;

pub type DynamicState = u32;
pub const DYNAMIC_STATE_VIEWPORT: u32 = 0;
pub const DYNAMIC_STATE_SCISSOR: u32 = 1;
pub const DYNAMIC_STATE_LINE_WIDTH: u32 = 2;
pub const DYNAMIC_STATE_DEPTH_BIAS: u32 = 3;
pub const DYNAMIC_STATE_BLEND_CONSTANTS: u32 = 4;
pub const DYNAMIC_STATE_DEPTH_BOUNDS: u32 = 5;
pub const DYNAMIC_STATE_STENCIL_COMPARE_MASK: u32 = 6;
pub const DYNAMIC_STATE_STENCIL_WRITE_MASK: u32 = 7;
pub const DYNAMIC_STATE_STENCIL_REFERENCE: u32 = 8;

pub type Filter = u32;
pub const FILTER_NEAREST: u32 = 0;
pub const FILTER_LINEAR: u32 = 1;

pub type SamplerMipmapMode = u32;
pub const SAMPLER_MIPMAP_MODE_NEAREST: u32 = 0;
pub const SAMPLER_MIPMAP_MODE_LINEAR: u32 = 1;

pub type SamplerAddressMode = u32;
pub const SAMPLER_ADDRESS_MODE_REPEAT: u32 = 0;
pub const SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT: u32 = 1;
pub const SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE: u32 = 2;
pub const SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: u32 = 3;
pub const SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE: u32 = 4;

pub type BorderColor = u32;
pub const BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: u32 = 0;
pub const BORDER_COLOR_INT_TRANSPARENT_BLACK: u32 = 1;
pub const BORDER_COLOR_FLOAT_OPAQUE_BLACK: u32 = 2;
pub const BORDER_COLOR_INT_OPAQUE_BLACK: u32 = 3;
pub const BORDER_COLOR_FLOAT_OPAQUE_WHITE: u32 = 4;
pub const BORDER_COLOR_INT_OPAQUE_WHITE: u32 = 5;

pub type DescriptorType = u32;
pub const DESCRIPTOR_TYPE_SAMPLER: u32 = 0;
pub const DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: u32 = 1;
pub const DESCRIPTOR_TYPE_SAMPLED_IMAGE: u32 = 2;
pub const DESCRIPTOR_TYPE_STORAGE_IMAGE: u32 = 3;
pub const DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: u32 = 4;
pub const DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: u32 = 5;
pub const DESCRIPTOR_TYPE_UNIFORM_BUFFER: u32 = 6;
pub const DESCRIPTOR_TYPE_STORAGE_BUFFER: u32 = 7;
pub const DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: u32 = 8;
pub const DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: u32 = 9;
pub const DESCRIPTOR_TYPE_INPUT_ATTACHMENT: u32 = 10;

pub type AttachmentLoadOp = u32;
pub const ATTACHMENT_LOAD_OP_LOAD: u32 = 0;
pub const ATTACHMENT_LOAD_OP_CLEAR: u32 = 1;
pub const ATTACHMENT_LOAD_OP_DONT_CARE: u32 = 2;

pub type AttachmentStoreOp = u32;
pub const ATTACHMENT_STORE_OP_STORE: u32 = 0;
pub const ATTACHMENT_STORE_OP_DONT_CARE: u32 = 1;

pub type PipelineBindPoint = u32;
pub const PIPELINE_BIND_POINT_GRAPHICS: u32 = 0;
pub const PIPELINE_BIND_POINT_COMPUTE: u32 = 1;

pub type CommandBufferLevel = u32;
pub const COMMAND_BUFFER_LEVEL_PRIMARY: u32 = 0;
pub const COMMAND_BUFFER_LEVEL_SECONDARY: u32 = 1;

pub type IndexType = u32;
pub const INDEX_TYPE_UINT16: u32 = 0;
pub const INDEX_TYPE_UINT32: u32 = 1;

pub type SubpassContents = u32;
pub const SUBPASS_CONTENTS_INLINE: u32 = 0;
pub const SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS: u32 = 1;

pub type InstanceCreateFlags = Flags;

pub type FormatFeatureFlagBits = u32;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_BIT: u32 = 0x00000001;
pub const FORMAT_FEATURE_STORAGE_IMAGE_BIT: u32 = 0x00000002;
pub const FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT: u32 = 0x00000004;
pub const FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT: u32 = 0x00000008;
pub const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT: u32 = 0x00000010;
pub const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: u32 = 0x00000020;
pub const FORMAT_FEATURE_VERTEX_BUFFER_BIT: u32 = 0x00000040;
pub const FORMAT_FEATURE_COLOR_ATTACHMENT_BIT: u32 = 0x00000080;
pub const FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT: u32 = 0x00000100;
pub const FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: u32 = 0x00000200;
pub const FORMAT_FEATURE_BLIT_SRC_BIT: u32 = 0x00000400;
pub const FORMAT_FEATURE_BLIT_DST_BIT: u32 = 0x00000800;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT: u32 = 0x00001000;
pub const FORMAT_FEATURE_TRANSFER_SRC_BIT: u32 = 0x00004000;
pub const FORMAT_FEATURE_TRANSFER_DST_BIT: u32 = 0x00008000;
pub const FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT: u32 = 0x00020000;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: u32 = 0x00040000;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: u32 =
    0x00080000;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: u32 =
    0x00100000;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: u32 = 0x00200000;
pub const FORMAT_FEATURE_DISJOINT_BIT: u32 = 0x00400000;
pub const FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT: u32 = 0x00800000;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT: u32 = 0x00010000;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG: u32 = 0x00002000;
pub const FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR: u32 = 0x20000000;
pub const FORMAT_FEATURE_FRAGMENT_DENSITY_MAP_BIT_EXT: u32 = 0x01000000;

pub const FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR: u32 = FORMAT_FEATURE_TRANSFER_SRC_BIT;
pub const FORMAT_FEATURE_TRANSFER_DST_BIT_KHR: u32 = FORMAT_FEATURE_TRANSFER_DST_BIT;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT: u32 =
    FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT;
pub const FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR: u32 =
    FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: u32 =
    FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR:
    u32 = FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR:
    u32 = FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: u32 = FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT;
pub const FORMAT_FEATURE_DISJOINT_BIT_KHR: u32 = FORMAT_FEATURE_DISJOINT_BIT;
pub const FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR: u32 =
    FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT;
pub const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: u32 =
    FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG;
pub type FormatFeatureFlags = Flags;

pub type ImageUsageFlagBits = u32;
pub const IMAGE_USAGE_TRANSFER_SRC_BIT: u32 = 0x00000001;
pub const IMAGE_USAGE_TRANSFER_DST_BIT: u32 = 0x00000002;
pub const IMAGE_USAGE_SAMPLED_BIT: u32 = 0x00000004;
pub const IMAGE_USAGE_STORAGE_BIT: u32 = 0x00000008;
pub const IMAGE_USAGE_COLOR_ATTACHMENT_BIT: u32 = 0x00000010;
pub const IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: u32 = 0x00000020;
pub const IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: u32 = 0x00000040;
pub const IMAGE_USAGE_INPUT_ATTACHMENT_BIT: u32 = 0x00000080;
pub type ImageUsageFlags = Flags;

pub type ImageCreateFlagBits = u32;
pub const IMAGE_CREATE_SPARSE_BINDING_BIT: u32 = 0x00000001;
pub const IMAGE_CREATE_SPARSE_RESIDENCY_BIT: u32 = 0x00000002;
pub const IMAGE_CREATE_SPARSE_ALIASED_BIT: u32 = 0x00000004;
pub const IMAGE_CREATE_MUTABLE_FORMAT_BIT: u32 = 0x00000008;
pub const IMAGE_CREATE_CUBE_COMPATIBLE_BIT: u32 = 0x00000010;
pub const IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR: u32 = 0x00000020;
pub type ImageCreateFlags = Flags;

pub type SampleCountFlagBits = u32;
pub const SAMPLE_COUNT_1_BIT: u32 = 0x00000001;
pub const SAMPLE_COUNT_2_BIT: u32 = 0x00000002;
pub const SAMPLE_COUNT_4_BIT: u32 = 0x00000004;
pub const SAMPLE_COUNT_8_BIT: u32 = 0x00000008;
pub const SAMPLE_COUNT_16_BIT: u32 = 0x00000010;
pub const SAMPLE_COUNT_32_BIT: u32 = 0x00000020;
pub const SAMPLE_COUNT_64_BIT: u32 = 0x00000040;
pub type SampleCountFlags = Flags;

pub type QueueFlagBits = u32;
pub const QUEUE_GRAPHICS_BIT: u32 = 0x00000001;
pub const QUEUE_COMPUTE_BIT: u32 = 0x00000002;
pub const QUEUE_TRANSFER_BIT: u32 = 0x00000004;
pub const QUEUE_SPARSE_BINDING_BIT: u32 = 0x00000008;
pub type QueueFlags = Flags;

pub type MemoryPropertyFlagBits = u32;
pub const MEMORY_PROPERTY_DEVICE_LOCAL_BIT: u32 = 0x00000001;
pub const MEMORY_PROPERTY_HOST_VISIBLE_BIT: u32 = 0x00000002;
pub const MEMORY_PROPERTY_HOST_COHERENT_BIT: u32 = 0x00000004;
pub const MEMORY_PROPERTY_HOST_CACHED_BIT: u32 = 0x00000008;
pub const MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: u32 = 0x00000010;
pub type MemoryPropertyFlags = Flags;

pub type MemoryHeapFlagBits = u32;
pub const MEMORY_HEAP_DEVICE_LOCAL_BIT: u32 = 0x00000001;
pub type MemoryHeapFlags = Flags;
pub type DeviceCreateFlags = Flags;
pub type DeviceQueueCreateFlags = Flags;

pub type PipelineStageFlagBits = u32;
pub const PIPELINE_STAGE_TOP_OF_PIPE_BIT: u32 = 0x00000001;
pub const PIPELINE_STAGE_DRAW_INDIRECT_BIT: u32 = 0x00000002;
pub const PIPELINE_STAGE_VERTEX_INPUT_BIT: u32 = 0x00000004;
pub const PIPELINE_STAGE_VERTEX_SHADER_BIT: u32 = 0x00000008;
pub const PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: u32 = 0x00000010;
pub const PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: u32 = 0x00000020;
pub const PIPELINE_STAGE_GEOMETRY_SHADER_BIT: u32 = 0x00000040;
pub const PIPELINE_STAGE_FRAGMENT_SHADER_BIT: u32 = 0x00000080;
pub const PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: u32 = 0x00000100;
pub const PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: u32 = 0x00000200;
pub const PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: u32 = 0x00000400;
pub const PIPELINE_STAGE_COMPUTE_SHADER_BIT: u32 = 0x00000800;
pub const PIPELINE_STAGE_TRANSFER_BIT: u32 = 0x00001000;
pub const PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: u32 = 0x00002000;
pub const PIPELINE_STAGE_HOST_BIT: u32 = 0x00004000;
pub const PIPELINE_STAGE_ALL_GRAPHICS_BIT: u32 = 0x00008000;
pub const PIPELINE_STAGE_ALL_COMMANDS_BIT: u32 = 0x00010000;
pub type PipelineStageFlags = Flags;
pub type MemoryMapFlags = Flags;

pub type ImageAspectFlagBits = u32;
pub const IMAGE_ASPECT_COLOR_BIT: u32 = 0x00000001;
pub const IMAGE_ASPECT_DEPTH_BIT: u32 = 0x00000002;
pub const IMAGE_ASPECT_STENCIL_BIT: u32 = 0x00000004;
pub const IMAGE_ASPECT_METADATA_BIT: u32 = 0x00000008;
pub type ImageAspectFlags = Flags;

pub type SparseImageFormatFlagBits = u32;
pub const SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT: u32 = 0x00000001;
pub const SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT: u32 = 0x00000002;
pub const SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT: u32 = 0x00000004;
pub type SparseImageFormatFlags = Flags;

pub type SparseMemoryBindFlagBits = u32;
pub const SPARSE_MEMORY_BIND_METADATA_BIT: u32 = 0x00000001;
pub type SparseMemoryBindFlags = Flags;

pub type FenceCreateFlagBits = u32;
pub const FENCE_CREATE_SIGNALED_BIT: u32 = 0x00000001;
pub type FenceCreateFlags = Flags;
pub type SemaphoreCreateFlags = Flags;
pub type EventCreateFlags = Flags;
pub type QueryPoolCreateFlags = Flags;

pub type QueryPipelineStatisticFlagBits = u32;
pub const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT: u32 = 0x00000001;
pub const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT: u32 = 0x00000002;
pub const QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT: u32 = 0x00000004;
pub const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT: u32 = 0x00000008;
pub const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT: u32 = 0x00000010;
pub const QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT: u32 = 0x00000020;
pub const QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT: u32 = 0x00000040;
pub const QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT: u32 = 0x00000080;
pub const QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT: u32 = 0x00000100;
pub const QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: u32 = 0x00000200;
pub const QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT: u32 = 0x00000400;
pub type QueryPipelineStatisticFlags = Flags;

pub type QueryResultFlagBits = u32;
pub const QUERY_RESULT_64_BIT: u32 = 0x00000001;
pub const QUERY_RESULT_WAIT_BIT: u32 = 0x00000002;
pub const QUERY_RESULT_WITH_AVAILABILITY_BIT: u32 = 0x00000004;
pub const QUERY_RESULT_PARTIAL_BIT: u32 = 0x00000008;
pub type QueryResultFlags = Flags;

pub type BufferCreateFlagBits = u32;
pub const BUFFER_CREATE_SPARSE_BINDING_BIT: u32 = 0x00000001;
pub const BUFFER_CREATE_SPARSE_RESIDENCY_BIT: u32 = 0x00000002;
pub const BUFFER_CREATE_SPARSE_ALIASED_BIT: u32 = 0x00000004;
pub type BufferCreateFlags = Flags;

pub type BufferUsageFlagBits = u32;
pub const BUFFER_USAGE_TRANSFER_SRC_BIT: u32 = 0x00000001;
pub const BUFFER_USAGE_TRANSFER_DST_BIT: u32 = 0x00000002;
pub const BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: u32 = 0x00000004;
pub const BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: u32 = 0x00000008;
pub const BUFFER_USAGE_UNIFORM_BUFFER_BIT: u32 = 0x00000010;
pub const BUFFER_USAGE_STORAGE_BUFFER_BIT: u32 = 0x00000020;
pub const BUFFER_USAGE_INDEX_BUFFER_BIT: u32 = 0x00000040;
pub const BUFFER_USAGE_VERTEX_BUFFER_BIT: u32 = 0x00000080;
pub const BUFFER_USAGE_INDIRECT_BUFFER_BIT: u32 = 0x00000100;
pub const BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT: u32 = 0x00020000;
pub type BufferUsageFlags = Flags;
pub type BufferViewCreateFlags = Flags;
pub type ImageViewCreateFlags = Flags;
pub type ShaderModuleCreateFlags = Flags;
pub type PipelineCacheCreateFlags = Flags;

pub type PipelineCreateFlagBits = u32;
pub const PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: u32 = 0x00000001;
pub const PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: u32 = 0x00000002;
pub const PIPELINE_CREATE_DERIVATIVE_BIT: u32 = 0x00000004;
pub type PipelineCreateFlags = Flags;
pub type PipelineShaderStageCreateFlags = Flags;

pub type ShaderStageFlagBits = u32;
pub const SHADER_STAGE_VERTEX_BIT: u32 = 0x00000001;
pub const SHADER_STAGE_TESSELLATION_CONTROL_BIT: u32 = 0x00000002;
pub const SHADER_STAGE_TESSELLATION_EVALUATION_BIT: u32 = 0x00000004;
pub const SHADER_STAGE_GEOMETRY_BIT: u32 = 0x00000008;
pub const SHADER_STAGE_FRAGMENT_BIT: u32 = 0x00000010;
pub const SHADER_STAGE_COMPUTE_BIT: u32 = 0x00000020;
pub const SHADER_STAGE_ALL_GRAPHICS: u32 = 0x1F;
pub const SHADER_STAGE_ALL: u32 = 0x7FFFFFFF;
pub type PipelineVertexInputStateCreateFlags = Flags;
pub type PipelineInputAssemblyStateCreateFlags = Flags;
pub type PipelineTessellationStateCreateFlags = Flags;
pub type PipelineViewportStateCreateFlags = Flags;
pub type PipelineRasterizationStateCreateFlags = Flags;

pub type CullModeFlagBits = u32;
pub const CULL_MODE_NONE: u32 = 0;
pub const CULL_MODE_FRONT_BIT: u32 = 0x00000001;
pub const CULL_MODE_BACK_BIT: u32 = 0x00000002;
pub const CULL_MODE_FRONT_AND_BACK: u32 = 0x3;
pub type CullModeFlags = Flags;
pub type PipelineMultisampleStateCreateFlags = Flags;
pub type PipelineDepthStencilStateCreateFlags = Flags;
pub type PipelineColorBlendStateCreateFlags = Flags;

pub type ColorComponentFlagBits = u32;
pub const COLOR_COMPONENT_R_BIT: u32 = 0x00000001;
pub const COLOR_COMPONENT_G_BIT: u32 = 0x00000002;
pub const COLOR_COMPONENT_B_BIT: u32 = 0x00000004;
pub const COLOR_COMPONENT_A_BIT: u32 = 0x00000008;
pub type ColorComponentFlags = Flags;
pub type PipelineDynamicStateCreateFlags = Flags;
pub type PipelineLayoutCreateFlags = Flags;
pub type ShaderStageFlags = Flags;
pub type SubgroupFeatureFlags = Flags;
pub type SamplerCreateFlags = Flags;
pub type DescriptorSetLayoutCreateFlags = Flags;

pub type DescriptorPoolCreateFlagBits = u32;
pub const DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: u32 = 0x00000001;
pub type DescriptorPoolCreateFlags = Flags;
pub type DescriptorPoolResetFlags = Flags;
pub type FramebufferCreateFlags = Flags;
pub type RenderPassCreateFlags = Flags;

pub type AttachmentDescriptionFlagBits = u32;
pub const ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: u32 = 0x00000001;
pub type AttachmentDescriptionFlags = Flags;
pub type SubpassDescriptionFlags = Flags;

pub type AccessFlagBits = u32;
pub const ACCESS_INDIRECT_COMMAND_READ_BIT: u32 = 0x00000001;
pub const ACCESS_INDEX_READ_BIT: u32 = 0x00000002;
pub const ACCESS_VERTEX_ATTRIBUTE_READ_BIT: u32 = 0x00000004;
pub const ACCESS_UNIFORM_READ_BIT: u32 = 0x00000008;
pub const ACCESS_INPUT_ATTACHMENT_READ_BIT: u32 = 0x00000010;
pub const ACCESS_SHADER_READ_BIT: u32 = 0x00000020;
pub const ACCESS_SHADER_WRITE_BIT: u32 = 0x00000040;
pub const ACCESS_COLOR_ATTACHMENT_READ_BIT: u32 = 0x00000080;
pub const ACCESS_COLOR_ATTACHMENT_WRITE_BIT: u32 = 0x00000100;
pub const ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: u32 = 0x00000200;
pub const ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: u32 = 0x00000400;
pub const ACCESS_TRANSFER_READ_BIT: u32 = 0x00000800;
pub const ACCESS_TRANSFER_WRITE_BIT: u32 = 0x00001000;
pub const ACCESS_HOST_READ_BIT: u32 = 0x00002000;
pub const ACCESS_HOST_WRITE_BIT: u32 = 0x00004000;
pub const ACCESS_MEMORY_READ_BIT: u32 = 0x00008000;
pub const ACCESS_MEMORY_WRITE_BIT: u32 = 0x00010000;
pub type AccessFlags = Flags;

pub type DependencyFlagBits = u32;
pub const DEPENDENCY_BY_REGION_BIT: u32 = 0x00000001;
pub type DependencyFlags = Flags;

pub type CommandPoolCreateFlagBits = u32;
pub const COMMAND_POOL_CREATE_TRANSIENT_BIT: u32 = 0x00000001;
pub const COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: u32 = 0x00000002;
pub type CommandPoolCreateFlags = Flags;

pub type CommandPoolResetFlagBits = u32;
pub const COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT: u32 = 0x00000001;
pub type CommandPoolResetFlags = Flags;

pub type CommandPoolTrimFlagsKHR = Flags;

pub type CommandBufferUsageFlagBits = u32;
pub const COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: u32 = 0x00000001;
pub const COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: u32 = 0x00000002;
pub const COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: u32 = 0x00000004;
pub type CommandBufferUsageFlags = Flags;

pub type QueryControlFlagBits = u32;
pub const QUERY_CONTROL_PRECISE_BIT: u32 = 0x00000001;
pub type QueryControlFlags = Flags;

pub type CommandBufferResetFlagBits = u32;
pub const COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT: u32 = 0x00000001;
pub type CommandBufferResetFlags = Flags;

pub type StencilFaceFlagBits = u32;
pub const STENCIL_FACE_FRONT_BIT: u32 = 0x00000001;
pub const STENCIL_FACE_BACK_BIT: u32 = 0x00000002;
pub const STENCIL_FRONT_AND_BACK: u32 = 0x3;
pub type StencilFaceFlags = Flags;

pub type DisplayPlaneAlphaFlagBitsKHR = u32;
pub const DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR: u32 = 0x00000001;
pub const DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR: u32 = 0x00000002;
pub const DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR: u32 = 0x00000004;
pub const DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR: u32 = 0x00000008;
pub type DisplayModeCreateFlagsKHR = Flags;
pub type DisplayPlaneAlphaFlagsKHR = Flags;
pub type DisplaySurfaceCreateFlagsKHR = Flags;

pub type ColorSpaceKHR = u32;
pub const COLOR_SPACE_SRGB_NONLINEAR_KHR: u32 = 0;
pub const COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT: u32 = 1000104001;
pub const COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT: u32 = 1000104002;
pub const COLOR_SPACE_DCI_P3_LINEAR_EXT: u32 = 1000104003;
pub const COLOR_SPACE_DCI_P3_NONLINEAR_EXT: u32 = 1000104004;
pub const COLOR_SPACE_BT709_LINEAR_EXT: u32 = 1000104005;
pub const COLOR_SPACE_BT709_NONLINEAR_EXT: u32 = 1000104006;
pub const COLOR_SPACE_BT2020_LINEAR_EXT: u32 = 1000104007;
pub const COLOR_SPACE_HDR10_ST2084_EXT: u32 = 1000104008;
pub const COLOR_SPACE_DOLBYVISION_EXT: u32 = 1000104009;
pub const COLOR_SPACE_HDR10_HLG_EXT: u32 = 1000104010;
pub const COLOR_SPACE_ADOBERGB_LINEAR_EXT: u32 = 1000104011;
pub const COLOR_SPACE_ADOBERGB_NONLINEAR_EXT: u32 = 1000104012;
pub const COLOR_SPACE_PASS_THROUGH_EXT: u32 = 1000104013;

pub type PresentModeKHR = u32;
pub const PRESENT_MODE_IMMEDIATE_KHR: u32 = 0;
pub const PRESENT_MODE_MAILBOX_KHR: u32 = 1;
pub const PRESENT_MODE_FIFO_KHR: u32 = 2;
pub const PRESENT_MODE_FIFO_RELAXED_KHR: u32 = 3;
pub const PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR: u32 = 1000111000;
pub const PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR: u32 = 1000111001;

pub type SurfaceTransformFlagBitsKHR = u32;
pub const SURFACE_TRANSFORM_IDENTITY_BIT_KHR: u32 = 0x00000001;
pub const SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: u32 = 0x00000002;
pub const SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: u32 = 0x00000004;
pub const SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: u32 = 0x00000008;
pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: u32 = 0x00000010;
pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: u32 = 0x00000020;
pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: u32 = 0x00000040;
pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: u32 = 0x00000080;
pub const SURFACE_TRANSFORM_INHERIT_BIT_KHR: u32 = 0x00000100;
pub type SurfaceTransformFlagsKHR = Flags;

pub type CompositeAlphaFlagBitsKHR = u32;
pub const COMPOSITE_ALPHA_OPAQUE_BIT_KHR: u32 = 0x00000001;
pub const COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: u32 = 0x00000002;
pub const COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: u32 = 0x00000004;
pub const COMPOSITE_ALPHA_INHERIT_BIT_KHR: u32 = 0x00000008;
pub type CompositeAlphaFlagsKHR = Flags;

pub type ObjectType = u32;
pub const OBJECT_TYPE_UNKNOWN: u32 = 0;
pub const OBJECT_TYPE_INSTANCE: u32 = 1;
pub const OBJECT_TYPE_PHYSICAL_DEVICE: u32 = 2;
pub const OBJECT_TYPE_DEVICE: u32 = 3;
pub const OBJECT_TYPE_QUEUE: u32 = 4;
pub const OBJECT_TYPE_SEMAPHORE: u32 = 5;
pub const OBJECT_TYPE_COMMAND_BUFFER: u32 = 6;
pub const OBJECT_TYPE_FENCE: u32 = 7;
pub const OBJECT_TYPE_DEVICE_MEMORY: u32 = 8;
pub const OBJECT_TYPE_BUFFER: u32 = 9;
pub const OBJECT_TYPE_IMAGE: u32 = 10;
pub const OBJECT_TYPE_EVENT: u32 = 11;
pub const OBJECT_TYPE_QUERY_POOL: u32 = 12;
pub const OBJECT_TYPE_BUFFER_VIEW: u32 = 13;
pub const OBJECT_TYPE_IMAGE_VIEW: u32 = 14;
pub const OBJECT_TYPE_SHADER_MODULE: u32 = 15;
pub const OBJECT_TYPE_PIPELINE_CACHE: u32 = 16;
pub const OBJECT_TYPE_PIPELINE_LAYOUT: u32 = 17;
pub const OBJECT_TYPE_RENDER_PASS: u32 = 18;
pub const OBJECT_TYPE_PIPELINE: u32 = 19;
pub const OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: u32 = 20;
pub const OBJECT_TYPE_SAMPLER: u32 = 21;
pub const OBJECT_TYPE_DESCRIPTOR_POOL: u32 = 22;
pub const OBJECT_TYPE_DESCRIPTOR_SET: u32 = 23;
pub const OBJECT_TYPE_FRAMEBUFFER: u32 = 24;
pub const OBJECT_TYPE_COMMAND_POOL: u32 = 25;
pub const OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION: u32 = 1000156000;
pub const OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE: u32 = 1000085000;
pub const OBJECT_TYPE_SURFACE_KHR: u32 = 1000000000;
pub const OBJECT_TYPE_SWAPCHAIN_KHR: u32 = 1000001000;
pub const OBJECT_TYPE_DISPLAY_KHR: u32 = 1000002000;
pub const OBJECT_TYPE_DISPLAY_MODE_KHR: u32 = 1000002001;
pub const OBJECT_TYPE_OBJECT_TABLE_NVX: u32 = 1000086000;
pub const OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX: u32 = 1000086001;
pub const OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT: u32 = 1000128000;
pub const OBJECT_TYPE_VALIDATION_CACHE_EXT: u32 = 1000160000;
pub const OBJECT_TYPE_ACCELERATION_STRUCTURE_NV: u32 = 1000165000;
pub const OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL: u32 = 1000210000;
pub const OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR: u32 = OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE;
pub const OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR: u32 = OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION;

pub type DebugUtilsMessageSeverityFlagBitsEXT = u32;
pub const DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT: u32 = 0x00000001;
pub const DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT: u32 = 0x00000010;
pub const DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT: u32 = 0x00000100;
pub const DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT: u32 = 0x00001000;
pub type DebugUtilsMessageSeverityFlagsEXT = Flags;

pub type DebugUtilsMessageTypeFlagBitsEXT = u32;
pub const DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT: u32 = 0x00000001;
pub const DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT: u32 = 0x00000002;
pub const DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT: u32 = 0x00000004;
pub type DebugUtilsMessageTypeFlagsEXT = Flags;

pub type DebugUtilsMessengerCreateFlagsEXT = Flags;

pub type DebugUtilsMessengerCallbackDataFlagsEXT = Flags;

pub type DebugUtilsMessengerEXT = u64;

pub type MacOSSurfaceCreateFlagsMVK = u32;

pub type IOSSurfaceCreateFlagsMVK = u32;

pub type MetalSurfaceCreateFlagsEXT = u32;

pub type DescriptorSetLayoutCreateFlagBits = u32;
pub const DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR: u32 = 0x00000001;

pub type DescriptorUpdateTemplateTypeKHR = u32;
pub const DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR: u32 = 0;
pub const DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR: u32 = 1;
pub const DESCRIPTOR_UPDATE_TEMPLATE_TYPE_BEGIN_RANGE_KHR: u32 =
    DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR;
pub const DESCRIPTOR_UPDATE_TEMPLATE_TYPE_END_RANGE_KHR: u32 =
    DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR;
pub const DESCRIPTOR_UPDATE_TEMPLATE_TYPE_RANGE_SIZE_KHR: u32 =
    DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR
        - DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR
        + 1;
pub type DescriptorUpdateTemplateCreateFlagsKHR = Flags;

pub type PFN_vkAllocationFunction =
    extern "system" fn(*mut c_void, usize, usize, SystemAllocationScope) -> *mut c_void;
pub type PFN_vkReallocationFunction = extern "system" fn(
    *mut c_void,
    *mut c_void,
    usize,
    usize,
    SystemAllocationScope,
) -> *mut c_void;
pub type PFN_vkFreeFunction = extern "system" fn(*mut c_void, *mut c_void);
pub type PFN_vkInternalAllocationNotification = extern "system" fn(
    *mut c_void,
    usize,
    InternalAllocationType,
    SystemAllocationScope,
) -> *mut c_void;
pub type PFN_vkInternalFreeNotification = extern "system" fn(
    *mut c_void,
    usize,
    InternalAllocationType,
    SystemAllocationScope,
) -> *mut c_void;
pub type PFN_vkDebugUtilsMessengerCallbackEXT = extern "system" fn(
    DebugUtilsMessageSeverityFlagBitsEXT,
    DebugUtilsMessageTypeFlagsEXT,
    *const DebugUtilsMessengerCallbackDataEXT,
    *mut c_void,
) -> Bool32;

pub type PFN_vkVoidFunction = extern "system" fn() -> ();

pub type FullScreenExclusiveEXT = u32;
pub const FULL_SCREEN_EXCLUSIVE_DEFAUlT_EXT: u32 = 0;
pub const FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: u32 = 1;
pub const FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: u32 = 2;
pub const FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: u32 = 3;
pub const FULL_SCREEN_EXCLUSIVE_MAX_ENUM_EXT: u32 = 0x7FFFFFFF;

#[repr(C)]
#[derive(Debug)]
pub struct ApplicationInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub pApplicationName: *const c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct InstanceCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: InstanceCreateFlags,
    pub pApplicationInfo: *const ApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
}

#[repr(C)]
pub struct AllocationCallbacks {
    pub pUserData: *mut c_void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    pub pfnInternalFree: PFN_vkInternalFreeNotification,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct PhysicalDeviceFeatures {
    pub robustBufferAccess: Bool32,
    pub fullDrawIndexUint32: Bool32,
    pub imageCubeArray: Bool32,
    pub independentBlend: Bool32,
    pub geometryShader: Bool32,
    pub tessellationShader: Bool32,
    pub sampleRateShading: Bool32,
    pub dualSrcBlend: Bool32,
    pub logicOp: Bool32,
    pub multiDrawIndirect: Bool32,
    pub drawIndirectFirstInstance: Bool32,
    pub depthClamp: Bool32,
    pub depthBiasClamp: Bool32,
    pub fillModeNonSolid: Bool32,
    pub depthBounds: Bool32,
    pub wideLines: Bool32,
    pub largePoints: Bool32,
    pub alphaToOne: Bool32,
    pub multiViewport: Bool32,
    pub samplerAnisotropy: Bool32,
    pub textureCompressionETC2: Bool32,
    pub textureCompressionASTC_LDR: Bool32,
    pub textureCompressionBC: Bool32,
    pub occlusionQueryPrecise: Bool32,
    pub pipelineStatisticsQuery: Bool32,
    pub vertexPipelineStoresAndAtomics: Bool32,
    pub fragmentStoresAndAtomics: Bool32,
    pub shaderTessellationAndGeometryPointSize: Bool32,
    pub shaderImageGatherExtended: Bool32,
    pub shaderStorageImageExtendedFormats: Bool32,
    pub shaderStorageImageMultisample: Bool32,
    pub shaderStorageImageReadWithoutFormat: Bool32,
    pub shaderStorageImageWriteWithoutFormat: Bool32,
    pub shaderUniformBufferArrayDynamicIndexing: Bool32,
    pub shaderSampledImageArrayDynamicIndexing: Bool32,
    pub shaderStorageBufferArrayDynamicIndexing: Bool32,
    pub shaderStorageImageArrayDynamicIndexing: Bool32,
    pub shaderClipDistance: Bool32,
    pub shaderCullDistance: Bool32,
    pub shaderFloat64: Bool32,
    pub shaderInt64: Bool32,
    pub shaderInt16: Bool32,
    pub shaderResourceResidency: Bool32,
    pub shaderResourceMinLod: Bool32,
    pub sparseBinding: Bool32,
    pub sparseResidencyBuffer: Bool32,
    pub sparseResidencyImage2D: Bool32,
    pub sparseResidencyImage3D: Bool32,
    pub sparseResidency2Samples: Bool32,
    pub sparseResidency4Samples: Bool32,
    pub sparseResidency8Samples: Bool32,
    pub sparseResidency16Samples: Bool32,
    pub sparseResidencyAliased: Bool32,
    pub variableMultisampleRate: Bool32,
    pub inheritedQueries: Bool32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct FormatProperties {
    pub linearTilingFeatures: FormatFeatureFlags,
    pub optimalTilingFeatures: FormatFeatureFlags,
    pub bufferFeatures: FormatFeatureFlags,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct ImageFormatProperties {
    pub maxExtent: Extent3D,
    pub maxMipLevels: u32,
    pub maxArrayLayers: u32,
    pub sampleCounts: SampleCountFlags,
    pub maxResourceSize: DeviceSize,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct PhysicalDeviceLimits {
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
    pub bufferImageGranularity: DeviceSize,
    pub sparseAddressSpaceSize: DeviceSize,
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
    pub minTexelBufferOffsetAlignment: DeviceSize,
    pub minUniformBufferOffsetAlignment: DeviceSize,
    pub minStorageBufferOffsetAlignment: DeviceSize,
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
    pub framebufferColorSampleCounts: SampleCountFlags,
    pub framebufferDepthSampleCounts: SampleCountFlags,
    pub framebufferStencilSampleCounts: SampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts: SampleCountFlags,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: SampleCountFlags,
    pub sampledImageIntegerSampleCounts: SampleCountFlags,
    pub sampledImageDepthSampleCounts: SampleCountFlags,
    pub sampledImageStencilSampleCounts: SampleCountFlags,
    pub storageImageSampleCounts: SampleCountFlags,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: Bool32,
    pub timestampPeriod: f32,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [f32; 2],
    pub lineWidthRange: [f32; 2],
    pub pointSizeGranularity: f32,
    pub lineWidthGranularity: f32,
    pub strictLines: Bool32,
    pub standardSampleLocations: Bool32,
    pub optimalBufferCopyOffsetAlignment: DeviceSize,
    pub optimalBufferCopyRowPitchAlignment: DeviceSize,
    pub nonCoherentAtomSize: DeviceSize,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct PhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: Bool32,
    pub residencyStandard2DMultisampleBlockShape: Bool32,
    pub residencyStandard3DBlockShape: Bool32,
    pub residencyAlignedMipSize: Bool32,
    pub residencyNonResidentStrict: Bool32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct PhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: PhysicalDeviceType,
    pub deviceName: [c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    pub pipelineCacheUUID: [u8; UUID_SIZE as usize],
    pub limits: PhysicalDeviceLimits,
    pub sparseProperties: PhysicalDeviceSparseProperties,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct QueueFamilyProperties {
    pub queueFlags: QueueFlags,
    pub queueCount: u32,
    pub timestampValidBits: u32,
    pub minImageTransferGranularity: Extent3D,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct MemoryType {
    pub propertyFlags: MemoryPropertyFlags,
    pub heapIndex: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct PhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [MemoryType; MAX_MEMORY_TYPES as usize],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [MemoryHeap; MAX_MEMORY_HEAPS as usize],
}

#[repr(C)]
#[derive(Debug)]
pub struct DeviceQueueCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queueFamilyIndex: u32,
    pub queueCount: u32,
    pub pQueuePriorities: *const f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct DeviceCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: DeviceCreateFlags,
    pub queueCreateInfoCount: u32,
    pub pQueueCreateInfos: *const DeviceQueueCreateInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
    pub pEnabledFeatures: *const PhysicalDeviceFeatures,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct ExtensionProperties {
    pub extensionName: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub specVersion: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct LayerProperties {
    pub layerName: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub specVersion: u32,
    pub implementationVersion: u32,
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
}

#[repr(C)]
#[derive(Debug)]
pub struct SubmitInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const Semaphore,
    pub pWaitDstStageMask: *const PipelineStageFlags,
    pub commandBufferCount: u32,
    pub pCommandBuffers: *const CommandBuffer,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const Semaphore,
}

#[repr(C)]
#[derive(Debug)]
pub struct MemoryAllocateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub allocationSize: DeviceSize,
    pub memoryTypeIndex: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct MappedMemoryRange {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct SparseImageFormatProperties {
    pub aspectMask: ImageAspectFlags,
    pub imageGranularity: Extent3D,
    pub flags: SparseImageFormatFlags,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct SparseImageMemoryRequirements {
    pub formatProperties: SparseImageFormatProperties,
    pub imageMipTailFirstLod: u32,
    pub imageMipTailSize: DeviceSize,
    pub imageMipTailOffset: DeviceSize,
    pub imageMipTailStride: DeviceSize,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct SparseMemoryBind {
    pub resourceOffset: DeviceSize,
    pub size: DeviceSize,
    pub memory: DeviceMemory,
    pub memoryOffset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}

#[repr(C)]
#[derive(Debug)]
pub struct SparseBufferMemoryBindInfo {
    pub buffer: Buffer,
    pub bindCount: u32,
    pub pBinds: *const SparseMemoryBind,
}

#[repr(C)]
#[derive(Debug)]
pub struct SparseImageOpaqueMemoryBindInfo {
    pub image: Image,
    pub bindCount: u32,
    pub pBinds: *const SparseMemoryBind,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct ImageSubresource {
    pub aspectMask: ImageAspectFlags,
    pub mipLevel: u32,
    pub arrayLayer: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Extent3D,
    pub memory: DeviceMemory,
    pub memoryOffset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}

#[repr(C)]
#[derive(Debug)]
pub struct SparseImageMemoryBindInfo {
    pub image: Image,
    pub bindCount: u32,
    pub pBinds: *const SparseImageMemoryBind,
}

#[repr(C)]
#[derive(Debug)]
pub struct BindSparseInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const Semaphore,
    pub bufferBindCount: u32,
    pub pBufferBinds: *const SparseBufferMemoryBindInfo,
    pub imageOpaqueBindCount: u32,
    pub pImageOpaqueBinds: *const SparseImageOpaqueMemoryBindInfo,
    pub imageBindCount: u32,
    pub pImageBinds: *const SparseImageMemoryBindInfo,
    pub signalSemaphoreCount: u32,
    pub pSignalSemaphores: *const Semaphore,
}

#[repr(C)]
#[derive(Debug)]
pub struct FenceCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: FenceCreateFlags,
}

#[repr(C)]
#[derive(Debug)]
pub struct SemaphoreCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: SemaphoreCreateFlags,
}

#[repr(C)]
#[derive(Debug)]
pub struct EventCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: EventCreateFlags,
}

#[repr(C)]
#[derive(Debug)]
pub struct QueryPoolCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: QueryPoolCreateFlags,
    pub queryType: QueryType,
    pub queryCount: u32,
    pub pipelineStatistics: QueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Debug)]
pub struct BufferCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharingMode: SharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct BufferViewCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}

#[repr(C)]
#[derive(Debug)]
pub struct ImageCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: ImageCreateFlags,
    pub imageType: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mipLevels: u32,
    pub arrayLayers: u32,
    pub samples: SampleCountFlagBits,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharingMode: SharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub initialLayout: ImageLayout,
}

#[repr(C)]
#[derive(Debug)]
pub struct BufferDeviceAddressInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub buffer: Buffer,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct SubresourceLayout {
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub rowPitch: DeviceSize,
    pub arrayPitch: DeviceSize,
    pub depthPitch: DeviceSize,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct ImageSubresourceRange {
    pub aspectMask: ImageAspectFlags,
    pub baseMipLevel: u32,
    pub levelCount: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ImageViewCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub viewType: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresourceRange: ImageSubresourceRange,
}

#[repr(C)]
#[derive(Debug)]
pub struct ShaderModuleCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: ShaderModuleCreateFlags,
    pub codeSize: usize,
    pub pCode: *const u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineCacheCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineCacheCreateFlags,
    pub initialDataSize: usize,
    pub pInitialData: *const c_void,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct SpecializationMapEntry {
    pub constantID: u32,
    pub offset: u32,
    pub size: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct SpecializationInfo {
    pub mapEntryCount: u32,
    pub pMapEntries: *const SpecializationMapEntry,
    pub dataSize: usize,
    pub pData: *const c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineShaderStageCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlagBits,
    pub module: ShaderModule,
    pub pName: *const c_char,
    pub pSpecializationInfo: *const SpecializationInfo,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub inputRate: VertexInputRate,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineVertexInputStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertexBindingDescriptionCount: u32,
    pub pVertexBindingDescriptions: *const VertexInputBindingDescription,
    pub vertexAttributeDescriptionCount: u32,
    pub pVertexAttributeDescriptions: *const VertexInputAttributeDescription,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitiveRestartEnable: Bool32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineTessellationStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineTessellationStateCreateFlags,
    pub patchControlPoints: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub minDepth: f32,
    pub maxDepth: f32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineViewportStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewportCount: u32,
    pub pViewports: *const Viewport,
    pub scissorCount: u32,
    pub pScissors: *const Rect2D,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineRasterizationStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineRasterizationStateCreateFlags,
    pub depthClampEnable: Bool32,
    pub rasterizerDiscardEnable: Bool32,
    pub polygonMode: PolygonMode,
    pub cullMode: CullModeFlags,
    pub frontFace: FrontFace,
    pub depthBiasEnable: Bool32,
    pub depthBiasConstantFactor: f32,
    pub depthBiasClamp: f32,
    pub depthBiasSlopeFactor: f32,
    pub lineWidth: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineMultisampleStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterizationSamples: SampleCountFlagBits,
    pub sampleShadingEnable: Bool32,
    pub minSampleShading: f32,
    pub pSampleMask: *const SampleMask,
    pub alphaToCoverageEnable: Bool32,
    pub alphaToOneEnable: Bool32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct StencilOpState {
    pub failOp: StencilOp,
    pub passOp: StencilOp,
    pub depthFailOp: StencilOp,
    pub compareOp: CompareOp,
    pub compareMask: u32,
    pub writeMask: u32,
    pub reference: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineDepthStencilStateCreateFlags,
    pub depthTestEnable: Bool32,
    pub depthWriteEnable: Bool32,
    pub depthCompareOp: CompareOp,
    pub depthBoundsTestEnable: Bool32,
    pub stencilTestEnable: Bool32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub minDepthBounds: f32,
    pub maxDepthBounds: f32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct PipelineColorBlendAttachmentState {
    pub blendEnable: Bool32,
    pub srcColorBlendFactor: BlendFactor,
    pub dstColorBlendFactor: BlendFactor,
    pub colorBlendOp: BlendOp,
    pub srcAlphaBlendFactor: BlendFactor,
    pub dstAlphaBlendFactor: BlendFactor,
    pub alphaBlendOp: BlendOp,
    pub colorWriteMask: ColorComponentFlags,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineColorBlendStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logicOpEnable: Bool32,
    pub logicOp: LogicOp,
    pub attachmentCount: u32,
    pub pAttachments: *const PipelineColorBlendAttachmentState,
    pub blendConstants: [f32; 4],
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineDynamicStateCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamicStateCount: u32,
    pub pDynamicStates: *const DynamicState,
}

#[repr(C)]
#[derive(Debug)]
pub struct GraphicsPipelineCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stageCount: u32,
    pub pStages: *const PipelineShaderStageCreateInfo,
    pub pVertexInputState: *const PipelineVertexInputStateCreateInfo,
    pub pInputAssemblyState: *const PipelineInputAssemblyStateCreateInfo,
    pub pTessellationState: *const PipelineTessellationStateCreateInfo,
    pub pViewportState: *const PipelineViewportStateCreateInfo,
    pub pRasterizationState: *const PipelineRasterizationStateCreateInfo,
    pub pMultisampleState: *const PipelineMultisampleStateCreateInfo,
    pub pDepthStencilState: *const PipelineDepthStencilStateCreateInfo,
    pub pColorBlendState: *const PipelineColorBlendStateCreateInfo,
    pub pDynamicState: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    pub renderPass: RenderPass,
    pub subpass: u32,
    pub basePipelineHandle: Pipeline,
    pub basePipelineIndex: i32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ComputePipelineCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    pub basePipelineHandle: Pipeline,
    pub basePipelineIndex: i32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct PushConstantRange {
    pub stageFlags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineLayoutCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub setLayoutCount: u32,
    pub pSetLayouts: *const DescriptorSetLayout,
    pub pushConstantRangeCount: u32,
    pub pPushConstantRanges: *const PushConstantRange,
}

#[repr(C)]
#[derive(Debug)]
pub struct SamplerCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: SamplerCreateFlags,
    pub magFilter: Filter,
    pub minFilter: Filter,
    pub mipmapMode: SamplerMipmapMode,
    pub addressModeU: SamplerAddressMode,
    pub addressModeV: SamplerAddressMode,
    pub addressModeW: SamplerAddressMode,
    pub mipLodBias: f32,
    pub anisotropyEnable: Bool32,
    pub maxAnisotropy: f32,
    pub compareEnable: Bool32,
    pub compareOp: CompareOp,
    pub minLod: f32,
    pub maxLod: f32,
    pub borderColor: BorderColor,
    pub unnormalizedCoordinates: Bool32,
}

#[repr(C)]
#[derive(Debug)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptorType: DescriptorType,
    pub descriptorCount: u32,
    pub stageFlags: ShaderStageFlags,
    pub pImmutableSamplers: *const Sampler,
}

#[repr(C)]
#[derive(Debug)]
pub struct DescriptorSetLayoutCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub bindingCount: u32,
    pub pBindings: *const DescriptorSetLayoutBinding,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct DescriptorPoolSize {
    pub ty: DescriptorType,
    pub descriptorCount: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct DescriptorPoolCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: DescriptorPoolCreateFlags,
    pub maxSets: u32,
    pub poolSizeCount: u32,
    pub pPoolSizes: *const DescriptorPoolSize,
}

#[repr(C)]
#[derive(Debug)]
pub struct DescriptorSetAllocateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub descriptorPool: DescriptorPool,
    pub descriptorSetCount: u32,
    pub pSetLayouts: *const DescriptorSetLayout,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub imageView: ImageView,
    pub imageLayout: ImageLayout,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}

#[repr(C)]
#[derive(Debug)]
pub struct WriteDescriptorSet {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub dstSet: DescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: DescriptorType,
    pub pImageInfo: *const DescriptorImageInfo,
    pub pBufferInfo: *const DescriptorBufferInfo,
    pub pTexelBufferView: *const BufferView,
}

#[repr(C)]
#[derive(Debug)]
pub struct CopyDescriptorSet {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub srcSet: DescriptorSet,
    pub srcBinding: u32,
    pub srcArrayElement: u32,
    pub dstSet: DescriptorSet,
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct FramebufferCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: FramebufferCreateFlags,
    pub renderPass: RenderPass,
    pub attachmentCount: u32,
    pub pAttachments: *const ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct AttachmentDescription {
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlagBits,
    pub loadOp: AttachmentLoadOp,
    pub storeOp: AttachmentStoreOp,
    pub stencilLoadOp: AttachmentLoadOp,
    pub stencilStoreOp: AttachmentStoreOp,
    pub initialLayout: ImageLayout,
    pub finalLayout: ImageLayout,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: ImageLayout,
}

#[repr(C)]
#[derive(Debug)]
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipelineBindPoint: PipelineBindPoint,
    pub inputAttachmentCount: u32,
    pub pInputAttachments: *const AttachmentReference,
    pub colorAttachmentCount: u32,
    pub pColorAttachments: *const AttachmentReference,
    pub pResolveAttachments: *const AttachmentReference,
    pub pDepthStencilAttachment: *const AttachmentReference,
    pub preserveAttachmentCount: u32,
    pub pPreserveAttachments: *const u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct SubpassDependency {
    pub srcSubpass: u32,
    pub dstSubpass: u32,
    pub srcStageMask: PipelineStageFlags,
    pub dstStageMask: PipelineStageFlags,
    pub srcAccessMask: AccessFlags,
    pub dstAccessMask: AccessFlags,
    pub dependencyFlags: DependencyFlags,
}

#[repr(C)]
#[derive(Debug)]
pub struct RenderPassCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachmentCount: u32,
    pub pAttachments: *const AttachmentDescription,
    pub subpassCount: u32,
    pub pSubpasses: *const SubpassDescription,
    pub dependencyCount: u32,
    pub pDependencies: *const SubpassDependency,
}

#[repr(C)]
#[derive(Debug)]
pub struct CommandPoolCreateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: CommandPoolCreateFlags,
    pub queueFamilyIndex: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct CommandBufferAllocateInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub commandPool: CommandPool,
    pub level: CommandBufferLevel,
    pub commandBufferCount: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct CommandBufferInheritanceInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub renderPass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    pub occlusionQueryEnable: Bool32,
    pub queryFlags: QueryControlFlags,
    pub pipelineStatistics: QueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Debug)]
pub struct CommandBufferBeginInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: CommandBufferUsageFlags,
    pub pInheritanceInfo: *const CommandBufferInheritanceInfo,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct BufferCopy {
    pub srcOffset: DeviceSize,
    pub dstOffset: DeviceSize,
    pub size: DeviceSize,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct ImageSubresourceLayers {
    pub aspectMask: ImageAspectFlags,
    pub mipLevel: u32,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct ImageCopy {
    pub srcSubresource: ImageSubresourceLayers,
    pub srcOffset: Offset3D,
    pub dstSubresource: ImageSubresourceLayers,
    pub dstOffset: Offset3D,
    pub extent: Extent3D,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct ImageBlit {
    pub srcSubresource: ImageSubresourceLayers,
    pub srcOffsets: [Offset3D; 2],
    pub dstSubresource: ImageSubresourceLayers,
    pub dstOffsets: [Offset3D; 2],
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct BufferImageCopy {
    pub bufferOffset: DeviceSize,
    pub bufferRowLength: u32,
    pub bufferImageHeight: u32,
    pub imageSubresource: ImageSubresourceLayers,
    pub imageOffset: Offset3D,
    pub imageExtent: Extent3D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

#[repr(C)]
#[derive(Debug,Copy,Clone)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

#[repr(C)]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depthStencil: ClearDepthStencilValue,
}

#[repr(C)]
pub struct ClearAttachment {
    pub aspectMask: ImageAspectFlags,
    pub colorAttachment: u32,
    pub clearValue: ClearValue,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub baseArrayLayer: u32,
    pub layerCount: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct ImageResolve {
    pub srcSubresource: ImageSubresourceLayers,
    pub srcOffset: Offset3D,
    pub dstSubresource: ImageSubresourceLayers,
    pub dstOffset: Offset3D,
    pub extent: Extent3D,
}

#[repr(C)]
#[derive(Debug)]
pub struct MemoryBarrier {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: AccessFlags,
    pub dstAccessMask: AccessFlags,
}

#[repr(C)]
#[derive(Debug)]
pub struct BufferMemoryBarrier {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: AccessFlags,
    pub dstAccessMask: AccessFlags,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}

#[repr(C)]
#[derive(Debug)]
pub struct ImageMemoryBarrier {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub srcAccessMask: AccessFlags,
    pub dstAccessMask: AccessFlags,
    pub oldLayout: ImageLayout,
    pub newLayout: ImageLayout,
    pub srcQueueFamilyIndex: u32,
    pub dstQueueFamilyIndex: u32,
    pub image: Image,
    pub subresourceRange: ImageSubresourceRange,
}

#[repr(C)]
#[derive(Debug)]
pub struct RenderPassBeginInfo {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub renderPass: RenderPass,
    pub framebuffer: Framebuffer,
    pub renderArea: Rect2D,
    pub clearValueCount: u32,
    pub pClearValues: *const ClearValue,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct DrawIndexedIndirectCommand {
    pub indexCount: u32,
    pub instanceCount: u32,
    pub firstIndex: u32,
    pub vertexOffset: i32,
    pub firstInstance: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct DrawIndirectCommand {
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub firstVertex: u32,
    pub firstInstance: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct SurfaceCapabilitiesKHR {
    pub minImageCount: u32,
    pub maxImageCount: u32,
    pub currentExtent: Extent2D,
    pub minImageExtent: Extent2D,
    pub maxImageExtent: Extent2D,
    pub maxImageArrayLayers: u32,
    pub supportedTransforms: SurfaceTransformFlagsKHR,
    pub currentTransform: SurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha: CompositeAlphaFlagsKHR,
    pub supportedUsageFlags: ImageUsageFlags,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct SurfaceFormatKHR {
    pub format: Format,
    pub colorSpace: ColorSpaceKHR,
}

pub type SwapchainCreateFlagsKHR = Flags;

#[repr(C)]
#[derive(Debug)]
pub struct SwapchainCreateInfoKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: SwapchainCreateFlagsKHR,
    pub surface: SurfaceKHR,
    pub minImageCount: u32,
    pub imageFormat: Format,
    pub imageColorSpace: ColorSpaceKHR,
    pub imageExtent: Extent2D,
    pub imageArrayLayers: u32,
    pub imageUsage: ImageUsageFlags,
    pub imageSharingMode: SharingMode,
    pub queueFamilyIndexCount: u32,
    pub pQueueFamilyIndices: *const u32,
    pub preTransform: SurfaceTransformFlagBitsKHR,
    pub compositeAlpha: CompositeAlphaFlagBitsKHR,
    pub presentMode: PresentModeKHR,
    pub clipped: Bool32,
    pub oldSwapchain: SwapchainKHR,
}

#[repr(C)]
#[derive(Debug)]
pub struct PresentInfoKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub waitSemaphoreCount: u32,
    pub pWaitSemaphores: *const Semaphore,
    pub swapchainCount: u32,
    pub pSwapchains: *const SwapchainKHR,
    pub pImageIndices: *const u32,
    pub pResults: *mut Result,
}

#[repr(C)]
#[derive(Debug)]
pub struct DisplayPropertiesKHR {
    pub display: DisplayKHR,
    pub displayName: *const c_char,
    pub physicalDimensions: Extent2D,
    pub physicalResolution: Extent2D,
    pub supportedTransforms: SurfaceTransformFlagsKHR,
    pub planeReorderPossible: Bool32,
    pub persistentContent: Bool32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct DisplayModeParametersKHR {
    pub visibleRegion: Extent2D,
    pub refreshRate: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct DisplayModePropertiesKHR {
    pub displayMode: DisplayModeKHR,
    pub parameters: DisplayModeParametersKHR,
}

#[repr(C)]
#[derive(Debug)]
pub struct DisplayModeCreateInfoKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: DisplayModeCreateFlagsKHR,
    pub parameters: DisplayModeParametersKHR,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct DisplayPlaneCapabilitiesKHR {
    pub supportedAlpha: DisplayPlaneAlphaFlagsKHR,
    pub minSrcPosition: Offset2D,
    pub maxSrcPosition: Offset2D,
    pub minSrcExtent: Extent2D,
    pub maxSrcExtent: Extent2D,
    pub minDstPosition: Offset2D,
    pub maxDstPosition: Offset2D,
    pub minDstExtent: Extent2D,
    pub maxDstExtent: Extent2D,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct DisplayPlanePropertiesKHR {
    pub currentDisplay: DisplayKHR,
    pub currentStackIndex: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct DisplaySurfaceCreateInfoKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: DisplaySurfaceCreateFlagsKHR,
    pub displayMode: DisplayModeKHR,
    pub planeIndex: u32,
    pub planeStackIndex: u32,
    pub transform: SurfaceTransformFlagBitsKHR,
    pub globalAlpha: f32,
    pub alphaMode: DisplayPlaneAlphaFlagBitsKHR,
    pub imageExtent: Extent2D,
}

#[repr(C)]
#[derive(Debug)]
pub struct DisplayPresentInfoKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub srcRect: Rect2D,
    pub dstRect: Rect2D,
    pub persistent: Bool32,
}

pub type XlibSurfaceCreateFlagsKHR = Flags;

#[repr(C)]
#[derive(Debug)]
pub struct XlibSurfaceCreateInfoKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: XlibSurfaceCreateFlagsKHR,
    pub dpy: *mut c_void,
    pub window: c_ulong,
}

pub type XcbSurfaceCreateFlagsKHR = Flags;

#[repr(C)]
#[derive(Debug)]
pub struct XcbSurfaceCreateInfoKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: XcbSurfaceCreateFlagsKHR,
    pub connection: *const c_void,
    pub window: u32,
}

pub type WaylandSurfaceCreateFlagsKHR = Flags;

#[repr(C)]
#[derive(Debug)]
pub struct WaylandSurfaceCreateInfoKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: WaylandSurfaceCreateFlagsKHR,
    pub display: *mut c_void,
    pub surface: *mut c_void,
}

pub type AndroidSurfaceCreateFlagsKHR = Flags;

#[repr(C)]
#[derive(Debug)]
pub struct AndroidSurfaceCreateInfoKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: AndroidSurfaceCreateFlagsKHR,
    pub window: *mut c_void,
}

pub type Win32SurfaceCreateFlagsKHR = Flags;

#[repr(C)]
#[derive(Debug)]
pub struct Win32SurfaceCreateInfoKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: Win32SurfaceCreateFlagsKHR,
    pub hinstance: *mut c_void,
    pub hwnd: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct IOSSurfaceCreateInfoMVK {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: IOSSurfaceCreateFlagsMVK,
    pub pView: *const c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct MacOSSurfaceCreateInfoMVK {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: MacOSSurfaceCreateFlagsMVK,
    pub pView: *const c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct MetalSurfaceCreateInfoEXT {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: MetalSurfaceCreateFlagsEXT,
    pub pLayer: *const c_void,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct MVKDeviceConfiguration {
    pub supportDisplayContentsScale: Bool32,
    pub imageFlipY: Bool32,
    pub shaderConversionFlipFragmentY: Bool32,
    pub shaderConversionFlipVertexY: Bool32,
    pub shaderConversionLogging: Bool32,
    pub performanceTracking: Bool32,
    pub performanceLoggingFrameCount: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct MVKPhysicalDeviceMetalFeatures {
    pub depthClipMode: Bool32,
    pub indirectDrawing: Bool32,
    pub baseVertexInstanceDrawing: Bool32,
    pub maxVertexBufferCount: u32,
    pub maxFragmentBufferCount: u32,
    pub bufferAlignment: DeviceSize,
    pub pushConstantsAlignment: DeviceSize,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct MVKSwapchainPerformance {
    pub lastFrameInterval: c_double,
    pub averageFrameInterval: c_double,
    pub averageFramesPerSecond: c_double,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDeviceFeatures2KHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub features: PhysicalDeviceFeatures,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDeviceProperties2KHR {
    pub sType: StructureType,
    pub pNext: *mut PhysicalDeviceSubgroupProperties,
    pub properties: PhysicalDeviceProperties,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDeviceSubgroupProperties {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub subgroupSize: u32,
    pub supportedStages: ShaderStageFlags,
    pub supportedOperations: SubgroupFeatureFlags,
    pub quadOperationsInAllStages: Bool32,
}

#[repr(C)]
#[derive(Debug)]
pub struct FormatProperties2KHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub formatProperties: FormatProperties,
}

#[repr(C)]
#[derive(Debug)]
pub struct ImageFormatProperties2KHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub imageFormatProperties: ImageFormatProperties,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDeviceImageFormatInfo2KHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub format: Format,
    pub imageType: ImageType,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub flags: ImageCreateFlags,
}

#[repr(C)]
#[derive(Debug)]
pub struct QueueFamilyProperties2KHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub queueFamilyProperties: QueueFamilyProperties,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDeviceMemoryProperties2KHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub memoryProperties: PhysicalDeviceMemoryProperties,
}

#[repr(C)]
#[derive(Debug)]
pub struct SparseImageFormatProperties2KHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub properties: SparseImageFormatProperties,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDeviceSparseImageFormatInfo2KHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub format: Format,
    pub imageType: ImageType,
    pub samples: SampleCountFlagBits,
    pub usage: ImageUsageFlags,
    pub tiling: ImageTiling,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDeviceBufferAddressFeaturesEXT {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub bufferDeviceAddress: Bool32,
    pub bufferDeviceAddressCaptureReplay: Bool32,
    pub bufferDeviceAddressMultiDevice: Bool32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDeviceVariablePointersFeatures {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub variablePointersStorageBuffer: Bool32,
    pub variablePointers: Bool32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDeviceShaderAtomicInt64Features {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub shaderBufferInt64Atomics: Bool32,
    pub shaderSharedInt64Atomics: Bool32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDevice8BitStorageFeatures {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub storageBuffer8BitAccess: Bool32,
    pub uniformAndStorageBuffer8BitAccess: Bool32,
    pub storagePushConstant8: Bool32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDevice16BitStorageFeatures {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub storageBuffer16BitAccess: Bool32,
    pub uniformAndStorageBuffer16BitAccess: Bool32,
    pub storagePushConstant16: Bool32,
    pub storageInputOutput16: Bool32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDeviceShaderFloat16Int8Features {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub shaderFloat16: Bool32,
    pub shaderInt8: Bool32,
}

pub type ViSurfaceCreateFlagsNN = Flags;

#[repr(C)]
#[derive(Debug)]
pub struct ViSurfaceCreateInfoNN {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: ViSurfaceCreateFlagsNN,
    pub window: *const c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDevicePushDescriptorPropertiesKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub maxPushDescriptors: u32,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct DescriptorUpdateTemplateEntryKHR {
    pub dstBinding: u32,
    pub dstArrayElement: u32,
    pub descriptorCount: u32,
    pub descriptorType: DescriptorType,
    pub offset: usize,
    pub stride: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct DescriptorUpdateTemplateCreateInfoKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: DescriptorUpdateTemplateCreateFlagsKHR,
    pub descriptorUpdateEntryCount: u32,
    pub pDescriptorUpdateEntries: *const DescriptorUpdateTemplateEntryKHR,
    pub templateType: DescriptorUpdateTemplateTypeKHR,
    pub descriptorSetLayout: DescriptorSetLayout,
    pub pipelineBindPoint: PipelineBindPoint,
    pub pipelineLayout: PipelineLayout,
    pub set: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct MemoryDedicatedRequirementsKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub prefersDedicatedAllocation: Bool32,
    pub requiresDedicatedAllocation: Bool32,
}

#[repr(C)]
#[derive(Debug)]
pub struct MemoryDedicatedAllocateInfoKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
}

#[repr(C)]
#[derive(Debug)]
pub struct BufferMemoryRequirementsInfo2KHR {
    pub sType: StructureType,
    pub pNext: *mut c_void,
    pub buffer: Buffer,
}

#[repr(C)]
#[derive(Debug)]
pub struct ImageMemoryRequirementsInfo2KHR {
    pub sType: StructureType,
    pub pNext: *mut c_void,
    pub image: Image,
}

#[repr(C)]
#[derive(Debug)]
pub struct MemoryRequirements2KHR {
    pub sType: StructureType,
    pub pNext: *mut c_void,
    pub memoryRequirements: MemoryRequirements,
}

#[repr(C)]
#[derive(Debug,Clone)]
pub struct RectLayerKHR {
    pub offset: Offset2D,
    pub extent: Extent2D,
    pub layer: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PresentRegionKHR {
    pub rectangleCount: u32,
    pub pRectangles: *const RectLayerKHR,
}

#[repr(C)]
#[derive(Debug)]
pub struct PresentRegionsKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub swapchainCount: u32,
    pub pRegions: *const PresentRegionKHR,
}

#[repr(C)]
pub struct DebugUtilsMessengerCreateInfoEXT {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: DebugUtilsMessengerCreateFlagsEXT,
    pub messageSeverity: DebugUtilsMessageSeverityFlagsEXT,
    pub messageType: DebugUtilsMessageTypeFlagsEXT,
    pub pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
    pub pUserData: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct DebugUtilsMessengerCallbackDataEXT {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    pub pMessageIdName: *const c_char,
    pub messageIdNumber: i32,
    pub pMessage: *const c_char,
    pub queueLabelCount: u32,
    pub pQueueLabels: *const DebugUtilsLabelEXT,
    pub cmdBufLabelCount: u32,
    pub pCmdBufLabels: *const DebugUtilsLabelEXT,
    pub objectCount: u32,
    pub pObject: *const DebugUtilsObjectNameInfoEXT,
}

#[repr(C)]
#[derive(Debug)]
pub struct DebugUtilsLabelEXT {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub pLabelName: *const c_char,
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Debug)]
pub struct PhysicalDevice16BitStorageFeaturesKHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub storageBuffer16BitAccess: Bool32,
    pub uniformAndStorageBuffer16BitAccess: Bool32,
    pub storagePushConstant16: Bool32,
    pub storageInputOutput16: Bool32,
}

#[repr(C)]
#[derive(Debug)]
pub struct DebugUtilsObjectNameInfoEXT {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub objectType: ObjectType,
    pub objectHandle: u64,
    pub pObjectName: *const c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct SurfaceFullScreenExclusiveInfoEXT {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub fullScreenExclusive: FullScreenExclusiveEXT,
}

macro_rules! ptrs {
    ($struct_name:ident, { $($name:ident => ($($param_n:ident: $param_ty:ty),*) -> $ret:ty,)+ }) => (
        $(
            paste! {
                pub type [<PFN_ $name>] = extern "system" fn($($param_ty),*) -> $ret;
            }
        )+
        pub struct $struct_name {
            $(
                pub $name: extern "system" fn($($param_ty),*) -> $ret,
            )+
        }

        impl fmt::Debug for $struct_name {
            #[inline]
            fn fmt(&self, fmt: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
                write!(fmt, "<Vulkan functions>")       // TODO:
            }
        }

        unsafe impl Send for $struct_name {}
        unsafe impl Sync for $struct_name {}

        impl $struct_name {
            pub fn load<F>(mut f: F) -> $struct_name
                where F: FnMut(&CStr) -> *const c_void
            {
                $struct_name {
                    $(
                        $name: unsafe {
                            extern "system" fn $name($(_: $param_ty),*) { panic!("function pointer `{}` not loaded", stringify!($name)) }
                            let name = CStr::from_bytes_with_nul_unchecked(concat!("vk", stringify!($name), "\0").as_bytes());
                            let val = f(name);
                            if val.is_null() { mem::transmute($name as *const ()) } else { mem::transmute(val) }
                        },
                    )+
                }
            }

            $(
                #[inline]
                pub unsafe fn $name(&self $(, $param_n: $param_ty)*) -> $ret {
                    let ptr = self.$name;
                    ptr($($param_n),*)
                }
            )+
        }
    )
}

ptrs!(Static, {
    GetInstanceProcAddr => (instance: Instance, pName: *const c_char) -> PFN_vkVoidFunction,
});

ptrs!(EntryPoints, {
    CreateInstance => (pCreateInfo: *const InstanceCreateInfo, pAllocator: *const AllocationCallbacks, pInstance: *mut Instance) -> Result,
    EnumerateInstanceExtensionProperties => (pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut ExtensionProperties) -> Result,
    EnumerateInstanceLayerProperties => (pPropertyCount: *mut u32, pProperties: *mut LayerProperties) -> Result,
});

ptrs!(InstancePointers, {
    DestroyInstance => (instance: Instance, pAllocator: *const AllocationCallbacks) -> (),
    GetDeviceProcAddr => (device: Device, pName: *const c_char) -> PFN_vkVoidFunction,
    EnumeratePhysicalDevices => (instance: Instance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut PhysicalDevice) -> Result,
    EnumerateDeviceExtensionProperties => (physicalDevice: PhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut ExtensionProperties) -> Result,
    EnumerateDeviceLayerProperties => (physicalDevice: PhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut LayerProperties) -> Result,
    CreateDevice => (physicalDevice: PhysicalDevice, pCreateInfo: *const DeviceCreateInfo, pAllocator: *const AllocationCallbacks, pDevice: *mut Device) -> Result,
    GetPhysicalDeviceFeatures => (physicalDevice: PhysicalDevice, pFeatures: *mut PhysicalDeviceFeatures) -> (),
    GetPhysicalDeviceFormatProperties => (physicalDevice: PhysicalDevice, format: Format, pFormatProperties: *mut FormatProperties) -> (),
    GetPhysicalDeviceImageFormatProperties => (physicalDevice: PhysicalDevice, format: Format, ty: ImageType, tiling: ImageTiling, usage: ImageUsageFlags, flags: ImageCreateFlags, pImageFormatProperties: *mut ImageFormatProperties) -> Result,
    GetPhysicalDeviceProperties => (physicalDevice: PhysicalDevice, pProperties: *mut PhysicalDeviceProperties) -> (),
    GetPhysicalDeviceQueueFamilyProperties => (physicalDevice: PhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut QueueFamilyProperties) -> (),
    GetPhysicalDeviceMemoryProperties => (physicalDevice: PhysicalDevice, pMemoryProperties: *mut PhysicalDeviceMemoryProperties) -> (),
    GetPhysicalDeviceSparseImageFormatProperties => (physicalDevice: PhysicalDevice, format: Format, ty: ImageType, samples: SampleCountFlagBits, usage: ImageUsageFlags, tiling: ImageTiling, pPropertyCount: *mut u32, pProperties: *mut SparseImageFormatProperties) -> (),
    DestroySurfaceKHR => (instance: Instance, surface: SurfaceKHR, pAllocator: *const AllocationCallbacks) -> (),
    CreateXlibSurfaceKHR => (instance: Instance, pCreateInfo: *const XlibSurfaceCreateInfoKHR, pAllocator: *const AllocationCallbacks, pSurface: *mut SurfaceKHR) -> Result,
    GetPhysicalDeviceXlibPresentationSupportKHR => (physicalDevice: PhysicalDevice, queueFamilyIndex: u32, dpy: *mut c_void, visualID: u32/* FIXME: VisualID */) -> Bool32,
    CreateXcbSurfaceKHR => (instance: Instance, pCreateInfo: *const XcbSurfaceCreateInfoKHR, pAllocator: *const AllocationCallbacks, pSurface: *mut SurfaceKHR) -> Result,
    GetPhysicalDeviceXcbPresentationSupportKHR => (physicalDevice: PhysicalDevice, queueFamilyIndex: u32, connection: *mut c_void, visual_id: u32 /* FIXME: xcb_visualid */) -> Bool32,
    CreateWaylandSurfaceKHR => (instance: Instance, pCreateInfo: *const WaylandSurfaceCreateInfoKHR, pAllocator: *const AllocationCallbacks, pSurface: *mut SurfaceKHR) -> Result,
    GetPhysicalDeviceWaylandPresentationSupportKHR => (physicalDevice: PhysicalDevice, queueFamilyIndex: u32, display: *mut c_void) -> Bool32,
    CreateAndroidSurfaceKHR => (instance: Instance, pCreateInfo: *const AndroidSurfaceCreateInfoKHR, pAllocator: *const AllocationCallbacks, pSurface: *mut SurfaceKHR) -> Result,
    CreateWin32SurfaceKHR => (instance: Instance, pCreateInfo: *const Win32SurfaceCreateInfoKHR, pAllocator: *const AllocationCallbacks, pSurface: *mut SurfaceKHR) -> Result,
    GetPhysicalDeviceWin32PresentationSupportKHR => (physicalDevice: PhysicalDevice, queueFamilyIndex: u32) -> Bool32,
    GetPhysicalDeviceDisplayPropertiesKHR => (physicalDevice: PhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut DisplayPropertiesKHR) -> Result,
    GetPhysicalDeviceDisplayPlanePropertiesKHR => (physicalDevice: PhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut DisplayPlanePropertiesKHR) -> Result,
    GetDisplayPlaneSupportedDisplaysKHR => (physicalDevice: PhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut DisplayKHR) -> Result,
    GetDisplayModePropertiesKHR => (physicalDevice: PhysicalDevice, display: DisplayKHR, pPropertyCount: *mut u32, pProperties: *mut DisplayModePropertiesKHR) -> Result,
    CreateDisplayModeKHR => (physicalDevice: PhysicalDevice, display: DisplayKHR, pCreateInfo: *const DisplayModeCreateInfoKHR, pAllocator: *const AllocationCallbacks, pMode: *mut DisplayModeKHR) -> Result,
    GetDisplayPlaneCapabilitiesKHR => (physicalDevice: PhysicalDevice, mode: DisplayModeKHR, planeIndex: u32, pCapabilities: *mut DisplayPlaneCapabilitiesKHR) -> Result,
    CreateDisplayPlaneSurfaceKHR => (instance: Instance, pCreateInfo: *const DisplaySurfaceCreateInfoKHR, pAllocator: *const AllocationCallbacks, pSurface: *mut SurfaceKHR) -> Result,
    GetPhysicalDeviceSurfaceSupportKHR => (physicalDevice: PhysicalDevice, queueFamilyIndex: u32, surface: SurfaceKHR, pSupported: *mut Bool32) -> Result,
    GetPhysicalDeviceSurfaceCapabilitiesKHR => (physicalDevice: PhysicalDevice, surface: SurfaceKHR, pSurfaceCapabilities: *mut SurfaceCapabilitiesKHR) -> Result,
    GetPhysicalDeviceSurfaceFormatsKHR => (physicalDevice: PhysicalDevice, surface: SurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut SurfaceFormatKHR) -> Result,
    GetPhysicalDeviceSurfacePresentModesKHR => (physicalDevice: PhysicalDevice, surface: SurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut PresentModeKHR) -> Result,
    CreateDebugUtilsMessengerEXT => (instance: Instance, pCreateInfo: *const DebugUtilsMessengerCreateInfoEXT, pAllocator: *const AllocationCallbacks, pMessenger: *const DebugUtilsMessengerEXT) -> Result,
    DestroyDebugUtilsMessengerEXT => (instance: Instance, messenger: DebugUtilsMessengerEXT, pAllocator: *const AllocationCallbacks) -> Result,

    // Provided by VK_EXT_debug_utils
    SubmitDebugUtilsMessageEXT => ( instance: Instance, messageSeverity: DebugUtilsMessageSeverityFlagBitsEXT, messageTypes: DebugUtilsMessageTypeFlagsEXT, pCallbackData: *const DebugUtilsMessengerCallbackDataEXT) -> (),

    CreateIOSSurfaceMVK => (instance: Instance, pCreateInfo: *const IOSSurfaceCreateInfoMVK, pAllocator: *const AllocationCallbacks, pSurface: *mut SurfaceKHR) -> Result,
    CreateMacOSSurfaceMVK => (instance: Instance, pCreateInfo: *const MacOSSurfaceCreateInfoMVK, pAllocator: *const AllocationCallbacks, pSurface: *mut SurfaceKHR) -> Result,
    CreateMetalSurfaceEXT => (instance: Instance, pCreateInfo: *const MetalSurfaceCreateInfoEXT, pAllocator: *const AllocationCallbacks, pSurface: *mut SurfaceKHR) -> Result,
    ActivateMoltenVKLicenseMVK => (licenseID: *const c_char, licenseKey: *const c_char, acceptLicenseTermsAndConditions: Bool32) -> Result,
    ActivateMoltenVKLicensesMVK => () -> Result,
    GetMoltenVKDeviceConfigurationMVK => (device: Device, pConfiguration: *mut MVKDeviceConfiguration) -> Result,
    SetMoltenVKDeviceConfigurationMVK => (device: Device, pConfiguration: *mut MVKDeviceConfiguration) -> Result,
    GetPhysicalDeviceMetalFeaturesMVK => (physicalDevice: PhysicalDevice, pMetalFeatures: *mut MVKPhysicalDeviceMetalFeatures) -> Result,
    GetSwapchainPerformanceMVK => (device: Device, swapchain: SwapchainKHR, pSwapchainPerf: *mut MVKSwapchainPerformance) -> Result,
    CreateViSurfaceNN => (instance: Instance, pCreateInfo: *const ViSurfaceCreateInfoNN, pAllocator: *const AllocationCallbacks, pSurface: *mut SurfaceKHR) -> Result,
    GetPhysicalDeviceFeatures2KHR => (physicalDevice: PhysicalDevice, pFeatures: *mut PhysicalDeviceFeatures2KHR) -> (),
    GetPhysicalDeviceProperties2KHR => (physicalDevice: PhysicalDevice, pProperties: *mut PhysicalDeviceProperties2KHR) -> (),
    GetPhysicalDeviceFormatProperties2KHR => (physicalDevice: PhysicalDevice, pFormatProperties: *mut FormatProperties2KHR) -> (),
    GetPhysicalDeviceImageFormatProperties2KHR => (physicalDevice: PhysicalDevice, pImageFormatInfo: *const PhysicalDeviceImageFormatInfo2KHR, pImageFormatProperties: *mut ImageFormatProperties2KHR) -> Result,
    GetPhysicalDeviceQueueFamilyProperties2KHR => (physicalDevice: PhysicalDevice, pQueueFamilyPropertiesCount: *mut u32, pQueueFamilyProperties: *mut QueueFamilyProperties2KHR) -> (),
    GetPhysicalDeviceMemoryProperties2KHR => (physicalDevice: PhysicalDevice, pMemoryProperties: *mut PhysicalDeviceMemoryProperties2KHR) -> (),
    GetPhysicalDeviceSparseImageFormatProperties2KHR => (physicalDevice: PhysicalDevice, pFormatInfo: *const PhysicalDeviceSparseImageFormatInfo2KHR, pPropertyCount: *mut u32, pProperties: *mut SparseImageFormatProperties2KHR) -> (),

    // Provided by VK_KHR_get_surface_capabilities2
    GetPhysicalDeviceSurfaceCapabilities2KHR => ( physicalDevice: PhysicalDevice, pSurfaceInfo: *const PhysicalDeviceSurfaceInfo2KHR, pSurfaceCapabilities: *mut SurfaceCapabilities2KHR ) -> Result,

    // Provided by VK_EXT_debug_report
    CreateDebugReportCallbackEXT => (instance: Instance, pCreateInfo: *const DebugReportCallbackCreateInfoEXT, pAllocator: *const AllocationCallbacks, pCallback: *mut DebugReportCallbackEXT) -> Result,

    // Provided by VK_EXT_debug_report
    DestroyDebugReportCallbackEXT => ( instance: Instance, callback: DebugReportCallbackEXT, pAllocator: *const AllocationCallbacks) -> (),

    // Provided by VK_EXT_debug_marker
    DebugMarkerSetObjectNameEXT => ( device: Device, pNameInfo: *const DebugMarkerObjectNameInfoEXT) -> Result,

});

// Provided by VK_EXT_debug_report
pub type DebugReportFlagBitsEXT = u32;
pub const DEBUG_REPORT_INFORMATION_BIT_EXT:         u32 = 0x00000001;
pub const DEBUG_REPORT_WARNING_BIT_EXT:             u32 = 0x00000002;
pub const DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT: u32 = 0x00000004;
pub const DEBUG_REPORT_ERROR_BIT_EXT:               u32 = 0x00000008;
pub const DEBUG_REPORT_DEBUG_BIT_EXT:               u32 = 0x00000010;


// Provided by VK_EXT_debug_report
#[derive(Debug)]
pub struct DebugReportCallbackCreateInfoEXT {
    pub sType:       StructureType,
    pub pNext:       *const c_void,
    pub flags:       DebugReportFlagsEXT,
    pub pfnCallback: DebugReportCallbackEXT,
    pub pUserData:   *mut c_void,
}

// Provided by VK_EXT_debug_marker
#[derive(Debug)]
pub struct DebugMarkerObjectNameInfoEXT {
    pub sType:      StructureType,
    pub pNext:      *const c_void,
    pub objectType: DebugReportObjectTypeEXT,
    pub object:     u64,
    pub pObjectName: *const char,
}

pub type DebugReportCallbackEXT = u64;
pub type DebugReportFlagsEXT = Flags;

// Provided by VK_EXT_debug_report, VK_EXT_debug_marker
pub type DebugReportObjectTypeEXT = u64;
pub const DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT:                   u64 = 0;
pub const DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT:                  u64 = 1;
pub const DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT:           u64 = 2;
pub const DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT:                    u64 = 3;
pub const DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT:                     u64 = 4;
pub const DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT:                 u64 = 5;
pub const DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT:            u64 = 6;
pub const DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT:                     u64 = 7;
pub const DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT:             u64 = 8;
pub const DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT:                    u64 = 9;
pub const DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT:                     u64 = 10;
pub const DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT:                     u64 = 11;
pub const DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT:                u64 = 12;
pub const DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT:               u64 = 13;
pub const DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT:                u64 = 14;
pub const DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT:             u64 = 15;
pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT:            u64 = 16;
pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT:           u64 = 17;
pub const DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT:               u64 = 18;
pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT:                  u64 = 19;
pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT:     u64 = 20;
pub const DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT:                   u64 = 21;
pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT:           u64 = 22;
pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT:            u64 = 23;
pub const DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT:               u64 = 24;
pub const DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT:              u64 = 25;
pub const DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT:               u64 = 26;
pub const DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT:             u64 = 27;
pub const DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT: u64 = 28;
pub const DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT:               u64 = 29;
pub const DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT:          u64 = 30;
pub const DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT:      u64 = 33;

// Provided by VK_KHR_sampler_ycbcr_conversion with VK_EXT_debug_report, VK_EXT_debug_report with VK_VERSION_1_1
pub const DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT:       u64 = 1000156000;

// Provided by VK_EXT_debug_report with VK_VERSION_1_1
pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT:     u64 = 1000085000;

// Provided by VK_KHR_acceleration_structure
pub const DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR_EXT:     u64 = 1000150000;

// Provided by VK_NV_ray_tracing
pub const DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV_EXT:      u64 = 1000165000;
pub const DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT:                   u64 = DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT;
pub const DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT:               u64 = DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT;

// Provided by VK_KHR_descriptor_update_template with VK_EXT_debug_report
pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT: u64 = DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT;

// Provided by VK_KHR_sampler_ycbcr_conversion
pub const DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR_EXT:   u64 = DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT;


/// Provided by VK_KHR_get_surface_capabilities2
///
/// Structure specifying a surface and related swapchain creation parameters
#[derive(Debug)]
pub struct PhysicalDeviceSurfaceInfo2KHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub surface: SurfaceKHR,
}

/// Provided by VK_KHR_get_surface_capabilities2
///
/// Structure describing capabilities of a surface
#[derive(Debug)]
pub struct SurfaceCapabilities2KHR {
    pub sType: StructureType,
    pub pNext: *const c_void,
    pub surfaceCapabilities: SurfaceCapabilitiesKHR,
}

ptrs!(DevicePointers, {
    DestroyDevice => (device: Device, pAllocator: *const AllocationCallbacks) -> (),
    GetDeviceQueue => (device: Device, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut Queue) -> (),
    QueueSubmit => (queue: Queue, submitCount: u32, pSubmits: *const SubmitInfo, fence: Fence) -> Result,
    QueueWaitIdle => (queue: Queue) -> Result,
    DeviceWaitIdle => (device: Device) -> Result,
    AllocateMemory => (device: Device, pAllocateInfo: *const MemoryAllocateInfo, pAllocator: *const AllocationCallbacks, pMemory: *mut DeviceMemory) -> Result,
    FreeMemory => (device: Device, memory: DeviceMemory, pAllocator: *const AllocationCallbacks) -> (),
    MapMemory => (device: Device, memory: DeviceMemory, offset: DeviceSize, size: DeviceSize, flags: MemoryMapFlags, ppData: *mut *mut c_void) -> Result,
    UnmapMemory => (device: Device, memory: DeviceMemory) -> (),
    FlushMappedMemoryRanges => (device: Device, memoryRangeCount: u32, pMemoryRanges: *const MappedMemoryRange) -> Result,
    InvalidateMappedMemoryRanges => (device: Device, memoryRangeCount: u32, pMemoryRanges: *const MappedMemoryRange) -> Result,
    GetDeviceMemoryCommitment => (device: Device, memory: DeviceMemory, pCommittedMemoryInBytes: *mut DeviceSize) -> (),
    BindBufferMemory => (device: Device, buffer: Buffer, memory: DeviceMemory, memoryOffset: DeviceSize) -> Result,
    BindImageMemory => (device: Device, image: Image, memory: DeviceMemory, memoryOffset: DeviceSize) -> Result,
    GetBufferMemoryRequirements => (device: Device, buffer: Buffer, pMemoryRequirements: *mut MemoryRequirements) -> (),
    GetImageMemoryRequirements => (device: Device, image: Image, pMemoryRequirements: *mut MemoryRequirements) -> (),
    GetImageSparseMemoryRequirements => (device: Device, image: Image, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut SparseImageMemoryRequirements) -> (),
    QueueBindSparse => (queue: Queue, bindInfoCount: u32, pBindInfo: *const BindSparseInfo, fence: Fence) -> Result,
    CreateFence => (device: Device, pCreateInfo: *const FenceCreateInfo, pAllocator: *const AllocationCallbacks, pFence: *mut Fence) -> Result,
    DestroyFence => (device: Device, fence: Fence, pAllocator: *const AllocationCallbacks) -> (),
    ResetFences => (device: Device, fenceCount: u32, pFences: *const Fence) -> Result,
    GetFenceStatus => (device: Device, fence: Fence) -> Result,
    WaitForFences => (device: Device, fenceCount: u32, pFences: *const Fence, waitAll: Bool32, timeout: u64) -> Result,
    CreateSemaphore => (device: Device, pCreateInfo: *const SemaphoreCreateInfo, pAllocator: *const AllocationCallbacks, pSemaphore: *mut Semaphore) -> Result,
    DestroySemaphore => (device: Device, semaphore: Semaphore, pAllocator: *const AllocationCallbacks) -> (),
    CreateEvent => (device: Device, pCreateInfo: *const EventCreateInfo, pAllocator: *const AllocationCallbacks, pEvent: *mut Event) -> Result,
    DestroyEvent => (device: Device, event: Event, pAllocator: *const AllocationCallbacks) -> (),
    GetEventStatus => (device: Device, event: Event) -> Result,
    SetEvent => (device: Device, event: Event) -> Result,
    ResetEvent => (device: Device, event: Event) -> Result,
    CreateQueryPool => (device: Device, pCreateInfo: *const QueryPoolCreateInfo, pAllocator: *const AllocationCallbacks, pQueryPool: *mut QueryPool) -> Result,
    DestroyQueryPool => (device: Device, queryPool: QueryPool, pAllocator: *const AllocationCallbacks) -> (),
    GetQueryPoolResults => (device: Device, queryPool: QueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut c_void, stride: DeviceSize, flags: QueryResultFlags) -> Result,
    CreateBuffer => (device: Device, pCreateInfo: *const BufferCreateInfo, pAllocator: *const AllocationCallbacks, pBuffer: *mut Buffer) -> Result,
    DestroyBuffer => (device: Device, buffer: Buffer, pAllocator: *const AllocationCallbacks) -> (),
    CreateBufferView => (device: Device, pCreateInfo: *const BufferViewCreateInfo, pAllocator: *const AllocationCallbacks, pView: *mut BufferView) -> Result,
    DestroyBufferView => (device: Device, bufferView: BufferView, pAllocator: *const AllocationCallbacks) -> (),
    CreateImage => (device: Device, pCreateInfo: *const ImageCreateInfo, pAllocator: *const AllocationCallbacks, pImage: *mut Image) -> Result,
    DestroyImage => (device: Device, image: Image, pAllocator: *const AllocationCallbacks) -> (),
    GetImageSubresourceLayout => (device: Device, image: Image, pSubresource: *const ImageSubresource, pLayout: *mut SubresourceLayout) -> (),
    CreateImageView => (device: Device, pCreateInfo: *const ImageViewCreateInfo, pAllocator: *const AllocationCallbacks, pView: *mut ImageView) -> Result,
    DestroyImageView => (device: Device, imageView: ImageView, pAllocator: *const AllocationCallbacks) -> (),
    CreateShaderModule => (device: Device, pCreateInfo: *const ShaderModuleCreateInfo, pAllocator: *const AllocationCallbacks, pShaderModule: *mut ShaderModule) -> Result,
    DestroyShaderModule => (device: Device, shaderModule: ShaderModule, pAllocator: *const AllocationCallbacks) -> (),
    CreatePipelineCache => (device: Device, pCreateInfo: *const PipelineCacheCreateInfo, pAllocator: *const AllocationCallbacks, pPipelineCache: *mut PipelineCache) -> Result,
    DestroyPipelineCache => (device: Device, pipelineCache: PipelineCache, pAllocator: *const AllocationCallbacks) -> (),
    GetPipelineCacheData => (device: Device, pipelineCache: PipelineCache, pDataSize: *mut usize, pData: *mut c_void) -> Result,
    MergePipelineCaches => (device: Device, dstCache: PipelineCache, srcCacheCount: u32, pSrcCaches: *const PipelineCache) -> Result,
    CreateGraphicsPipelines => (device: Device, pipelineCache: PipelineCache, createInfoCount: u32, pCreateInfos: *const GraphicsPipelineCreateInfo, pAllocator: *const AllocationCallbacks, pPipelines: *mut Pipeline) -> Result,
    CreateComputePipelines => (device: Device, pipelineCache: PipelineCache, createInfoCount: u32, pCreateInfos: *const ComputePipelineCreateInfo, pAllocator: *const AllocationCallbacks, pPipelines: *mut Pipeline) -> Result,
    DestroyPipeline => (device: Device, pipeline: Pipeline, pAllocator: *const AllocationCallbacks) -> (),
    CreatePipelineLayout => (device: Device, pCreateInfo: *const PipelineLayoutCreateInfo, pAllocator: *const AllocationCallbacks, pPipelineLayout: *mut PipelineLayout) -> Result,
    DestroyPipelineLayout => (device: Device, pipelineLayout: PipelineLayout, pAllocator: *const AllocationCallbacks) -> (),
    CreateSampler => (device: Device, pCreateInfo: *const SamplerCreateInfo, pAllocator: *const AllocationCallbacks, pSampler: *mut Sampler) -> Result,
    DestroySampler => (device: Device, sampler: Sampler, pAllocator: *const AllocationCallbacks) -> (),
    CreateDescriptorSetLayout => (device: Device, pCreateInfo: *const DescriptorSetLayoutCreateInfo, pAllocator: *const AllocationCallbacks, pSetLayout: *mut DescriptorSetLayout) -> Result,
    DestroyDescriptorSetLayout => (device: Device, descriptorSetLayout: DescriptorSetLayout, pAllocator: *const AllocationCallbacks) -> (),
    CreateDescriptorPool => (device: Device, pCreateInfo: *const DescriptorPoolCreateInfo, pAllocator: *const AllocationCallbacks, pDescriptorPool: *mut DescriptorPool) -> Result,
    DestroyDescriptorPool => (device: Device, descriptorPool: DescriptorPool, pAllocator: *const AllocationCallbacks) -> (),
    ResetDescriptorPool => (device: Device, descriptorPool: DescriptorPool, flags: DescriptorPoolResetFlags) -> Result,
    AllocateDescriptorSets => (device: Device, pAllocateInfo: *const DescriptorSetAllocateInfo, pDescriptorSets: *mut DescriptorSet) -> Result,
    FreeDescriptorSets => (device: Device, descriptorPool: DescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const DescriptorSet) -> Result,
    UpdateDescriptorSets => (device: Device, descriptorWriteCount: u32, pDescriptorWrites: *const WriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const CopyDescriptorSet) -> (),
    CreateFramebuffer => (device: Device, pCreateInfo: *const FramebufferCreateInfo, pAllocator: *const AllocationCallbacks, pFramebuffer: *mut Framebuffer) -> Result,
    DestroyFramebuffer => (device: Device, framebuffer: Framebuffer, pAllocator: *const AllocationCallbacks) -> (),
    CreateRenderPass => (device: Device, pCreateInfo: *const RenderPassCreateInfo, pAllocator: *const AllocationCallbacks, pRenderPass: *mut RenderPass) -> Result,
    DestroyRenderPass => (device: Device, renderPass: RenderPass, pAllocator: *const AllocationCallbacks) -> (),
    GetRenderAreaGranularity => (device: Device, renderPass: RenderPass, pGranularity: *mut Extent2D) -> (),
    CreateCommandPool => (device: Device, pCreateInfo: *const CommandPoolCreateInfo, pAllocator: *const AllocationCallbacks, pCommandPool: *mut CommandPool) -> Result,
    DestroyCommandPool => (device: Device, commandPool: CommandPool, pAllocator: *const AllocationCallbacks) -> (),
    ResetCommandPool => (device: Device, commandPool: CommandPool, flags: CommandPoolResetFlags) -> Result,
    TrimCommandPoolKHR => (device: Device, commandPool: CommandPool, flags: CommandPoolTrimFlagsKHR) -> (),
    AllocateCommandBuffers => (device: Device, pAllocateInfo: *const CommandBufferAllocateInfo, pCommandBuffers: *mut CommandBuffer) -> Result,
    FreeCommandBuffers => (device: Device, commandPool: CommandPool, commandBufferCount: u32, pCommandBuffers: *const CommandBuffer) -> (),
    BeginCommandBuffer => (commandBuffer: CommandBuffer, pBeginInfo: *const CommandBufferBeginInfo) -> Result,
    EndCommandBuffer => (commandBuffer: CommandBuffer) -> Result,
    ResetCommandBuffer => (commandBuffer: CommandBuffer, flags: CommandBufferResetFlags) -> Result,
    CmdBindPipeline => (commandBuffer: CommandBuffer, pipelineBindPoint: PipelineBindPoint, pipeline: Pipeline) -> (),
    CmdSetViewport => (commandBuffer: CommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const Viewport) -> (),
    CmdSetScissor => (commandBuffer: CommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const Rect2D) -> (),
    CmdSetLineWidth => (commandBuffer: CommandBuffer, lineWidth: f32) -> (),
    CmdSetDepthBias => (commandBuffer: CommandBuffer, depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32) -> (),
    CmdSetBlendConstants => (commandBuffer: CommandBuffer, blendConstants: &[f32; 4]) -> (),
    CmdSetDepthBounds => (commandBuffer: CommandBuffer, minDepthBounds: f32, maxDepthBounds: f32) -> (),
    CmdSetStencilCompareMask => (commandBuffer: CommandBuffer, faceMask: StencilFaceFlags, compareMask: u32) -> (),
    CmdSetStencilWriteMask => (commandBuffer: CommandBuffer, faceMask: StencilFaceFlags, writeMask: u32) -> (),
    CmdSetStencilReference => (commandBuffer: CommandBuffer, faceMask: StencilFaceFlags, reference: u32) -> (),
    CmdBindDescriptorSets => (commandBuffer: CommandBuffer, pipelineBindPoint: PipelineBindPoint, layout: PipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const DescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const u32) -> (),
    CmdBindIndexBuffer => (commandBuffer: CommandBuffer, buffer: Buffer, offset: DeviceSize, indexType: IndexType) -> (),
    CmdBindVertexBuffers => (commandBuffer: CommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const Buffer, pOffsets: *const DeviceSize) -> (),
    CmdDraw => (commandBuffer: CommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32) -> (),
    CmdDrawIndexed => (commandBuffer: CommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32) -> (),
    CmdDrawIndirect => (commandBuffer: CommandBuffer, buffer: Buffer, offset: DeviceSize, drawCount: u32, stride: u32) -> (),
    CmdDrawIndexedIndirect => (commandBuffer: CommandBuffer, buffer: Buffer, offset: DeviceSize, drawCount: u32, stride: u32) -> (),
    CmdDispatch => (commandBuffer: CommandBuffer, x: u32, y: u32, z: u32) -> (),
    CmdDispatchIndirect => (commandBuffer: CommandBuffer, buffer: Buffer, offset: DeviceSize) -> (),
    CmdCopyBuffer => (commandBuffer: CommandBuffer, srcBuffer: Buffer, dstBuffer: Buffer, regionCount: u32, pRegions: *const BufferCopy) -> (),
    CmdCopyImage => (commandBuffer: CommandBuffer, srcImage: Image, srcImageLayout: ImageLayout, dstImage: Image, dstImageLayout: ImageLayout, regionCount: u32, pRegions: *const ImageCopy) -> (),
    CmdBlitImage => (commandBuffer: CommandBuffer, srcImage: Image, srcImageLayout: ImageLayout, dstImage: Image, dstImageLayout: ImageLayout, regionCount: u32, pRegions: *const ImageBlit, filter: Filter) -> (),
    CmdCopyBufferToImage => (commandBuffer: CommandBuffer, srcBuffer: Buffer, dstImage: Image, dstImageLayout: ImageLayout, regionCount: u32, pRegions: *const BufferImageCopy) -> (),
    CmdCopyImageToBuffer => (commandBuffer: CommandBuffer, srcImage: Image, srcImageLayout: ImageLayout, dstBuffer: Buffer, regionCount: u32, pRegions: *const BufferImageCopy) -> (),
    CmdUpdateBuffer => (commandBuffer: CommandBuffer, dstBuffer: Buffer, dstOffset: DeviceSize, dataSize: DeviceSize, pData: *const u32) -> (),
    CmdFillBuffer => (commandBuffer: CommandBuffer, dstBuffer: Buffer, dstOffset: DeviceSize, size: DeviceSize, data: u32) -> (),
    CmdClearColorImage => (commandBuffer: CommandBuffer, image: Image, imageLayout: ImageLayout, pColor: *const ClearColorValue, rangeCount: u32, pRanges: *const ImageSubresourceRange) -> (),
    CmdClearDepthStencilImage => (commandBuffer: CommandBuffer, image: Image, imageLayout: ImageLayout, pDepthStencil: *const ClearDepthStencilValue, rangeCount: u32, pRanges: *const ImageSubresourceRange) -> (),
    CmdClearAttachments => (commandBuffer: CommandBuffer, attachmentCount: u32, pAttachments: *const ClearAttachment, rectCount: u32, pRects: *const ClearRect) -> (),
    CmdResolveImage => (commandBuffer: CommandBuffer, srcImage: Image, srcImageLayout: ImageLayout, dstImage: Image, dstImageLayout: ImageLayout, regionCount: u32, pRegions: *const ImageResolve) -> (),
    CmdSetEvent => (commandBuffer: CommandBuffer, event: Event, stageMask: PipelineStageFlags) -> (),
    CmdResetEvent => (commandBuffer: CommandBuffer, event: Event, stageMask: PipelineStageFlags) -> (),
    CmdWaitEvents => (commandBuffer: CommandBuffer, eventCount: u32, pEvents: *const Event, srcStageMask: PipelineStageFlags, dstStageMask: PipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const MemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const BufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const ImageMemoryBarrier) -> (),
    CmdPipelineBarrier => (commandBuffer: CommandBuffer, srcStageMask: PipelineStageFlags, dstStageMask: PipelineStageFlags, dependencyFlags: DependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const MemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const BufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const ImageMemoryBarrier) -> (),
    CmdBeginQuery => (commandBuffer: CommandBuffer, queryPool: QueryPool, query: u32, flags: QueryControlFlags) -> (),
    CmdEndQuery => (commandBuffer: CommandBuffer, queryPool: QueryPool, query: u32) -> (),
    CmdResetQueryPool => (commandBuffer: CommandBuffer, queryPool: QueryPool, firstQuery: u32, queryCount: u32) -> (),
    CmdWriteTimestamp => (commandBuffer: CommandBuffer, pipelineStage: PipelineStageFlagBits, queryPool: QueryPool, query: u32) -> (),
    CmdCopyQueryPoolResults => (commandBuffer: CommandBuffer, queryPool: QueryPool, firstQuery: u32, queryCount: u32, dstBuffer: Buffer, dstOffset: DeviceSize, stride: DeviceSize, flags: QueryResultFlags) -> (),
    CmdPushConstants => (commandBuffer: CommandBuffer, layout: PipelineLayout, stageFlags: ShaderStageFlags, offset: u32, size: u32, pValues: *const c_void) -> (),
    CmdBeginRenderPass => (commandBuffer: CommandBuffer, pRenderPassBegin: *const RenderPassBeginInfo, contents: SubpassContents) -> (),
    CmdNextSubpass => (commandBuffer: CommandBuffer, contents: SubpassContents) -> (),
    CmdEndRenderPass => (commandBuffer: CommandBuffer) -> (),
    CmdExecuteCommands => (commandBuffer: CommandBuffer, commandBufferCount: u32, pCommandBuffers: *const CommandBuffer) -> (),
    CreateSwapchainKHR => (device: Device, pCreateInfo: *const SwapchainCreateInfoKHR, pAllocator: *const AllocationCallbacks, pSwapchain: *mut SwapchainKHR) -> Result,
    DestroySwapchainKHR => (device: Device, swapchain: SwapchainKHR, pAllocator: *const AllocationCallbacks) -> (),
    GetSwapchainImagesKHR => (device: Device, swapchain: SwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut Image) -> Result,
    AcquireNextImageKHR => (device: Device, swapchain: SwapchainKHR, timeout: u64, semaphore: Semaphore, fence: Fence, pImageIndex: *mut u32) -> Result,
    QueuePresentKHR => (queue: Queue, pPresentInfo: *const PresentInfoKHR) -> Result,
    CreateSharedSwapchainsKHR => (device: Device, swapchainCount: u32, pCreateInfos: *const SwapchainCreateInfoKHR, pAllocator: *const AllocationCallbacks, pSwapchains: *mut SwapchainKHR) -> Result,
    CmdPushDescriptorSetKHR => (commandBuffer: CommandBuffer, pipelineBindPoint: PipelineBindPoint, layout: PipelineLayout, set: u32, descriptorWriteCount: u32, pDescriptorWrites: *const WriteDescriptorSet) -> (),
    CreateDescriptorUpdateTemplateKHR => (device: Device, pCreateInfo: *const DescriptorUpdateTemplateCreateInfoKHR, pAllocator: *const AllocationCallbacks, pDescriptorUpdateTemplate: *mut DescriptorUpdateTemplateKHR) -> Result,
    DestroyDescriptorUpdateTemplateKHR => (device: Device, descriptorUpdateTemplate: DescriptorUpdateTemplateKHR, pAllocator: *const AllocationCallbacks) -> (),
    UpdateDescriptorSetWithTemplateKHR => (device: Device, descriptorSet: DescriptorSet, descriptorUpdateTemplate: DescriptorUpdateTemplateKHR, pData: *const c_void) -> (),
    CmdPushDescriptorSetWithTemplateKHR => (commandBuffer: CommandBuffer, descriptorUpdateTemplate: DescriptorUpdateTemplateKHR, layout: PipelineLayout, set: u32, pData: *const c_void) -> (),
    GetImageMemoryRequirements2KHR => (device: Device, pInfo: *const ImageMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut MemoryRequirements2KHR) -> (),
    GetBufferMemoryRequirements2KHR => (device: Device, pInfo: *const BufferMemoryRequirementsInfo2KHR, pMemoryRequirements: *mut MemoryRequirements2KHR) -> (),
    SetDebugUtilsObjectNameEXT => (device: Device, pNameInfo: *const DebugUtilsObjectNameInfoEXT) -> Result,
    CmdBeginDebugUtilsLabelEXT => (commandBuffer: CommandBuffer, pLabelInfo: *const DebugUtilsLabelEXT) -> Result,
    CmdEndDebugUtilsLabelEXT => (commandBuffer: CommandBuffer) -> Result,
    CmdInsertDebugUtilsLabelEXT => (commandBuffer: CommandBuffer, pLabelInfo: *const DebugUtilsLabelEXT) -> Result,
    AcquireFullScreenExclusiveModeEXT => (device: Device, swapchain: SwapchainKHR) -> Result,
    ReleaseFullScreenExclusiveModeEXT => (device: Device, swapchain: SwapchainKHR) -> Result,
    GetBufferDeviceAddressEXT => (device: Device, pInfo: *const BufferDeviceAddressInfo) -> DeviceAddress,

    GetPastPresentationTimingGOOGLE => (device: Device, swapchain: SwapchainKHR, pPresentationTimingCount: *mut u32, pPresentationTimings: *mut PastPresentationTimingGOOGLE) -> Result,
    GetRefreshCycleDurationGOOGLE => (device: Device, swapchain: SwapchainKHR, pDisplayTimingProperties: *mut RefreshCycleDurationGOOGLE) -> Result,
});

//------------------------------------------------------------------------------------section: Provided by VK_GOOGLE_display_timing
//Note:
// GetPastPresentationTimingGOOGLE is Provided by VK_GOOGLE_display_timing
//
//The implementation will maintain a limited amount of history of timing information about previous presents. 
//Because of the asynchronous nature of the presentation engine, the timing information for a given 
//vkQueuePresentKHR command will become available some time later. These time values can be asynchronously queried, 
//and will be returned if available. All time values are in nanoseconds, relative to a monotonically-increasing clock (e.g. CLOCK_MONOTONIC (see clock_gettime(2)) on Android and Linux).
//
//To asynchronously query the presentation engine, for newly-available timing information about one or more previous presents to a given swapchain, call:
//GetPastPresentationTimingGOOGLE
//
//If pPresentationTimings is NULL, then the number of newly-available timing records for the given swapchain is returned in pPresentationTimingCount. 
//Otherwise, pPresentationTimingCount must point to a variable set by the user to the number of elements in the pPresentationTimings array, 
//and on return the variable is overwritten with the number of structures actually written to pPresentationTimings. 
//If the value of pPresentationTimingCount is less than the number of newly-available timing records, at most pPresentationTimingCount structures will be written. 
//If pPresentationTimingCount is smaller than the number of newly-available timing records for the given swapchain, VK_INCOMPLETE will be returned instead of VK_SUCCESS 
//to indicate that not all the available values were returned.

/// Provided by VK_GOOGLE_display_timing
#[repr(C)]
#[derive(Debug,Clone)]
pub struct PresentTimeGOOGLE {
    pub presentID:          u32,
    pub desiredPresentTime: u64,
}


/// Provided by VK_GOOGLE_display_timing
///
/// When the VK_GOOGLE_display_timing extension is enabled, additional fields can be specified that allow an application 
/// to specify the earliest time that an image should be displayed.  This allows an application to avoid stutter that is caused by 
/// an image being displayed earlier than planned.  Such stuttering can occur with both fixed and variable-refresh-rate displays, 
/// because stuttering occurs when the geometry is not correctly positioned for when the image is displayed. An application can instruct the presentation engine that 
/// an image should not be displayed earlier than a specified time by adding a VkPresentTimesInfoGOOGLE structure to the pNext chain of the VkPresentInfoKHR structure.
#[repr(C)]
#[derive(Debug)]
pub struct PresentTimesInfoGOOGLE {
    pub sType: StructureType,
    pub pNext: *const c_void,

    ///swapchainCount is the number of swapchains being presented to by this command.
    pub swapchainCount: u32,

    ///pTimes is NULL or a pointer to an array of VkPresentTimeGOOGLE elements with swapchainCount entries. 
    ///If not NULL, each element of pTimes contains the earliest time to present the image corresponding to the entry 
    ///in the VkPresentInfoKHR::pImageIndices array.
    pub pTimes: *const PresentTimeGOOGLE,
}

/// Provided by VK_GOOGLE_display_timing
///
///Structure containing timing information about a previously-presented image The results for a given swapchain and presentID are only returned 
///once from vkGetPastPresentationTimingGOOGLE.
///
///The application can use the VkPastPresentationTimingGOOGLE values to occasionally adjust its timing. For example, 
///if actualPresentTime is later than expected (e.g. one refreshDuration late), the application may increase its target IPD to a higher multiple of refreshDuration 
///(e.g. decrease its frame rate from 60Hz to 30Hz).  If actualPresentTime and earliestPresentTime are consistently different, and if presentMargin is consistently large enough, 
///the application may decrease its target IPD to a smaller multiple of refreshDuration (e.g. increase its frame rate from 30Hz to 60Hz).  
///If actualPresentTime and earliestPresentTime are same, and if presentMargin is consistently high, the application may delay the start of its input-render-present loop in order to 
///decrease the latency between user input and the corresponding present (always leaving some margin in case a new image takes longer to render than the previous image). 
///An application that desires its target IPD to always be the same as refreshDuration, can also adjust features until actualPresentTime is never late and presentMargin is satisfactory.
#[repr(C)]
#[derive(Debug,Clone)]
pub struct PastPresentationTimingGOOGLE {
    pub presentID:              u32,
    pub desiredPresentTime:     u64,
    pub actualPresentTime:      u64,
    pub earliestPresentTime:    u64,
    pub presentMargin:          u64,
}

/// Provided by VK_GOOGLE_display_timing
/// Structure containing the RC duration of a display
#[repr(C)]
#[derive(Debug,Clone)]
pub struct RefreshCycleDurationGOOGLE {
    ///refreshDuration is the number of nanoseconds from the start of one refresh cycle to the next.
    pub refreshDuration: u64,
}
//------------------------------------------------------------------------------------end section: Provided by VK_GOOGLE_display_timing


