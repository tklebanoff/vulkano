use paste::paste;

// Provided by VK_VERSION_1_0, VK_VERSION_1_0
pub type StructureType = u32;

macro_rules! stype {
    ($n:ident, $val:expr) => {
        paste! { pub const [<STRUCTURE_TYPE_ $n>]: u32 = $val; }
    }
}

macro_rules! stype2 {
    ($n:ident, $val:expr) => {
        paste! { pub const [<STRUCTURE_TYPE_ $n>]: u32 = [<STRUCTURE_TYPE_ $val>]; }
    }
}

stype![APPLICATION_INFO                                                , 0];
stype![INSTANCE_CREATE_INFO                                            , 1];
stype![DEVICE_QUEUE_CREATE_INFO                                        , 2];
stype![DEVICE_CREATE_INFO                                              , 3];
stype![SUBMIT_INFO                                                     , 4];
stype![MEMORY_ALLOCATE_INFO                                            , 5];
stype![MAPPED_MEMORY_RANGE                                             , 6];
stype![BIND_SPARSE_INFO                                                , 7];
stype![FENCE_CREATE_INFO                                               , 8];
stype![SEMAPHORE_CREATE_INFO                                           , 9];
stype![EVENT_CREATE_INFO                                               , 10];
stype![QUERY_POOL_CREATE_INFO                                          , 11];
stype![BUFFER_CREATE_INFO                                              , 12];
stype![BUFFER_VIEW_CREATE_INFO                                         , 13];
stype![IMAGE_CREATE_INFO                                               , 14];
stype![IMAGE_VIEW_CREATE_INFO                                          , 15];
stype![SHADER_MODULE_CREATE_INFO                                       , 16];
stype![PIPELINE_CACHE_CREATE_INFO                                      , 17];
stype![PIPELINE_SHADER_STAGE_CREATE_INFO                               , 18];
stype![PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO                         , 19];
stype![PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO                       , 20];
stype![PIPELINE_TESSELLATION_STATE_CREATE_INFO                         , 21];
stype![PIPELINE_VIEWPORT_STATE_CREATE_INFO                             , 22];
stype![PIPELINE_RASTERIZATION_STATE_CREATE_INFO                        , 23];
stype![PIPELINE_MULTISAMPLE_STATE_CREATE_INFO                          , 24];
stype![PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO                        , 25];
stype![PIPELINE_COLOR_BLEND_STATE_CREATE_INFO                          , 26];
stype![PIPELINE_DYNAMIC_STATE_CREATE_INFO                              , 27];
stype![GRAPHICS_PIPELINE_CREATE_INFO                                   , 28];
stype![COMPUTE_PIPELINE_CREATE_INFO                                    , 29];
stype![PIPELINE_LAYOUT_CREATE_INFO                                     , 30];
stype![SAMPLER_CREATE_INFO                                             , 31];
stype![DESCRIPTOR_SET_LAYOUT_CREATE_INFO                               , 32];
stype![DESCRIPTOR_POOL_CREATE_INFO                                     , 33];
stype![DESCRIPTOR_SET_ALLOCATE_INFO                                    , 34];
stype![WRITE_DESCRIPTOR_SET                                            , 35];
stype![COPY_DESCRIPTOR_SET                                             , 36];
stype![FRAMEBUFFER_CREATE_INFO                                         , 37];
stype![RENDER_PASS_CREATE_INFO                                         , 38];
stype![COMMAND_POOL_CREATE_INFO                                        , 39];
stype![COMMAND_BUFFER_ALLOCATE_INFO                                    , 40];
stype![COMMAND_BUFFER_INHERITANCE_INFO                                 , 41];
stype![COMMAND_BUFFER_BEGIN_INFO                                       , 42];
stype![RENDER_PASS_BEGIN_INFO                                          , 43];
stype![BUFFER_MEMORY_BARRIER                                           , 44];
stype![IMAGE_MEMORY_BARRIER                                            , 45];
stype![MEMORY_BARRIER                                                  , 46];
stype![LOADER_INSTANCE_CREATE_INFO                                     , 47];
stype![LOADER_DEVICE_CREATE_INFO                                       , 48];

