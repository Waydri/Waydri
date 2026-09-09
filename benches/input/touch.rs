use std::time::Instant;

use waydri_core::input::{InputEvent, InputManager};
use waydri_core::utils::Vec2;

fn main() {
    let mut manager = InputManager::new();
    let start = Instant::now();
    for i in 0..300_000i32 {
        manager.push(InputEvent::Touch {
            id: 1,
            down: true,
            x: i as f32 % 500.0,
            y: 100.0,
        });
        manager.push(InputEvent::Touch {
            id: 1,
            down: false,
            x: i as f32 % 500.0,
            y: 100.0,
        });
        manager.dispatch();
    }
    let elapsed = start.elapsed();
    println!("touch pairs: {:.0}/s", 300_000.0 / elapsed.as_secs_f64());
    assert_eq!(manager.touch.active_count(), 0);
    assert!(manager.touch.centroid().is_none());

    let mut touch = waydri_core::input::Touch::new();
    let start = Instant::now();
    for i in 0..200_000i32 {
        let pos = Vec2::new((i % 200) as f32, 50.0);
        touch.down(1, pos, i as f64 / 1000.0);
        touch.move_point(1, pos + Vec2::new(2.0, 2.0));
        touch.up(1, i as f64 / 1000.0 + 0.01);
    }
    let elapsed = start.elapsed();
    println!("raw touch: {:.0} cycles/s", 200_000.0 / elapsed.as_secs_f64());
    println!("taps detected: {}", touch.tap_count);
}