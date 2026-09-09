#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <xcb/xcb.h>
#include <xcb/xfixes.h>
#include "xwayland.h"
#include "xwm.h"

struct damage_rect {
    int16_t x, y;
    uint16_t w, h;
    struct damage_rect *next;
};

struct damage {
    struct xwayland *server;
    struct xwm *xwm;
    struct damage_rect *rects;
    int count;
    xcb_xfixes_region_t xf_region;
    bool region_valid;
};

static void damage_free_rects(struct damage *damage) {
    struct damage_rect *rect, *tmp;
    rect = damage->rects;
    while (rect) {
        tmp = rect->next;
        free(rect);
        rect = tmp;
    }
    damage->rects = NULL;
    damage->count = 0;
    damage->region_valid = false;
}

static bool damage_rects_intersect(int16_t ax, int16_t ay, uint16_t aw, uint16_t ah,
                                    int16_t bx, int16_t by, uint16_t bw, uint16_t bh) {
    int16_t ix1 = ax > bx ? ax : bx;
    int16_t iy1 = ay > by ? ay : by;
    int16_t ix2 = (ax + aw) < (bx + bw) ? (ax + aw) : (bx + bw);
    int16_t iy2 = (ay + ah) < (by + bh) ? (ay + ah) : (by + bh);
    return (ix1 < ix2 && iy1 < iy2);
}

struct damage *damage_create(struct xwayland *server) {
    struct damage *damage;

    damage = calloc(1, sizeof(struct damage));
    if (!damage) {
        return NULL;
    }

    damage->server = server;
    damage->xwm = server ? server->xwm : NULL;
    damage->rects = NULL;
    damage->count = 0;
    damage->region_valid = false;

    if (damage->xwm && damage->xwm->connection) {
        damage->xf_region = xcb_generate_id(damage->xwm->connection);
        xcb_xfixes_create_region(damage->xwm->connection,
                                  damage->xf_region, NULL, 0);
        damage->region_valid = true;
    }

    return damage;
}

void damage_destroy(struct damage *damage) {
    if (!damage) {
        return;
    }

    damage_free_rects(damage);

    if (damage->region_valid && damage->xwm && damage->xwm->connection) {
        xcb_xfixes_destroy_region(damage->xwm->connection,
                                   damage->xf_region);
        xcb_flush(damage->xwm->connection);
    }

    free(damage);
}

void damage_add_rect(struct damage *damage, int16_t x, int16_t y, uint16_t w, uint16_t h) {
    struct damage_rect *rect;
    struct damage_rect *existing;

    if (!damage || w == 0 || h == 0) {
        return;
    }

    existing = damage->rects;
    while (existing) {
        if (existing->x == x && existing->y == y &&
            existing->w == w && existing->h == h) {
            return;
        }
        existing = existing->next;
    }

    rect = calloc(1, sizeof(struct damage_rect));
    if (!rect) {
        return;
    }

    rect->x = x;
    rect->y = y;
    rect->w = w;
    rect->h = h;
    rect->next = damage->rects;
    damage->rects = rect;
    damage->count++;

    if (damage->region_valid && damage->xwm && damage->xwm->connection) {
        xcb_rectangle_t xcb_rect;
        xcb_rect.x = x;
        xcb_rect.y = y;
        xcb_rect.width = w;
        xcb_rect.height = h;
        xcb_xfixes_union_region_with_input(damage->xwm->connection,
                                            damage->xf_region,
                                            damage->xf_region,
                                            damage->xf_region);
        xcb_xfixes_set_region(damage->xwm->connection,
                               damage->xf_region,
                               &xcb_rect, 1);
    }
}

void damage_add_box(struct damage *damage, int16_t x, int16_t y, uint16_t w, uint16_t h) {
    damage_add_rect(damage, x, y, w, h);
}

void damage_translate(struct damage *damage, int16_t dx, int16_t dy) {
    struct damage_rect *rect;

    if (!damage) {
        return;
    }

    rect = damage->rects;
    while (rect) {
        rect->x += dx;
        rect->y += dy;
        rect = rect->next;
    }

    if (damage->region_valid && damage->xwm && damage->xwm->connection) {
        xcb_xfixes_translate_region(damage->xwm->connection,
                                     damage->xf_region, dx, dy);
    }
}

void damage_intersect(struct damage *damage, xcb_xfixes_region_t region) {
    if (!damage || !damage->region_valid || !damage->xwm) {
        return;
    }

    xcb_xfixes_intersect_region(damage->xwm->connection,
                                 damage->xf_region,
                                 region,
                                 damage->xf_region);

    xcb_rectangle_t *rects;
    int count;

    xcb_xfixes_fetch_region_reply_t *reply;
    reply = xcb_xfixes_fetch_region_reply(damage->xwm->connection,
                                           xcb_xfixes_fetch_region(
                                               damage->xwm->connection,
                                               damage->xf_region),
                                           NULL);

    damage_free_rects(damage);

    if (reply) {
        xcb_rectangle_t *xcb_rects = xcb_xfixes_fetch_region_rectangles(reply);
        count = xcb_xfixes_fetch_region_rectangles_length(reply);

        for (int i = 0; i < count; i++) {
            struct damage_rect *rect = calloc(1, sizeof(struct damage_rect));
            if (rect) {
                rect->x = xcb_rects[i].x;
                rect->y = xcb_rects[i].y;
                rect->w = xcb_rects[i].width;
                rect->h = xcb_rects[i].height;
                rect->next = damage->rects;
                damage->rects = rect;
                damage->count++;
            }
        }
        free(reply);
    }
}

void damage_union(struct damage *damage, struct damage *other_damage) {
    struct damage_rect *rect;

    if (!damage || !other_damage) {
        return;
    }

    rect = other_damage->rects;
    while (rect) {
        damage_add_rect(damage, rect->x, rect->y, rect->w, rect->h);
        rect = rect->next;
    }

    if (damage->region_valid && other_damage->region_valid && damage->xwm) {
        xcb_xfixes_union_region(damage->xwm->connection,
                                 damage->xf_region,
                                 other_damage->xf_region,
                                 damage->xf_region);
    }
}

bool damage_not_empty(struct damage *damage) {
    if (!damage) {
        return false;
    }
    return damage->count > 0;
}

void damage_clear(struct damage *damage) {
    if (!damage) {
        return;
    }

    damage_free_rects(damage);

    if (damage->region_valid && damage->xwm && damage->xwm->connection) {
        xcb_xfixes_destroy_region(damage->xwm->connection,
                                   damage->xf_region);
        damage->xf_region = xcb_generate_id(damage->xwm->connection);
        xcb_xfixes_create_region(damage->xwm->connection,
                                  damage->xf_region, NULL, 0);
    }
}