// Provided by VK_VERSION_1_1
stype![PHYSICAL_DEVICE_SUBGROUP_PROPERTIES                             , 1000094000];
stype![BIND_BUFFER_MEMORY_INFO                                         , 1000157000];
stype![BIND_IMAGE_MEMORY_INFO                                          , 1000157001];
stype![PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES                          , 1000083000];
stype![MEMORY_DEDICATED_REQUIREMENTS                                   , 1000127000];
stype![MEMORY_DEDICATED_ALLOCATE_INFO                                  , 1000127001];
stype![MEMORY_ALLOCATE_FLAGS_INFO                                      , 1000060000];
stype![DEVICE_GROUP_RENDER_PASS_BEGIN_INFO                             , 1000060003];
stype![DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO                          , 1000060004];
stype![DEVICE_GROUP_SUBMIT_INFO                                        , 1000060005];
stype![DEVICE_GROUP_BIND_SPARSE_INFO                                   , 1000060006];
stype![BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO                            , 1000060013];
stype![BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO                             , 1000060014];
stype![PHYSICAL_DEVICE_GROUP_PROPERTIES                                , 1000070000];
stype![DEVICE_GROUP_DEVICE_CREATE_INFO                                 , 1000070001];
stype![BUFFER_MEMORY_REQUIREMENTS_INFO_2                               , 1000146000];
stype![IMAGE_MEMORY_REQUIREMENTS_INFO_2                                , 1000146001];
stype![IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2                         , 1000146002];
stype![MEMORY_REQUIREMENTS_2                                           , 1000146003];
stype![SPARSE_IMAGE_MEMORY_REQUIREMENTS_2                              , 1000146004];
stype![PHYSICAL_DEVICE_FEATURES_2                                      , 1000059000];
stype![PHYSICAL_DEVICE_PROPERTIES_2                                    , 1000059001];
stype![FORMAT_PROPERTIES_2                                             , 1000059002];
stype![IMAGE_FORMAT_PROPERTIES_2                                       , 1000059003];
stype![PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2                             , 1000059004];
stype![QUEUE_FAMILY_PROPERTIES_2                                       , 1000059005];
stype![PHYSICAL_DEVICE_MEMORY_PROPERTIES_2                             , 1000059006];
stype![SPARSE_IMAGE_FORMAT_PROPERTIES_2                                , 1000059007];
stype![PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2                      , 1000059008];
stype![PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES                       , 1000117000];
stype![RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO                 , 1000117001];
stype![IMAGE_VIEW_USAGE_CREATE_INFO                                    , 1000117002];
stype![PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO           , 1000117003];
stype![RENDER_PASS_MULTIVIEW_CREATE_INFO                               , 1000053000];
stype![PHYSICAL_DEVICE_MULTIVIEW_FEATURES                              , 1000053001];
stype![PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES                            , 1000053002];
stype![PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES                      , 1000120000];
stype![PROTECTED_SUBMIT_INFO                                           , 1000145000];
stype![PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES                       , 1000145001];
stype![PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES                     , 1000145002];
stype![DEVICE_QUEUE_INFO_2                                             , 1000145003];
stype![SAMPLER_YCBCR_CONVERSION_CREATE_INFO                            , 1000156000];
stype![SAMPLER_YCBCR_CONVERSION_INFO                                   , 1000156001];
stype![BIND_IMAGE_PLANE_MEMORY_INFO                                    , 1000156002];
stype![IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO                            , 1000156003];
stype![PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES               , 1000156004];
stype![SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES                , 1000156005];
stype![DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO                          , 1000085000];
stype![PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO                      , 1000071000];
stype![EXTERNAL_IMAGE_FORMAT_PROPERTIES                                , 1000071001];
stype![PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO                            , 1000071002];
stype![EXTERNAL_BUFFER_PROPERTIES                                      , 1000071003];
stype![PHYSICAL_DEVICE_ID_PROPERTIES                                   , 1000071004];
stype![EXTERNAL_MEMORY_BUFFER_CREATE_INFO                              , 1000072000];
stype![EXTERNAL_MEMORY_IMAGE_CREATE_INFO                               , 1000072001];
stype![EXPORT_MEMORY_ALLOCATE_INFO                                     , 1000072002];
stype![PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO                             , 1000112000];
stype![EXTERNAL_FENCE_PROPERTIES                                       , 1000112001];
stype![EXPORT_FENCE_CREATE_INFO                                        , 1000113000];
stype![EXPORT_SEMAPHORE_CREATE_INFO                                    , 1000077000];
stype![PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO                         , 1000076000];
stype![EXTERNAL_SEMAPHORE_PROPERTIES                                   , 1000076001];
stype![PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES                        , 1000168000];
stype![DESCRIPTOR_SET_LAYOUT_SUPPORT                                   , 1000168001];
stype![PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES                 , 1000063000];

// Provided by VK_VERSION_1_2
stype![PHYSICAL_DEVICE_VULKAN_1_1_FEATURES                             , 49];
stype![PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES                           , 50];
stype![PHYSICAL_DEVICE_VULKAN_1_2_FEATURES                             , 51];
stype![PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES                           , 52];
stype![IMAGE_FORMAT_LIST_CREATE_INFO                                   , 1000147000];
stype![ATTACHMENT_DESCRIPTION_2                                        , 1000109000];
stype![ATTACHMENT_REFERENCE_2                                          , 1000109001];
stype![SUBPASS_DESCRIPTION_2                                           , 1000109002];
stype![SUBPASS_DEPENDENCY_2                                            , 1000109003];
stype![RENDER_PASS_CREATE_INFO_2                                       , 1000109004];
stype![SUBPASS_BEGIN_INFO                                              , 1000109005];
stype![SUBPASS_END_INFO                                                , 1000109006];
stype![PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES                           , 1000177000];
stype![PHYSICAL_DEVICE_DRIVER_PROPERTIES                               , 1000196000];
stype![PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES                    , 1000180000];
stype![PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES                    , 1000082000];
stype![PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES                       , 1000197000];
stype![DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO                 , 1000161000];
stype![PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES                    , 1000161001];
stype![PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES                  , 1000161002];
stype![DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO          , 1000161003];
stype![DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT         , 1000161004];
stype![PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES                , 1000199000];
stype![SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE                       , 1000199001];
stype![PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES                    , 1000221000];
stype![IMAGE_STENCIL_USAGE_CREATE_INFO                                 , 1000246000];
stype![PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES                , 1000130000];
stype![SAMPLER_REDUCTION_MODE_CREATE_INFO                              , 1000130001];
stype![PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES                    , 1000211000];
stype![PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES                  , 1000108000];
stype![FRAMEBUFFER_ATTACHMENTS_CREATE_INFO                             , 1000108001];
stype![FRAMEBUFFER_ATTACHMENT_IMAGE_INFO                               , 1000108002];
stype![RENDER_PASS_ATTACHMENT_BEGIN_INFO                               , 1000108003];
stype![PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES         , 1000253000];
stype![PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES         , 1000175000];
stype![PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES         , 1000241000];
stype![ATTACHMENT_REFERENCE_STENCIL_LAYOUT                             , 1000241001];
stype![ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT                           , 1000241002];
stype![PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES                       , 1000261000];
stype![PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES                     , 1000207000];
stype![PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES                   , 1000207001];
stype![SEMAPHORE_TYPE_CREATE_INFO                                      , 1000207002];
stype![TIMELINE_SEMAPHORE_SUBMIT_INFO                                  , 1000207003];
stype![SEMAPHORE_WAIT_INFO                                             , 1000207004];
stype![SEMAPHORE_SIGNAL_INFO                                           , 1000207005];
stype![PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES                  , 1000257000];
stype![BUFFER_DEVICE_ADDRESS_INFO                                      , 1000244001];
stype![BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO                       , 1000257002];
stype![MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO                     , 1000257003];
stype![DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO                       , 1000257004];

// Provided by VK_KHR_swapchain
stype![SWAPCHAIN_CREATE_INFO_KHR                                       , 1000001000];
stype![PRESENT_INFO_KHR                                                , 1000001001];

// Provided by VK_KHR_swapchain with VK_VERSION_1_1;
// VK_KHR_device_group with VK_KHR_surface
stype![DEVICE_GROUP_PRESENT_CAPABILITIES_KHR                           , 1000060007];

