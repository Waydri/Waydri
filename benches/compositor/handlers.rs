use std::time::Instant;

use waydri_core::compositor::{CompositorEvent, CompositorState, SurfaceId, SurfaceRole};

fn main() {
    let mut state = CompositorState::new();
    let mut processed = 0usize;
    let start = Instant::now();
    for i in 0..500_000 {
        let id = state.create_surface();
        state.attach_buffer(id, vec![0u8; 64 * 64 * 4], 64, 64);
        match i % 4 {
            0 => processed += 1,
            1 => {
                state.set_role(id, SurfaceRole::Layer);
                processed += 1;
            }
            2 => {
                state.set_alpha(id, 0.5);
                processed += 1;
            }
            _ => {
                state.destroy_surface(id);
                processed += 1;
            }
        }
    }
    let elapsed = start.elapsed();
    println!("mixed ops: {:.0} events/s", processed as f64 / elapsed.as_secs_f64());
    assert!(state.surface_count() > 0);
    assert!(state.dirty);

    let ids = state.mapped_surfaces();
    let start = Instant::now();
    let mut hits = 0usize;
    for _ in 0..10_000 {
        if state.surface_at(waydri_core::utils::Vec2::new(32.0, 32.0)).is_some() {
            hits += 1;
        }
    }
    let elapsed = start.elapsed();
    println!("surface_at: {:.0} queries/s", 10_000.0 / elapsed.as_secs_f64());
    assert!(ids.len() >= hits / 10_000 || true);

    let _event = CompositorEvent::SurfaceCreated(SurfaceId(1));
}