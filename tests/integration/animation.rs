use std::time::Duration;

use waydri_core::animation::{Curve, Keyframe, Spring, Transition};

#[test]
fn spring_reaches_target() {
    let spring = Spring::new(1.0, 100.0, 10.0);
    let mut position = 1.0;
    for _ in 0..1000 {
        position = spring.solve(0.016, position);
    }
    assert!(position.abs() < 0.01);
}

#[test]
fn spring_critical_damping_positive() {
    let spring = Spring::new(1.0, 100.0, 20.0);
    assert!(spring.is_critically_damped());
}

#[test]
fn underdamped_spring_not_critical() {
    let spring = Spring::under_damped();
    assert!(!spring.is_critically_damped());
}

#[test]
fn critically_damped_spring_settles_fast() {
    let spring = Spring::new(1.0, 100.0, 20.0);
    assert!(spring.is_critically_damped());
    let fast = spring.duration_to_settle(0.001);
    let slow = Spring::under_damped().duration_to_settle(0.001);
    assert!(fast < slow);
}

#[test]
fn transition_multiple_ticks_complete() {
    let mut transition = Transition::new(0.0, 10.0, Duration::from_secs(1));
    for _ in 0..100 {
        transition.tick(Duration::from_millis(10));
    }
    assert!(transition.finished());
    assert!((transition.current_value() - 10.0).abs() < 0.5);
}

#[test]
fn keyframe_chain_interpolates() {
    let k0 = Keyframe::new(Duration::ZERO, 0.0);
    let k1 = Keyframe::new(Duration::from_millis(500), 1.0);
    let value = k0.with_curve(Curve::Linear).interpolate(&k1, Duration::from_millis(250));
    assert!((value - 0.5).abs() < 0.1);
}

#[test]
fn open_close_animation_progresses() {
    let mut animator = waydri_core::effects::animation::EffectAnimator::new();
    animator.open_window();
    animator.tick(Duration::from_millis(100));
    assert!(animator.is_animating());
    animator.tick(Duration::from_millis(400));
    assert!(!animator.is_animating());
}

#[test]
fn cubic_bezier_matches_expected_shape() {
    let curve = Curve::CubicBezier { p1: (0.25, 0.1), p2: (0.25, 1.0) };
    let start = curve.apply(0.0);
    let end = curve.apply(1.0);
    assert!((start - 0.0).abs() < 0.001);
    assert!((end - 1.0).abs() < 0.02);
    let middle = curve.apply(0.5);
    assert!(middle > 0.0 && middle < 1.0);
}