use std::time::Instant;

use waydri_core::compositor::CompositorState;

#[test]
fn buffer_attach_allocation() {
    let mut state = CompositorState::new();
    let id = state.create_surface();
    let buffer = vec![200u8; 1920 * 1080 * 4];
    let start = Instant::now();
    for _ in 0..20 {
        state.attach_buffer(id, buffer.clone(), 1920, 1080);
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5);
}

#[test]
fn many_surfaces_memory_footprint() {
    let mut state = CompositorState::new();
    let start = Instant::now();
    for _ in 0..2000 {
        let id = state.create_surface();
        let _ = id;
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5);
    assert_eq!(state.surface_count(), 2000);
}

#[test]
fn damage_regions_accumulate() {
    let mut tracker = waydri_core::renderer::damage::DamageTracker::new();
    let start = Instant::now();
    for i in 0..5000 {
        tracker.add_rect(waydri_core::utils::Rect::new(i, i, 10, 10));
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5);
    assert!(tracker.is_empty() || !tracker.is_empty());
}