// Provided by VK_KHR_swapchain with VK_VERSION_1_1;
// VK_KHR_device_group with VK_KHR_swapchain
stype![IMAGE_SWAPCHAIN_CREATE_INFO_KHR                                 , 1000060008];
stype![BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR                            , 1000060009];
stype![ACQUIRE_NEXT_IMAGE_INFO_KHR                                     , 1000060010];
stype![DEVICE_GROUP_PRESENT_INFO_KHR                                   , 1000060011];
stype![DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR                          , 1000060012];

// Provided by VK_KHR_display
stype![DISPLAY_MODE_CREATE_INFO_KHR                                    , 1000002000];
stype![DISPLAY_SURFACE_CREATE_INFO_KHR                                 , 1000002001];

// Provided by VK_KHR_display_swapchain
stype![DISPLAY_PRESENT_INFO_KHR                                        , 1000003000];

// Provided by VK_KHR_xlib_surface
stype![XLIB_SURFACE_CREATE_INFO_KHR                                    , 1000004000];

// Provided by VK_KHR_xcb_surface
stype![XCB_SURFACE_CREATE_INFO_KHR                                     , 1000005000];

// Provided by VK_KHR_wayland_surface
stype![WAYLAND_SURFACE_CREATE_INFO_KHR                                 , 1000006000];

// Provided by VK_KHR_android_surface
stype![ANDROID_SURFACE_CREATE_INFO_KHR                                 , 1000008000];

// Provided by VK_KHR_win32_surface
stype![WIN32_SURFACE_CREATE_INFO_KHR                                   , 1000009000];

// Provided by VK_EXT_debug_report
stype![DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT                           , 1000011000];

// Provided by VK_AMD_rasterization_order
stype![PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD            , 1000018000];

// Provided by VK_EXT_debug_marker
stype![DEBUG_MARKER_OBJECT_NAME_INFO_EXT                               , 1000022000];
stype![DEBUG_MARKER_OBJECT_TAG_INFO_EXT                                , 1000022001];
stype![DEBUG_MARKER_MARKER_INFO_EXT                                    , 1000022002];

// Provided by VK_NV_dedicated_allocation
stype![DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV                       , 1000026000];
stype![DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV                      , 1000026001];
stype![DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV                    , 1000026002];

// Provided by VK_EXT_transform_feedback
stype![PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT                 , 1000028000];
stype![PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT               , 1000028001];
stype![PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT             , 1000028002];

// Provided by VK_NVX_image_view_handle
stype![IMAGE_VIEW_HANDLE_INFO_NVX                                      , 1000030000];
stype![IMAGE_VIEW_ADDRESS_PROPERTIES_NVX                               , 1000030001];

// Provided by VK_AMD_texture_gather_bias_lod
stype![TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD                        , 1000041000];

// Provided by VK_GGP_stream_descriptor_surface
stype![STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP                       , 1000049000];

// Provided by VK_NV_corner_sampled_image
stype![PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV                , 1000050000];

// Provided by VK_NV_external_memory
stype![EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV                            , 1000056000];
stype![EXPORT_MEMORY_ALLOCATE_INFO_NV                                  , 1000056001];

// Provided by VK_NV_external_memory_win32
stype![IMPORT_MEMORY_WIN32_HANDLE_INFO_NV                              , 1000057000];
stype![EXPORT_MEMORY_WIN32_HANDLE_INFO_NV                              , 1000057001];

// Provided by VK_NV_win32_keyed_mutex
stype![WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV                       , 1000058000];

// Provided by VK_EXT_validation_flags
stype![VALIDATION_FLAGS_EXT                                            , 1000061000];

// Provided by VK_NN_vi_surface
stype![VI_SURFACE_CREATE_INFO_NN                                       , 1000062000];

// Provided by VK_EXT_texture_compression_astc_hdr
stype![PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT       , 1000066000];

// Provided by VK_EXT_astc_decode_mode
stype![IMAGE_VIEW_ASTC_DECODE_MODE_EXT                                 , 1000067000];
stype![PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT                        , 1000067001];

// Provided by VK_KHR_external_memory_win32
stype![IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR                             , 1000073000];
stype![EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR                             , 1000073001];
stype![MEMORY_WIN32_HANDLE_PROPERTIES_KHR                              , 1000073002];
stype![MEMORY_GET_WIN32_HANDLE_INFO_KHR                                , 1000073003];

// Provided by VK_KHR_external_memory_fd
stype![IMPORT_MEMORY_FD_INFO_KHR                                       , 1000074000];
stype![MEMORY_FD_PROPERTIES_KHR                                        , 1000074001];
stype![MEMORY_GET_FD_INFO_KHR                                          , 1000074002];

// Provided by VK_KHR_win32_keyed_mutex
stype![WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR                      , 1000075000];

// Provided by VK_KHR_external_semaphore_win32
stype![IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR                          , 1000078000];
stype![EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR                          , 1000078001];
stype![D3D12_FENCE_SUBMIT_INFO_KHR                                     , 1000078002];
stype![SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR                             , 1000078003];

// Provided by VK_KHR_external_semaphore_fd
stype![IMPORT_SEMAPHORE_FD_INFO_KHR                                    , 1000079000];
stype![SEMAPHORE_GET_FD_INFO_KHR                                       , 1000079001];

// Provided by VK_KHR_push_descriptor
stype![PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR                  , 1000080000];

// Provided by VK_EXT_conditional_rendering
stype![COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT       , 1000081000];
stype![PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT              , 1000081001];
stype![CONDITIONAL_RENDERING_BEGIN_INFO_EXT                            , 1000081002];

// Provided by VK_KHR_incremental_present
stype![PRESENT_REGIONS_KHR                                             , 1000084000];

// Provided by VK_NV_clip_space_w_scaling
stype![PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV                , 1000087000];

// Provided by VK_EXT_display_surface_counter
stype![SURFACE_CAPABILITIES_2_EXT                                      , 1000090000];

// Provided by VK_EXT_display_control
stype![DISPLAY_POWER_INFO_EXT                                          , 1000091000];
stype![DEVICE_EVENT_INFO_EXT                                           , 1000091001];
stype![DISPLAY_EVENT_INFO_EXT                                          , 1000091002];
stype![SWAPCHAIN_COUNTER_CREATE_INFO_EXT                               , 1000091003];

// Provided by VK_GOOGLE_display_timing
stype![PRESENT_TIMES_INFO_GOOGLE                                       , 1000092000];

// Provided by VK_NVX_multiview_per_view_attributes
stype![PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX    , 1000097000];

