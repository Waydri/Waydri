#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <xcb/xcb.h>
#include <xcb/composite.h>
#include <xcb/xfixes.h>
#include "xwayland.h"
#include "xwm.h"

struct composite_damage_rect {
    int16_t x, y;
    uint16_t w, h;
    struct composite_damage_rect *next;
};

struct composite_region {
    xcb_xfixes_region_t region_id;
    struct composite_damage_rect *rects;
    int rect_count;
};

struct composite {
    struct xwayland *server;
    struct xwm *xwm;
    xcb_colormap_t colormap;
    bool has_composite;
    struct composite_region regions[64];
    int region_count;
    struct composite_damage_rect *damage_list;
    int damage_count;
    uint32_t sequence;
};

static void composite_flush_damage(struct composite *composite) {
    struct composite_damage_rect *rect, *tmp;

    if (!composite->damage_list) {
        return;
    }

    xcb_rectangle_t *rects = calloc(composite->damage_count, sizeof(xcb_rectangle_t));
    if (!rects) {
        return;
    }

    int i = 0;
    rect = composite->damage_list;
    while (rect) {
        rects[i].x = rect->x;
        rects[i].y = rect->y;
        rects[i].width = rect->w;
        rects[i].height = rect->h;
        i++;
        rect = rect->next;
    }

    rect = composite->damage_list;
    while (rect) {
        tmp = rect->next;
        free(rect);
        rect = tmp;
    }
    composite->damage_list = NULL;
    composite->damage_count = 0;

    free(rects);
}

struct composite *composite_create(struct xwayland *server, xcb_colormap_t colormap) {
    struct composite *composite;

    composite = calloc(1, sizeof(struct composite));
    if (!composite) {
        return NULL;
    }

    composite->server = server;
    composite->xwm = server ? server->xwm : NULL;
    composite->colormap = colormap;
    composite->region_count = 0;
    composite->damage_list = NULL;
    composite->damage_count = 0;
    composite->sequence = 0;

    if (composite->xwm) {
        composite->has_composite = composite->xwm->has_composite;
    }

    return composite;
}

void composite_destroy(struct composite *composite) {
    struct composite_damage_rect *rect, *tmp;

    if (!composite) {
        return;
    }

    for (int i = 0; i < composite->region_count; i++) {
        rect = composite->regions[i].rects;
        while (rect) {
            tmp = rect->next;
            free(rect);
            rect = tmp;
        }
    }

    rect = composite->damage_list;
    while (rect) {
        tmp = rect->next;
        free(rect);
        rect = tmp;
    }

    free(composite);
}

int composite_redirect_window(struct composite *composite, xcb_window_t window, uint8_t update) {
    if (!composite || !composite->xwm || !composite->has_composite) {
        return -1;
    }

    xcb_composite_redirect_subwindows(composite->xwm->connection,
                                       window, update);

    xcb_xfixes_region_t region = xcb_generate_id(composite->xwm->connection);
    xcb_xfixes_create_region(composite->xwm->connection, region, NULL, 0);

    xcb_composite_create_region_from_border_clip(composite->xwm->connection,
                                                 region, window);

    xcb_composite_set_window_damage_reporting(composite->xwm->connection,
                                               window,
                                               XCB_COMPOSITE_NOTIFY_DAMAGE,
                                               region);

    xcb_xfixes_destroy_region(composite->xwm->connection, region);
    xcb_flush(composite->xwm->connection);

    return 0;
}

int composite_create_region(struct composite *composite, uint32_t region_id) {
    if (!composite || composite->region_count >= 64) {
        return -1;
    }

    struct composite_region *region = &composite->regions[composite->region_count];
    region->region_id = xcb_generate_id(composite->xwm->connection);
    region->rects = NULL;
    region->rect_count = 0;

    xcb_xfixes_create_region(composite->xwm->connection,
                              region->region_id, NULL, 0);
    xcb_flush(composite->xwm->connection);

    composite->region_count++;
    return 0;
}

int composite_damage(struct composite *composite, int16_t x, int16_t y, uint16_t w, uint16_t h) {
    struct composite_damage_rect *rect;

    if (!composite) {
        return -1;
    }

    rect = calloc(1, sizeof(struct composite_damage_rect));
    if (!rect) {
        return -1;
    }

    rect->x = x;
    rect->y = y;
    rect->w = w;
    rect->h = h;
    rect->next = composite->damage_list;
    composite->damage_list = rect;
    composite->damage_count++;

    if (composite->damage_count > 256) {
        composite_flush_damage(composite);
    }

    return 0;
}

int composite_synchronize(struct composite *composite) {
    if (!composite) {
        return -1;
    }

    composite_flush_damage(composite);

    if (composite->xwm && composite->xwm->connection) {
        xcb_flush(composite->xwm->connection);
    }

    composite->sequence++;
    return 0;
}
