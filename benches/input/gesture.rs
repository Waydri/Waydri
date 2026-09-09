use std::time::Instant;

use waydri_core::input::gesture::GestureRecognizer;
use waydri_core::input::GestureKind;
use waydri_core::utils::Vec2;

fn main() {
    let mut recognizer = GestureRecognizer::new();
    let start = Instant::now();
    let mut taps = 0usize;
    for i in 0..100_000 {
        let t = i as f64 * 0.001;
        recognizer.on_touch_down(Vec2::new(10.0, 10.0), t, 1);
        let event = recognizer.on_touch_up(Vec2::new(12.0, 12.0), t + 0.05);
        if event.kind == GestureKind::Tap {
            taps += 1;
        }
    }
    let elapsed = start.elapsed();
    println!("tap cycles: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());
    assert!(taps > 0);

    let mut recognizer = GestureRecognizer::new();
    let start = Instant::now();
    let mut swipes = 0usize;
    for i in 0..100_000 {
        let t = i as f64 * 0.001;
        recognizer.on_touch_down(Vec2::new(0.0, 0.0), t, 1);
        let event = recognizer.on_touch_up(Vec2::new(400.0, 0.0), t + 0.2);
        if event.kind == GestureKind::Swipe {
            swipes += 1;
        }
    }
    let elapsed = start.elapsed();
    println!("swipe cycles: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());
    assert!(swipes > 0);
}