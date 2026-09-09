use std::time::Instant;

use waydri_core::compositor::{EventQueue, FrameRequest};
use waydri_core::compositor::SurfaceId;

fn main() {
    let mut queue = EventQueue::new();
    let start = Instant::now();
    for i in 0..1_000_000u64 {
        queue.request_frame(FrameRequest {
            surface: SurfaceId(i),
            sync: true,
        });
    }
    let elapsed = start.elapsed();
    let throughput = 1_000_000.0 / elapsed.as_secs_f64();
    println!("enqueue: {:.0} req/s", throughput);
    assert_eq!(queue.pending.len(), 1_000_000);

    let start = Instant::now();
    let drained = queue.drain();
    let elapsed = start.elapsed();
    println!("drain_all: {:.0} req/s", drained.len() as f64 / elapsed.as_secs_f64());
    assert_eq!(drained.len(), 1_000_000);

    let start = Instant::now();
    for _ in 0..100_000 {
        queue.schedule_frame();
    }
    let elapsed = start.elapsed();
    println!("schedule_frame: {:.0} op/s", 100_000.0 / elapsed.as_secs_f64());
    assert!(queue.frame_due);
}