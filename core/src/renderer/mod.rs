use crate::utils::{Color, Rect};

pub mod blur;
pub mod damage;
pub mod framebuffer;
pub mod gles;
pub mod gradient;
pub mod opacity;
pub mod rounded;
pub mod shader;
pub mod shadow;
pub mod texture;
pub mod vulkan;

pub use framebuffer::{Frame, RenderRegion};
pub use texture::{Texture, TextureFormat, TexturePool};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RendererKind {
    Software,
    Gles,
    Vulkan,
}

impl RendererKind {
    pub fn name(&self) -> &'static str {
        match self {
            RendererKind::Software => "software",
            RendererKind::Gles => "gles",
            RendererKind::Vulkan => "vulkan",
        }
    }
}

#[derive(Debug)]
pub struct SoftwareRenderer {
    pub frame: Frame,
    pub kind: RendererKind,
    pub textures: TexturePool,
    pub clear_color: Color,
    pub gles: Option<gles::GlesContext>,
    pub vulkan: Option<vulkan::VulkanInstance>,
}

impl SoftwareRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        SoftwareRenderer {
            frame: Frame::new(width, height),
            kind: RendererKind::Software,
            textures: TexturePool::new(),
            clear_color: Color::from_hex(0x121212),
            gles: None,
            vulkan: None,
        }
    }

    pub fn new_with_fallback(width: usize, height: usize) -> Self {
        let mut renderer = SoftwareRenderer::new(width, height);
        match detect_backend() {
            RendererKind::Vulkan => {
                match vulkan::VulkanInstance::try_create() {
                    Ok(instance) => {
                        renderer.vulkan = Some(instance);
                        renderer.kind = RendererKind::Vulkan;
                    }
                    Err(_) => match gles::GlesContext::try_load() {
                        Ok(context) => {
                            renderer.gles = Some(context);
                            renderer.kind = RendererKind::Gles;
                        }
                        Err(_) => {}
                    },
                }
            }
            RendererKind::Gles => {
                match gles::GlesContext::try_load() {
                    Ok(context) => {
                        renderer.gles = Some(context);
                        renderer.kind = RendererKind::Gles;
                    }
                    Err(_) => {}
                }
            }
            RendererKind::Software => {}
        }
        renderer
    }

    pub fn backend_status(&self) -> String {
        match self.kind {
            RendererKind::Software => "software".to_string(),
            RendererKind::Gles => self
                .gles
                .as_ref()
                .map(|ctx| format!("gles({})", ctx.version()))
                .unwrap_or_else(|| "gles(uninitialized)".to_string()),
            RendererKind::Vulkan => {
                if self.vulkan.as_ref().map(|v| v.is_loaded()).unwrap_or(false) {
                    "vulkan".to_string()
                } else {
                    "vulkan(unavailable)".to_string()
                }
            }
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.frame = Frame::new(width, height);
    }

    pub fn frame(&self) -> &Frame {
        &self.frame
    }

    pub fn frame_mut(&mut self) -> &mut Frame {
        &mut self.frame
    }

    pub fn clear(&mut self) {
        self.frame.clear_with(self.clear_color);
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.frame.fill_rect(
            rect.x as i64,
            rect.y as i64,
            rect.width.max(0) as usize,
            rect.height.max(0) as usize,
            color.to_rgba8(),
        );
    }

    pub fn blend_rect(&mut self, rect: Rect, color: Color) {
        let rgba = color.to_rgba8();
        for y in rect.y..rect.y + rect.height.max(0) {
            for x in rect.x..rect.x + rect.width.max(0) {
                self.frame.blend_pixel(x as i64, y as i64, rgba);
            }
        }
    }

    pub fn blit_surface(
        &mut self,
        pixels: &[u8],
        width: usize,
        height: usize,
        dst: Rect,
        alpha: f32,
    ) {
        for sy in 0..height {
            let dy = dst.y + sy as i32;
            if dy < 0 || dy as usize >= self.frame.height {
                continue;
            }
            for sx in 0..width {
                let dx = dst.x + sx as i32;
                if dx < 0 || dx as usize >= self.frame.width {
                    continue;
                }
                let si = (sy * width + sx) * 4;
                let mut rgba: [u8; 4] = pixels[si..si + 4].try_into().unwrap_or([0, 0, 0, 255]);
                if alpha < 1.0 {
                    rgba[3] = (rgba[3] as f32 * alpha) as u8;
                }
                if dx >= 0 && dy >= 0 && dst.width > 0 && dst.height > 0 {
                    let (lx, ly) = (dx as i64, dy as i64);
                    let lx0 = dst.x as i64;
                    let ly0 = dst.y as i64;
                    if lx >= lx0 && ly >= ly0 && lx < lx0 + dst.width as i64 && ly < ly0 + dst.height as i64 {
                        self.frame.blend_pixel(lx, ly, rgba);
                    }
                }
            }
        }
    }

    pub fn draw_texture(&mut self, id: u64, dst: Rect) -> bool {
        let texture = match self.textures.get(id).cloned() {
            Some(texture) => texture,
            None => return false,
        };
        self.blit_surface(
            &texture.data,
            texture.width as usize,
            texture.height as usize,
            dst,
            1.0,
        );
        true
    }

    pub fn render_shadow(&mut self, spec: &shadow::ShadowSpec, target: Rect) {
        shadow::shadow_via_blur(&mut self.frame.pixels, self.frame.width, self.frame.height, spec, target);
    }

    pub fn apply_rounded(&mut self, rect: Rect, radius: usize) {
        if radius == 0 {
            return;
        }
        let width = rect.width.clamp(0, self.frame.width as i32) as usize;
        let height = rect.height.clamp(0, self.frame.height as i32) as usize;
        if width == 0 || height == 0 {
            return;
        }
        if let Some(mut region) = self.frame.sub(rect.x as i64, rect.y as i64, width, height) {
            rounded::apply_rounded_corners(&mut region.pixels, width, height, radius);
            self.frame.blit(&region.pixels, width, height, rect.x as i64, rect.y as i64);
        }
    }

    pub fn fill_gradient(&mut self, rect: Rect, stops: &[gradient::GradientStop], angle: f32) {
        if rect.width <= 0 || rect.height <= 0 {
            return;
        }
        let width = rect.width.clamp(0, self.frame.width as i32) as usize;
        let height = rect.height.clamp(0, self.frame.height as i32) as usize;
        if let Some(mut region) = self.frame.sub(rect.x as i64, rect.y as i64, width, height) {
            let gradient = gradient::LinearGradient::new(stops.to_vec());
            gradient::fill_gradient(&mut region.pixels, width, height, &gradient, angle);
            self.frame.blit(&region.pixels, width, height, rect.x as i64, rect.y as i64);
        }
    }

    pub fn blur_region(
        &mut self,
        source: &[u8],
        width: usize,
        height: usize,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        radius: usize,
    ) {
        let mut region = vec![0u8; w * h * 4];
        blur::blur_premultiplied_region(source, &mut region, width, height, x, y, w, h, radius);
        for sy in 0..h {
            for sx in 0..w {
                let dx = x + sx;
                let dy = y + sy;
                let si = (sy * w + sx) * 4;
                let rgba: [u8; 4] = region[si..si + 4].try_into().unwrap_or([0, 0, 0, 255]);
                self.frame.blend_pixel(dx as i64, dy as i64, rgba);
            }
        }
    }
}

pub fn detect_backend() -> RendererKind {
    if vulkan::VulkanInstance::try_create().is_ok() {
        return RendererKind::Vulkan;
    }
    if gles::GlesContext::try_load().is_ok() {
        return RendererKind::Gles;
    }
    RendererKind::Software
}

pub trait Renderer: Send {
    fn kind(&self) -> RendererKind;
    fn clear(&mut self);
    fn resize(&mut self, width: usize, height: usize);
}

impl Renderer for SoftwareRenderer {
    fn kind(&self) -> RendererKind {
        self.kind
    }

    fn clear(&mut self) {
        SoftwareRenderer::clear(self);
    }

    fn resize(&mut self, width: usize, height: usize) {
        SoftwareRenderer::resize(self, width, height);
    }
}