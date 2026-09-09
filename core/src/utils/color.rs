#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b, a: 1.0 }
    }

    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
    pub const TRANSPARENT: Color = Color::new(0.0, 0.0, 0.0, 0.0);

    pub fn from_hex(hex: u32) -> Self {
        Color {
            r: ((hex >> 16) & 0xFF) as f32 / 255.0,
            g: ((hex >> 8) & 0xFF) as f32 / 255.0,
            b: (hex & 0xFF) as f32 / 255.0,
            a: 1.0,
        }
    }

    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    pub fn to_rgba8(&self) -> [u8; 4] {
        [
            (self.r.clamp(0.0, 1.0) * 255.0) as u8,
            (self.g.clamp(0.0, 1.0) * 255.0) as u8,
            (self.b.clamp(0.0, 1.0) * 255.0) as u8,
            (self.a.clamp(0.0, 1.0) * 255.0) as u8,
        ]
    }

    pub fn to_hex(&self) -> u32 {
        let [r, g, b, _] = self.to_rgba8();
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    pub fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        Color {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    pub fn mix(&self, other: &Color, weight: f32) -> Color {
        self.lerp(other, weight.clamp(0.0, 1.0))
    }

    pub fn lighten(&self, amount: f32) -> Color {
        Color {
            r: (self.r + amount).clamp(0.0, 1.0),
            g: (self.g + amount).clamp(0.0, 1.0),
            b: (self.b + amount).clamp(0.0, 1.0),
            a: self.a,
        }
    }

    pub fn darken(&self, amount: f32) -> Color {
        Color {
            r: (self.r - amount).clamp(0.0, 1.0),
            g: (self.g - amount).clamp(0.0, 1.0),
            b: (self.b - amount).clamp(0.0, 1.0),
            a: self.a,
        }
    }

    pub fn luminance(&self) -> f32 {
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }

    pub fn contrast_ratio(&self, other: &Color) -> f32 {
        let l1 = self.luminance() + 0.05;
        let l2 = other.luminance() + 0.05;
        if l1 > l2 {
            l1 / l2
        } else {
            l2 / l1
        }
    }
}

impl From<[f32; 4]> for Color {
    fn from(v: [f32; 4]) -> Self {
        Color::new(v[0], v[1], v[2], v[3])
    }
}

impl From<u32> for Color {
    fn from(hex: u32) -> Self {
        Color::from_hex(hex)
    }
}