// Provided by VK_NV_viewport_swizzle
stype![PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV                  , 1000098000];

// Provided by VK_EXT_discard_rectangles
stype![PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT                , 1000099000];
stype![PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT                , 1000099001];

// Provided by VK_EXT_conservative_rasterization
stype![PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT       , 1000101000];
stype![PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT       , 1000101001];

// Provided by VK_EXT_depth_clip_enable
stype![PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT                  , 1000102000];
stype![PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT         , 1000102001];

// Provided by VK_EXT_hdr_metadata
stype![HDR_METADATA_EXT                                                , 1000105000];

// Provided by VK_KHR_shared_presentable_image
stype![SHARED_PRESENT_SURFACE_CAPABILITIES_KHR                         , 1000111000];

// Provided by VK_KHR_external_fence_win32
stype![IMPORT_FENCE_WIN32_HANDLE_INFO_KHR                              , 1000114000];
stype![EXPORT_FENCE_WIN32_HANDLE_INFO_KHR                              , 1000114001];
stype![FENCE_GET_WIN32_HANDLE_INFO_KHR                                 , 1000114002];

// Provided by VK_KHR_external_fence_fd
stype![IMPORT_FENCE_FD_INFO_KHR                                        , 1000115000];
stype![FENCE_GET_FD_INFO_KHR                                           , 1000115001];

// Provided by VK_KHR_performance_query
stype![PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR                  , 1000116000];
stype![PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR                , 1000116001];
stype![QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR                          , 1000116002];
stype![PERFORMANCE_QUERY_SUBMIT_INFO_KHR                               , 1000116003];
stype![ACQUIRE_PROFILING_LOCK_INFO_KHR                                 , 1000116004];
stype![PERFORMANCE_COUNTER_KHR                                         , 1000116005];
stype![PERFORMANCE_COUNTER_DESCRIPTION_KHR                             , 1000116006];

// Provided by VK_KHR_get_surface_capabilities2
stype![PHYSICAL_DEVICE_SURFACE_INFO_2_KHR                              , 1000119000];
stype![SURFACE_CAPABILITIES_2_KHR                                      , 1000119001];
stype![SURFACE_FORMAT_2_KHR                                            , 1000119002];

// Provided by VK_KHR_get_display_properties2
stype![DISPLAY_PROPERTIES_2_KHR                                        , 1000121000];
stype![DISPLAY_PLANE_PROPERTIES_2_KHR                                  , 1000121001];
stype![DISPLAY_MODE_PROPERTIES_2_KHR                                   , 1000121002];
stype![DISPLAY_PLANE_INFO_2_KHR                                        , 1000121003];
stype![DISPLAY_PLANE_CAPABILITIES_2_KHR                                , 1000121004];

// Provided by VK_MVK_ios_surface
stype![IOS_SURFACE_CREATE_INFO_MVK                                     , 1000122000];

// Provided by VK_MVK_macos_surface
stype![MACOS_SURFACE_CREATE_INFO_MVK                                   , 1000123000];

// Provided by VK_EXT_debug_utils
stype![DEBUG_UTILS_OBJECT_NAME_INFO_EXT                                , 1000128000];
stype![DEBUG_UTILS_OBJECT_TAG_INFO_EXT                                 , 1000128001];
stype![DEBUG_UTILS_LABEL_EXT                                           , 1000128002];
stype![DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT                         , 1000128003];
stype![DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT                           , 1000128004];

// Provided by VK_ANDROID_external_memory_android_hardware_buffer
stype![ANDROID_HARDWARE_BUFFER_USAGE_ANDROID                           , 1000129000];
stype![ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID                      , 1000129001];
stype![ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID               , 1000129002];
stype![IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID                     , 1000129003];
stype![MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID                 , 1000129004];
stype![EXTERNAL_FORMAT_ANDROID                                         , 1000129005];

// Provided by VK_EXT_inline_uniform_block
stype![PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT               , 1000138000];
stype![PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT             , 1000138001];
stype![WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT                   , 1000138002];
stype![DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT            , 1000138003];

// Provided by VK_EXT_sample_locations
stype![SAMPLE_LOCATIONS_INFO_EXT                                       , 1000143000];
stype![RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT                     , 1000143001];
stype![PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT                 , 1000143002];
stype![PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT                 , 1000143003];
stype![MULTISAMPLE_PROPERTIES_EXT                                      , 1000143004];

// Provided by VK_EXT_blend_operation_advanced
stype![PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT           , 1000148000];
stype![PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT         , 1000148001];
stype![PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT             , 1000148002];

// Provided by VK_NV_fragment_coverage_to_color
stype![PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV                 , 1000149000];

// Provided by VK_KHR_acceleration_structure
stype![WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR                 , 1000150007];
stype![ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR                  , 1000150000];
stype![ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR                  , 1000150002];
stype![ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR                  , 1000150003];
stype![ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR              , 1000150004];
stype![ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR              , 1000150005];
stype![ACCELERATION_STRUCTURE_GEOMETRY_KHR                             , 1000150006];
stype![ACCELERATION_STRUCTURE_VERSION_INFO_KHR                         , 1000150009];
stype![COPY_ACCELERATION_STRUCTURE_INFO_KHR                            , 1000150010];
stype![COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR                  , 1000150011];
stype![COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR                  , 1000150012];
stype![PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR             , 1000150013];
stype![PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR           , 1000150014];
stype![ACCELERATION_STRUCTURE_CREATE_INFO_KHR                          , 1000150017];
stype![ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR                     , 1000150020];

// Provided by VK_KHR_ray_tracing_pipeline
stype![PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR               , 1000347000];
stype![PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR             , 1000347001];
stype![RAY_TRACING_PIPELINE_CREATE_INFO_KHR                            , 1000150015];
stype![RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR                        , 1000150016];
stype![RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR                  , 1000150018];

// Provided by VK_KHR_ray_query
stype![PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR                          , 1000348013];

// Provided by VK_NV_framebuffer_mixed_samples
stype![PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV               , 1000152000];

// Provided by VK_NV_shader_sm_builtins
stype![PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV                  , 1000154000];
stype![PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV                , 1000154001];

// Provided by VK_EXT_image_drm_format_modifier
stype![DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT                         , 1000158000];
stype![PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT              , 1000158002];
stype![IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT                  , 1000158003];
stype![IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT              , 1000158004];
stype![IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT                        , 1000158005];

