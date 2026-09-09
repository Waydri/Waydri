#include <stdlib.h>
#include <string.h>
#include <xcb/xcb.h>
#include <xcb/xfixes.h>
#include "xwayland.h"
#include "xwm.h"

struct fixes {
    struct xwayland *server;
    struct xwm *xwm;
    uint32_t version_major;
    uint32_t version_minor;
    bool has_fixes;
    xcb_xfixes_region_t global_region;
    xcb_window_t hidden_windows[128];
    int hidden_count;
};

struct fixes *fixes_create(struct xwayland *server) {
    struct fixes *fixes;
    xcb_generic_error_t *error;

    fixes = calloc(1, sizeof(struct fixes));
    if (!fixes) {
        return NULL;
    }

    fixes->server = server;
    fixes->xwm = server ? server->xwm : NULL;
    fixes->has_fixes = false;
    fixes->hidden_count = 0;

    if (fixes->xwm && fixes->xwm->connection) {
        const xcb_query_extension_reply_t *ext;
        ext = xcb_get_extension_data(fixes->xwm->connection, &xcb_xfixes_id);

        if (ext && ext->present) {
            xcb_xfixes_query_version_cookie_t cookie;
            xcb_xfixes_query_version_reply_t *reply;

            cookie = xcb_xfixes_query_version(fixes->xwm->connection, 4, 0);
            reply = xcb_xfixes_query_version_reply(fixes->xwm->connection, cookie, &error);

            if (reply) {
                fixes->version_major = reply->major_version;
                fixes->version_minor = reply->minor_version;
                fixes->has_fixes = true;
                free(reply);
            }
        }

        if (fixes->has_fixes) {
            fixes->global_region = xcb_generate_id(fixes->xwm->connection);
            xcb_xfixes_create_region(fixes->xwm->connection,
                                      fixes->global_region, NULL, 0);
        }
    }

    return fixes;
}

void fixes_destroy(struct fixes *fixes) {
    if (!fixes) {
        return;
    }

    if (fixes->has_fixes && fixes->xwm && fixes->xwm->connection) {
        for (int i = 0; i < fixes->hidden_count; i++) {
            xcb_xfixes_show_window(fixes->xwm->connection,
                                    fixes->hidden_windows[i]);
        }

        xcb_xfixes_destroy_region(fixes->xwm->connection,
                                   fixes->global_region);
        xcb_flush(fixes->xwm->connection);
    }

    free(fixes);
}

void fixes_hide_window(struct fixes *fixes, xcb_window_t window) {
    if (!fixes || !fixes->has_fixes || !fixes->xwm) {
        return;
    }

    xcb_xfixes_hide_window(fixes->xwm->connection, window);
    xcb_flush(fixes->xwm->connection);

    if (fixes->hidden_count < 128) {
        fixes->hidden_windows[fixes->hidden_count++] = window;
    }
}

void fixes_show_window(struct fixes *fixes, xcb_window_t window) {
    if (!fixes || !fixes->has_fixes || !fixes->xwm) {
        return;
    }

    xcb_xfixes_show_window(fixes->xwm->connection, window);
    xcb_flush(fixes->xwm->connection);

    for (int i = 0; i < fixes->hidden_count; i++) {
        if (fixes->hidden_windows[i] == window) {
            fixes->hidden_windows[i] = fixes->hidden_windows[fixes->hidden_count - 1];
            fixes->hidden_count--;
            break;
        }
    }
}

void fixes_set_window_shape(struct fixes *fixes, xcb_window_t window,
                             int shape_kind, const xcb_rectangle_t *rects,
                             int rect_count) {
    xcb_shape_op_t op;

    if (!fixes || !fixes->xwm || !rects || rect_count <= 0) {
        return;
    }

    op = XCB_SHAPE_OP_SET;

    xcb_shape_rectangles(fixes->xwm->connection,
                          op, shape_kind, XCB_CLIP_ORDERING_UNSORTED,
                          window, 0, 0,
                          rect_count, rects);
    xcb_flush(fixes->xwm->connection);
}

xcb_xfixes_region_t fixes_create_region_from_window(struct fixes *fixes,
                                                     xcb_window_t window) {
    xcb_xfixes_region_t region;

    if (!fixes || !fixes->has_fixes || !fixes->xwm) {
        return 0;
    }

    region = xcb_generate_id(fixes->xwm->connection);
    xcb_xfixes_create_region(fixes->xwm->connection, region, NULL, 0);

    xcb_composite_create_region_from_border_clip(fixes->xwm->connection,
                                                  region, window);

    xcb_xfixes_translate_region(fixes->xwm->connection, region, 0, 0);
    xcb_flush(fixes->xwm->connection);

    return region;
}
