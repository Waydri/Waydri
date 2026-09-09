use waydri_core::wayland::wlr_layer_shell::{Layer, LayerSurface, ANCHOR_BOTTOM, ANCHOR_LEFT, ANCHOR_RIGHT, ANCHOR_TOP};

#[test]
fn layer_surface_anchors_mask() {
    let mut surface = LayerSurface::new(1, Layer::Top, "panel".to_string());
    surface.set_anchors(ANCHOR_TOP | ANCHOR_LEFT | ANCHOR_RIGHT);
    assert!(surface.is_top(ANCHOR_TOP));
    assert!(surface.is_top(ANCHOR_LEFT));
    assert!(surface.is_top(ANCHOR_RIGHT));
    assert!(!surface.is_top(ANCHOR_BOTTOM));
}

#[test]
fn layers_map_from_integer() {
    assert_eq!(Layer::from_layer(0), Layer::Background);
    assert_eq!(Layer::from_layer(1), Layer::Bottom);
    assert_eq!(Layer::from_layer(2), Layer::Top);
    assert_eq!(Layer::from_layer(3), Layer::Overlay);
}

#[test]
fn layer_zs_are_ordered() {
    assert!(Layer::Background.z_index() < Layer::Bottom.z_index());
    assert!(Layer::Bottom.z_index() < Layer::Top.z_index());
    assert!(Layer::Top.z_index() < Layer::Overlay.z_index());
}

#[test]
fn anchor_names_readable() {
    assert_eq!(waydri_core::wayland::wlr_layer_shell::anchor_name(ANCHOR_TOP | ANCHOR_BOTTOM), "top bottom");
    assert_eq!(waydri_core::wayland::wlr_layer_shell::anchor_name(0), "none");
}

#[test]
fn layer_surface_fills() {
    let mut surface = LayerSurface::new(2, Layer::Bottom, "bar".to_string());
    surface.width = 800;
    surface.height = 30;
    surface.exclusive_zone = 30;
    assert_eq!(surface.width, 800);
    assert_eq!(surface.exclusive_zone, 30);
}