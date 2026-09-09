use waydri_core::effects::EffectsState;
use waydri_core::renderer::Frame;
use waydri_core::utils::Color;

#[test]
fn effects_state_defaults() {
    let effects = EffectsState::new();
    assert!(effects.blur.enabled);
    assert!(effects.shadow.enabled);
    assert!(effects.rounded.enabled);
}

#[test]
fn effects_apply_does_not_panic() {
    let effects = EffectsState::new();
    let mut frame = Frame::new(16, 16);
    frame.clear_with(Color::WHITE);
    effects.apply_all(&mut frame);
    let px = frame.get_pixel(2, 2);
    assert_eq!(px.len(), 4);
}

#[test]
fn blur_effect_toggle() {
    let mut effects = EffectsState::new();
    effects.blur.enabled = false;
    assert!(!effects.blur.enabled);
    effects.blur.enabled = true;
    assert!(effects.blur.enabled);
}

#[test]
fn opacity_step() {
    let mut effects = EffectsState::new();
    effects.opacity.target_alpha = 0.5;
    effects.opacity.step(0.01);
    assert!(effects.opacity.alpha >= 0.0);
}

#[test]
fn rounded_effect_radius() {
    let effects = EffectsState::new();
    assert_eq!(effects.rounded.radius, 8);
}

#[test]
fn shadow_spec_defaults() {
    use waydri_core::renderer::shadow::ShadowSpec;
    let spec = ShadowSpec::default();
    assert_eq!(spec.blur_radius, 12.0);
    assert!((spec.opacity - 0.6).abs() < 0.001);
}

#[test]
fn radius_is_positive() {
    let effects = EffectsState::new();
    assert!(effects.rounded.radius >= 0);
}