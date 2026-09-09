use waydri_core::renderer::{
    Frame, RenderRegion, SoftwareRenderer, TextureFormat, TexturePool,
};
use waydri_core::renderer::damage::DamageTracker;
use waydri_core::utils::{Color, Rect};

#[test]
fn frame_clear_and_pixel() {
    let mut frame = Frame::new(4, 4);
    frame.clear_with(Color::from_hex(0xFF0000));
    assert_eq!(frame.get_pixel(0, 0), [255, 0, 0, 255]);
    frame.set_pixel(1, 1, [0, 255, 0, 255]);
    assert_eq!(frame.get_pixel(1, 1), [0, 255, 0, 255]);
}

#[test]
fn frame_blend_alpha() {
    let mut frame = Frame::new(4, 4);
    frame.clear();
    frame.blend_pixel(0, 0, [255, 0, 0, 128]);
    let px = frame.get_pixel(0, 0);
    assert!(px[0] > 100, "red channel should accumulate");
}

#[test]
fn frame_blit_and_sub() {
    let mut frame = Frame::new(10, 10);
    let src = vec![255u8; 4 * 4];
    frame.blit(&src, 2, 2, 3, 3);
    assert_eq!(frame.get_pixel(3, 3), [255, 255, 255, 255]);
    let sub = frame.sub(0, 0, 5, 5).unwrap();
    assert_eq!(sub.width, 5);
    assert_eq!(sub.height, 5);
}

#[test]
fn renderer_fill_rect() {
    let mut renderer = SoftwareRenderer::new(64, 64);
    renderer.fill_rect(Rect::new(0, 0, 64, 64), Color::WHITE);
    let frame = renderer.frame();
    assert_eq!(frame.get_pixel(32, 32), [255, 255, 255, 255]);
}

#[test]
fn renderer_blur_preserves_size() {
    let mut renderer = SoftwareRenderer::new(32, 32);
    let source = vec![200u8; 32 * 32 * 4];
    renderer.blur_region(&source, 32, 32, 0, 0, 32, 32, 3);
    let px = renderer.frame().get_pixel(16, 16);
    assert_eq!(px[0], 200);
}

#[test]
fn renderer_gradient_fills() {
    let mut renderer = SoftwareRenderer::new(16, 16);
    let stops = [
        waydri_core::renderer::gradient::GradientStop {
            position: 0.0,
            color: Color::BLACK,
        },
        waydri_core::renderer::gradient::GradientStop {
            position: 1.0,
            color: Color::WHITE,
        },
    ];
    renderer.fill_gradient(Rect::new(0, 0, 16, 16), &stops, 90.0);
    let top = renderer.frame().get_pixel(8, 0);
    let bottom = renderer.frame().get_pixel(8, 15);
    assert!(bottom[0] > top[0]);
}

#[test]
fn rounded_corners_alpha() {
    let mut renderer = SoftwareRenderer::new(16, 16);
    renderer.fill_rect(Rect::new(0, 0, 16, 16), Color::WHITE);
    renderer.apply_rounded(Rect::new(0, 0, 16, 16), 4);
    let corner = renderer.frame().get_pixel(0, 0);
    let center = renderer.frame().get_pixel(8, 8);
    assert!(corner[3] < center[3], "corner should gain transparency");
}

#[test]
fn texture_pool_lifecycle() {
    let mut pool = TexturePool::new();
    let id = pool.create(4, 4, TextureFormat::Rgba8);
    assert_eq!(pool.count(), 1);
    let tex = pool.get(id).unwrap();
    assert_eq!(tex.width, 4);
    assert_eq!(pool.destroy(id), true);
    assert_eq!(pool.count(), 0);
}

#[test]
fn damage_tracker_aggregates() {
    let mut tracker = DamageTracker::new();
    assert!(tracker.is_empty());
    tracker.add_rect(Rect::new(0, 0, 10, 10));
    tracker.add_rect(Rect::new(5, 5, 10, 10));
    assert_eq!(tracker.take_rects().len(), 2);
    tracker.add_rect(Rect::new(0, 0, 10, 10));
    tracker.add_rect(Rect::new(5, 5, 10, 10));
    let bounds = tracker.union(Rect::new(0, 0, 100, 100));
    assert_eq!((bounds.width, bounds.height), (15, 15));
    tracker.mark_full();
    assert!(tracker.take_rects().is_empty());
}

#[test]
fn render_region_intersects() {
    let a = RenderRegion { x: 0, y: 0, width: 10, height: 10 };
    let b = RenderRegion { x: 5, y: 5, width: 10, height: 10 };
    assert!(a.intersects(&b));
    assert_eq!(RenderRegion::full(8, 8).width, 8);
}