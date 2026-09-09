#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <wayland-server-core.h>
#include "xwayland.h"
#include "xwm.h"

static const char *known_atom_names[] = {
    "WM_DELETE_WINDOW",
    "WM_PROTOCOLS",
    "WM_NAME",
    "WM_CLASS",
    "WM_STATE",
    "WM_HINTS",
    "WM_NORMAL_HINTS",
    "WM_SIZE_HINTS",
    "_NET_WM_NAME",
    "_NET_WM_CLASS",
    "_NET_WM_STATE",
    "_NET_WM_STATE_FULLSCREEN",
    "_NET_WM_STATE_MAXIMIZED_VERT",
    "_NET_WM_STATE_MAXIMIZED_HORZ",
    "_NET_WM_STATE_MODAL",
    "_NET_WM_STATE_ABOVE",
    "_NET_WM_STATE_BELOW",
    "_NET_WM_WINDOW_TYPE",
    "_NET_WM_WINDOW_TYPE_DIALOG",
    "_NET_WM_WINDOW_TYPE_SPLASH",
    "_NET_WM_WINDOW_TYPE_DOCK",
    "_NET_WM_WINDOW_TYPE_TOOLBAR",
    "_NET_WM_WINDOW_TYPE_UTILITY",
    "_NET_WM_WINDOW_TYPE_MENU",
    "_NET_WM_WINDOW_TYPE_POPUP_MENU",
    "_NET_WM_PID",
    "_NET_WM_USER_TIME",
    "_NET_WM_DESKTOP",
    "_NET_ACTIVE_WINDOW",
    "_NET_WM_MOVERESIZE",
    "_NET_SUPPORTING_WM_CHECK",
    "_NET_WM_SUPPORTING_WM_CHECK",
    "_NET_SUPPORTED",
    "UTF8_STRING",
    "text/plain",
    "text/plain;charset=utf-8",
    "text/uri-list",
    "text/html",
    "image/png",
    "text/x-moz-url",
    "CLIPBOARD",
    "PRIMARY",
    "TARGETS",
    "XdndAware",
    "XdndEnter",
    "XdndLeave",
    "XdndPosition",
    "XdndDrop",
    "XdndStatus",
    "XdndFinished",
    "XdndActionCopy",
    "XdndActionMove",
    "XdndActionAsk",
    "XdndActionPrivate",
    "XdndSelection",
    "XdndTypeList",
    "COMPOUND_TEXT",
    "_NET_WM_OPAQUE_REGION",
    "_NET_WM_SYNC_REQUEST",
    "_NET_WM_SYNC_REQUEST_COUNTER",
    "WL_SURFACE_ID",
    "_MOTIF_WM_HINTS",
    NULL
};

static void xwm_init_atoms(struct xwm *xwm) {
    xcb_intern_atom_cookie_t cookies[sizeof(known_atom_names) / sizeof(known_atom_names[0])];
    xcb_intern_atom_reply_t *reply;
    int count = 0;

    while (known_atom_names[count] != NULL) {
        cookies[count] = xcb_intern_atom(xwm->connection, 0,
                                         strlen(known_atom_names[count]),
                                         known_atom_names[count]);
        count++;
    }

    for (int i = 0; i < count; i++) {
        reply = xcb_intern_atom_reply(xwm->connection, cookies[i], NULL);
        if (reply) {
            xwm->atoms[xwm->atom_count].atom = reply->atom;
            xwm->atoms[xwm->atom_count].name = known_atom_names[i];
            xwm->atom_count++;
            free(reply);
        }
    }
}

static struct xwm_window *xwm_window_new(struct xwm *xwm, xcb_window_t id) {
    if (xwm->window_count >= XWM_MAX_WINDOWS) {
        return NULL;
    }
    struct xwm_window *win = &xwm->windows[xwm->window_count];
    memset(win, 0, sizeof(struct xwm_window));
    win->id = id;
    win->frame_id = 0;
    win->surface = NULL;
    win->mapped = false;
    win->fullscreen = false;
    win->maximized = false;
    win->floating = false;
    win->title = NULL;
    win->app_id = NULL;
    win->window_type = 0;
    win->wm_protocols_count = 0;
    win->takes_focus = true;
    xwm->window_count++;
    wl_list_insert(&xwm->window_list, &win->link);
    return win;
}

