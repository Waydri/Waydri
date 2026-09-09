use std::time::Instant;

use waydri_core::renderer::{SoftwareRenderer, TextureFormat};
use waydri_core::utils::{Color, Rect};

fn main() {
    let mut renderer = SoftwareRenderer::new(1920, 1080);
    let start = Instant::now();
    for _ in 0..120 {
        renderer.clear();
    }
    let elapsed = start.elapsed();
    println!("clear: {:.1} fps", 120.0 / elapsed.as_secs_f64());

    let start = Instant::now();
    for _ in 0..5000 {
        renderer.fill_rect(Rect::new(0, 0, 320, 240), Color::WHITE);
    }
    let elapsed = start.elapsed();
    println!("fill_rect: {:.0} ops/s", 5000.0 / elapsed.as_secs_f64());

    let id = renderer.textures.create(128, 128, TextureFormat::Rgba8);
    {
        let texture = renderer.textures.get_mut(id).unwrap();
        texture.upload(&vec![10u8; 128 * 128 * 4]);
    }
    let start = Instant::now();
    for _ in 0..10_000 {
        let _ = renderer.draw_texture(id, Rect::new(0, 0, 128, 128));
    }
    let elapsed = start.elapsed();
    println!("draw_texture: {:.0} ops/s", 10_000.0 / elapsed.as_secs_f64());

    renderer.apply_rounded(Rect::new(100, 100, 400, 300), 12);
    println!("backend: {}", renderer.backend_status());
}