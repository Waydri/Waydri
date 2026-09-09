const std = @import("std");

const c = @cImport({
    @cInclude("vulkan/vulkan.h");
});

pub const VkError = error{
    NotReady,
    Timeout,
    EventSet,
    EventReset,
    Incomplete,
    ErrorOutOfHostMemory,
    ErrorOutOfDeviceMemory,
    ErrorInitializationFailed,
    ErrorDeviceLost,
    ErrorMemoryMapFailed,
    ErrorLayerNotPresent,
    ErrorExtensionNotPresent,
    ErrorFeatureNotPresent,
    ErrorIncompatibleDriver,
    ErrorTooManyObjects,
    ErrorFormatNotSupported,
    ErrorFragmentedPool,
    ErrorUnknown,
    ErrorSurfaceLost,
    ErrorNativeWindowInUseKhr,
    ErrorOutOfDateKhr,
    ErrorSuboptimalKhr,
    ErrorIncompatibleDisplayKhr,
    ErrorValidationFailedExt,
    ErrorInvalidShaderNv,
    ErrorInvalidDryRunVk,
    ErrorPipelineCompileRequiredExt,
    ErrorNotPermittedExt,
};

pub const VkPhysicalDeviceType = enum(u32) {
    other = 0,
    integrated_gpu = 1,
    discrete_gpu = 2,
    virtual_gpu = 3,
    cpu = 4,

    pub fn name(self: VkPhysicalDeviceType) []const u8 {
        return switch (self) {
            .other => "Other",
            .integrated_gpu => "Integrated GPU",
            .discrete_gpu => "Discrete GPU",
            .virtual_gpu => "Virtual GPU",
            .cpu => "CPU",
        };
    }
};

pub const QueueFamilyProperties = struct {
    queue_flags: QueueFlags,
    queue_count: u32,
    timestamp_valid_bits: u32,
    min_image_transfer_granularity: ImageExtent3D,

    pub const QueueFlags = packed struct(u32) {
        graphics: bool = false,
        compute: bool = false,
        transfer: bool = false,
        sparse_binding: bool = false,
        _padding: u28 = 0,

        pub fn supportsGraphics(self: QueueFlags) bool {
            return self.graphics;
        }

        pub fn supportsCompute(self: QueueFlags) bool {
            return self.compute;
        }

        pub fn supportsTransfer(self: QueueFlags) bool {
            return self.transfer;
        }

        pub fn isDedicatedTransfer(self: QueueFlags) bool {
            return self.transfer and !self.graphics and !self.compute;
        }

        pub fn isDedicatedCompute(self: QueueFlags) bool {
            return self.compute and !self.graphics;
        }
    };

    pub const ImageExtent3D = struct {
        width: u32,
        height: u32,
        depth: u32,
    };
};

