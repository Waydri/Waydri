use std::time::Instant;

use waydri_core::compositor::CompositorState;

fn main() {
    let mut state = CompositorState::new();
    let id = state.create_surface();
    let buffer = vec![64u8; 1920 * 1080 * 4];
    let start = Instant::now();
    for _ in 0..60 {
        state.attach_buffer(id, buffer.clone(), 1920, 1080);
    }
    let elapsed = start.elapsed();
    println!("1080p attach: {:.3} ms/frame", elapsed.as_secs_f64() * 1000.0 / 60.0);
    assert!(state.surface(id).unwrap().is_ready());
    assert!(state.dirty);

    let small = vec![0u8; 64 * 64 * 4];
    let start = Instant::now();
    for _ in 0..100_000 {
        state.attach_buffer(id, small.clone(), 64, 64);
    }
    let elapsed = start.elapsed();
    println!("64px attach: {:.0} allocs/s", 100_000.0 / elapsed.as_secs_f64());
    assert_eq!(state.surface(id).unwrap().serial, 100_061);
}