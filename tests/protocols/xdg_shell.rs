use waydri_core::wayland::xdg_shell::{XdgSurface, XdgSurfaceKind, XdgToplevel};
use waydri_core::window::WindowId;

#[test]
fn toplevel_tracks_title_state() {
    let mut toplevel = XdgToplevel::new(WindowId(1));
    toplevel.set_title("Browser".to_string());
    toplevel.set_app_id("org.mozilla.firefox".to_string());
    toplevel.set_min_size(640, 480);
    toplevel.set_max_size(4096, 2160);
    assert_eq!(toplevel.title, "Browser");
    assert_eq!(toplevel.effective_min(), (640, 480));
    assert_eq!(toplevel.max_size, (4096, 2160));
    assert!(!toplevel.activated);
    assert!(!toplevel.maximized);
    assert!(!toplevel.fullscreen);
}

#[test]
fn toplevel_zero_min_clamps_to_one() {
    let mut toplevel = XdgToplevel::new(WindowId(2));
    toplevel.set_min_size(0, 0);
    assert_eq!(toplevel.effective_min(), (1, 1));
}

#[test]
fn xdg_surface_uses_window_id() {
    let surface = XdgSurface::new(41, WindowId(9));
    assert_eq!(surface.surface, WindowId(9));
    assert_eq!(surface.kind, XdgSurfaceKind::None);
}

#[test]
fn xdg_surface_kind_transitions() {
    let mut surface = XdgSurface::new(1, WindowId(1));
    surface.set_toplevel();
    assert_eq!(surface.kind, XdgSurfaceKind::Toplevel);
    surface.set_popup();
    assert_eq!(surface.kind, XdgSurfaceKind::Popup);
}