static void xwm_handle_create_notify(struct xwm *xwm, xcb_create_notify_event_t *event) {
    if (event->parent == xwm->screen->root) {
        struct xwm_window *win = xwm_find_window(xwm, event->window);
        if (!win) {
            win = xwm_window_new(xwm, event->window);
            if (win) {
                win->x = event->x;
                win->y = event->y;
                win->width = event->width;
                win->height = event->height;
            }
        }
    }
}

static void xwm_handle_map_request(struct xwm *xwm, xcb_map_request_event_t *event) {
    struct xwm_window *win = xwm_find_window(xwm, event->window);
    if (!win) {
        win = xwm_window_new(xwm, event->window);
        if (!win) {
            return;
        }
    }

    if (win->wm_protocols_count > 0) {
        for (int i = 0; i < win->wm_protocols_count; i++) {
            xcb_atom_t proto = win->wm_protocols[i];
            xcb_atom_t delete_atom = xwm_get_atom(xwm, "WM_DELETE_WINDOW");
            if (proto == delete_atom) {
                win->takes_focus = true;
            }
        }
    }

    xwm_update_window_title(xwm, win);
    xwm_map_window(xwm, event->window);
}

static void xwm_handle_unmap_notify(struct xwm *xwm, xcb_unmap_notify_event_t *event) {
    xwm_unmap_window(xwm, event->window);
}

static void xwm_handle_configure_request(struct xwm *xwm, xcb_configure_request_event_t *event) {
    uint16_t mask = event->value_mask;
    uint32_t values[7];
    int idx = 0;

    if (mask & XCB_CONFIG_WINDOW_X) {
        values[idx++] = event->x;
    }
    if (mask & XCB_CONFIG_WINDOW_Y) {
        values[idx++] = event->y;
    }
    if (mask & XCB_CONFIG_WINDOW_WIDTH) {
        values[idx++] = event->width;
    }
    if (mask & XCB_CONFIG_WINDOW_HEIGHT) {
        values[idx++] = event->height;
    }
    if (mask & XCB_CONFIG_WINDOW_BORDER_WIDTH) {
        values[idx++] = event->border_width;
    }
    if (mask & XCB_CONFIG_WINDOW_SIBLING) {
        values[idx++] = event->sibling;
    }
    if (mask & XCB_CONFIG_WINDOW_STACK_MODE) {
        values[idx++] = event->stack_mode;
    }

    xwm_configure_window(xwm, event->window, mask, values);

    xcb_configure_notify_event_t ce;
    memset(&ce, 0, sizeof(ce));
    ce.response_type = XCB_CONFIGURE_NOTIFY;
    ce.event = event->window;
    ce.window = event->window;
    ce.x = event->x;
    ce.y = event->y;
    ce.width = event->width;
    ce.height = event->height;
    ce.border_width = event->border_width;
    ce.above_sibling = XCB_NONE;
    ce.override_redirect = false;

    xcb_send_event(xwm->connection, false, event->window,
                   XCB_EVENT_MASK_STRUCTURE_NOTIFY, (const char *)&ce);
    xcb_flush(xwm->connection);
}

