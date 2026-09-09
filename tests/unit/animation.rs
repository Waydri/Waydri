use std::time::Duration;

use waydri_core::animation::{Curve, Keyframe, Spring, Transition};
use waydri_core::animation::eased;

#[test]
fn curve_endpoints() {
    for curve in [
        Curve::Linear,
        Curve::EaseIn,
        Curve::EaseOut,
        Curve::EaseInOut,
        Curve::Bounce,
        Curve::Elastic,
    ] {
        let a = eased(0.0, curve);
        let b = eased(1.0, curve);
        assert!((a - 0.0).abs() < 0.01, "start invalid for {curve:?}");
        assert!((b - 1.0).abs() < 0.01, "end invalid for {curve:?}");
    }
}

#[test]
fn curve_bezier() {
    let curve = Curve::CubicBezier { p1: (0.2, 0.8), p2: (0.8, 0.2) };
    let mid = eased(0.5, curve);
    assert!(mid >= 0.0 && mid <= 1.0);
}

#[test]
fn keyframe_interpolation() {
    let start = Keyframe::new(Duration::ZERO, 0.0);
    let end = Keyframe::new(Duration::from_millis(100), 10.0);
    let half = start.interpolate(&end, Duration::from_millis(50));
    assert!(half > 3.0 && half < 7.0);
    let full = start.interpolate(&end, Duration::from_millis(100));
    assert!((full - 10.0).abs() < 0.01);
}

#[test]
fn spring_settles() {
    let spring = Spring::new(1.0, 100.0, 20.0);
    assert!(spring.is_critically_damped());
    let mut pos = 100.0;
    for _ in 0..400 {
        pos = spring.solve(0.05, pos);
    }
    assert!(pos.abs() < 1.0);
}

#[test]
fn spring_underdamped_oscillates() {
    let spring = Spring::under_damped();
    assert!(!spring.is_critically_damped());
    assert!(spring.duration_to_settle(0.001) > 0.0);
}

#[test]
fn transition_progress() {
    let mut t = Transition::new(0.0, 10.0, Duration::from_millis(100)).with_curve(Curve::Linear);
    t.tick(Duration::from_millis(50));
    assert!((t.progress() - 0.5).abs() < 0.01);
    assert!((t.current_value() - 5.0).abs() < 1.0);
    t.tick(Duration::from_millis(50));
    assert!(t.finished());
    assert!((t.current_value() - 10.0).abs() < 0.01);
}

#[test]
fn transition_pause_resume() {
    let mut t = Transition::new(0.0, 1.0, Duration::from_millis(100));
    t.tick(Duration::from_millis(50));
    t.pause();
    let before = t.progress();
    t.tick(Duration::from_millis(50));
    assert_eq!(before, t.progress(), "paused transition must not advance");
    t.resume();
    t.tick(Duration::from_millis(50));
    assert!(t.finished());
}

#[test]
fn transition_calls_on_finish() {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    let ticks = Arc::new(AtomicU32::new(0));
    let captured = Arc::clone(&ticks);
    let mut t = Transition::new(0.0, 1.0, Duration::from_millis(10));
    t = t.on_finish(move || {
        captured.fetch_add(1, Ordering::Relaxed);
    });
    t.tick(Duration::from_millis(20));
    drop(t);
    assert_eq!(ticks.load(Ordering::Relaxed), 1);
}