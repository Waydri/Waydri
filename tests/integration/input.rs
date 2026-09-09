use waydri_core::input::{Button, InputEvent, MotionSource, ScrollAxis};
use waydri_core::utils::Vec2;

#[test]
fn keyboard_modifiers_follow_press() {
    let mut keyboard = waydri_core::input::Keyboard::new();
    assert!(keyboard.press(waydri_core::input::keyboard::KEY_LEFT_SHIFT));
    assert!(keyboard.is_pressed(waydri_core::input::keyboard::KEY_LEFT_SHIFT));
}

#[test]
fn pointer_accumulates_motion() {
    let mut pointer = waydri_core::input::Pointer::new();
    pointer.move_by(10.0, 5.0);
    let motion = pointer.take_motion();
    assert!((motion.x - 10.0).abs() < 0.001);
    assert!((motion.y - 5.0).abs() < 0.001);
}

#[test]
fn scroll_y_accumulates() {
    let mut pointer = waydri_core::input::Pointer::new();
    pointer.scroll(ScrollAxis::Vertical, 3.0);
    let amount = pointer.take_scroll_y();
    assert!((amount - 3.0).abs() < 0.001);
}

#[test]
fn touch_pinch_reports_scale() {
    let mut touch = waydri_core::input::Touch::new();
    touch.down(1, Vec2::new(0.0, 0.0), 0.0);
    touch.down(2, Vec2::new(100.0, 0.0), 0.0);
    let initial = touch.average_start_distance();
    assert!(initial > 0.0);
}

#[test]
fn gesture_long_press_detected() {
    let mut recognizer = waydri_core::input::gesture::GestureRecognizer::new();
    let _down = recognizer.on_touch_down(Vec2::new(10.0, 10.0), 0.0, 1);
    let event = recognizer.on_touch_up(Vec2::new(11.0, 11.0), 1.0);
    assert_eq!(event.kind, waydri_core::input::GestureKind::LongPress);
}

#[test]
fn input_manager_routes_key_events() {
    let mut manager = waydri_core::input::InputManager::new();
    manager.push(InputEvent::Key { code: 30, down: true });
    manager.dispatch();
    assert!(manager.keyboard.is_pressed(30));
}

#[test]
fn motion_sources_report_relative() {
    assert!(MotionSource::Mouse.is_relative());
    assert!(!MotionSource::Touchscreen.is_relative());
}

#[test]
fn button_codes_round_trip() {
    assert_eq!(Button::Left.code(), 0x110);
    assert_eq!(Button::from_code(0x110), Some(Button::Left));
}