static void xwm_handle_property_notify(struct xwm *xwm, xcb_property_notify_event_t *event) {
    struct xwm_window *win = xwm_find_window(xwm, event->window);
    if (!win) {
        return;
    }

    xcb_atom_t wm_name = xwm_get_atom(xwm, "WM_NAME");
    xcb_atom_t net_wm_name = xwm_get_atom(xwm, "_NET_WM_NAME");
    xcb_atom_t wm_class = xwm_get_atom(xwm, "WM_CLASS");
    xcb_atom_t net_wm_state = xwm_get_atom(xwm, "_NET_WM_STATE");
    xcb_atom_t net_wm_type = xwm_get_atom(xwm, "_NET_WM_WINDOW_TYPE");
    xcb_atom_t net_wm_pid = xwm_get_atom(xwm, "_NET_WM_PID");
    xcb_atom_t wm_protocols = xwm_get_atom(xwm, "WM_PROTOCOLS");
    xcb_atom_t net_wm_state_fs = xwm_get_atom(xwm, "_NET_WM_STATE_FULLSCREEN");
    xcb_atom_t net_wm_state_max_v = xwm_get_atom(xwm, "_NET_WM_STATE_MAXIMIZED_VERT");
    xcb_atom_t net_wm_state_max_h = xwm_get_atom(xwm, "_NET_WM_STATE_MAXIMIZED_HORZ");
    xcb_atom_t wl_surface_id = xwm_get_atom(xwm, "WL_SURFACE_ID");

    if (event->atom == net_wm_name || event->atom == wm_name) {
        xwm_update_window_title(xwm, win);
    } else if (event->atom == wm_class) {
        xcb_get_property_cookie_t cookie;
        xcb_get_property_reply_t *reply;

        cookie = xcb_get_property(xwm->connection, 0, win->id,
                                  wm_class, XCB_ATOM_STRING, 0, 512);
        reply = xcb_get_property_reply(xwm->connection, cookie, NULL);
        if (reply && xcb_get_property_value_length(reply) > 0) {
            char *class_str = xcb_get_property_value(reply);
            int len = xcb_get_property_value_length(reply);
            free(win->app_id);
            win->app_id = strndup(class_str, len);
        }
        free(reply);
    } else if (event->atom == net_wm_state) {
        xcb_get_property_cookie_t cookie;
        xcb_get_property_reply_t *reply;

        cookie = xcb_get_property(xwm->connection, 0, win->id,
                                  net_wm_state, XCB_ATOM_ATOM, 0, 32);
        reply = xcb_get_property_reply(xwm->connection, cookie, NULL);
        if (reply && xcb_get_property_value_length(reply) > 0) {
            xcb_atom_t *states = xcb_get_property_value(reply);
            int count = xcb_get_property_value_length(reply) / sizeof(xcb_atom_t);
            win->fullscreen = false;
            win->maximized = false;
            for (int i = 0; i < count; i++) {
                if (states[i] == net_wm_state_fs) {
                    win->fullscreen = true;
                } else if (states[i] == net_wm_state_max_v || states[i] == net_wm_state_max_h) {
                    win->maximized = true;
                }
            }
        }
        free(reply);
    } else if (event->atom == net_wm_type) {
        xcb_get_property_cookie_t cookie;
        xcb_get_property_reply_t *reply;

        cookie = xcb_get_property(xwm->connection, 0, win->id,
                                  net_wm_type, XCB_ATOM_ATOM, 0, 32);
        reply = xcb_get_property_reply(xwm->connection, cookie, NULL);
        if (reply && xcb_get_property_value_length(reply) > 0) {
            xcb_atom_t *types = xcb_get_property_value(reply);
            win->window_type = types[0];
        }
        free(reply);
    } else if (event->atom == wm_protocols) {
        xcb_get_property_cookie_t cookie;
        xcb_get_property_reply_t *reply;

        cookie = xcb_get_property(xwm->connection, 0, win->id,
                                  wm_protocols, XCB_ATOM_ATOM, 0, 32);
        reply = xcb_get_property_reply(xwm->connection, cookie, NULL);
        if (reply && xcb_get_property_value_length(reply) > 0) {
            xcb_atom_t *protos = xcb_get_property_value(reply);
            int count = xcb_get_property_value_length(reply) / sizeof(xcb_atom_t);
            win->wm_protocols_count = count < 32 ? count : 32;
            memcpy(win->wm_protocols, protos, win->wm_protocols_count * sizeof(xcb_atom_t));
        }
        free(reply);
    } else if (event->atom == wl_surface_id) {
        xcb_get_property_cookie_t cookie;
        xcb_get_property_reply_t *reply;

        cookie = xcb_get_property(xwm->connection, 0, win->id,
                                  wl_surface_id, XCB_ATOM_INTEGER, 0, 1);
        reply = xcb_get_property_reply(xwm->connection, cookie, NULL);
        if (reply && xcb_get_property_value_length(reply) >= (int)sizeof(uint32_t)) {
            uint32_t *surface_id = xcb_get_property_value(reply);
            win->surface = wl_compositor_create_surface(
                wl_display_get_compositor(xwm->server->wl_display));
        }
        free(reply);
    }
}

static void xwm_handle_client_message(struct xwm *xwm, xcb_client_message_event_t *event) {
    struct xwm_window *win = xwm_find_window(xwm, event->window);
    if (!win) {
        return;
    }

    xcb_atom_t net_wm_state = xwm_get_atom(xwm, "_NET_WM_STATE");
    xcb_atom_t net_active_window = xwm_get_atom(xwm, "_NET_ACTIVE_WINDOW");
    xcb_atom_t net_wm_moveresize = xwm_get_atom(xwm, "_NET_WM_MOVERESIZE");
    xcb_atom_t net_wm_state_fs = xwm_get_atom(xwm, "_NET_WM_STATE_FULLSCREEN");

    if (event->type == net_wm_state) {
        if (event->data.data32[1] == net_wm_state_fs) {
            if (event->data.data32[0] == 1) {
                win->fullscreen = true;
            } else if (event->data.data32[0] == 0) {
                win->fullscreen = false;
            } else {
                win->fullscreen = !win->fullscreen;
            }
        }
    } else if (event->type == net_active_window) {
        xwm_focus_window(xwm, win);
    } else if (event->type == net_wm_moveresize) {
        int detail = event->data.data32[2];
        if (detail == 8 || detail == 9) {
            win->floating = !win->floating;
        }
    }
}