// Provided by VK_EXT_validation_cache
stype![VALIDATION_CACHE_CREATE_INFO_EXT                                , 1000160000];
stype![SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT                  , 1000160001];

// Provided by VK_KHR_portability_subset
#[cfg(enable_beta_extensions)] stype![PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR,   1000163000];
#[cfg(enable_beta_extensions)] stype![PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR, 1000163001];

// Provided by VK_NV_shading_rate_image
stype![PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV       , 1000164000];
stype![PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV                  , 1000164001];
stype![PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV                , 1000164002];
stype![PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV      , 1000164005];

// Provided by VK_NV_ray_tracing
stype![RAY_TRACING_PIPELINE_CREATE_INFO_NV                             , 1000165000];
stype![ACCELERATION_STRUCTURE_CREATE_INFO_NV                           , 1000165001];
stype![GEOMETRY_NV                                                     , 1000165003];
stype![GEOMETRY_TRIANGLES_NV                                           , 1000165004];
stype![GEOMETRY_AABB_NV                                                , 1000165005];
stype![BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV                      , 1000165006];
stype![WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV                  , 1000165007];
stype![ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV              , 1000165008];
stype![PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV                       , 1000165009];
stype![RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV                         , 1000165011];
stype![ACCELERATION_STRUCTURE_INFO_NV                                  , 1000165012];

// Provided by VK_NV_representative_fragment_test
stype![PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV        , 1000166000];
stype![PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV      , 1000166001];

// Provided by VK_EXT_filter_cubic
stype![PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT                , 1000170000];
stype![FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT             , 1000170001];

// Provided by VK_EXT_global_priority
stype![DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT                    , 1000174000];

// Provided by VK_EXT_external_memory_host
stype![IMPORT_MEMORY_HOST_POINTER_INFO_EXT                             , 1000178000];
stype![MEMORY_HOST_POINTER_PROPERTIES_EXT                              , 1000178001];
stype![PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT             , 1000178002];

// Provided by VK_KHR_shader_clock
stype![PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR                       , 1000181000];

// Provided by VK_AMD_pipeline_compiler_control
stype![PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD                       , 1000183000];

// Provided by VK_EXT_calibrated_timestamps
stype![CALIBRATED_TIMESTAMP_INFO_EXT                                   , 1000184000];

// Provided by VK_AMD_shader_core_properties
stype![PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD                      , 1000185000];

// Provided by VK_AMD_memory_overallocation_behavior
stype![DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD                    , 1000189000];

// Provided by VK_EXT_vertex_attribute_divisor
stype![PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT         , 1000190000];
stype![PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT             , 1000190001];
stype![PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT           , 1000190002];

// Provided by VK_GGP_frame_token
stype![PRESENT_FRAME_TOKEN_GGP                                         , 1000191000];

// Provided by VK_EXT_pipeline_creation_feedback
stype![PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT                      , 1000192000];

// Provided by VK_NV_compute_shader_derivatives
stype![PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV          , 1000201000];

// Provided by VK_NV_mesh_shader
stype![PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV                         , 1000202000];
stype![PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV                       , 1000202001];

// Provided by VK_NV_fragment_shader_barycentric
stype![PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV         , 1000203000];

// Provided by VK_NV_shader_image_footprint
stype![PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV              , 1000204000];

// Provided by VK_NV_scissor_exclusive
stype![PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV        , 1000205000];
stype![PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV                   , 1000205002];

// Provided by VK_NV_device_diagnostic_checkpoints
stype![CHECKPOINT_DATA_NV                                              , 1000206000];
stype![QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV                           , 1000206001];

// Provided by VK_INTEL_shader_integer_functions2
stype![PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL       , 1000209000];

// Provided by VK_INTEL_performance_query
stype![QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL                  , 1000210000];
stype![INITIALIZE_PERFORMANCE_API_INFO_INTEL                           , 1000210001];
stype![PERFORMANCE_MARKER_INFO_INTEL                                   , 1000210002];
stype![PERFORMANCE_STREAM_MARKER_INFO_INTEL                            , 1000210003];
stype![PERFORMANCE_OVERRIDE_INFO_INTEL                                 , 1000210004];
stype![PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL                    , 1000210005];

// Provided by VK_EXT_pci_bus_info
stype![PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT                     , 1000212000];

// Provided by VK_AMD_display_native_hdr
stype![DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD                     , 1000213000];
stype![SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD                    , 1000213001];

// Provided by VK_FUCHSIA_imagepipe_surface
stype![IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA                           , 1000214000];

// Provided by VK_KHR_shader_terminate_invocation
stype![PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR        , 1000215000];

// Provided by VK_EXT_metal_surface
stype![METAL_SURFACE_CREATE_INFO_EXT                                   , 1000217000];

// Provided by VK_EXT_fragment_density_map
stype![PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT               , 1000218000];
stype![PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT             , 1000218001];
stype![RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT                , 1000218002];

// Provided by VK_EXT_subgroup_size_control
stype![PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT            , 1000225000];
stype![PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT    , 1000225001];
stype![PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT              , 1000225002];

// Provided by VK_KHR_fragment_shading_rate
stype![FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR                       , 1000226000];
stype![PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR            , 1000226001];
stype![PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR            , 1000226002];
stype![PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR              , 1000226003];
stype![PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR                       , 1000226004];

// Provided by VK_AMD_shader_core_properties2
stype![PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD                    , 1000227000];

// Provided by VK_AMD_device_coherent_memory
stype![PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD                    , 1000229000];

// Provided by VK_EXT_shader_image_atomic_int64
stype![PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT          , 1000234000];

// Provided by VK_EXT_memory_budget
stype![PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT                    , 1000237000];

// Provided by VK_EXT_memory_priority
stype![PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT                    , 1000238000];
stype![MEMORY_PRIORITY_ALLOCATE_INFO_EXT                               , 1000238001];

// Provided by VK_KHR_surface_protected_capabilities
stype![SURFACE_PROTECTED_CAPABILITIES_KHR                              , 1000239000];

// Provided by VK_NV_dedicated_allocation_image_aliasing
stype![PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV , 1000240000];

// Provided by VK_EXT_buffer_device_address
stype![PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT              , 1000244000];
stype![BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT                           , 1000244002];

// Provided by VK_EXT_tooling_info
stype![PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT                             , 1000245000];

