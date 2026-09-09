use crate::utils::Color;

#[derive(Debug, Clone)]
pub struct Frame {
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Frame {
    pub fn new(width: usize, height: usize) -> Self {
        Frame {
            pixels: vec![0u8; width * height * 4],
            width,
            height,
        }
    }

    pub fn stride(&self) -> usize {
        self.width * 4
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn clear_with(&mut self, color: Color) {
        let rgba = color.to_rgba8();
        for chunk in self.pixels.chunks_mut(4) {
            chunk.copy_from_slice(&rgba);
        }
    }

    pub fn set_pixel(&mut self, x: i64, y: i64, rgba: [u8; 4]) {
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            return;
        }
        let index = (y as usize * self.width + x as usize) * 4;
        self.pixels[index..index + 4].copy_from_slice(&rgba);
    }

    pub fn get_pixel(&self, x: i64, y: i64) -> [u8; 4] {
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            return [0, 0, 0, 0];
        }
        let index = (y as usize * self.width + x as usize) * 4;
        self.pixels[index..index + 4].try_into().unwrap_or([0, 0, 0, 0])
    }

    pub fn blend_pixel(&mut self, x: i64, y: i64, rgba: [u8; 4]) {
        let src_a = rgba[3] as f32 / 255.0;
        if src_a <= 0.0 {
            return;
        }
        if src_a >= 1.0 {
            self.set_pixel(x, y, rgba);
            return;
        }
        let dst = self.get_pixel(x, y);
        let dst_a = dst[3] as f32 / 255.0;
        let out_a = src_a + dst_a * (1.0 - src_a);
        if out_a <= 0.0 {
            self.set_pixel(x, y, [0, 0, 0, 0]);
            return;
        }
        let mut out = [0u8; 4];
        for i in 0..3 {
            let s = rgba[i] as f32 / 255.0;
            let d = dst[i] as f32 / 255.0;
            out[i] = (((s * src_a + d * dst_a * (1.0 - src_a)) / out_a) * 255.0) as u8;
        }
        out[3] = (out_a * 255.0) as u8;
        self.set_pixel(x, y, out);
    }

    pub fn fill_rect(&mut self, x: i64, y: i64, width: usize, height: usize, rgba: [u8; 4]) {
        for dy in 0..height as i64 {
            for dx in 0..width as i64 {
                if x + dx >= 0 && y + dy >= 0 && x + dx < self.width as i64 && y + dy < self.height as i64 {
                    let index = ((y + dy) as usize * self.width + (x + dx) as usize) * 4;
                    self.pixels[index..index + 4].copy_from_slice(&rgba);
                }
            }
        }
    }

    pub fn blit(&mut self, src: &[u8], src_w: usize, src_h: usize, dst_x: i64, dst_y: i64) {
        for sy in 0..src_h as i64 {
            let dy = dst_y + sy;
            if dy < 0 || dy >= self.height as i64 {
                continue;
            }
            for sx in 0..src_w as i64 {
                let dx = dst_x + sx;
                if dx < 0 || dx >= self.width as i64 {
                    continue;
                }
                let si = (sy as usize * src_w + sx as usize) * 4;
                let rgba: [u8; 4] = src[si..si + 4].try_into().unwrap_or([0, 0, 0, 255]);
                let di = (dy as usize * self.width + dx as usize) * 4;
                self.pixels[di..di + 4].copy_from_slice(&rgba);
            }
        }
    }

    pub fn sub(&self, x: i64, y: i64, width: usize, height: usize) -> Option<Frame> {
        if x < 0 || y < 0 || x + width as i64 > self.width as i64 || y + height as i64 > self.height as i64 {
            return None;
        }
        let mut out = Frame::new(width, height);
        for dy in 0..height {
            let src_start = ((y as usize + dy) * self.width + x as usize) * 4;
            let dst_start = dy * width * 4;
            out.pixels[dst_start..dst_start + width * 4]
                .copy_from_slice(&self.pixels[src_start..src_start + width * 4]);
        }
        Some(out)
    }
}

#[derive(Debug)]
pub struct RenderRegion {
    pub x: i64,
    pub y: i64,
    pub width: usize,
    pub height: usize,
}

impl RenderRegion {
    pub fn full(width: usize, height: usize) -> Self {
        RenderRegion {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    pub fn intersects(&self, other: &RenderRegion) -> bool {
        self.x < other.x + other.width as i64
            && self.x + self.width as i64 > other.x
            && self.y < other.y + other.height as i64
            && self.y + self.height as i64 > other.y
    }
}