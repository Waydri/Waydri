use std::time::Instant;

use waydri_core::compositor::CompositorState;

fn main() {
    let mut state = CompositorState::new();
    let start = Instant::now();
    let mut ids = Vec::new();
    for _ in 0..100_000 {
        ids.push(state.create_surface());
    }
    let elapsed = start.elapsed();
    println!("create: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());
    assert_eq!(state.surface_count(), 100_000);

    let buffer = vec![255u8; 320 * 240 * 4];
    let start = Instant::now();
    for id in &ids {
        state.attach_buffer(*id, buffer.clone(), 320, 240);
    }
    let elapsed = start.elapsed();
    println!("attach: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());

    let start = Instant::now();
    for id in ids {
        state.destroy_surface(id);
    }
    let elapsed = start.elapsed();
    println!("destroy: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());
    assert_eq!(state.surface_count(), 0);
    assert!(!state.dirty);
}