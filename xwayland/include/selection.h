#ifndef WAYDRI_SELECTION_H
#define WAYDRI_SELECTION_H

#include <xcb/xcb.h>
#include <wayland-server-core.h>

#define SELECTION_MAX_OFFERS 32

struct selection_offer {
    xcb_atom_t target;
    char *mime_type;
    struct wl_list link;
};

struct selection_source {
    xcb_atom_t selection;
    char *mime_type;
    int fd;
    struct wl_list offers;
    struct wl_list link;
};

struct selection {
    struct xwayland *server;
    struct xwm *xwm;
    struct xwayland_seat *seat;
    uint32_t serial;
    xcb_atom_t clipboard_atom;
    xcb_atom_t primary_atom;
    xcb_atom_t atom_text_plain_utf8;
    xcb_atom_t atom_text_uri_list;
    xcb_atom_t atom_text_html;
    xcb_atom_t atom_image_png;
    xcb_atom_t atom_text_x_moz_url;
    struct selection_source *current_source;
    struct wl_list sources;
    struct wl_list offers;
    bool active;
};

struct selection *selection_create(struct xwayland *server, struct xwm *xwm, struct xwayland_seat *seat);
void selection_destroy(struct selection *selection);
int selection_handle_event(struct selection *selection, xcb_generic_event_t *event);
int selection_acquire(struct selection *selection, xcb_atom_t atom);
int selection_send(struct selection *selection, xcb_window_t requestor, xcb_atom_t target, xcb_atom_t property);
void selection_notify(struct selection *selection, xcb_window_t requestor, xcb_atom_t property);
void selection_add_offer(struct selection *selection, xcb_atom_t target, const char *mime_type);
void selection_clear(struct selection *selection);
bool selection_has_owner(struct selection *selection, xcb_atom_t atom);

#endif
