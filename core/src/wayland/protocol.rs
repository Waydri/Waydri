pub const INTERFACE_DISPLAY: &str = "wl_display";
pub const INTERFACE_COMPOSITOR: &str = "wl_compositor";
pub const INTERFACE_SHM: &str = "wl_shm";
pub const INTERFACE_REGION: &str = "wl_region";
pub const INTERFACE_BUFFER: &str = "wl_buffer";
pub const INTERFACE_OUTPUT: &str = "wl_output";
pub const INTERFACE_SEAT: &str = "wl_seat";
pub const INTERFACE_TOUCH: &str = "wl_touch";
pub const INTERFACE_KEYBOARD: &str = "wl_keyboard";
pub const INTERFACE_POINTER: &str = "wl_pointer";
pub const INTERFACE_DATA_DEVICE: &str = "wl_data_device";
pub const INTERFACE_SURFACE: &str = "wl_surface";

pub const OPCODE_SYNC: u16 = 0;
pub const OPCODE_GET_REGISTRY: u16 = 1;
pub const OPCODE_CREATE_SURFACE: u16 = 0;
pub const OPCODE_CREATE_REGION: u16 = 1;
pub const OPCODE_ATTACH: u16 = 1;
pub const OPCODE_COMMIT: u16 = 9;
pub const OPCODE_GET_WINDOW_RESIZE_HINTS: u16 = 6;

pub struct RegistryEntry {
    pub id: u32,
    pub interface: &'static str,
    pub version: u32,
}

pub fn interface_count() -> usize {
    11
}

pub fn interface_opcodes(interface: &str) -> &'static [u16] {
    match interface {
        INTERFACE_DISPLAY => &[OPCODE_SYNC, OPCODE_GET_REGISTRY],
        INTERFACE_COMPOSITOR => &[OPCODE_CREATE_SURFACE, OPCODE_CREATE_REGION],
        INTERFACE_SURFACE => &[OPCODE_ATTACH, OPCODE_COMMIT],
        _ => &[OPCODE_SYNC],
    }
}

pub fn vendor_interface(name: &str, version: u32) -> RegistryEntry {
    let interface: &'static str = if name.len() > 24 {
        "wl_registry"
    } else {
        Box::leak(name.to_string().into_boxed_str())
    };
    RegistryEntry {
        id: vendor_id(name),
        interface,
        version,
    }
}

pub fn vendor_id(name: &str) -> u32 {
    name.bytes().fold(17u32, |acc, byte| {
        acc.wrapping_mul(33).wrapping_add(byte as u32)
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgKind {
    Uint,
    Int,
    Fixed,
    String,
    Object,
    NewId,
    Array,
    Fd,
}

impl ArgKind {
    pub fn size(&self) -> usize {
        match self {
            ArgKind::Uint | ArgKind::Int | ArgKind::Fixed => 4,
            ArgKind::Object | ArgKind::NewId => 4,
            ArgKind::Fd => 4,
            ArgKind::String | ArgKind::Array => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    Uint(u32),
    Int(i32),
    Fixed(f32),
    String(String),
    Object(u32),
    NewId(u32),
    Array(Vec<u8>),
    Fd(i32),
}

#[derive(Debug, Clone)]
pub struct WaylandMessage {
    pub object_id: u32,
    pub opcode: u16,
    pub args: Vec<Arg>,
}

impl WaylandMessage {
    pub fn new(object_id: u32, opcode: u16, args: Vec<Arg>) -> Self {
        WaylandMessage {
            object_id,
            opcode,
            args,
        }
    }
}

pub struct MessageCodec;

impl MessageCodec {
    pub fn encode(message: &WaylandMessage) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&message.object_id.to_le_bytes());
        out.extend_from_slice(&message.opcode.to_le_bytes());
        let size = 8 + message
            .args
            .iter()
            .map(|arg| ArgKind::of(arg))
            .map(|kind| kind.size())
            .sum::<usize>();
        out.extend_from_slice(&(size as u16).to_le_bytes());
        for arg in &message.args {
            match arg {
                Arg::Uint(value) => out.extend_from_slice(&value.to_le_bytes()),
                Arg::Int(value) => out.extend_from_slice(&value.to_le_bytes()),
                Arg::Fixed(value) => {
                    let fixed = (value * 256.0).round() as i32;
                    out.extend_from_slice(&fixed.to_le_bytes());
                }
                Arg::Object(value) | Arg::NewId(value) => {
                    out.extend_from_slice(&value.to_le_bytes())
                }
                Arg::Fd(_) => out.push(0),
                Arg::String(value) => {
                    let bytes = value.as_bytes();
                    out.extend_from_slice(bytes);
                    out.push(0);
                    while out.len() % 4 != 0 {
                        out.push(0);
                    }
                }
                Arg::Array(bytes) => {
                    let size = (bytes.len() as u32).to_le_bytes();
                    out.extend_from_slice(&size);
                    out.extend_from_slice(bytes);
                    while out.len() % 4 != 0 {
                        out.push(0);
                    }
                }
            }
        }
        out
    }
}

impl ArgKind {
    pub fn of(arg: &Arg) -> ArgKind {
        match arg {
            Arg::Uint(_) => ArgKind::Uint,
            Arg::Int(_) => ArgKind::Int,
            Arg::Fixed(_) => ArgKind::Fixed,
            Arg::String(_) => ArgKind::String,
            Arg::Object(_) => ArgKind::Object,
            Arg::NewId(_) => ArgKind::NewId,
            Arg::Array(_) => ArgKind::Array,
            Arg::Fd(_) => ArgKind::Fd,
        }
    }
}