use std::time::Instant;

use waydri_core::renderer::vulkan::VulkanInstance;

fn main() {
    let start = Instant::now();
    match VulkanInstance::try_create() {
        Ok(instance) => {
            let loaded = instance.is_loaded();
            let elapsed = start.elapsed();
            println!("vulkan init: {:.3} ms loaded={}", elapsed.as_secs_f64() * 1000.0, loaded);
            assert!(loaded);
        }
        Err(error) => {
            let elapsed = start.elapsed();
            println!("vulkan init failed after {:.3} ms: {}", elapsed.as_secs_f64() * 1000.0, error);
        }
    }

    let start = Instant::now();
    for _ in 0..10_000 {
        let _ = "vkCreateInstance";
    }
    let elapsed = start.elapsed();
    println!("loader noop: {:.0}/s", 10_000.0 / elapsed.as_secs_f64());
}