pub const PhysicalDeviceInfo = struct {
    api_version: u32,
    driver_version: u32,
    vendor_id: u32,
    device_id: u32,
    device_type: VkPhysicalDeviceType,
    device_name: [256:0]u8,
    pipeline_cache_uuid: [16]u8,
    limits: PhysicalDeviceLimits,
    sparse_properties: SparseProperties,

    pub const PhysicalDeviceLimits = struct {
        max_image_dimension_1d: u32,
        max_image_dimension_2d: u32,
        max_image_dimension_3d: u32,
        max_image_dimension_cube: u32,
        max_image_array_layers: u32,
        max_texel_buffer_elements: u32,
        max_uniform_buffer_range: u32,
        max_storage_buffer_range: u32,
        max_push_constants_size: u32,
        max_memory_allocation_count: u32,
        max_sampler_allocation_count: u32,
        buffer_image_granularity: u64,
        sparse_address_space_size: u64,
        max_bound_descriptor_sets: u32,
        max_per_stage_descriptor_samplers: u32,
        max_per_stage_descriptor_uniform_buffers: u32,
        max_per_stage_descriptor_storage_buffers: u32,
        max_per_stage_descriptor_sampled_images: u32,
        max_per_stage_descriptor_storage_images: u32,
        max_per_stage_descriptors: u32,
        max_descriptor_set_samplers: u32,
        max_descriptor_set_uniform_buffers: u32,
        max_descriptor_set_storage_buffers: u32,
        max_descriptor_set_sampled_images: u32,
        max_descriptor_set_storage_images: u32,
        max_vertex_input_attributes: u32,
        max_vertex_input_bindings: u32,
        max_vertex_input_attribute_offset: u32,
        max_vertex_input_binding_stride: u32,
        max_vertex_output_components: u32,
        max_tessellation_generation_level: u32,
        max_tessellation_patch_size: u32,
        max_tessellation_control_per_vertex_input: u32,
        max_tessellation_control_per_patch_output: u32,
        max_tessellation_control_total_output: u32,
        max_tessellation_evaluation_input: u32,
        max_tessellation_evaluation_output: u32,
        max_geometry_shader_invocations: u32,
        max_geometry_input: u32,
        max_geometry_output: u32,
        max_geometry_output_vertices: u32,
        max_geometry_total_output: u32,
        max_fragment_input: u32,
        max_fragment_output_attachments: u32,
        max_fragment_dual_src_attachments: u32,
        max_fragment_combined_output: u32,
        max_compute_work_group_count: [3]u32,
        max_compute_work_group_size: [3]u32,
        max_compute_work_group_invocations: u32,
        max_compute_uniform_buffers: u32,
        max_compute_uniform_components: u32,
        max_compute_texture_image_layers: u32,
        max_compute_storage_images: u32,
        max_compute_shared_memory_size: u32,
        max_pixel_constant_vectors: u32,
        max_texel_gather_offset: u32,
        max_uniform_buffer_range_max: u32,
        max_storage_buffer_range_max: u32,
        max_clip_distances: u32,
        max_cull_distances: u32,
        max_combined_clip_and_cull_distances: u32,
        sub_pixel_precision_bits: u32,
        sub_texel_precision_bits: u32,
        mipmap_integer_precision_bits: u32,
        max_draw_indexed_index: u32,
        max_draw_indirect_count: u32,
        max_sampler_lod_bias: f32,
        max_sampler_anisotropy: f32,
        max_viewports: u32,
        max_viewport_dimensions: [2]u32,
        viewport_bounds_range: [2]f32,
        viewport_sub_pixel_bits: u32,
        min_memory_map_alignment: usize,
        min_texel_buffer_offset_alignment: u64,
        min_uniform_buffer_offset_alignment: u64,
        min_storage_buffer_offset_alignment: u64,
        min_texel_offset: i32,
        max_texel_offset: u32,
        min_texel_gather_offset: i32,
        max_texel_gather_offset: u32,
        min_uniform_buffer_offset_alignment_exp: u32,
        max_uniform_buffer_size: u64,
        max_storage_buffer_size: u64,
        max_push_constants_size_aligned: u32,
        max_memory_allocation_count_aligned: u32,
        max_sample_count: u32,
        timestamp_period: f32,
        max_clip_distances_mask: u32,
        max_cull_distances_mask: u32,
        max_combined_clip_and_cull_distances_mask: u32,
    };

    pub const SparseProperties = struct {
        residency_standard_2dblock_shape: bool,
        residency_standard_2dmultisample_shape: bool,
        residency_standard_3dblock_shape: bool,
        residency_aligned_mip_size: bool,
        residency_non_resident_strict: bool,
    };
};

