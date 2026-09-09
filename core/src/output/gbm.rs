use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GbmBo {
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub stride: u32,
    pub fd: i32,
    pub modifier: u64,
}

impl GbmBo {
    pub fn size_bytes(&self) -> usize {
        self.height as usize * self.stride as usize
    }
}

pub const FORMAT_XRGB8888: u32 = 0x34325258;
pub const FORMAT_ARGB8888: u32 = 0x34325241;
pub const FORMAT_XBGR8888: u32 = 0x34324258;
pub const FORMAT_ABGR8888: u32 = 0x34324241;

#[derive(Debug, Default)]
pub struct BoPool {
    pub buffers: HashMap<u32, GbmBo>,
    next_id: u32,
}

impl BoPool {
    pub fn new() -> Self {
        BoPool {
            buffers: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn allocate(&mut self, width: u32, height: u32, format: u32, fd: i32) -> u32 {
        let stride = (width * 4).max(64);
        let id = self.next_id;
        self.next_id += 1;
        self.buffers.insert(
            id,
            GbmBo {
                width,
                height,
                format,
                stride,
                fd,
                modifier: 0,
            },
        );
        id
    }

    pub fn get(&self, id: u32) -> Option<&GbmBo> {
        self.buffers.get(&id)
    }

    pub fn release(&mut self, id: u32) -> bool {
        self.buffers.remove(&id).is_some()
    }

    pub fn check_format(format: u32) -> bool {
        matches!(
            format,
            FORMAT_XRGB8888 | FORMAT_ARGB8888 | FORMAT_XBGR8888 | FORMAT_ABGR8888
        )
    }
}

pub fn format_name(format: u32) -> String {
    let bytes = format.to_le_bytes();
    let printable: String = bytes
        .iter()
        .map(|b| {
            if b.is_ascii_graphic() {
                *b as char
            } else {
                '?'
            }
        })
        .collect();
    format!("0x{format:08x}({printable})")
}