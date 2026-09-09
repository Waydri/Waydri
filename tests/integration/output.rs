use waydri_core::output::{OutputManager, BackendContext};
use waydri_core::utils::Rect;

#[test]
fn multiple_headless_outputs() {
    let mut manager = OutputManager::new();
    manager.add_headless("primary".to_string(), 1920, 1080, 60);
    manager.add_headless("secondary".to_string(), 1280, 720, 60);
    assert_eq!(manager.count(), 2);
}

#[test]
fn focus_switches_between_outputs() {
    let mut manager = OutputManager::new();
    let first = manager.add_headless("a".to_string(), 800, 600, 60);
    let second = manager.add_headless("b".to_string(), 800, 600, 60);
    manager.set_focused(second);
    assert!(manager.output(first).is_some());
    assert_eq!(manager.clone_geometries().len(), 2);
}

#[test]
fn total_area_union_of_outputs() {
    let mut manager = OutputManager::new();
    manager.add_headless("a".to_string(), 100, 100, 60);
    manager.add_headless("b".to_string(), 100, 100, 60);
    let area = manager.total_area();
    assert!(area.width >= 100 && area.height >= 100);
}

#[test]
fn backend_suitable_modes_filtered() {
    let context = BackendContext::new(waydri_core::output::BackendKind::Headless);
    let modes = context.suitable_modes(800, 600);
    for mode in &modes {
        assert!(mode.width <= 800 && mode.height <= 600);
        assert!(!modes.is_empty());
    }
}

#[test]
fn remove_output_releases_slot() {
    let mut manager = OutputManager::new();
    let id = manager.add_headless("a".to_string(), 100, 100, 60);
    assert!(manager.remove_output(id));
    assert_eq!(manager.count(), 0);
}

#[test]
fn headless_output_geometry_matches() {
    let mut manager = OutputManager::new();
    manager.add_headless("a".to_string(), 640, 480, 60);
    let geo = manager.focused_geometry();
    assert_eq!(geo.width, 640);
    assert_eq!(geo.height, 480);
}

#[test]
fn rect_intersection_correct() {
    let a = Rect::new(0, 0, 100, 100);
    let b = Rect::new(50, 50, 100, 100);
    let intersection = a.intersection(&b).unwrap();
    assert_eq!((intersection.width, intersection.height), (50, 50));
}