static void xwm_handle_key_event(struct xwm *xwm, xcb_key_press_event_t *event) {
    xcb_keysym_t keysym;
    xcb_key_symbols_t *syms;

    syms = xcb_key_symbols_alloc(xwm->connection);
    if (!syms) {
        return;
    }

    keysym = xcb_key_symbols_get_keysym(syms, event->detail, 0);
    xcb_key_symbols_free(syms);

    xcb_atom_t wm_protocols = xwm_get_atom(xwm, "WM_PROTOCOLS");
    xcb_atom_t net_wm_sync = xwm_get_atom(xwm, "_NET_WM_SYNC_REQUEST");

    for (int i = 0; i < xwm->window_count; i++) {
        struct xwm_window *win = &xwm->windows[i];
        if (!win->mapped) {
            continue;
        }
        if (win->takes_focus) {
            xwm_focus_window(xwm, win);
            break;
        }
    }
}

static void xwm_handle_button_event(struct xwm *xwm, xcb_button_press_event_t *event) {
    xcb_atom_t net_active = xwm_get_atom(xwm, "_NET_ACTIVE_WINDOW");
    struct xwm_window *win = xwm_find_window(xwm, event->event);
    if (win) {
        xwm_focus_window(xwm, win);
    }

    xcb_allow_events(xwm->connection, XCB_ALLOW_REPLAY_POINTER, event->time);
    xcb_flush(xwm->connection);
}

static void xwm_handle_motion_notify(struct xwm *xwm, xcb_motion_notify_event_t *event) {
    struct xwm_window *win = xwm_find_window(xwm, event->event);
    if (!win) {
        return;
    }
    xcb_flush(xwm->connection);
}

struct xwm *xwm_create(struct xwayland *server, void *compositor) {
    struct xwm *xwm;
    xcb_screen_iterator_t iter;
    int screen_num;
    xcb_generic_error_t *error;

    xwm = calloc(1, sizeof(struct xwm));
    if (!xwm) {
        return NULL;
    }

    xwm->server = server;
    xwm->compositor = compositor;
    xwm->atom_count = 0;
    xwm->window_count = 0;
    xwm->has_composite = false;
    xwm->has_randr = false;
    xwm->has_render = false;
    xwm->has_shape = false;
    xwm->last_configured_serial = 0;

    wl_list_init(&xwm->window_list);

    const char *display_env = xwayland_get_display(server);
    int display_num = 0;
    if (display_env && display_env[0] == ':') {
        display_num = atoi(display_env + 1);
    }

    xwm->connection = xcb_connect_to_display_with_auth_info(
        display_env, NULL, NULL, &screen_num);

    if (xcb_connection_has_error(xwm->connection)) {
        free(xwm);
        return NULL;
    }

    iter = xcb_setup_roots_iterator(xcb_get_setup(xwm->connection));
    xwm->screen = iter.data;
    xwm->screen_width = xwm->screen->width_in_pixels;
    xwm->screen_height = xwm->screen->height_in_pixels;

    xwm->grab_window = xcb_generate_id(xwm->connection);
    xcb_create_window(xwm->connection,
                      xwm->screen->root_depth,
                      xwm->grab_window,
                      xwm->screen->root,
                      0, 0, 1, 1, 0,
                      XCB_WINDOW_CLASS_INPUT_ONLY,
                      xwm->screen->root_visual, 0, NULL);

    uint32_t grab_values[] = {XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT |
                               XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY |
                               XCB_EVENT_MASK_PROPERTY_CHANGE};
    xcb_change_window_attributes(xwm->connection,
                                  xwm->screen->root,
                                  XCB_CW_EVENT_MASK,
                                  grab_values);

    xwm_init_atoms(xwm);

    const xcb_query_extension_reply_t *ext;