// Provided by VK_EXT_validation_features
stype![VALIDATION_FEATURES_EXT                                         , 1000247000];

// Provided by VK_NV_cooperative_matrix
stype![PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV                  , 1000249000];
stype![COOPERATIVE_MATRIX_PROPERTIES_NV                                , 1000249001];
stype![PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV                , 1000249002];

// Provided by VK_NV_coverage_reduction_mode
stype![PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV             , 1000250000];
stype![PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV                , 1000250001];
stype![FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV                        , 1000250002];

// Provided by VK_EXT_fragment_shader_interlock
stype![PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT          , 1000251000];

// Provided by VK_EXT_ycbcr_image_arrays
stype![PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT                 , 1000252000];

// Provided by VK_EXT_full_screen_exclusive
stype![SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT                          , 1000255000];
stype![SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT                  , 1000255002];

// Provided by VK_EXT_full_screen_exclusive with VK_KHR_win32_surface
stype![SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT                    , 1000255001];

// Provided by VK_EXT_headless_surface
stype![HEADLESS_SURFACE_CREATE_INFO_EXT                                , 1000256000];

// Provided by VK_EXT_line_rasterization
stype![PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT                 , 1000259000];
stype![PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT               , 1000259001];
stype![PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT               , 1000259002];

// Provided by VK_EXT_shader_atomic_float
stype![PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT                , 1000260000];

// Provided by VK_EXT_index_type_uint8
stype![PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT                   , 1000265000];

// Provided by VK_EXT_extended_dynamic_state
stype![PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT             , 1000267000];

// Provided by VK_KHR_pipeline_executable_properties
stype![PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR     , 1000269000];
stype![PIPELINE_INFO_KHR                                               , 1000269001];
stype![PIPELINE_EXECUTABLE_PROPERTIES_KHR                              , 1000269002];
stype![PIPELINE_EXECUTABLE_INFO_KHR                                    , 1000269003];
stype![PIPELINE_EXECUTABLE_STATISTIC_KHR                               , 1000269004];
stype![PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR                 , 1000269005];

// Provided by VK_EXT_shader_demote_to_helper_invocation
stype![PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT , 1000276000];

// Provided by VK_NV_device_generated_commands
stype![PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV         , 1000277000];
stype![GRAPHICS_SHADER_GROUP_CREATE_INFO_NV                            , 1000277001];
stype![GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV                  , 1000277002];
stype![INDIRECT_COMMANDS_LAYOUT_TOKEN_NV                               , 1000277003];
stype![INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV                         , 1000277004];
stype![GENERATED_COMMANDS_INFO_NV                                      , 1000277005];
stype![GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV                  , 1000277006];
stype![PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV           , 1000277007];

// Provided by VK_EXT_texel_buffer_alignment
stype![PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT             , 1000281000];
stype![PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT           , 1000281001];

// Provided by VK_QCOM_render_pass_transform
stype![COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM      , 1000282000];
stype![RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM                           , 1000282001];

// Provided by VK_EXT_device_memory_report
stype![PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT               , 1000284000];
stype![DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT                     , 1000284001];
stype![DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT                          , 1000284002];

// Provided by VK_EXT_robustness2
stype![PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT                       , 1000286000];
stype![PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT                     , 1000286001];

// Provided by VK_EXT_custom_border_color
stype![SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT                     , 1000287000];
stype![PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT              , 1000287001];
stype![PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT                , 1000287002];

// Provided by VK_KHR_pipeline_library
stype![PIPELINE_LIBRARY_CREATE_INFO_KHR                                , 1000290000];

// Provided by VK_EXT_private_data
stype![PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT                       , 1000295000];
stype![DEVICE_PRIVATE_DATA_CREATE_INFO_EXT                             , 1000295001];
stype![PRIVATE_DATA_SLOT_CREATE_INFO_EXT                               , 1000295002];

// Provided by VK_EXT_pipeline_creation_cache_control
stype![PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT    , 1000297000];

// Provided by VK_NV_device_diagnostics_config
stype![PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV                  , 1000300000];
stype![DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV                        , 1000300001];

// Provided by VK_KHR_synchronization2
stype![MEMORY_BARRIER_2_KHR                                            , 1000314000];
stype![BUFFER_MEMORY_BARRIER_2_KHR                                     , 1000314001];
stype![IMAGE_MEMORY_BARRIER_2_KHR                                      , 1000314002];
stype![DEPENDENCY_INFO_KHR                                             , 1000314003];
stype![SUBMIT_INFO_2_KHR                                               , 1000314004];
stype![SEMAPHORE_SUBMIT_INFO_KHR                                       , 1000314005];
stype![COMMAND_BUFFER_SUBMIT_INFO_KHR                                  , 1000314006];
stype![PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR                  , 1000314007];

// Provided by VK_KHR_synchronization2 with VK_NV_device_diagnostic_checkpoints
stype![QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV                         , 1000314008];
stype![CHECKPOINT_DATA_2_NV                                            , 1000314009];

// Provided by VK_KHR_zero_initialize_workgroup_memory
stype![PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR   , 1000325000];

// Provided by VK_NV_fragment_shading_rate_enums
stype![PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV       , 1000326000];
stype![PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV         , 1000326001];
stype![PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV        , 1000326002];

// Provided by VK_EXT_fragment_density_map2
stype![PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT             , 1000332000];
stype![PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT           , 1000332001];

// Provided by VK_QCOM_rotated_copy_commands
stype![COPY_COMMAND_TRANSFORM_INFO_QCOM                                , 1000333000];

// Provided by VK_EXT_image_robustness
stype![PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT                   , 1000335000];

// Provided by VK_KHR_workgroup_memory_explicit_layout
stype![PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR   , 1000336000];

// Provided by VK_KHR_copy_commands2
stype![COPY_BUFFER_INFO_2_KHR                                          , 1000337000];
stype![COPY_IMAGE_INFO_2_KHR                                           , 1000337001];
stype![COPY_BUFFER_TO_IMAGE_INFO_2_KHR                                 , 1000337002];
stype![COPY_IMAGE_TO_BUFFER_INFO_2_KHR                                 , 1000337003];
stype![BLIT_IMAGE_INFO_2_KHR                                           , 1000337004];
stype![RESOLVE_IMAGE_INFO_2_KHR                                        , 1000337005];
stype![BUFFER_COPY_2_KHR                                               , 1000337006];
stype![IMAGE_COPY_2_KHR                                                , 1000337007];
stype![IMAGE_BLIT_2_KHR                                                , 1000337008];
stype![BUFFER_IMAGE_COPY_2_KHR                                         , 1000337009];
stype![IMAGE_RESOLVE_2_KHR                                             , 1000337010];

