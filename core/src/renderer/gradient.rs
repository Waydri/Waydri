use crate::utils::Color;

#[derive(Debug, Clone)]
pub struct GradientStop {
    pub position: f32,
    pub color: Color,
}

impl GradientStop {
    pub fn new(position: f32, color: Color) -> Self {
        GradientStop {
            position: position.clamp(0.0, 1.0),
            color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinearGradient {
    pub stops: Vec<GradientStop>,
}

impl LinearGradient {
    pub fn new(stops: Vec<GradientStop>) -> Self {
        LinearGradient { stops }
    }

    pub fn two(from: Color, to: Color) -> Self {
        LinearGradient {
            stops: vec![GradientStop::new(0.0, from), GradientStop::new(1.0, to)],
        }
    }

    pub fn color_at(&self, position: f32) -> Color {
        let position = position.clamp(0.0, 1.0);
        if self.stops.is_empty() {
            return Color::TRANSPARENT;
        }
        if self.stops.len() == 1 {
            return self.stops[0].color;
        }
        let mut lower = &self.stops[0];
        let mut upper = &self.stops[self.stops.len() - 1];
        for stop in &self.stops {
            if stop.position <= position {
                lower = stop;
            }
            if stop.position > position {
                upper = stop;
                break;
            }
        }
        let span = (upper.position - lower.position).max(1e-6);
        let t = ((position - lower.position) / span).clamp(0.0, 1.0);
        lower.color.lerp(&upper.color, t)
    }
}

pub fn project(angle: f32, x: i64, y: i64, width: usize, height: usize) -> f32 {
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let dx = x as f32 - center_x;
    let dy = y as f32 - center_y;
    let radians = angle.to_radians();
    let projected = dx * radians.cos() + dy * radians.sin();
    let half_diag = (width as f32 * width as f32 + height as f32 * height as f32).sqrt() / 2.0;
    (projected / half_diag).clamp(-0.5, 0.5) + 0.5
}

pub fn fill_gradient(
    pixels: &mut [u8],
    width: usize,
    height: usize,
    gradient: &LinearGradient,
    angle: f32,
) {
    for y in 0..height as i64 {
        for x in 0..width as i64 {
            let position = project(angle, x, y, width, height);
            let color = gradient.color_at(position);
            let index = (y as usize * width + x as usize) * 4;
            let rgba = color.to_rgba8();
            pixels[index..index + 4].copy_from_slice(&rgba);
        }
    }
}

pub fn fill_gradient_rect(
    pixels: &mut [u8],
    width: usize,
    height: usize,
    rect: crate::utils::Rect,
    gradient: &LinearGradient,
    angle: f32,
) {
    let x0 = rect.x.clamp(0, width as i32 - 1) as i64;
    let y0 = rect.y.clamp(0, height as i32 - 1) as i64;
    let x1 = (rect.x + rect.width).clamp(x0 as i32 + 1, width as i32) as i64;
    let y1 = (rect.y + rect.height).clamp(y0 as i32 + 1, height as i32) as i64;
    for y in y0..y1 {
        for x in x0..x1 {
            let position = project(angle, x - x0, y - y0, (x1 - x0) as usize, (y1 - y0) as usize);
            let color = gradient.color_at(position);
            let index = (y as usize * width + x as usize) * 4;
            let rgba = color.to_rgba8();
            pixels[index..index + 4].copy_from_slice(&rgba);
        }
    }
}