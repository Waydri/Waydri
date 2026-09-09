#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    Rgba8,
    Rgb8,
}

impl TextureFormat {
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            TextureFormat::Rgba8 => 4,
            TextureFormat::Rgb8 => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub id: u64,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn new(id: u64, width: u32, height: u32, format: TextureFormat) -> Self {
        Texture {
            id,
            width,
            height,
            format,
            data: vec![0u8; width as usize * height as usize * format.bytes_per_pixel()],
        }
    }

    pub fn from_rgba(id: u64, width: u32, height: u32, data: Vec<u8>) -> Self {
        Texture {
            id,
            width,
            height,
            format: TextureFormat::Rgba8,
            data,
        }
    }

    pub fn upload(&mut self, data: &[u8]) {
        let expected = self.width as usize * self.height as usize * self.format.bytes_per_pixel();
        if data.len() >= expected {
            self.data[..expected].copy_from_slice(&data[..expected]);
        }
    }
}

#[derive(Debug, Default)]
pub struct TexturePool {
    textures: std::collections::HashMap<u64, Texture>,
    next_id: u64,
}

impl TexturePool {
    pub fn new() -> Self {
        TexturePool {
            textures: std::collections::HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create(&mut self, width: u32, height: u32, format: TextureFormat) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.textures.insert(id, Texture::new(id, width, height, format));
        id
    }

    pub fn get(&self, id: u64) -> Option<&Texture> {
        self.textures.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Texture> {
        self.textures.get_mut(&id)
    }

    pub fn destroy(&mut self, id: u64) -> bool {
        self.textures.remove(&id).is_some()
    }

    pub fn count(&self) -> usize {
        self.textures.len()
    }
}