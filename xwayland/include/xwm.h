#ifndef WAYDRI_XWM_H
#define WAYDRI_XWM_H

#include <xcb/xcb.h>
#include <xcb/xfixes.h>
#include <xcb/composite.h>
#include <xcb/render.h>
#include <xcb/randr.h>
#include <xcb/shape.h>
#include <xcb/xcb_ewmh.h>
#include <wayland-server-core.h>

#define XWM_MAX_ATOMS 64
#define XWM_MAX_WINDOWS 256

struct xwm_window {
    xcb_window_t id;
    xcb_window_t frame_id;
    struct wl_surface *surface;
    struct wl_list link;
    int x, y;
    uint32_t width, height;
    bool mapped;
    bool fullscreen;
    bool maximized;
    bool floating;
    uint32_t events;
    char *title;
    char *app_id;
    uint32_t window_type;
    uint32_t wm_protocols[32];
    int wm_protocols_count;
    bool takes_focus;
};

struct xwm_atom {
    xcb_atom_t atom;
    const char *name;
};

struct xwm {
    xcb_connection_t *connection;
    xcb_screen_t *screen;
    struct xwayland *server;
    void *compositor;
    xcb_window_t grab_window;
    xcb_window_t cm_window;
    struct xwm_atom atoms[XWM_MAX_ATOMS];
    int atom_count;
    struct xwm_window windows[XWM_MAX_WINDOWS];
    int window_count;
    struct wl_list window_list;
    uint32_t fixes_version_major;
    uint32_t fixes_version_minor;
    bool has_composite;
    bool has_randr;
    bool has_render;
    bool has_shape;
    uint16_t screen_width;
    uint16_t screen_height;
    uint32_t last_configured_serial;
};

struct xwm *xwm_create(struct xwayland *server, void *compositor);
void xwm_destroy(struct xwm *xwm);
int xwm_handle_event(struct xwm *xwm);
void xwm_set_window_geometry(struct xwm *xwm, struct xwm_window *window, int x, int y, uint32_t w, uint32_t h);
void xwm_configure_window(struct xwm *xwm, xcb_window_t window, uint16_t mask, const uint32_t *values);
struct xwm_window *xwm_find_window(struct xwm *xwm, xcb_window_t id);
struct xwm_window *xwm_create_surface(struct xwm *xwm, xcb_window_t window_id);
void xwm_destroy_surface(struct xwm *xwm, struct xwm_window *window);
xcb_atom_t xwm_get_atom(struct xwm *xwm, const char *name);
void xwm_update_window_title(struct xwm *xwm, struct xwm_window *window);
void xwm_map_window(struct xwm *xwm, xcb_window_t window_id);
void xwm_unmap_window(struct xwm *xwm, xcb_window_t window_id);
void xwm_focus_window(struct xwm *xwm, struct xwm_window *window);
void xwm_send_focus_out(struct xwm *xwm, xcb_window_t window_id);
void xwm_raise_window(struct xwm *xwm, xcb_window_t window_id);

#endif
