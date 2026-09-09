use std::os::raw::{c_char, c_int, c_void};

#[link(name = "dl")]
extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

const RTLD_NOW: c_int = 2;
const RTLD_GLOBAL: c_int = 0x100;

type EglGetProcAddress = extern "C" fn(*const c_char) -> *const c_void;
type EglGetError = extern "C" fn() -> u32;
type EglInitialize = extern "C" fn(*mut c_void, *mut c_int, *mut c_int) -> bool;
type EglBindApi = extern "C" fn(u32) -> bool;
type GlCreateShader = extern "C" fn(u32) -> u32;
type GlShaderSource = extern "C" fn(u32, usize, *const *const c_char, *const c_int);
type GlCompileShader = extern "C" fn(u32);
type GlAttachShader = extern "C" fn(u32, u32);
type GlLinkProgram = extern "C" fn(u32);
type GlUseProgram = extern "C" fn(u32);

#[derive(Debug)]
pub struct GlesContext {
    pub display: *mut c_void,
    egl_get_error: EglGetError,
    egl_initialize: EglInitialize,
    egl_bind_api: EglBindApi,
    proc_loaded: bool,
    gl_create_shader: Option<GlCreateShader>,
    gl_shader_source: Option<GlShaderSource>,
    gl_compile_shader: Option<GlCompileShader>,
    gl_attach_shader: Option<GlAttachShader>,
    gl_link_program: Option<GlLinkProgram>,
    gl_use_program: Option<GlUseProgram>,
    pub pending_error: u32,
}

unsafe impl Send for GlesContext {}
unsafe impl Sync for GlesContext {}

extern "C" fn null_proc(_name: *const c_char) -> *const c_void {
    std::ptr::null()
}

impl GlesContext {
    pub fn try_load() -> Result<GlesContext, String> {
        let library_flags = RTLD_NOW | RTLD_GLOBAL;
        let mut egl_proc: EglGetProcAddress = null_proc;
        let mut proc_loaded = false;
        for lib in ["libEGL.so", "libEGL.so.1", "libEGL.so.2"] {
            let handle = unsafe { dlopen(c_str(lib), library_flags) };
            if !handle.is_null() {
                let sym = unsafe { dlsym(handle, c_str("eglGetProcAddress")) };
                if !sym.is_null() {
                    egl_proc = unsafe { std::mem::transmute(sym) };
                    proc_loaded = true;
                    break;
                }
            }
        }
        let get_error_addr = load_symbol("libEGL.so", "eglGetError");
        let initialize_addr = load_symbol("libEGL.so", "eglInitialize");
        let bind_api_addr = load_symbol("libEGL.so", "eglBindAPI");
        let get_error = unsafe { std::mem::transmute::<*mut c_void, EglGetError>(get_error_addr) };
        let initialize = unsafe { std::mem::transmute::<*mut c_void, EglInitialize>(initialize_addr) };
        let bind_api = unsafe { std::mem::transmute::<*mut c_void, EglBindApi>(bind_api_addr) };
        let display = load_display(&initialize);
        let gl_create_shader = load_proc(egl_proc, "glCreateShader");
        let gl_shader_source = load_proc(egl_proc, "glShaderSource");
        let gl_compile_shader = load_proc(egl_proc, "glCompileShader");
        let gl_attach_shader = load_proc(egl_proc, "glAttachShader");
        let gl_link_program = load_proc(egl_proc, "glLinkProgram");
        let gl_use_program = load_proc(egl_proc, "glUseProgram");
        let error = (get_error)();
        Ok(GlesContext {
            display,
            egl_get_error: get_error,
            egl_initialize: initialize,
            egl_bind_api: bind_api,
            proc_loaded,
            gl_create_shader,
            gl_shader_source,
            gl_compile_shader,
            gl_attach_shader,
            gl_link_program,
            gl_use_program,
            pending_error: error,
        })
    }

    pub fn initialize(&mut self) -> Result<(), u32> {
        let major: *mut c_int = std::ptr::null_mut();
        let minor: *mut c_int = std::ptr::null_mut();
        let ok = (self.egl_initialize)(self.display, major, minor);
        if !ok {
            return Err(self.error());
        }
        (self.egl_bind_api)(0x30A0);
        Ok(())
    }

    pub fn error(&self) -> u32 {
        (self.egl_get_error)()
    }

    pub fn version(&self) -> &'static str {
        if self.proc_loaded {
            env!("WAYDRI_TARGET_ARCH")
        } else {
            "unavailable"
        }
    }

    pub fn compile_shader(&self, shader_type: u32, source: &str) -> Result<u32, u32> {
        let create = self.gl_create_shader.ok_or_else(|| self.pending_error)?;
        let source_fn = self.gl_shader_source.ok_or(0u32)?;
        let compile = self.gl_compile_shader.ok_or(0u32)?;
        let shader = (create)(shader_type);
        let c_source = c_str(source);
        let sources: Vec<*const c_char> = vec![c_source];
        let lengths: Vec<c_int> = vec![-1];
        (source_fn)(shader, 1, sources.as_ptr(), lengths.as_ptr());
        (compile)(shader);
        Ok(shader)
    }

    pub fn link_program(&self, program: u32, shaders: &[u32]) -> Result<(), u32> {
        let attach = self.gl_attach_shader.ok_or(0u32)?;
        let link = self.gl_link_program.ok_or(0u32)?;
        for shader in shaders {
            (attach)(program, *shader);
        }
        (link)(program);
        if let Some(use_program) = self.gl_use_program {
            (use_program)(program);
        }
        Ok(())
    }
}

pub(crate) fn c_str(s: &str) -> *const c_char {
    let bytes = s.as_bytes();
    let mut owned = bytes.to_vec();
    owned.push(0);
    owned.as_ptr() as *const c_char
}

fn load_symbol(library: &str, symbol: &str) -> *mut c_void {
    let handle = unsafe { dlopen(c_str(library), RTLD_NOW) };
    if handle.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { dlsym(handle, c_str(symbol)) }
}

fn load_proc<T>(proc_address: EglGetProcAddress, symbol: &str) -> Option<T> {
    let ptr = proc_address(c_str(symbol));
    if ptr.is_null() {
        None
    } else {
        let mut cell: T = unsafe { std::mem::zeroed() };
        unsafe {
            std::ptr::write(&mut cell as *mut T as *mut *const c_void, ptr);
        }
        Some(cell)
    }
}

fn load_display(initialize: &EglInitialize) -> *mut c_void {
    let display: *mut c_void = std::ptr::null_mut();
    let major: *mut c_int = std::ptr::null_mut();
    let minor: *mut c_int = std::ptr::null_mut();
    let _ = (initialize)(display, major, minor);
    display
}