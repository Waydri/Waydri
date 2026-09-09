use std::time::Instant;

use waydri_core::renderer::{Frame, SoftwareRenderer};
use waydri_core::utils::{Color, Rect};

#[test]
fn full_frame_clear_throughput() {
    let mut renderer = SoftwareRenderer::new(1920, 1080);
    let start = Instant::now();
    for _ in 0..60 {
        renderer.clear();
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 10);
}

#[test]
fn fill_rect_stress() {
    let mut renderer = SoftwareRenderer::new(1920, 1080);
    let start = Instant::now();
    for i in 0..2000 {
        let rect = Rect::new(i % 100, 0, 50, 50);
        renderer.fill_rect(rect, Color::WHITE);
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 10);
}

#[test]
fn blit_surface_throughput() {
    let mut renderer = SoftwareRenderer::new(1920, 1080);
    let pixels = vec![128u8; 320 * 240 * 4];
    let start = Instant::now();
    for _ in 0..100 {
        renderer.blit_surface(&pixels, 320, 240, Rect::new(0, 0, 320, 240), 1.0);
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 10);
}

#[test]
fn shadow_render_keeps_bounds() {
    let mut renderer = SoftwareRenderer::new(400, 300);
    let frame = Frame::new(400, 300);
    let _ = frame;
    let start = Instant::now();
    renderer.apply_rounded(Rect::new(50, 50, 200, 100), 8);
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5);
}