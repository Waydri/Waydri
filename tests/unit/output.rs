use waydri_core::output::{OutputManager, OutputMode};
use waydri_core::renderer::{SoftwareRenderer, RendererKind};
use waydri_core::utils::{Rect, Transform};
use waydri_core::window::WindowManager;

#[test]
fn headless_output_created() {
    let mut outputs = OutputManager::new();
    let id = outputs.add_headless("default".into(), 1920, 1080, 120);
    assert_eq!(outputs.count(), 1);
    let output = outputs.output(id).unwrap();
    assert_eq!(output.name, "default");
    assert_eq!(output.current_mode.width, 1920);
    assert_eq!(output.current_mode.height, 1080);
}

#[test]
fn output_mode_refresh_hz() {
    let mode = OutputMode::new(1920, 1080, 120);
    assert_eq!(mode.refresh_hz(), 120);
    assert_eq!(mode.width, 1920);
}

#[test]
fn focused_geometry() {
    let mut outputs = OutputManager::new();
    outputs.add_headless("a".into(), 800, 600, 60);
    outputs.add_headless("b".into(), 1280, 720, 60);
    assert_eq!(outputs.focused_geometry(), Rect::new(0, 0, 800, 600));
    assert_eq!(outputs.total_area().width, 1280);
}

#[test]
fn output_set_transform() {
    let mut outputs = OutputManager::new();
    let id = outputs.add_headless("a".into(), 1920, 1080, 60);
    outputs.set_transform(id, Transform::Rotated90);
    assert_eq!(outputs.output(id).unwrap().transform, Transform::Rotated90);
}

#[test]
fn output_contains_point() {
    let output = waydri_core::output::Output::headless(1, "test".into(), 100, 50, 60);
    assert!(output.contains(50, 25));
    assert!(!output.contains(200, 25));
    assert_eq!(output.position(), (0, 0));
}

#[test]
fn output_removal() {
    let mut outputs = OutputManager::new();
    let id = outputs.add_headless("a".into(), 800, 600, 60);
    assert_eq!(outputs.remove_output(id), true);
    assert_eq!(outputs.count(), 0);
}

#[test]
fn compose_runs_against_framework() {
    let mut outputs = OutputManager::new();
    outputs.add_headless("big".into(), 640, 480, 60);
    let mut renderer = SoftwareRenderer::new(640, 480);
    let compositor = waydri_core::compositor::CompositorState::new();
    let mut windows = WindowManager::new();
    let id = windows.create_window("T".into(), "app".into(), 0);
    windows.set_placement(id, Rect::new(10, 10, 100, 100));
    outputs.compose(&mut renderer, &compositor, &windows);
    assert_eq!(renderer.kind, RendererKind::Software);
}

#[test]
fn set_mode_updates() {
    let mut outputs = OutputManager::new();
    let id = outputs.add_headless("a".into(), 800, 600, 60);
    outputs.set_mode(id, OutputMode::new(1024, 768, 60));
    assert_eq!(outputs.output(id).unwrap().current_mode.width, 1024);
}

#[test]
fn clone_geometries_matches_count() {
    let mut outputs = OutputManager::new();
    outputs.add_headless("a".into(), 640, 480, 60);
    outputs.add_headless("b".into(), 640, 480, 60);
    assert_eq!(outputs.clone_geometries().len(), 2);
}