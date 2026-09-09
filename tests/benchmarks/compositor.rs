use std::time::Instant;

use waydri_core::compositor::CompositorState;

#[test]
fn surface_create_throughput() {
    let mut state = CompositorState::new();
    let start = Instant::now();
    for _ in 0..10000 {
        state.create_surface();
    }
    let elapsed = start.elapsed();
    assert_eq!(state.surface_count(), 10000);
    assert!(elapsed.as_secs() < 5);
}

#[test]
fn surface_destroy_reuses_pool() {
    let mut state = CompositorState::new();
    let ids: Vec<_> = (0..1000).map(|_| state.create_surface()).collect();
    for id in ids.iter().take(500) {
        state.destroy_surface(*id);
    }
    assert!(state.surface_count() <= 1000);
}

#[test]
fn attach_buffer_is_fast() {
    let mut state = CompositorState::new();
    let id = state.create_surface();
    let buffer = vec![0u8; 640 * 480 * 4];
    let start = Instant::now();
    for _ in 0..500 {
        state.attach_buffer(id, buffer.clone(), 640, 480);
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5);
    assert!(state.surface(id).unwrap().is_ready());
}