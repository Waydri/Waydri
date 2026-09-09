#include <stdlib.h>
#include <string.h>
#include <xcb/xcb.h>
#include "xwayland.h"
#include "xwm.h"
#include "selection.h"

struct selection_source_entry {
    xcb_atom_t atom;
    char *mime_type;
    struct selection_source_entry *next;
};

struct selection_owner {
    xcb_atom_t selection;
    xcb_window_t window;
    xcb_window_t requestor;
    xcb_atom_t property;
    bool owned;
    struct selection_owner *next;
};

struct selection *selection_create(struct xwayland *server, struct xwm *xwm, void *seat) {
    struct selection *sel;

    sel = calloc(1, sizeof(struct selection));
    if (!sel) {
        return NULL;
    }

    sel->server = server;
    sel->xwm = xwm;
    sel->seat = seat;
    sel->serial = 0;
    sel->current_source = NULL;
    sel->active = false;

    wl_list_init(&sel->sources);
    wl_list_init(&sel->offers);

    if (xwm && xwm->connection) {
        sel->clipboard_atom = xcb_intern_atom(xwm->connection, 0, 8, "CLIPBOARD").atom;
        sel->primary_atom = xcb_intern_atom(xwm->connection, 0, 7, "PRIMARY").atom;
        sel->atom_text_plain_utf8 = xcb_intern_atom(xwm->connection, 0, 25,
                                                     "text/plain;charset=utf-8").atom;
        sel->atom_text_uri_list = xcb_intern_atom(xwm->connection, 0, 12,
                                                    "text/uri-list").atom;
        sel->atom_text_html = xcb_intern_atom(xwm->connection, 0, 9, "text/html").atom;
        sel->atom_image_png = xcb_intern_atom(xwm->connection, 0, 9, "image/png").atom;
        sel->atom_text_x_moz_url = xcb_intern_atom(xwm->connection, 0, 14,
                                                      "text/x-moz-url").atom;
    }

    return sel;
}

void selection_destroy(struct selection *selection) {
    struct selection_source *source, *s_tmp;
    struct selection_offer *offer, *o_tmp;

    if (!selection) {
        return;
    }

    wl_list_for_each_safe(source, s_tmp, &selection->sources, link) {
        wl_list_remove(&source->link);
        free(source->mime_type);
        if (source->fd >= 0) {
            close(source->fd);
        }
        free(source);
    }

    wl_list_for_each_safe(offer, o_tmp, &selection->offers, link) {
        wl_list_remove(&offer->link);
        free(offer->mime_type);
        free(offer);
    }

    if (selection->current_source) {
        free(selection->current_source->mime_type);
        if (selection->current_source->fd >= 0) {
            close(selection->current_source->fd);
        }
        free(selection->current_source);
    }

    free(selection);
}

int selection_handle_event(struct selection *selection, xcb_generic_event_t *event) {
    xcb_selection_request_event_t *sre;
    xcb_selection_notify_event_t sn;
    xcb_get_property_cookie_t cookie;
    xcb_get_property_reply_t *reply;

    if (!selection || !selection->xwm || !event) {
        return -1;
    }

    uint8_t type = event->response_type & ~0x80;

    switch (type) {
    case XCB_SELECTION_REQUEST:
        sre = (xcb_selection_request_event_t *)event;

        memset(&sn, 0, sizeof(sn));
        sn.response_type = XCB_SELECTION_NOTIFY;
        sn.sequence = 0;
        sn.time = sre->time;
        sn.requestor = sre->requestor;
        sn.selection = sre->selection;
        sn.target = sre->target;
        sn.property = XCB_NONE;

        if (sre->target == XCB_NONE) {
            break;
        }

        xcb_atom_t targets_atom = xcb_intern_atom(selection->xwm->connection,
                                                    0, 7, "TARGETS").atom;
        xcb_atom_t atom_atom = xcb_intern_atom(selection->xwm->connection,
                                                 0, 4, "ATOM").atom;

        if (sre->target == targets_atom) {
            int offer_count = 0;
            struct selection_offer *offer;
            wl_list_for_each(offer, &selection->offers, link) {
                offer_count++;
            }

            xcb_atom_t *target_list = calloc(offer_count + 2, sizeof(xcb_atom_t));
            if (target_list) {
                target_list[0] = targets_atom;
                int idx = 1;
                wl_list_for_each(offer, &selection->offers, link) {
                    target_list[idx++] = offer->target;
                }

                xcb_change_property(selection->xwm->connection,
                                     XCB_PROP_MODE_REPLACE,
                                     sre->requestor, sre->property,
                                     atom_atom, 32,
                                     offer_count + 1, target_list);
                free(target_list);
            }
            sn.property = sre->property;
        } else {
            bool found = false;
            struct selection_offer *offer;
            wl_list_for_each(offer, &selection->offers, link) {
                if (offer->target == sre->target) {
                    found = true;
                    break;
                }
            }

            if (found) {
                cookie = xcb_get_property(selection->xwm->connection,
                                           0, sre->requestor,
                                           sre->property,
                                           XCB_GET_PROPERTY_TYPE_ANY,
                                           0, 65536);
                reply = xcb_get_property_reply(selection->xwm->connection,
                                                cookie, NULL);
                if (reply) {
                    int len = xcb_get_property_value_length(reply);
                    if (len > 0) {
                        sn.property = sre->property;
                    }
                    free(reply);
                }
            }
        }

        xcb_send_event(selection->xwm->connection, false, sre->requestor,
                       0, (const char *)&sn);
        xcb_flush(selection->xwm->connection);
        break;

    case XCB_SELECTION_NOTIFY:
        break;

    default:
        return 0;
    }

    return 1;
}

