use std::time::Instant;

use waydri_core::input::{Button, MotionSource, Pointer, ScrollAxis};

fn main() {
    let mut pointer = Pointer::new();
    let start = Instant::now();
    for i in 0..1_000_000u32 {
        let x = (i % 1920) as f32;
        let y = (i % 1080) as f32;
        pointer.move_to(waydri_core::utils::Vec2::new(x, y));
        if i % 50 == 0 {
            pointer.press(Button::Left);
            pointer.release(Button::Left);
        }
        if i % 200 == 0 {
            pointer.scroll(ScrollAxis::Vertical, 1.0);
        }
    }
    let elapsed = start.elapsed();
    println!("pointer ops: {:.0}/s", 1_000_000.0 / elapsed.as_secs_f64());
    assert!(pointer.scroll_y > 0.0);
    assert!(!pointer.is_pressed(Button::Left));
    let motion = pointer.take_motion();
    assert_eq!(motion.length(), 0.0);

    let source = MotionSource::Mouse;
    assert!(source.is_relative());
}