use waydri_core::input::{
    Button, GestureKind, InputEvent, InputManager, Keyboard, MotionSource, Pointer, ScrollAxis,
    Touch,
};
use waydri_core::input::keyboard::{key_name, key_to_char};
use waydri_core::utils::Vec2;

#[test]
fn keyboard_press_release() {
    let mut kb = Keyboard::new();
    assert_eq!(kb.press(30), true);
    assert!(kb.is_pressed(30));
    assert_eq!(kb.release(30), true);
    assert!(!kb.is_pressed(30));
}

#[test]
fn keyboard_repeat_ignored() {
    let mut kb = Keyboard::new();
    kb.press(30);
    assert_eq!(kb.press(30), false, "duplicate press should report false");
}

#[test]
fn keyboard_char_mapping() {
    assert_eq!(key_to_char(30, false), Some('a'));
    assert_eq!(key_to_char(30, true), Some('A'));
    assert_eq!(key_to_char(57, false), Some(' '));
}

#[test]
fn key_names_exist() {
    for code in [1, 29, 42, 57, 30, 72, 105, 106, 103, 108, 59, 88] {
        assert!(!key_name(code).is_empty());
    }
}

#[test]
fn pointer_motion() {
    let mut ptr = Pointer::new();
    ptr.move_to(Vec2::new(10.0, 20.0));
    let motion = ptr.take_motion();
    assert_eq!(motion, Vec2::new(10.0, 20.0));
    assert_eq!(ptr.position, Vec2::new(10.0, 20.0));
}

#[test]
fn pointer_buttons() {
    let mut ptr = Pointer::new();
    assert_eq!(ptr.press(Button::Left), true);
    assert!(ptr.is_pressed(Button::Left));
    assert_eq!(ptr.release(Button::Left), true);
    assert!(!ptr.is_pressed(Button::Left));
    assert_eq!(Button::from_code(0x110), Some(Button::Left));
}

#[test]
fn pointer_scroll_accumulates() {
    let mut ptr = Pointer::new();
    ptr.scroll(ScrollAxis::Vertical, 3.0);
    ptr.scroll(ScrollAxis::Vertical, 2.0);
    assert_eq!(ptr.take_scroll_y(), 5.0);
}

#[test]
fn touch_down_up() {
    let mut touch = Touch::new();
    touch.down(1, Vec2::new(0.0, 0.0), 0.0);
    assert_eq!(touch.active_count(), 1);
    let point = touch.up(1, 0.05);
    assert_eq!(point.as_ref().map(|p| p.id), Some(1));
    assert_eq!(touch.active_count(), 0);
}

#[test]
fn touch_pinch_detection() {
    let mut touch = Touch::new();
    touch.down(1, Vec2::new(0.0, 0.0), 0.0);
    touch.down(2, Vec2::new(100.0, 0.0), 0.0);
    assert!(touch.pinch_ratio() > 0.0);
}

#[test]
fn gesture_double_tap() {
    let mut recognizer = waydri_core::input::gesture::GestureRecognizer::new();
    let first_down = recognizer.on_touch_down(Vec2::new(5.0, 5.0), 0.0, 1);
    let first_up = recognizer.on_touch_up(Vec2::new(5.0, 5.0), 0.05);
    let second_down = recognizer.on_touch_down(Vec2::new(5.0, 5.0), 0.1, 1);
    let second_up = recognizer.on_touch_up(Vec2::new(5.0, 5.0), 0.15);
    assert_eq!(first_down.kind, GestureKind::Pan);
    assert_eq!(first_up.kind, GestureKind::Tap);
    assert_eq!(second_down.kind, GestureKind::Pan);
    assert_eq!(second_up.kind, GestureKind::DoubleTap);
}

#[test]
fn input_manager_dispatches_and_clears() {
    let mut im = InputManager::new();
    im.push(InputEvent::Key { code: 30, down: true });
    im.push(InputEvent::Pointer { button: Some(Button::Right.code()), x: 5.0, y: 5.0, source: MotionSource::Mouse });
    im.push(InputEvent::Touch { id: 1, down: true, x: 5.0, y: 5.0 });
    im.dispatch();
    assert!(im.keyboard.is_pressed(30));
    assert!(im.pointer.is_pressed(Button::Right));
    assert_eq!(im.position(), Vec2::new(5.0, 5.0));
    im.clear();
    assert!(!im.keyboard.is_pressed(30));
}

#[test]
fn motion_source_relative() {
    assert!(MotionSource::Mouse.is_relative());
    assert!(MotionSource::Touchpad.is_relative());
    assert!(!MotionSource::Touchscreen.is_relative());
}