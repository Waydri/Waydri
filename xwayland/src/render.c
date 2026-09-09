#include <stdlib.h>
#include <string.h>
#include <xcb/xcb.h>
#include <xcb/render.h>
#include "xwayland.h"
#include "xwm.h"

struct render_picture {
    xcb_render_picture_t id;
    xcb_render_pictformat_t format;
    struct render_picture *next;
};

struct render {
    struct xwayland *server;
    struct xwm *xwm;
    bool has_render;
    xcb_render_pictformat_t picture_format_rgb24;
    xcb_render_pictformat_t picture_format_argb32;
    xcb_render_pictformat_t picture_format_a8;
    xcb_colormap_t colormap;
    struct render_picture *pictures;
    int picture_count;
};

static xcb_render_pictformat_t render_find_format(struct render *render, uint32_t depth) {
    xcb_render_pictforminfo_cookie_t cookie;
    xcb_render_pictforminfo_reply_t *reply;
    xcb_render_pictforminfo_t *formats;
    int count;
    xcb_render_pictformat_t result = 0;

    if (!render->xwm || !render->xwm->connection) {
        return 0;
    }

    cookie = xcb_render_query_pict_formats(render->xwm->connection);
    reply = xcb_render_query_pict_formats_reply(render->xwm->connection, cookie, NULL);
    if (!reply) {
        return 0;
    }

    formats = xcb_render_query_pict_formats_formats(reply);
    count = xcb_render_query_pict_formats_formats_length(reply);

    for (int i = 0; i < count; i++) {
        if (formats[i].depth == depth &&
            formats[i].type == XCB_RENDER_PICT_TYPE_DIRECT) {
            result = formats[i].id;
            break;
        }
    }

    free(reply);
    return result;
}

struct render *render_create(struct xwayland *server) {
    struct render *render;
    xcb_generic_error_t *error;

    render = calloc(1, sizeof(struct render));
    if (!render) {
        return NULL;
    }

    render->server = server;
    render->xwm = server ? server->xwm : NULL;
    render->has_render = false;
    render->pictures = NULL;
    render->picture_count = 0;

    if (render->xwm && render->xwm->connection) {
        const xcb_query_extension_reply_t *ext;
        ext = xcb_get_extension_data(render->xwm->connection, &xcb_render_id);

        if (ext && ext->present) {
            xcb_render_query_version_cookie_t cookie;
            xcb_render_query_version_reply_t *reply;

            cookie = xcb_render_query_version(render->xwm->connection, 0, 11);
            reply = xcb_render_query_version_reply(render->xwm->connection, cookie, &error);
            if (reply) {
                render->has_render = true;
                free(reply);
            }
        }

        if (render->has_render) {
            render->picture_format_rgb24 = render_find_format(render, 24);
            render->picture_format_argb32 = render_find_format(render, 32);
            render->picture_format_a8 = render_find_format(render, 8);

            render->colormap = xcb_generate_id(render->xwm->connection);
            xcb_create_colormap(render->xwm->connection,
                                XCB_COLORMAP_ALLOC_NONE,
                                render->colormap,
                                render->xwm->screen->root,
                                render->xwm->screen->root_visual);
        }
    }

    return render;
}

void render_destroy(struct render *render) {
    struct render_picture *pic, *tmp;

    if (!render) {
        return;
    }

    pic = render->pictures;
    while (pic) {
        tmp = pic->next;
        if (render->xwm && render->xwm->connection) {
            xcb_render_free_picture(render->xwm->connection, pic->id);
        }
        free(pic);
        pic = tmp;
    }

    if (render->colormap && render->xwm && render->xwm->connection) {
        xcb_free_colormap(render->xwm->connection, render->colormap);
    }

    free(render);
}

xcb_render_picture_t render_create_picture(struct render *render,
                                            xcb_drawable_t drawable,
                                            xcb_render_pictformat_t format,
                                            const uint32_t *attrs,
                                            int attr_count) {
    xcb_render_picture_t picture;

    if (!render || !render->has_render || !render->xwm) {
        return 0;
    }

    picture = xcb_generate_id(render->xwm->connection);

    xcb_render_create_picture(render->xwm->connection,
                               picture, drawable, format,
                               attr_count, attrs);

    struct render_picture *pic = calloc(1, sizeof(struct render_picture));
    if (pic) {
        pic->id = picture;
        pic->format = format;
        pic->next = render->pictures;
        render->pictures = pic;
        render->picture_count++;
    }

    xcb_flush(render->xwm->connection);
    return picture;
}

void render_free_picture(struct render *render, xcb_render_picture_t picture) {
    struct render_picture *pic, *prev;

    if (!render || !render->xwm) {
        return;
    }

    xcb_render_free_picture(render->xwm->connection, picture);

    prev = NULL;
    pic = render->pictures;
    while (pic) {
        if (pic->id == picture) {
            if (prev) {
                prev->next = pic->next;
            } else {
                render->pictures = pic->next;
            }
            free(pic);
            render->picture_count--;
            break;
        }
        prev = pic;
        pic = pic->next;
    }

    xcb_flush(render->xwm->connection);
}

void render_composite(struct render *render,
                       xcb_render_pict_op_t op,
                       xcb_render_picture_t src,
                       xcb_render_picture_t mask,
                       xcb_render_picture_t dst,
                       int16_t src_x, int16_t src_y,
                       int16_t mask_x, int16_t mask_y,
                       int16_t dst_x, int16_t dst_y,
                       uint16_t width, uint16_t height) {
    if (!render || !render->has_render || !render->xwm) {
        return;
    }

    xcb_render_composite(render->xwm->connection,
                          op,
                          src, mask, dst,
                          src_x, src_y,
                          mask_x, mask_y,
                          dst_x, dst_y,
                          width, height);

    xcb_flush(render->xwm->connection);
}

void render_fill_rectangles(struct render *render,
                             xcb_render_pict_op_t op,
                             xcb_render_picture_t dst,
                             const xcb_render_color_t *color,
                             const xcb_rectangle_t *rects,
                             int rect_count) {
    if (!render || !render->has_render || !render->xwm || !rects || rect_count <= 0) {
        return;
    }

    xcb_render_fill_rectangles(render->xwm->connection,
                                op, dst, color,
                                rect_count, rects);
    xcb_flush(render->xwm->connection);
}

void render_set_transform(struct render *render,
                           xcb_render_picture_t picture,
                           uint32_t transform) {
    if (!render || !render->has_render || !render->xwm) {
        return;
    }

    xcb_render_set_picture_transform(render->xwm->connection,
                                      picture,
                                      transform, 0, 0,
                                      0, 0, 0);
    xcb_flush(render->xwm->connection);
}
