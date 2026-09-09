use std::time::Instant;

use waydri_core::compositor::CompositorState;
use waydri_core::layout::LayoutManager;
use waydri_core::utils::Rect;
use waydri_core::window::{Window, WindowId};

fn main() {
    let start = Instant::now();
    let mut state = CompositorState::new();
    let mut manager = LayoutManager::with_default();
    let _ = manager.set_layout("master_stack");
    let elapsed = start.elapsed();
    println!("init core: {:.3} ms", elapsed.as_secs_f64() * 1000.0);

    let mut windows = Vec::new();
    for i in 1..=16u64 {
        windows.push(Window::new(WindowId(i), "bench".to_string(), "bench.app".to_string(), 1));
        state.attach_buffer(
            waydri_core::compositor::SurfaceId(i),
            vec![0u8; 640 * 480 * 4],
            640,
            480,
        );
    }

    let bounds = Rect::new(0, 0, 1920, 1080);
    let start = Instant::now();
    for _ in 0..10_000 {
        let ids: Vec<_> = windows.iter().map(|w| w.id).collect();
        let rects = manager.arrange("master_stack", bounds, &ids);
        assert_eq!(rects.len(), 16);
    }
    let elapsed = start.elapsed();
    println!("arrange: {:.0} layouts/s", 10_000.0 / elapsed.as_secs_f64());

    println!("mapped surfaces: {}", state.mapped_count());
    assert!(!windows.is_empty());
}