// Provided by VK_EXT_4444_formats
stype![PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT                       , 1000340000];

// Provided by VK_EXT_directfb_surface
stype![DIRECTFB_SURFACE_CREATE_INFO_EXT                                , 1000346000];

// Provided by VK_VALVE_mutable_descriptor_type
stype![PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE          , 1000351000];
stype![MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE                       , 1000351002];

// Provided by VK_QNX_screen_surface
stype![SCREEN_SURFACE_CREATE_INFO_QNX                                  , 1000378000];
stype2![PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES                      , PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES];
stype2![PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES                 , PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES];
stype2![DEBUG_REPORT_CREATE_INFO_EXT                                   , DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT];

// Provided by VK_KHR_multiview
stype2![RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR                          , RENDER_PASS_MULTIVIEW_CREATE_INFO];
stype2![PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR                         , PHYSICAL_DEVICE_MULTIVIEW_FEATURES];
stype2![PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR                       , PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES];

// Provided by VK_KHR_get_physical_device_properties2
stype2![PHYSICAL_DEVICE_FEATURES_2_KHR                                 , PHYSICAL_DEVICE_FEATURES_2];
stype2![PHYSICAL_DEVICE_PROPERTIES_2_KHR                               , PHYSICAL_DEVICE_PROPERTIES_2];
stype2![FORMAT_PROPERTIES_2_KHR                                        , FORMAT_PROPERTIES_2];
stype2![IMAGE_FORMAT_PROPERTIES_2_KHR                                  , IMAGE_FORMAT_PROPERTIES_2];
stype2![PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR                        , PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2];
stype2![QUEUE_FAMILY_PROPERTIES_2_KHR                                  , QUEUE_FAMILY_PROPERTIES_2];
stype2![PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR                        , PHYSICAL_DEVICE_MEMORY_PROPERTIES_2];
stype2![SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR                           , SPARSE_IMAGE_FORMAT_PROPERTIES_2];
stype2![PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR                 , PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2];

// Provided by VK_KHR_device_group
stype2![MEMORY_ALLOCATE_FLAGS_INFO_KHR                                 , MEMORY_ALLOCATE_FLAGS_INFO];
stype2![DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR                        , DEVICE_GROUP_RENDER_PASS_BEGIN_INFO];
stype2![DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR                     , DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO];
stype2![DEVICE_GROUP_SUBMIT_INFO_KHR                                   , DEVICE_GROUP_SUBMIT_INFO];
stype2![DEVICE_GROUP_BIND_SPARSE_INFO_KHR                              , DEVICE_GROUP_BIND_SPARSE_INFO];

// Provided by VK_KHR_device_group with VK_KHR_bind_memory2
stype2![BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR                       , BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO];
stype2![BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR                        , BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO];

// Provided by VK_KHR_device_group_creation
stype2![PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR                           , PHYSICAL_DEVICE_GROUP_PROPERTIES];
stype2![DEVICE_GROUP_DEVICE_CREATE_INFO_KHR                            , DEVICE_GROUP_DEVICE_CREATE_INFO];

// Provided by VK_KHR_external_memory_capabilities
stype2![PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR                 , PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO];
stype2![EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR                           , EXTERNAL_IMAGE_FORMAT_PROPERTIES];
stype2![PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR                       , PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO];
stype2![EXTERNAL_BUFFER_PROPERTIES_KHR                                 , EXTERNAL_BUFFER_PROPERTIES];

// Provided by VK_KHR_external_memory_capabilities; 
// VK_KHR_external_semaphore_capabilities; 
// VK_KHR_external_fence_capabilities
stype2![PHYSICAL_DEVICE_ID_PROPERTIES_KHR                              , PHYSICAL_DEVICE_ID_PROPERTIES];

// Provided by VK_KHR_external_memory
stype2![EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR                         , EXTERNAL_MEMORY_BUFFER_CREATE_INFO];
stype2![EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR                          , EXTERNAL_MEMORY_IMAGE_CREATE_INFO];
stype2![EXPORT_MEMORY_ALLOCATE_INFO_KHR                                , EXPORT_MEMORY_ALLOCATE_INFO];

// Provided by VK_KHR_external_semaphore_capabilities
stype2![PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR                    , PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO];
stype2![EXTERNAL_SEMAPHORE_PROPERTIES_KHR                              , EXTERNAL_SEMAPHORE_PROPERTIES];

// Provided by VK_KHR_external_semaphore
stype2![EXPORT_SEMAPHORE_CREATE_INFO_KHR                               , EXPORT_SEMAPHORE_CREATE_INFO];

// Provided by VK_KHR_shader_float16_int8
stype2![PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR               , PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES];
stype2![PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR                      , PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES];

// Provided by VK_KHR_16bit_storage
stype2![PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR                     , PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES];

// Provided by VK_KHR_descriptor_update_template
stype2![DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR                     , DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO];
stype2![SURFACE_CAPABILITIES2_EXT                                      , SURFACE_CAPABILITIES_2_EXT];

// Provided by VK_KHR_imageless_framebuffer
stype2![PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR             , PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES];
stype2![FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR                        , FRAMEBUFFER_ATTACHMENTS_CREATE_INFO];
stype2![FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR                          , FRAMEBUFFER_ATTACHMENT_IMAGE_INFO];
stype2![RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR                          , RENDER_PASS_ATTACHMENT_BEGIN_INFO];

// Provided by VK_KHR_create_renderpass2
stype2![ATTACHMENT_DESCRIPTION_2_KHR                                   , ATTACHMENT_DESCRIPTION_2];
stype2![ATTACHMENT_REFERENCE_2_KHR                                     , ATTACHMENT_REFERENCE_2];
stype2![SUBPASS_DESCRIPTION_2_KHR                                      , SUBPASS_DESCRIPTION_2];
stype2![SUBPASS_DEPENDENCY_2_KHR                                       , SUBPASS_DEPENDENCY_2];
stype2![RENDER_PASS_CREATE_INFO_2_KHR                                  , RENDER_PASS_CREATE_INFO_2];
stype2![SUBPASS_BEGIN_INFO_KHR                                         , SUBPASS_BEGIN_INFO];
stype2![SUBPASS_END_INFO_KHR                                           , SUBPASS_END_INFO];

