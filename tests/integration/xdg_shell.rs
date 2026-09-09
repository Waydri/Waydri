use waydri_core::wayland::xdg_shell::{XdgSurface, XdgSurfaceKind, XdgToplevel};
use waydri_core::window::WindowId;

#[test]
fn toplevel_effective_min() {
    let mut toplevel = XdgToplevel::new(WindowId(1));
    toplevel.set_min_size(0, 0);
    assert_eq!(toplevel.effective_min(), (1, 1));
    toplevel.set_min_size(320, 240);
    assert_eq!(toplevel.effective_min(), (320, 240));
}

#[test]
fn toplevel_title_app_id() {
    let mut toplevel = XdgToplevel::new(WindowId(2));
    toplevel.set_title("Terminal".to_string());
    toplevel.set_app_id("org.waydri.terminal".to_string());
    assert_eq!(toplevel.title, "Terminal");
    assert_eq!(toplevel.app_id, "org.waydri.terminal");
}

#[test]
fn xdg_surface_kind_transition() {
    let mut surface = XdgSurface::new(10, WindowId(3));
    assert_eq!(surface.kind, XdgSurfaceKind::None);
    surface.set_toplevel();
    assert_eq!(surface.kind, XdgSurfaceKind::Toplevel);
    surface.set_popup();
    assert_eq!(surface.kind, XdgSurfaceKind::Popup);
}