use std::time::Instant;

use waydri_core::input::{InputManager, InputEvent};

#[test]
fn keyboard_dispatch_throughput() {
    let mut manager = InputManager::new();
    let start = Instant::now();
    for code in 0..5000u32 {
        manager.push(InputEvent::Key { code, down: true });
        manager.dispatch();
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5);
    assert!(manager.keyboard.is_pressed(4999));
}

#[test]
fn pointer_motion_throughput() {
    let mut manager = InputManager::new();
    let start = Instant::now();
    for i in 0..5000 {
        manager.push(InputEvent::Pointer {
            button: None,
            x: i as f32,
            y: i as f32,
            source: waydri_core::input::MotionSource::Mouse,
        });
        manager.dispatch();
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5);
    assert!(manager.position().x > 0.0);
}

#[test]
fn gesture_recognition_cost() {
    let mut manager = InputManager::new();
    let start = Instant::now();
    for i in 0..2000 {
        manager.push(InputEvent::Touch { id: 1, down: true, x: i as f32, y: 100.0 });
        manager.dispatch();
        manager.push(InputEvent::Touch { id: 1, down: false, x: i as f32, y: 100.0 });
        manager.dispatch();
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5);
}