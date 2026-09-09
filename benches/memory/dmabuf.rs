use std::time::Instant;

use waydri_core::renderer::{TextureFormat, TexturePool};

fn main() {
    let mut pool = TexturePool::new();
    let id = pool.create(256, 256, TextureFormat::Rgba8);
    let payload = vec![128u8; 256 * 256 * 4];

    let start = Instant::now();
    for _ in 0..10_000 {
        let texture = pool.get_mut(id).unwrap();
        texture.upload(&payload);
    }
    let elapsed = start.elapsed();
    println!("texture upload: {:.0} MB/s", 10_000.0 * 256.0 * 256.0 * 4.0 / 1024.0 / 1024.0 / elapsed.as_secs_f64());

    let start = Instant::now();
    let mut total = 0usize;
    for _ in 0..1_000_000 {
        let texture = pool.get(id).unwrap();
        total += texture.data.len();
    }
    let elapsed = start.elapsed();
    println!("texture read: {:.0} ops/s", 1_000_000.0 / elapsed.as_secs_f64());
    assert_eq!(total, 1_000_000 * 256 * 256 * 4);
    assert_eq!(pool.count(), 1);

    let start = Instant::now();
    for _ in 0..100_000 {
        pool.create(1, 1, TextureFormat::Rgb8);
    }
    let elapsed = start.elapsed();
    println!("rgb8 alloc: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());
    assert_eq!(pool.count(), 100_001);
}