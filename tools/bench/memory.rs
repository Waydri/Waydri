use std::time::Instant;

use waydri_core::renderer::{TextureFormat, TexturePool};

fn main() {
    let mut pool = TexturePool::new();
    let start = Instant::now();
    let mut ids = Vec::new();
    for _ in 0..200_000 {
        ids.push(pool.create(32, 32, TextureFormat::Rgba8));
    }
    let elapsed = start.elapsed();
    println!("texture alloc: {:.0} ops/s", 200_000.0 / elapsed.as_secs_f64());

    let payload = vec![0u8; 32 * 32 * 4];
    let start = Instant::now();
    for id in &ids {
        let texture = pool.get_mut(*id).unwrap();
        texture.upload(&payload);
    }
    let elapsed = start.elapsed();
    println!("texture upload: {:.0} ops/s", 200_000.0 / elapsed.as_secs_f64());

    let start = Instant::now();
    for id in ids {
        let _ = pool.destroy(id);
    }
    let elapsed = start.elapsed();
    println!("texture free: {:.0} ops/s", 200_000.0 / elapsed.as_secs_f64());
    println!("remaining textures: {}", pool.count());
}