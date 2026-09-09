use std::time::Instant;

use waydri_core::input::{Button, InputEvent, InputManager, MotionSource};
use waydri_core::input::keyboard::KEY_A;

fn main() {
    let mut manager = InputManager::new();
    let start = Instant::now();
    for i in 0..1_000_000u32 {
        manager.push(InputEvent::Key { code: KEY_A, down: i % 2 == 0 });
        if i % 10 == 0 {
            manager.push(InputEvent::Pointer {
                button: Some(Button::Left.code()),
                x: (i % 1920) as f32,
                y: (i % 1080) as f32,
                source: MotionSource::Mouse,
            });
        }
        manager.dispatch();
    }
    let elapsed = start.elapsed();
    println!("input dispatch: {:.0} events/s", 2_000_000.0 / elapsed.as_secs_f64());
    println!("pointer position: {:?}", manager.position());
    println!("key A pressed: {}", manager.keyboard.is_pressed(KEY_A));
    manager.clear();
    println!("cleared events: {}", manager.events.len());
}