    ext = xcb_get_extension_data(xwm->connection, &xcb_composite_id);
    if (ext && ext->present) {
        xwm->has_composite = true;
        xcb_composite_query_version_cookie_t ver_cookie;
        xcb_composite_query_version_reply_t *ver_reply;
        ver_cookie = xcb_composite_query_version(xwm->connection, 0, 4);
        ver_reply = xcb_composite_query_version_reply(xwm->connection, ver_cookie, &error);
        free(ver_reply);
    }

    ext = xcb_get_extension_data(xwm->connection, &xcb_randr_id);
    if (ext && ext->present) {
        xwm->has_randr = true;
        xcb_randr_query_version_cookie_t ver_cookie;
        xcb_randr_query_version_reply_t *ver_reply;
        ver_cookie = xcb_randr_query_version(xwm->connection, 1, 5);
        ver_reply = xcb_randr_query_version_reply(xwm->connection, ver_cookie, &error);
        free(ver_reply);
    }

    ext = xcb_get_extension_data(xwm->connection, &xcb_render_id);
    if (ext && ext->present) {
        xwm->has_render = true;
    }

    ext = xcb_get_extension_data(xwm->connection, &xcb_xfixes_id);
    if (ext && ext->present) {
        xcb_xfixes_query_version_cookie_t fix_cookie;
        xcb_xfixes_query_version_reply_t *fix_reply;
        fix_cookie = xcb_xfixes_query_version(xwm->connection, 4, 0);
        fix_reply = xcb_xfixes_query_version_reply(xwm->connection, fix_cookie, &error);
        if (fix_reply) {
            xwm->fixes_version_major = fix_reply->major_version;
            xwm->fixes_version_minor = fix_reply->minor_version;
            free(fix_reply);
        }
    }

    ext = xcb_get_extension_data(xwm->connection, &xcb_shape_id);
    if (ext && ext->present) {
        xwm->has_shape = true;
    }

    xcb_atom_t supported[] = {
        xwm_get_atom(xwm, "_NET_WM_STATE_FULLSCREEN"),
        xwm_get_atom(xwm, "_NET_WM_STATE_MAXIMIZED_VERT"),
        xwm_get_atom(xwm, "_NET_WM_STATE_MAXIMIZED_HORZ"),
        xwm_get_atom(xwm, "_NET_WM_STATE_ABOVE"),
        xwm_get_atom(xwm, "_NET_WM_STATE_MODAL"),
        xwm_get_atom(xwm, "_NET_WM_WINDOW_TYPE_DIALOG"),
        xwm_get_atom(xwm, "_NET_WM_WINDOW_TYPE_SPLASH"),
        xwm_get_atom(xwm, "_NET_WM_WINDOW_TYPE_DOCK"),
        xwm_get_atom(xwm, "_NET_WM_WINDOW_TYPE_TOOLBAR"),
        xwm_get_atom(xwm, "_NET_WM_WINDOW_TYPE_UTILITY"),
    };

    xcb_change_property(xwm->connection, XCB_PROP_MODE_REPLACE,
                        xwm->screen->root,
                        xwm_get_atom(xwm, "_NET_SUPPORTED"),
                        XCB_ATOM_ATOM, 32,
                        sizeof(supported) / sizeof(supported[0]),
                        supported);

    xcb_change_property(xwm->connection, XCB_PROP_MODE_REPLACE,
                        xwm->screen->root,
                        xwm_get_atom(xwm, "_NET_SUPPORTING_WM_CHECK"),
                        XCB_ATOM_WINDOW, 32,
                        1, &xwm->grab_window);

    xcb_change_property(xwm->connection, XCB_PROP_MODE_REPLACE,
                        xwm->grab_window,
                        xwm_get_atom(xwm, "_NET_WM_NAME"),
                        xwm_get_atom(xwm, "UTF8_STRING"), 8,
                        strlen("Waydri"), "Waydri");

    xcb_change_property(xwm->connection, XCB_PROP_MODE_REPLACE,
                        xwm->grab_window,
                        xwm_get_atom(xwm, "_NET_WM_CLASS"),
                        XCB_ATOM_STRING, 8,
                        strlen("waydri\x00Waydri"), "waydri\x00Waydri");

    xcb_change_property(xwm->connection, XCB_PROP_MODE_REPLACE,
                        xwm->grab_window,
                        XCB_ATOM_WM_CLASS,
                        XCB_ATOM_STRING, 8,
                        strlen("waydri\x00Waydri"), "waydri\x00Waydri");

