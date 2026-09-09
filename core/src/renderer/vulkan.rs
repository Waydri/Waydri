use std::collections::HashMap;
use std::os::raw::{c_char, c_int, c_void};

use super::gles::c_str;

#[link(name = "dl")]
extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

const RTLD_NOW: c_int = 2;

type VkCreateInstance =
    extern "C" fn(*const c_void, *const c_void, *mut c_void) -> i32;
type VkDestroyInstance = extern "C" fn(*const c_void, *const c_void);
type VkEnumerateInstanceExtensionProperties =
    extern "C" fn(*const c_void, *mut u32, *mut c_void) -> i32;

#[derive(Debug, Default)]
pub struct VulkanInstance {
    pub handle: *mut c_void,
    pub extensions: Vec<String>,
    loaded: bool,
}

unsafe impl Send for VulkanInstance {}
unsafe impl Sync for VulkanInstance {}

impl VulkanInstance {
    pub fn try_create() -> Result<VulkanInstance, String> {
        let handle = unsafe { dlopen(c_str("libvulkan.so.1"), RTLD_NOW) };
        if handle.is_null() {
            return Err("libvulkan.so.1 not available".to_string());
        }
        let create_instance_addr = unsafe { dlsym(handle, c_str("vkCreateInstance")) };
        let destroy_addr = unsafe { dlsym(handle, c_str("vkDestroyInstance")) };
        let enumerate_addr =
            unsafe { dlsym(handle, c_str("vkEnumerateInstanceExtensionProperties")) };
        if create_instance_addr.is_null() || destroy_addr.is_null() {
            return Err("vulkan loader incomplete".to_string());
        }
        let create: VkCreateInstance = unsafe { std::mem::transmute(create_instance_addr) };
        let destroy: VkDestroyInstance = unsafe { std::mem::transmute(destroy_addr) };
        let enumerate: VkEnumerateInstanceExtensionProperties =
            unsafe { std::mem::transmute(enumerate_addr) };
        let extensions = enumerate_extensions(&enumerate);
        let mut instance: *mut c_void = std::ptr::null_mut();
        let create_info = build_instance_info(&extensions);
        let result = (create)(
            (&create_info as *const VkInstanceCreateInfo) as *const c_void,
            std::ptr::null(),
            (&mut instance as *mut *mut c_void) as *mut c_void,
        );
        if result != 0 {
            return Err(format!("vkCreateInstance failed with {result}"));
        }
        let _ = destroy;
        Ok(VulkanInstance {
            handle: instance,
            extensions,
            loaded: true,
        })
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded && !self.handle.is_null()
    }
}

fn enumerate_extensions(enumerate: &VkEnumerateInstanceExtensionProperties) -> Vec<String> {
    let mut count: u32 = 0;
    let raw = (enumerate)(std::ptr::null(), &mut count, std::ptr::null_mut());
    if raw != 0 || count == 0 {
        return Vec::new();
    }
    (0..count)
        .map(|i| format!("VK_KHR_extension_{i}"))
        .collect()
}

fn build_instance_info(extensions: &[String]) -> VkInstanceCreateInfo {
    let application_name = leak_cstr("Waydri");
    let engine_name = leak_cstr("waydri-core");
    let app_info = Box::leak(Box::new(VkApplicationInfo {
        s_type: 0,
        p_next: std::ptr::null(),
        p_application_name: application_name.as_ptr() as *const c_char,
        application_version: 1,
        p_engine_name: engine_name.as_ptr() as *const c_char,
        engine_version: 1,
        api_version: 0x00402000,
    }));
    let extension_names: Vec<*const c_char> = extensions
        .iter()
        .map(|e| leak_cstr(e).as_ptr() as *const c_char)
        .collect();
    let extension_names = Box::leak(Box::new(extension_names));
    VkInstanceCreateInfo {
        s_type: 1,
        p_next: std::ptr::null(),
        flags: 0,
        p_application_info: app_info,
        enabled_layer_count: 0,
        pp_enabled_layer_names: std::ptr::null(),
        enabled_extension_count: extensions.len() as u32,
        pp_enabled_extension_names: extension_names.as_ptr() as *const _,
    }
}

fn leak_cstr(s: &str) -> &'static [u8] {
    let mut out = s.as_bytes().to_vec();
    out.push(0);
    Box::leak(out.into_boxed_slice())
}

#[repr(C)]
struct VkApplicationInfo {
    s_type: u32,
    p_next: *const c_void,
    p_application_name: *const c_char,
    application_version: u32,
    p_engine_name: *const c_char,
    engine_version: u32,
    api_version: u32,
}

#[repr(C)]
struct VkInstanceCreateInfo {
    s_type: u32,
    p_next: *const c_void,
    flags: u32,
    p_application_info: *const VkApplicationInfo,
    enabled_layer_count: u32,
    pp_enabled_layer_names: *const c_void,
    enabled_extension_count: u32,
    pp_enabled_extension_names: *const c_void,
}

pub fn api_version_string(_instance: &VulkanInstance) -> String {
    let mut lookup = HashMap::new();
    lookup.insert("android".to_string(), 0x00400000u32);
    lookup
        .get(env!("WAYDRI_TARGET_OS"))
        .map(|v| format!("1.{}", v & 0xFFF))
        .unwrap_or_else(|| "1.0".to_string())
}