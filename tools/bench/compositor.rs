use std::time::Instant;

use waydri_core::compositor::{CompositorState, SurfaceId};

fn main() {
    let mut state = CompositorState::new();
    let mut ids = Vec::new();
    let start = Instant::now();
    for _ in 0..250_000 {
        ids.push(state.create_surface());
    }
    let elapsed = start.elapsed();
    println!("create_surface: {:.0} ops/s", 250_000.0 / elapsed.as_secs_f64());

    let start = Instant::now();
    for id in &ids {
        state.attach_buffer(*id, vec![0u8; 480 * 270 * 4], 480, 270);
    }
    let elapsed = start.elapsed();
    println!("attach_buffer: {:.0} ops/s", 250_000.0 / elapsed.as_secs_f64());

    let start = Instant::now();
    for id in ids {
        let _ = state.destroy_surface(id);
    }
    let elapsed = start.elapsed();
    println!("destroy_surface: {:.0} ops/s", 250_000.0 / elapsed.as_secs_f64());

    println!("remaining surfaces: {}", state.surface_count());
    let probe = SurfaceId(1);
    let _ = probe;
}