    xcb_flush(xwm->connection);

    return xwm;
}

void xwm_destroy(struct xwm *xwm) {
    if (!xwm) {
        return;
    }

    for (int i = 0; i < xwm->window_count; i++) {
        struct xwm_window *win = &xwm->windows[i];
        free(win->title);
        free(win->app_id);
        if (win->surface) {
            wl_surface_destroy(win->surface);
        }
    }

    if (xwm->grab_window) {
        xcb_destroy_window(xwm->connection, xwm->grab_window);
    }

    if (xwm->connection) {
        xcb_disconnect(xwm->connection);
    }

    free(xwm);
}

int xwm_handle_event(struct xwm *xwm) {
    xcb_generic_event_t *event;
    int count = 0;

    if (!xwm || !xwm->connection) {
        return -1;
    }

    while ((event = xcb_poll_for_event(xwm->connection))) {
        count++;
        uint8_t type = event->response_type & ~0x80;

        switch (type) {
        case XCB_CREATE_NOTIFY:
            xwm_handle_create_notify(xwm, (xcb_create_notify_event_t *)event);
            break;

        case XCB_MAP_REQUEST:
            xwm_handle_map_request(xwm, (xcb_map_request_event_t *)event);
            break;

        case XCB_UNMAP_NOTIFY:
            xwm_handle_unmap_notify(xwm, (xcb_unmap_notify_event_t *)event);
            break;

        case XCB_CONFIGURE_REQUEST:
            xwm_handle_configure_request(xwm, (xcb_configure_request_event_t *)event);
            break;

        case XCB_PROPERTY_NOTIFY:
            xwm_handle_property_notify(xwm, (xcb_property_notify_event_t *)event);
            break;

        case XCB_CLIENT_MESSAGE:
            xwm_handle_client_message(xwm, (xcb_client_message_event_t *)event);
            break;

        case XCB_KEY_PRESS:
        case XCB_KEY_RELEASE:
            xwm_handle_key_event(xwm, (xcb_key_press_event_t *)event);
            break;

        case XCB_BUTTON_PRESS:
        case XCB_BUTTON_RELEASE:
            xwm_handle_button_event(xwm, (xcb_button_press_event_t *)event);
            break;

        case XCB_MOTION_NOTIFY:
            xwm_handle_motion_notify(xwm, (xcb_motion_notify_event_t *)event);
            break;

        case XCB_DESTROY_NOTIFY: {
            xcb_destroy_notify_event_t *dne = (xcb_destroy_notify_event_t *)event;
            struct xwm_window *win = xwm_find_window(xwm, dne->window);
            if (win) {
                xwm_destroy_surface(xwm, win);
            }
            break;
        }

        case XCB_CONFIGURE_NOTIFY: {
            xcb_configure_notify_event_t *cne = (xcb_configure_notify_event_t *)event;
            struct xwm_window *win = xwm_find_window(xwm, cne->window);
            if (win) {
                win->x = cne->x;
                win->y = cne->y;
                win->width = cne->width;
                win->height = cne->height;
            }
            break;
        }

        case XCB_MAP_NOTIFY: {
            xcb_map_notify_event_t *mne = (xcb_map_notify_event_t *)event;
            struct xwm_window *win = xwm_find_window(xwm, mne->window);
            if (win) {
                win->mapped = true;
            }
            break;
        }

        case XCB_SELECTION_REQUEST: {
            xcb_selection_request_event_t *sre = (xcb_selection_request_event_t *)event;
            xcb_selection_notify_event_t sn;
            memset(&sn, 0, sizeof(sn));
            sn.response_type = XCB_SELECTION_NOTIFY;
            sn.sequence = 0;
            sn.time = sre->time;
            sn.requestor = sre->requestor;
            sn.selection = sre->selection;
            sn.target = sre->target;
            sn.property = XCB_NONE;

            xcb_send_event(xwm->connection, false, sre->requestor,
                           0, (const char *)&sn);
            xcb_flush(xwm->connection);
            break;
        }

        case XCB_SELECTION_NOTIFY: {
            break;
        }

        default:
            break;
        }

        free(event);
    }

    xcb_flush(xwm->connection);
    return count;
}