pub const VkInstance = struct {
    instance: c.VkInstance,
    physical_devices: []PhysicalDeviceInfo,
    extensions: []const ExtensionProperties,

    pub const ExtensionProperties = struct {
        extension_name: [256:0]u8,
        spec_version: u32,
    };

    pub fn init(allocator: std.mem.Allocator) !VkInstance {
        var app_info = std.mem.zeroes(c.VkApplicationInfo);
        app_info.sType = c.VK_STRUCTURE_TYPE_APPLICATION_INFO;
        app_info.pApplicationName = "Waydri";
        app_info.applicationVersion = c.VK_MAKE_VERSION(0, 11, 0);
        app_info.pEngineName = "Waydri";
        app_info.engineVersion = c.VK_MAKE_VERSION(0, 11, 0);
        app_info.apiVersion = c.VK_MAKE_VERSION(1, 0, 0);

        var create_info = std.mem.zeroes(c.VkInstanceCreateInfo);
        create_info.sType = c.VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
        create_info.pApplicationInfo = &app_info;

        var instance: c.VkInstance = null;
        const result = c.vkCreateInstance(&create_info, null, &instance);
        if (result != c.VK_SUCCESS) return error.ErrorInitializationFailed;

        return .{
            .instance = instance,
            .physical_devices = try allocator.alloc(PhysicalDeviceInfo, 0),
            .extensions = try allocator.alloc(ExtensionProperties, 0),
        };
    }

    pub fn deinit(self: *VkInstance, allocator: std.mem.Allocator) void {
        if (self.instance != null) {
            c.vkDestroyInstance(self.instance, null);
        }
        allocator.free(self.physical_devices);
        allocator.free(self.extensions);
    }

    pub fn enumeratePhysicalDevices(self: *VkInstance, allocator: std.mem.Allocator) ![]PhysicalDeviceInfo {
        var count: u32 = 0;
        _ = c.vkEnumeratePhysicalDevices(self.instance, &count, null);
        if (count == 0) return &.{};

        const devices = try allocator.alloc(c.VkPhysicalDevice, count);
        defer allocator.free(devices);
        _ = c.vkEnumeratePhysicalDevices(self.instance, &count, devices.ptr);

        var result = try allocator.alloc(PhysicalDeviceInfo, count);
        for (devices, 0..) |device, i| {
            var props = std.mem.zeroes(c.VkPhysicalDeviceProperties2);
            props.sType = c.VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;
            c.vkGetPhysicalDeviceProperties2(device, &props);

            var name_buf: [256:0]u8 = std.mem.zeroes([256:0]u8);
            const src_name: [*]const u8 = @ptrCast(&props.properties.deviceName);
            @memcpy(&name_buf, src_name[0..256]);

            result[i] = .{
                .api_version = props.properties.apiVersion,
                .driver_version = props.properties.driverVersion,
                .vendor_id = props.properties.vendorID,
                .device_id = props.properties.deviceID,
                .device_type = @enumFromInt(@as(u32, @intCast(props.properties.deviceType))),
                .device_name = name_buf,
                .pipeline_cache_uuid = props.properties.pipelineCacheUUID,
                .limits = std.mem.zeroes(PhysicalDeviceInfo.PhysicalDeviceLimits),
                .sparse_properties = .{
                    .residency_standard_2dblock_shape = props.properties.sparseProperties.residencyStandard2DBlockShape != 0,
                    .residency_standard_2dmultisample_shape = props.properties.sparseProperties.residencyStandard2DMultisampleShape != 0,
                    .residency_standard_3dblock_shape = props.properties.sparseProperties.residencyStandard3DBlockShape != 0,
                    .residency_aligned_mip_size = props.properties.sparseProperties.residencyAlignedMipSize != 0,
                    .residency_non_resident_strict = props.properties.sparseProperties.residencyNonResidentStrict != 0,
                },
            };
        }

        self.physical_devices = result;
        return result;
    }

    pub fn getQueueFamilyProperties(self: *VkInstance, device_index: usize) ![]QueueFamilyProperties {
        if (device_index >= self.physical_devices.len) return error.InvalidDeviceIndex;
        var count: u32 = 0;
        c.vkGetPhysicalDeviceQueueFamilyProperties2.?(null, &count, null);
        if (count == 0) return &.{};

        var props = std.heap.page_allocator.alloc(c.VkQueueFamilyProperties2, count) catch return error.OutOfMemory;
        defer std.heap.page_allocator.free(props);
        for (props) |*p| {
            p.* = std.mem.zeroes(c.VkQueueFamilyProperties2);
            p.sType = c.VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2;
        }
        c.vkGetPhysicalDeviceQueueFamilyProperties2.?(null, &count, props.ptr);

        var result = std.heap.page_allocator.alloc(QueueFamilyProperties, count) catch return error.OutOfMemory;
        for (props, 0..) |p, i| {
            result[i] = .{
                .queue_flags = @bitCast(p.queueFamilyProperties.queueFlags),
                .queue_count = p.queueFamilyProperties.queueCount,
                .timestamp_valid_bits = p.queueFamilyProperties.timestampValidBits,
                .min_image_transfer_granularity = .{
                    .width = p.queueFamilyProperties.minImageTransferGranularity.width,
                    .height = p.queueFamilyProperties.minImageTransferGranularity.height,
                    .depth = p.queueFamilyProperties.minImageTransferGranularity.depth,
                },
            };
        }

        return result;
    }

    pub fn createDevice(self: *VkInstance, physical_device_index: usize, queue_family_index: u32) !VkDevice {
        if (physical_device_index >= self.physical_devices.len) return error.InvalidDeviceIndex;

        var queue_info = std.mem.zeroes(c.VkDeviceQueueCreateInfo);
        queue_info.sType = c.VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
        queue_info.queueFamilyIndex = queue_family_index;
        queue_info.queueCount = 1;
        const priority: f32 = 1.0;
        queue_info.pQueuePriorities = &priority;

        var device_features = std.mem.zeroes(c.VkPhysicalDeviceFeatures);

        var create_info = std.mem.zeroes(c.VkDeviceCreateInfo);
        create_info.sType = c.VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
        create_info.queueCreateInfoCount = 1;
        create_info.pQueueCreateInfos = &queue_info;
        create_info.pEnabledFeatures = &device_features;

        var device: c.VkDevice = null;
        const result = c.vkCreateDevice.?(null, &create_info, null, &device);
        if (result != c.VK_SUCCESS) return error.ErrorDeviceLost;

        return .{
            .device = device,
            .queue = null,
            .queue_family_index = queue_family_index,
        };
    }
};

