use waydri_core::wayland::wlr_layer_shell::{Layer, LayerSurface, ANCHOR_BOTTOM, ANCHOR_TOP};

#[test]
fn layer_surface_defaults() {
    let surface = LayerSurface::new(3, Layer::Top, "panel".to_string());
    assert_eq!(surface.layer, Layer::Top);
    assert_eq!(surface.namespace, "panel");
    assert_eq!(surface.exclusive_zone, 0);
}

#[test]
fn bottom_bar_anchors_bottom() {
    let mut surface = LayerSurface::new(1, Layer::Bottom, "bar".to_string());
    surface.set_anchors(ANCHOR_BOTTOM);
    assert!(surface.is_top(ANCHOR_BOTTOM));
    assert!(!surface.is_top(ANCHOR_TOP));
}

#[test]
fn layer_z_ordering() {
    assert!(Layer::Background.z_index() < Layer::Bottom.z_index());
    assert!(Layer::Bottom.z_index() < Layer::Top.z_index());
    assert!(Layer::Top.z_index() < Layer::Overlay.z_index());
}

#[test]
fn unknown_layer_maps_to_overlay() {
    assert_eq!(Layer::from_layer(99), Layer::Overlay);
}