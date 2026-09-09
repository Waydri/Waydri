use waydri_core::input::{Pointer, ScrollAxis, MotionSource};
use waydri_core::wayland::seat::Seat;
use waydri_core::utils::Vec2;

#[test]
fn relative_motion_converts_to_deltas() {
    let mut pointer = Pointer::new();
    pointer.move_by(3.0, 4.0);
    let motion = pointer.take_motion();
    assert!((motion.length() - 5.0).abs() < 0.001);
}

#[test]
fn seat_tracks_pointer_position() {
    let mut seat = Seat::new("seat0".to_string(), Vec::new());
    assert_eq!(seat.pointer_position, Vec2::ZERO);
    seat.pointer_position = Vec2::new(120.0, 80.0);
    assert_eq!(seat.pointer_position.x, 120.0);
}

#[test]
fn vertical_scroll_reports_amount() {
    let mut pointer = Pointer::new();
    pointer.scroll(ScrollAxis::Vertical, 1.5);
    assert_eq!(pointer.take_scroll_y(), 1.5);
    assert_eq!(pointer.take_scroll_x(), 0.0);
}

#[test]
fn relative_sources_distinguish() {
    assert!(!MotionSource::Pen.is_relative());
    assert!(!MotionSource::Touchscreen.is_relative());
    assert!(MotionSource::Touchpad.is_relative());
    assert!(MotionSource::Mouse.is_relative());
}

#[test]
fn serial_increments_monotonically() {
    let mut seat = Seat::new("seat0".to_string(), Vec::new());
    let first = seat.next_pointer_serial();
    let second = seat.next_pointer_serial();
    assert_eq!(second, first.wrapping_add(1));
}