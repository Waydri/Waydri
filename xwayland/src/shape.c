#include <stdlib.h>
#include <string.h>
#include <xcb/xcb.h>
#include <xcb/shape.h>
#include "xwayland.h"
#include "xwm.h"

struct shape {
    struct xwayland *server;
    struct xwm *xwm;
    bool has_shape;
    uint32_t version;
};

struct shape *shape_create(struct xwayland *server) {
    struct shape *shp;
    xcb_generic_error_t *error;

    shp = calloc(1, sizeof(struct shape));
    if (!shp) {
        return NULL;
    }

    shp->server = server;
    shp->xwm = server ? server->xwm : NULL;
    shp->has_shape = false;
    shp->version = 0;

    if (shp->xwm && shp->xwm->connection) {
        const xcb_query_extension_reply_t *ext;
        ext = xcb_get_extension_data(shp->xwm->connection, &xcb_shape_id);

        if (ext && ext->present) {
            xcb_shape_query_version_cookie_t cookie;
            xcb_shape_query_version_reply_t *reply;

            cookie = xcb_shape_query_version(shp->xwm->connection);
            reply = xcb_shape_query_version_reply(shp->xwm->connection, cookie, &error);
            if (reply) {
                shp->has_shape = true;
                shp->version = reply->server_version;
                free(reply);
            }
        }
    }

    return shp;
}

void shape_destroy(struct shape *shp) {
    if (!shp) {
        return;
    }
    free(shp);
}

void shape_set_rects(struct shape *shp, xcb_window_t window,
                      int shape_kind, const xcb_rectangle_t *rects,
                      int rect_count, int operation) {
    xcb_shape_op_t op;

    if (!shp || !shp->has_shape || !shp->xwm || !rects || rect_count <= 0) {
        return;
    }

    switch (operation) {
    case 0:
        op = XCB_SHAPE_OP_SET;
        break;
    case 1:
        op = XCB_SHAPE_OP_UNION;
        break;
    case 2:
        op = XCB_SHAPE_OP_INTERSECT;
        break;
    case 3:
        op = XCB_SHAPE_OP_SUBTRACT;
        break;
    case 4:
        op = XCB_SHAPE_OP_INVERT;
        break;
    default:
        op = XCB_SHAPE_OP_SET;
        break;
    }

    xcb_shape_rectangles(shp->xwm->connection,
                          op, shape_kind, XCB_CLIP_ORDERING_UNSORTED,
                          window, 0, 0,
                          rect_count, rects);
    xcb_flush(shp->xwm->connection);
}

xcb_rectangle_t *shape_get_rects(struct shape *shp, xcb_window_t window,
                                   int shape_kind, int *count) {
    xcb_shape_get_rectangles_cookie_t cookie;
    xcb_shape_get_rectangles_reply_t *reply;
    xcb_rectangle_t *result;
    int n;

    if (!shp || !shp->has_shape || !shp->xwm || !count) {
        *count = 0;
        return NULL;
    }

    cookie = xcb_shape_get_rectangles(shp->xwm->connection,
                                       window, shape_kind);
    reply = xcb_shape_get_rectangles_reply(shp->xwm->connection, cookie, NULL);
    if (!reply) {
        *count = 0;
        return NULL;
    }

    n = xcb_shape_get_rectangles_rectangles_length(reply);
    if (n <= 0) {
        *count = 0;
        free(reply);
        return NULL;
    }

    xcb_rectangle_t *rects = xcb_shape_get_rectangles_rectangles(reply);

    result = calloc(n, sizeof(xcb_rectangle_t));
    if (result) {
        memcpy(result, rects, n * sizeof(xcb_rectangle_t));
        *count = n;
    } else {
        *count = 0;
    }

    free(reply);
    return result;
}

void shape_mask(struct shape *shp, xcb_window_t window,
                 int shape_kind, int operation, xcb_window_t src_window,
                 int16_t src_x, int16_t src_y) {
    xcb_shape_op_t op;

    if (!shp || !shp->has_shape || !shp->xwm) {
        return;
    }

    switch (operation) {
    case 0:
        op = XCB_SHAPE_OP_SET;
        break;
    case 1:
        op = XCB_SHAPE_OP_UNION;
        break;
    case 2:
        op = XCB_SHAPE_OP_INTERSECT;
        break;
    case 3:
        op = XCB_SHAPE_OP_SUBTRACT;
        break;
    case 4:
        op = XCB_SHAPE_OP_INVERT;
        break;
    default:
        op = XCB_SHAPE_OP_SET;
        break;
    }

    xcb_shape_mask(shp->xwm->connection,
                    op, shape_kind,
                    window, src_x, src_y,
                    src_window);
    xcb_flush(shp->xwm->connection);
}

void shape_input_rects(struct shape *shp, xcb_window_t window,
                         const xcb_rectangle_t *rects, int rect_count) {
    xcb_shape_op_t op;
    xcb_rectangle_t *combined;
    int combined_count;
    int x, y;
    uint16_t w, h;

    if (!shp || !shp->has_shape || !shp->xwm) {
        return;
    }

    op = XCB_SHAPE_OP_SET;

    xcb_shape_rectangles(shp->xwm->connection,
                          op, XCB_SHAPE_KIND_INPUT,
                          XCB_CLIP_ORDERING_UNSORTED,
                          window, 0, 0,
                          rect_count, rects);

    xcb_get_geometry_cookie_t geom_cookie;
    xcb_get_geometry_reply_t *geom_reply;

    geom_cookie = xcb_get_geometry(shp->xwm->connection, window);
    geom_reply = xcb_get_geometry_reply(shp->xwm->connection, geom_cookie, NULL);

    if (geom_reply) {
        x = 0;
        y = 0;
        w = geom_reply->width;
        h = geom_reply->height;
        free(geom_reply);
    } else {
        x = 0;
        y = 0;
        w = 1;
        h = 1;
    }

    xcb_shape_get_rectangles_cookie_t get_cookie;
    xcb_shape_get_rectangles_reply_t *get_reply;

    get_cookie = xcb_shape_get_rectangles(shp->xwm->connection,
                                            window, XCB_SHAPE_KIND_BOUNDING);
    get_reply = xcb_shape_get_rectangles_reply(shp->xwm->connection, get_cookie, NULL);

    if (get_reply) {
        xcb_rectangle_t *bounding = xcb_shape_get_rectangles_rectangles(get_reply);
        int bounding_count = xcb_shape_get_rectangles_rectangles_length(get_reply);

        combined_count = bounding_count + rect_count;
        combined = calloc(combined_count, sizeof(xcb_rectangle_t));
        if (combined) {
            for (int i = 0; i < bounding_count; i++) {
                combined[i] = bounding[i];
            }
            for (int i = 0; i < rect_count; i++) {
                combined[bounding_count + i] = rects[i];
            }

            xcb_shape_rectangles(shp->xwm->connection,
                                  XCB_SHAPE_OP_INTERSECT,
                                  XCB_SHAPE_KIND_INPUT,
                                  XCB_CLIP_ORDERING_UNSORTED,
                                  window, 0, 0,
                                  combined_count, combined);
            free(combined);
        }
        free(get_reply);
    }

    xcb_flush(shp->xwm->connection);
}
