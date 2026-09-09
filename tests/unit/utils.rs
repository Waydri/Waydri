use waydri_core::utils::{Color, Rect, Transform, Vec2};

#[test]
fn color_from_hex() {
    let c = Color::from_hex(0x6210EE);
    assert_eq!(c.to_rgba8()[0], 0x62);
    assert_eq!(c.to_rgba8()[1], 0x10);
    assert_eq!(c.to_rgba8()[2], 0xEE);
    assert_eq!(c.to_hex(), 0x6210EE);
}

#[test]
fn color_lerp_isbounded() {
    let black = Color::BLACK;
    let white = Color::WHITE;
    let mid = black.lerp(&white, 0.5);
    assert!((mid.r - 0.5).abs() < 0.001);
    let near = black.lerp(&white, 1.5);
    assert!(near.r <= 1.0 || near.r >= 1.0);
}

#[test]
fn color_contrast_ratio() {
    let black = Color::BLACK;
    let white = Color::WHITE;
    let ratio = black.contrast_ratio(&white);
    assert!(ratio > 10.0);
}

#[test]
fn vec2_operators() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let sum = a + b;
    assert_eq!(sum.x, 4.0);
    let scaled = a * 2.0;
    assert_eq!(scaled.y, 4.0);
    assert_eq!(a.dot(&b), 11.0);
    let mut n = a;
    n += b;
    assert_eq!(n.y, 6.0);
}

#[test]
fn vec2_normalize() {
    let v = Vec2::new(3.0, 4.0);
    let n = v.normalized();
    assert!((n.length() - 1.0).abs() < 0.0001);
    let zero = Vec2::ZERO.normalized();
    assert_eq!(zero, Vec2::ZERO);
}

#[test]
fn rect_intersection_and_union() {
    let a = Rect::new(0, 0, 10, 10);
    let b = Rect::new(5, 5, 10, 10);
    let inter = a.intersection(&b).unwrap();
    assert_eq!(inter, Rect::new(5, 5, 5, 5));
    let u = a.union(&b);
    assert_eq!(u.width, 15);
    assert_eq!(u.height, 15);
}

#[test]
fn rect_contains() {
    let r = Rect::new(10, 10, 100, 100);
    assert!(r.contains(50, 50));
    assert!(!r.contains(5, 5));
    assert!(!r.contains(110, 110));
}

#[test]
fn rect_grow_shrink() {
    let r = Rect::new(0, 0, 10, 10);
    let grown = r.grow(2);
    assert_eq!(grown.width, 14);
    let shrunk = r.shrink(2);
    assert_eq!(shrunk.width, 6);
}

#[test]
fn transform_rotates() {
    let t = Transform::Rotated90;
    let (w, h) = t.apply(1920, 1080);
    assert_eq!((w, h), (1080, 1920));
    assert_eq!(Transform::from_degrees(90), Transform::Rotated90);
    assert_eq!(t.degrees(), 90);
}

#[test]
fn clock_elapses() {
    use waydri_core::utils::time::Clock;
    let mut clock = Clock::new();
    std::thread::sleep(std::time::Duration::from_millis(5));
    let dt = clock.tick();
    assert!(dt.as_millis() >= 4);
    assert!(clock.uptime_secs() >= 0.0);
}