void xwm_set_window_geometry(struct xwm *xwm, struct xwm_window *window, int x, int y, uint32_t w, uint32_t h) {
    uint32_t values[4];
    uint16_t mask = 0;

    if (window->x != x || window->y != y) {
        mask |= XCB_CONFIG_WINDOW_X | XCB_CONFIG_WINDOW_Y;
        values[0] = x;
        values[1] = y;
    }

    if (window->width != w || window->height != h) {
        mask |= XCB_CONFIG_WINDOW_WIDTH | XCB_CONFIG_WINDOW_HEIGHT;
        values[mask & XCB_CONFIG_WINDOW_X ? 2 : 0] = w;
        values[mask & XCB_CONFIG_WINDOW_Y ? 3 : 1] = h;
    }

    if (mask) {
        xcb_configure_window(xwm->connection, window->id, mask, values);
    }

    xcb_configure_notify_event_t ce;
    memset(&ce, 0, sizeof(ce));
    ce.response_type = XCB_CONFIGURE_NOTIFY;
    ce.event = window->id;
    ce.window = window->id;
    ce.x = x;
    ce.y = y;
    ce.width = w;
    ce.height = h;
    ce.border_width = 0;
    ce.above_sibling = XCB_NONE;
    ce.override_redirect = false;

    xcb_send_event(xwm->connection, false, window->id,
                   XCB_EVENT_MASK_STRUCTURE_NOTIFY, (const char *)&ce);

    xcb_atom_t net_wm_sync = xwm_get_atom(xwm, "_NET_WM_SYNC_REQUEST");
    xcb_atom_t net_wm_sync_counter = xwm_get_atom(xwm, "_NET_WM_SYNC_REQUEST_COUNTER");
    xwm->last_configured_serial++;

    xcb_flush(xwm->connection);

    window->x = x;
    window->y = y;
    window->width = w;
    window->height = h;
}

void xwm_configure_window(struct xwm *xwm, xcb_window_t window, uint16_t mask, const uint32_t *values) {
    xcb_configure_window(xwm->connection, window, mask, values);
    xcb_flush(xwm->connection);
}

struct xwm_window *xwm_find_window(struct xwm *xwm, xcb_window_t id) {
    for (int i = 0; i < xwm->window_count; i++) {
        if (xwm->windows[i].id == id) {
            return &xwm->windows[i];
        }
    }
    return NULL;
}

struct xwm_window *xwm_create_surface(struct xwm *xwm, xcb_window_t window_id) {
    struct xwm_window *win = xwm_find_window(xwm, window_id);
    if (win) {
        return win;
    }

    win = xwm_window_new(xwm, window_id);
    if (!win) {
        return NULL;
    }

    xcb_get_geometry_cookie_t geom_cookie;
    xcb_get_geometry_reply_t *geom_reply;
    geom_cookie = xcb_get_geometry(xwm->connection, window_id);
    geom_reply = xcb_get_geometry_reply(xwm->connection, geom_cookie, NULL);
    if (geom_reply) {
        win->x = geom_reply->x;
        win->y = geom_reply->y;
        win->width = geom_reply->width;
        win->height = geom_reply->height;
        free(geom_reply);
    }

    return win;
}

void xwm_destroy_surface(struct xwm *xwm, struct xwm_window *window) {
    if (!window) {
        return;
    }

    if (window->surface) {
        wl_surface_destroy(window->surface);
        window->surface = NULL;
    }

    free(window->title);
    window->title = NULL;
    free(window->app_id);
    window->app_id = NULL;
    window->mapped = false;

    wl_list_remove(&window->link);
    wl_list_init(&window->link);

    if (window != &xwm->windows[xwm->window_count - 1]) {
        int idx = window - xwm->windows;
        struct xwm_window tmp;
        memcpy(&tmp, window, sizeof(tmp));
        memcpy(window, &xwm->windows[xwm->window_count - 1], sizeof(struct xwm_window));
        memcpy(&xwm->windows[xwm->window_count - 1], &tmp, sizeof(struct xwm_window));
    }
    xwm->window_count--;
    xcb_flush(xwm->connection);
}

xcb_atom_t xwm_get_atom(struct xwm *xwm, const char *name) {
    for (int i = 0; i < xwm->atom_count; i++) {
        if (strcmp(xwm->atoms[i].name, name) == 0) {
            return xwm->atoms[i].atom;
        }
    }

    xcb_intern_atom_cookie_t cookie;
    xcb_intern_atom_reply_t *reply;

    cookie = xcb_intern_atom(xwm->connection, 0, strlen(name), name);
    reply = xcb_intern_atom_reply(xwm->connection, cookie, NULL);
    if (!reply) {
        return 0;
    }

    xcb_atom_t atom = reply->atom;
    free(reply);

    if (xwm->atom_count < XWM_MAX_ATOMS) {
        xwm->atoms[xwm->atom_count].atom = atom;
        xwm->atoms[xwm->atom_count].name = strdup(name);
        xwm->atom_count++;
    }

    return atom;
}

