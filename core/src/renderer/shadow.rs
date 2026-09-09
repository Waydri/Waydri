use crate::utils::{Color, Rect};

#[derive(Debug, Clone, Copy)]
pub struct ShadowSpec {
    pub blur_radius: f32,
    pub opacity: f32,
    pub offset: (f32, f32),
    pub color: Color,
}

impl Default for ShadowSpec {
    fn default() -> Self {
        ShadowSpec {
            blur_radius: 12.0,
            opacity: 0.6,
            offset: (0.0, 4.0),
            color: Color::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}

impl ShadowSpec {
    pub fn shadow_bounds(&self, rect: Rect) -> Rect {
        let spread = self.blur_radius.ceil() as i32;
        rect.translate(self.offset.0 as i32, self.offset.1 as i32)
            .grow(spread)
    }
}

pub fn shadow_region(
    dst: &mut [u8],
    width: usize,
    height: usize,
    shadow: &ShadowSpec,
    target: Rect,
) {
    let bounds = shadow.shadow_bounds(target);
    let x0 = bounds.x.max(0) as usize;
    let y0 = bounds.y.max(0) as usize;
    let x1 = ((bounds.x + bounds.width) as usize).min(width);
    let y1 = ((bounds.y + bounds.height) as usize).min(height);
    for y in y0..y1 {
        for x in x0..x1 {
            let ix = x as i64;
            let iy = y as i64;
            let dx = (ix - shadow.offset.0 as i64 - target.x as i64).clamp(0, target.width as i64 - 1) as f32;
            let dy = (iy - shadow.offset.1 as i64 - target.y as i64).clamp(0, target.height as i64 - 1) as f32;
            let nx = dx / target.width.max(1) as f32 - 0.5;
            let ny = dy / target.height.max(1) as f32 - 0.5;
            let falloff = (nx * nx + ny * ny).sqrt() * 2.0;
            let strength = (1.0 - falloff).clamp(0.0, 1.0) * shadow.opacity;
            if strength <= 0.0 {
                continue;
            }
            let index = (y * width + x) * 4;
            let base = (strength * 255.0) as u8;
            let src = dst[index..index + 4].to_vec();
            let sr = src[0] as u8;
            dst[index] = sr;
            dst[index + 1] = ((src[1] as f32 + shadow.color.to_rgba8()[1] as f32 * strength) as u8).min(255);
            dst[index + 2] = ((src[2] as f32 + shadow.color.to_rgba8()[2] as f32 * strength) as u8).min(255);
            dst[index + 3] = base;
        }
    }
}

pub fn shadow_via_blur(
    dst: &mut [u8],
    width: usize,
    height: usize,
    shadow: &ShadowSpec,
    target: Rect,
) {
    let mut work = dst.to_vec();
    shadow_region(&mut work, width, height, shadow, target);
    let radius = shadow.blur_radius as usize;
    super::blur::blur(&mut work, width, height, radius.min(16));
    let x0 = target.x.max(0) as usize;
    let y0 = target.y.max(0) as usize;
    for y in y0..target.height.max(0) as usize + y0 {
        if y >= height {
            break;
        }
        for x in x0..target.width.max(0) as usize + x0 {
            if x >= width {
                break;
            }
            let index = (y * width + x) * 4;
            let src_alpha = work[index + 3];
            let dst_alpha = dst[index + 3];
            if src_alpha <= 0 {
                continue;
            }
            let src_r = work[index];
            let src_g = work[index + 1];
            let src_b = work[index + 2];
            let src_a = src_alpha as f32 / 255.0;
            let out_a = src_a + dst_alpha as f32 / 255.0 * (1.0 - src_a);
            if out_a <= 0.0 {
                dst[index + 3] = 0;
                continue;
            }
            let dst_a = dst_alpha as f32 / 255.0;
            dst[index] = (((src_r as f32 * src_a + dst[index] as f32 * dst_a * (1.0 - src_a)) / out_a) * 255.0) as u8;
            dst[index + 1] = (((src_g as f32 * src_a + dst[index + 1] as f32 * dst_a * (1.0 - src_a)) / out_a) * 255.0) as u8;
            dst[index + 2] = (((src_b as f32 * src_a + dst[index + 2] as f32 * dst_a * (1.0 - src_a)) / out_a) * 255.0) as u8;
            dst[index + 3] = (out_a * 255.0) as u8;
        }
    }
}