int selection_acquire(struct selection *selection, xcb_atom_t atom) {
    xcb_get_selection_owner_cookie_t cookie;
    xcb_get_selection_owner_reply_t *reply;

    if (!selection || !selection->xwm) {
        return -1;
    }

    cookie = xcb_get_selection_owner(selection->xwm->connection, atom);
    reply = xcb_get_selection_owner_reply(selection->xwm->connection, cookie, NULL);
    if (!reply) {
        return -1;
    }

    xcb_window_t current_owner = reply->owner;
    free(reply);

    if (current_owner != XCB_NONE && current_owner != selection->xwm->grab_window) {
        xcb_set_selection_owner(selection->xwm->connection,
                                 selection->xwm->grab_window,
                                 atom,
                                 XCB_CURRENT_TIME);
    } else {
        xcb_set_selection_owner(selection->xwm->connection,
                                 selection->xwm->grab_window,
                                 atom,
                                 XCB_CURRENT_TIME);
    }

    selection->serial++;
    selection->active = true;

    xcb_flush(selection->xwm->connection);
    return 0;
}

int selection_send(struct selection *selection, xcb_window_t requestor,
                    xcb_atom_t target, xcb_atom_t property) {
    xcb_selection_request_event_t event;

    if (!selection || !selection->xwm) {
        return -1;
    }

    memset(&event, 0, sizeof(event));
    event.response_type = XCB_SELECTION_REQUEST;
    event.time = XCB_CURRENT_TIME;
    event.owner = selection->xwm->grab_window;
    event.requestor = requestor;
    event.selection = selection->clipboard_atom;
    event.target = target;
    event.property = property;

    xcb_send_event(selection->xwm->connection, false, requestor,
                   XCB_EVENT_MASK_PROPERTY_CHANGE,
                   (const char *)&event);
    xcb_flush(selection->xwm->connection);

    return 0;
}

void selection_notify(struct selection *selection, xcb_window_t requestor,
                       xcb_atom_t property) {
    xcb_selection_notify_event_t sn;

    if (!selection || !selection->xwm) {
        return;
    }

    memset(&sn, 0, sizeof(sn));
    sn.response_type = XCB_SELECTION_NOTIFY;
    sn.time = XCB_CURRENT_TIME;
    sn.requestor = requestor;
    sn.selection = selection->clipboard_atom;
    sn.target = XCB_NONE;
    sn.property = property;

    xcb_send_event(selection->xwm->connection, false, requestor,
                   0, (const char *)&sn);
    xcb_flush(selection->xwm->connection);
}

void selection_add_offer(struct selection *selection, xcb_atom_t target,
                          const char *mime_type) {
    struct selection_offer *offer;

    if (!selection || !mime_type) {
        return;
    }

    offer = calloc(1, sizeof(struct selection_offer));
    if (!offer) {
        return;
    }

    offer->target = target;
    offer->mime_type = strdup(mime_type);

    wl_list_insert(&selection->offers, &offer->link);
}

void selection_clear(struct selection *selection) {
    struct selection_offer *offer, *tmp;

    if (!selection) {
        return;
    }

    wl_list_for_each_safe(offer, tmp, &selection->offers, link) {
        wl_list_remove(&offer->link);
        free(offer->mime_type);
        free(offer);
    }

    wl_list_init(&selection->offers);
    selection->active = false;
}

bool selection_has_owner(struct selection *selection, xcb_atom_t atom) {
    xcb_get_selection_owner_cookie_t cookie;
    xcb_get_selection_owner_reply_t *reply;

    if (!selection || !selection->xwm) {
        return false;
    }

    cookie = xcb_get_selection_owner(selection->xwm->connection, atom);
    reply = xcb_get_selection_owner_reply(selection->xwm->connection, cookie, NULL);
    if (!reply) {
        return false;
    }

    bool has = (reply->owner != XCB_NONE);
    free(reply);
    return has;
}