void xwm_update_window_title(struct xwm *xwm, struct xwm_window *window) {
    xcb_get_property_cookie_t cookie;
    xcb_get_property_reply_t *reply;
    xcb_atom_t net_wm_name = xwm_get_atom(xwm, "_NET_WM_NAME");
    xcb_atom_t utf8 = xwm_get_atom(xwm, "UTF8_STRING");

    cookie = xcb_get_property(xwm->connection, 0, window->id,
                              net_wm_name, utf8, 0, 1024);
    reply = xcb_get_property_reply(xwm->connection, cookie, NULL);
    if (reply && xcb_get_property_value_length(reply) > 0) {
        char *data = xcb_get_property_value(reply);
        int len = xcb_get_property_value_length(reply);
        free(window->title);
        window->title = strndup(data, len);
    } else {
        xcb_atom_t wm_name = xwm_get_atom(xwm, "WM_NAME");
        free(reply);

        cookie = xcb_get_property(xwm->connection, 0, window->id,
                                  wm_name, XCB_ATOM_STRING, 0, 1024);
        reply = xcb_get_property_reply(xwm->connection, cookie, NULL);
        if (reply && xcb_get_property_value_length(reply) > 0) {
            char *data = xcb_get_property_value(reply);
            int len = xcb_get_property_value_length(reply);
            free(window->title);
            window->title = strndup(data, len);
        }
    }
    free(reply);
}

void xwm_map_window(struct xwm *xwm, xcb_window_t window_id) {
    xcb_map_window(xwm->connection, window_id);
    xcb_flush(xwm->connection);

    struct xwm_window *win = xwm_find_window(xwm, window_id);
    if (win) {
        win->mapped = true;
    }
}

void xwm_unmap_window(struct xwm *xwm, xcb_window_t window_id) {
    xcb_unmap_window(xwm->connection, window_id);
    xcb_flush(xwm->connection);

    struct xwm_window *win = xwm_find_window(xwm, window_id);
    if (win) {
        win->mapped = false;
    }
}

void xwm_focus_window(struct xwm *xwm, struct xwm_window *window) {
    if (!window || !window->mapped) {
        xcb_set_input_focus(xwm->connection, XCB_INPUT_FOCUS_POINTER_ROOT,
                            xwm->screen->root, XCB_CURRENT_TIME);
        xcb_flush(xwm->connection);
        return;
    }

    uint32_t values[] = {XCB_STACK_MODE_ABOVE};
    xcb_configure_window(xwm->connection, window->id,
                         XCB_CONFIG_WINDOW_STACK_MODE, values);

    xcb_set_input_focus(xwm->connection, XCB_INPUT_FOCUS_PARENT,
                        window->id, XCB_CURRENT_TIME);

    xcb_atom_t net_active = xwm_get_atom(xwm, "_NET_ACTIVE_WINDOW");
    xcb_change_property(xwm->connection, XCB_PROP_MODE_REPLACE,
                        xwm->screen->root, net_active,
                        XCB_ATOM_WINDOW, 32, 1, &window->id);

    xcb_flush(xwm->connection);
}

void xwm_send_focus_out(struct xwm *xwm, xcb_window_t window_id) {
    xcb_focus_out_event_t event;
    memset(&event, 0, sizeof(event));
    event.response_type = XCB_FOCUS_OUT;
    event.detail = XCB_NOTIFY_DETAIL_NORMAL;
    event.window = window_id;
    event.mode = XCB_NOTIFY_MODE_NORMAL;
    xcb_send_event(xwm->connection, false, window_id,
                   XCB_EVENT_MASK_FOCUS_CHANGE, (const char *)&event);
    xcb_flush(xwm->connection);
}

void xwm_raise_window(struct xwm *xwm, xcb_window_t window_id) {
    uint32_t values[] = {XCB_STACK_MODE_ABOVE};
    xcb_configure_window(xwm->connection, window_id,
                         XCB_CONFIG_WINDOW_STACK_MODE, values);
    xcb_flush(xwm->connection);
}
