use waydri_core::effects::Effect;
use waydri_core::renderer::framebuffer::Frame;

#[test]
fn apply_all_keeps_frame_bounds() {
    let mut frame = Frame::new(64, 64);
    for y in 0..64 {
        for x in 0..64 {
            frame.set_pixel(x, y, [40, 42, 54, 255]);
        }
    }
    let effects = waydri_core::effects::EffectsState::new();
    effects.apply_all(&mut frame);
    for y in 0..64 {
        for x in 0..64 {
            let pixel = frame.get_pixel(x, y);
            assert_eq!(pixel.len(), 4);
        }
    }
}

#[test]
fn blur_effect_darkens_neighbourhood() {
    let mut frame = Frame::new(16, 16);
    for y in 0..16 {
        for x in 0..16 {
            frame.set_pixel(x, y, [0, 0, 0, 255]);
        }
    }
    for y in 6..11 {
        for x in 6..11 {
            frame.set_pixel(x, y, [255, 255, 255, 255]);
        }
    }
    let blur = waydri_core::effects::blur::BlurEffect::default().with_radius(2.0);
    blur.apply(&mut frame);
    assert!(frame.get_pixel(8, 8)[0] > 128);
    assert!(frame.get_pixel(0, 0)[0] < 128);
}

#[test]
fn disabled_blur_leaves_frame_untouched() {
    let mut frame = Frame::new(8, 8);
    for y in 0..8 {
        for x in 0..8 {
            frame.set_pixel(x, y, [255, 255, 255, 255]);
        }
    }
    let mut blur = waydri_core::effects::blur::BlurEffect::default();
    blur.enabled = false;
    blur.apply(&mut frame);
    assert_eq!(frame.get_pixel(0, 0)[0], 255);
}

#[test]
fn shadow_spec_has_sane_defaults() {
    let shadow = waydri_core::effects::shadow::ShadowEffect::default();
    let spec = shadow.spec();
    assert!(spec.opacity > 0.0 && spec.opacity <= 1.0);
    assert!(spec.blur_radius > 0.0);
}

#[test]
fn opacity_effect_moves_towards_target() {
    let mut effect = waydri_core::effects::opacity::OpacityEffect {
        enabled: true,
        alpha: 1.0,
        target_alpha: 0.5,
    };
    effect.step(0.25);
    assert!(effect.alpha < 1.0);
    effect.step(1.0);
    assert_eq!(effect.alpha, 0.5);
}

#[test]
fn gradient_effect_applies() {
    let mut frame = Frame::new(16, 16);
    let gradient = waydri_core::effects::gradient::GradientEffect::default();
    gradient.apply(&mut frame);
    assert_eq!(frame.get_pixel(0, 0).len(), 4);
}

#[test]
fn rounded_effect_masks_corners() {
    let mut frame = Frame::new(16, 16);
    for y in 0..16 {
        for x in 0..16 {
            frame.set_pixel(x, y, [10, 10, 10, 255]);
        }
    }
    let mut rounded = waydri_core::effects::rounded::RoundedEffect::default();
    rounded.radius = 4;
    rounded.apply(&mut frame);
    assert!(frame.get_pixel(0, 0)[3] < 255);
}