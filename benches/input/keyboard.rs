use std::time::Instant;

use waydri_core::input::keyboard::{key_name, key_to_char};
use waydri_core::input::Keyboard;

fn main() {
    let mut keyboard = Keyboard::new();
    let start = Instant::now();
    for key in 0..100_000u32 {
        keyboard.press(key % 110);
        keyboard.release(key % 110);
    }
    let elapsed = start.elapsed();
    println!("press/release: {:.0} keys/s", 100_000.0 / elapsed.as_secs_f64());
    assert!(!keyboard.pressed.contains(&1));

    let start = Instant::now();
    let mut name = "";
    for key in 0..200_000u32 {
        name = key_name(key);
    }
    let elapsed = start.elapsed();
    println!("key_name: {:.0}/s", 200_000.0 / elapsed.as_secs_f64());
    assert!(!name.is_empty());

    let start = Instant::now();
    let mut chars = 0usize;
    for key in 0..100_000u32 {
        if key_to_char(key, true).is_some() {
            chars += 1;
        }
    }
    let elapsed = start.elapsed();
    println!("key_to_char: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());
    assert!(chars > 0);
}