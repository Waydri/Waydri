use std::time::Instant;

use waydri_core::renderer::{Texture, TextureFormat, TexturePool};

fn main() {
    let mut pool = TexturePool::new();
    let start = Instant::now();
    let mut ids = Vec::with_capacity(100_000);
    for _ in 0..100_000 {
        ids.push(pool.create(16, 16, TextureFormat::Rgba8));
    }
    let elapsed = start.elapsed();
    println!("texture alloc: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());
    assert_eq!(pool.count(), 100_000);

    let start = Instant::now();
    for id in ids {
        pool.destroy(id);
    }
    let elapsed = start.elapsed();
    println!("texture free: {:.0}/s", 100_000.0 / elapsed.as_secs_f64());
    assert_eq!(pool.count(), 0);

    let texture = Texture::from_rgba(1, 8, 8, vec![0u8; 8 * 8 * 4]);
    assert_eq!(texture.format, TextureFormat::Rgba8);
}