pub const VkDevice = struct {
    device: c.VkDevice,
    queue: ?c.VkQueue,
    queue_family_index: u32,

    pub fn deinit(self: *VkDevice) void {
        if (self.device != null) {
            c.vkDestroyDevice.?(self.device, null);
        }
    }

    pub fn getQueue(self: *VkDevice) ?c.VkQueue {
        if (self.queue == null and self.device != null) {
            var queue: c.VkQueue = null;
            c.vkGetDeviceQueue.?(self.device, self.queue_family_index, 0, &queue);
            self.queue = queue;
        }
        return self.queue;
    }

    pub fn waitIdle(self: *VkDevice) !void {
        if (self.device) |device| {
            const result = c.vkDeviceWaitIdle.?(device);
            if (result != c.VK_SUCCESS) return error.ErrorDeviceLost;
        }
    }
};

pub const VkSurfaceKHR = struct {
    surface: c.VkSurfaceKHR,
    instance: c.VkInstance,

    pub fn destroy(self: *VkSurfaceKHR) void {
        if (self.surface != null and self.instance != null) {
            c.vkDestroySurfaceKHR.?(self.instance, self.surface, null);
        }
    }

    pub fn getCapabilities(self: *const VkSurfaceKHR) !SurfaceCapabilities {
        _ = self;
        return .{
            .min_image_count = 1,
            .max_image_count = 4,
            .current_extent = .{ .width = 0, .height = 0 },
            .min_image_extent = .{ .width = 1, .height = 1 },
            .max_image_extent = .{ .width = 8192, .height = 8192 },
            .max_image_array_layers = 1,
            .supported_transforms = .{ .identity = true },
            .current_transform = .{},
            .supported_composite_alpha = .{ .opaque = true, .pre_multiplied = true },
            .supported_usage_flags = .{ .color_attachment = true },
        };
    }

    pub const SurfaceCapabilities = struct {
        min_image_count: u32,
        max_image_count: u32,
        current_extent: Extent2D,
        min_image_extent: Extent2D,
        max_image_extent: Extent2D,
        max_image_array_layers: u32,
        supported_transforms: SurfaceTransformFlags,
        current_transform: SurfaceTransformFlags,
        supported_composite_alpha: CompositeAlphaFlags,
        supported_usage_flags: UsageFlags,
    };

    pub const Extent2D = struct {
        width: u32,
        height: u32,
    };

    pub const SurfaceTransformFlags = packed struct(u32) {
        identity: bool = false,
        rotate_90: bool = false,
        rotate_180: bool = false,
        rotate_270: bool = false,
        horizontal_mirror: bool = false,
        horizontal_mirror_rotate_90: bool = false,
        horizontal_mirror_rotate_180: bool = false,
        horizontal_mirror_rotate_270: bool = false,
        inherit: bool = false,
        _padding: u23 = 0,
    };

    pub const CompositeAlphaFlags = packed struct(u32) {
        opaque: bool = false,
        pre_multiplied: bool = false,
        pre_multiplied_straight: bool = false,
        post_multiplied: bool = false,
        inherit: bool = false,
        _padding: u27 = 0,
    };

    pub const UsageFlags = packed struct(u32) {
        transfer_src: bool = false,
        transfer_dst: bool = false,
        sampled: bool = false,
        color_attachment: bool = false,
        depth_stencil_attachment: bool = false,
        transient_attachment: bool = false,
        input_attachment: bool = false,
        _padding: u25 = 0,
    };
};