// Provided by VK_KHR_external_fence_capabilities
stype2![PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR                        , PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO];
stype2![EXTERNAL_FENCE_PROPERTIES_KHR                                  , EXTERNAL_FENCE_PROPERTIES];

// Provided by VK_KHR_external_fence
stype2![EXPORT_FENCE_CREATE_INFO_KHR                                   , EXPORT_FENCE_CREATE_INFO];

// Provided by VK_KHR_maintenance2
stype2![PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR                  , PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES];
stype2![RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR            , RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO];
stype2![IMAGE_VIEW_USAGE_CREATE_INFO_KHR                               , IMAGE_VIEW_USAGE_CREATE_INFO];
stype2![PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR      , PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO];

// Provided by VK_KHR_variable_pointers
stype2![PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR                 , PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES];
stype2![PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR                  , PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR];

// Provided by VK_KHR_dedicated_allocation
stype2![MEMORY_DEDICATED_REQUIREMENTS_KHR                              , MEMORY_DEDICATED_REQUIREMENTS];
stype2![MEMORY_DEDICATED_ALLOCATE_INFO_KHR                             , MEMORY_DEDICATED_ALLOCATE_INFO];

// Provided by VK_EXT_sampler_filter_minmax
stype2![PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT           , PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES];
stype2![SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT                         , SAMPLER_REDUCTION_MODE_CREATE_INFO];

// Provided by VK_KHR_get_memory_requirements2
stype2![BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR                          , BUFFER_MEMORY_REQUIREMENTS_INFO_2];
stype2![IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR                           , IMAGE_MEMORY_REQUIREMENTS_INFO_2];
stype2![IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR                    , IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2];
stype2![MEMORY_REQUIREMENTS_2_KHR                                      , MEMORY_REQUIREMENTS_2];
stype2![SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR                         , SPARSE_IMAGE_MEMORY_REQUIREMENTS_2];

// Provided by VK_KHR_image_format_list
stype2![IMAGE_FORMAT_LIST_CREATE_INFO_KHR                              , IMAGE_FORMAT_LIST_CREATE_INFO];

// Provided by VK_KHR_sampler_ycbcr_conversion
stype2![SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR                       , SAMPLER_YCBCR_CONVERSION_CREATE_INFO];
stype2![SAMPLER_YCBCR_CONVERSION_INFO_KHR                              , SAMPLER_YCBCR_CONVERSION_INFO];
stype2![BIND_IMAGE_PLANE_MEMORY_INFO_KHR                               , BIND_IMAGE_PLANE_MEMORY_INFO];
stype2![IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR                       , IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO];
stype2![PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR          , PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES];
stype2![SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR           , SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES];

// Provided by VK_KHR_bind_memory2
stype2![BIND_BUFFER_MEMORY_INFO_KHR                                    , BIND_BUFFER_MEMORY_INFO];
stype2![BIND_IMAGE_MEMORY_INFO_KHR                                     , BIND_IMAGE_MEMORY_INFO];

// Provided by VK_EXT_descriptor_indexing
stype2![DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT            , DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO];
stype2![PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT               , PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES];
stype2![PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT             , PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES];
stype2![DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT     , DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO];
stype2![DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT    , DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT];

// Provided by VK_KHR_maintenance3
stype2![PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR                   , PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES];
stype2![DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR                              , DESCRIPTOR_SET_LAYOUT_SUPPORT];

// Provided by VK_KHR_shader_subgroup_extended_types
stype2![PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR    , PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES];

// Provided by VK_KHR_8bit_storage
stype2![PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR                      , PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES];

// Provided by VK_KHR_shader_atomic_int64
stype2![PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR               , PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES];

// Provided by VK_KHR_driver_properties
stype2![PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR                          , PHYSICAL_DEVICE_DRIVER_PROPERTIES];

// Provided by VK_KHR_shader_float_controls
stype2![PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR                  , PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES];

// Provided by VK_KHR_depth_stencil_resolve
stype2![PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR           , PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES];
stype2![SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR                  , SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE];

// Provided by VK_KHR_timeline_semaphore
stype2![PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR                , PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES];
stype2![PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR              , PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES];
stype2![SEMAPHORE_TYPE_CREATE_INFO_KHR                                 , SEMAPHORE_TYPE_CREATE_INFO];
stype2![TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR                             , TIMELINE_SEMAPHORE_SUBMIT_INFO];
stype2![SEMAPHORE_WAIT_INFO_KHR                                        , SEMAPHORE_WAIT_INFO];
stype2![SEMAPHORE_SIGNAL_INFO_KHR                                      , SEMAPHORE_SIGNAL_INFO];
stype2![QUERY_POOL_CREATE_INFO_INTEL                                   , QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL];

// Provided by VK_KHR_vulkan_memory_model
stype2![PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR               , PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES];

// Provided by VK_EXT_scalar_block_layout
stype2![PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT               , PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES];

// Provided by VK_KHR_separate_depth_stencil_layouts
stype2![PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR    , PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES];
stype2![ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR                        , ATTACHMENT_REFERENCE_STENCIL_LAYOUT];
stype2![ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR                      , ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT];
stype2![PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT                    , PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT];

// Provided by VK_EXT_buffer_device_address
stype2![BUFFER_DEVICE_ADDRESS_INFO_EXT                                 , BUFFER_DEVICE_ADDRESS_INFO];

// Provided by VK_EXT_separate_stencil_usage
stype2![IMAGE_STENCIL_USAGE_CREATE_INFO_EXT                            , IMAGE_STENCIL_USAGE_CREATE_INFO];

// Provided by VK_KHR_uniform_buffer_standard_layout
stype2![PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR    , PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES];

// Provided by VK_KHR_buffer_device_address
stype2![PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR             , PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES];
stype2![BUFFER_DEVICE_ADDRESS_INFO_KHR                                 , BUFFER_DEVICE_ADDRESS_INFO];
stype2![BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR                  , BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO];
stype2![MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR                , MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO];
stype2![DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR                  , DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO];

// Provided by VK_EXT_host_query_reset
stype2![PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT                  , PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES];
