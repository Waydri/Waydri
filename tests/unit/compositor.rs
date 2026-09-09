use waydri_core::compositor::{CompositorState, SurfaceId};
use waydri_core::utils::Vec2;

#[test]
fn surface_lifecycle() {
    let mut state = CompositorState::new();
    let id: SurfaceId = state.create_surface();
    assert_eq!(state.surface_count(), 1);
    state.attach_buffer(id, vec![0u8; 100 * 100 * 4], 100, 100);
    assert!(state.surface(id).is_some());
    assert_eq!(state.mapped_count(), 1);
    assert_eq!(state.destroy_surface(id), true);
    assert_eq!(state.surface_count(), 0);
}

#[test]
fn surface_ids_unique() {
    let mut state = CompositorState::new();
    let a = state.create_surface();
    let b = state.create_surface();
    assert_ne!(a, b);
}

#[test]
fn surface_attach_sets_geometry() {
    let mut state = CompositorState::new();
    let id = state.create_surface();
    state.attach_buffer(id, vec![0u8; 200 * 100 * 4], 200, 100);
    let surface = state.surface(id).unwrap();
    assert_eq!(surface.width, 200);
    assert_eq!(surface.height, 100);
    assert!(surface.is_ready());
    assert_eq!(surface.damage_rect().width, 200);
}

#[test]
fn surface_at_hit_test() {
    let mut state = CompositorState::new();
    let id = state.create_surface();
    state.attach_buffer(id, vec![0u8; 100 * 100 * 4], 100, 100);
    let hit = state.surface_at(Vec2::new(50.0, 50.0));
    assert_eq!(hit, Some(id));
    assert_eq!(state.surface_at(Vec2::new(500.0, 500.0)), None);
}

#[test]
fn roles_tracking() {
    let mut state = CompositorState::new();
    let id = state.create_surface();
    state.set_role(id, waydri_core::compositor::SurfaceRole::Layer);
    let roles = state.roles();
    assert!(roles.contains(&(id, waydri_core::compositor::SurfaceRole::Layer)));
}

#[test]
fn handlers_add_remove() {
    use waydri_core::compositor::handlers::CompositorHandler;
    let mut state = CompositorState::new();
    let id = CompositorHandler::add_surface(&mut state);
    assert_eq!(state.surface_count(), 1);
    CompositorHandler::remove_surface(&mut state, id);
    assert_eq!(state.surface_count(), 0);
}

#[test]
fn clear_resets() {
    let mut state = CompositorState::new();
    state.create_surface();
    state.clear();
    assert_eq!(state.surface_count(), 0);
    assert_eq!(state.mapped_count(), 0);
}