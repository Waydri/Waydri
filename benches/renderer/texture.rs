use std::time::Instant;

use waydri_core::renderer::{SoftwareRenderer, Texture, TextureFormat};
use waydri_core::utils::Rect;

fn main() {
    let mut renderer = SoftwareRenderer::new(1280, 720);
    let mut tex = Texture::from_rgba(1, 64, 64, vec![200u8; 64 * 64 * 4]);
    tex.upload(&vec![90u8; 64 * 64 * 4]);
    renderer.textures = waydri_core::renderer::TexturePool::new();
    let id = renderer.textures.create(64, 64, TextureFormat::Rgba8);
    {
        let t = renderer.textures.get_mut(id).unwrap();
        *t = tex;
    }

    let start = Instant::now();
    for i in 0..100_000i32 {
        let x = (i % 1000) * 64;
        let y = ((i / 1000) % 10) * 64;
        renderer.draw_texture(id, Rect::new(x, y, 64, 64));
    }
    let elapsed = start.elapsed();
    println!("draw_texture: {:.0} ops/s", 100_000.0 / elapsed.as_secs_f64());

    let start = Instant::now();
    let pixels = vec![10u8; 32 * 32 * 4];
    for i in 0..50_000i32 {
        let x = (i % 1000) * 32;
        renderer.blit_surface(&pixels, 32, 32, Rect::new(x, 0, 32, 32), 0.7);
    }
    let elapsed = start.elapsed();
    println!("blit: {:.0} ops/s", 50_000.0 / elapsed.as_secs_f64());
}