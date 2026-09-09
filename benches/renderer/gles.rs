use std::time::Instant;

use waydri_core::renderer::{RendererKind, SoftwareRenderer, detect_backend};

fn main() {
    let start = Instant::now();
    let mut renderer = SoftwareRenderer::new(1920, 1080);
    let elapsed = start.elapsed();
    println!("gles setup: {:.3} ms", elapsed.as_secs_f64() * 1000.0);
    assert_eq!(renderer.frame.width, 1920);
    assert_eq!(renderer.frame.height, 1080);

    let backend = renderer.backend_status();
    assert!(!backend.is_empty());

    let start = Instant::now();
    for _ in 0..10_000 {
        renderer.clear();
    }
    let elapsed = start.elapsed();
    println!("gles clear: {:.1} fps", 10_000.0 / elapsed.as_secs_f64());

    let detected = detect_backend();
    assert!(matches!(detected, RendererKind::Vulkan | RendererKind::Gles | RendererKind::Software));
}