#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <xcb/xcb.h>
#include "xwayland.h"
#include "xwm.h"

enum dnd_action {
    DND_ACTION_NONE = 0,
    DND_ACTION_COPY = 1,
    DND_ACTION_MOVE = 2,
    DND_ACTION_ASK = 4,
    DND_ACTION_PRIVATE = 8,
};

struct dnd_drop {
    xcb_window_t source;
    xcb_window_t target;
    int32_t x, y;
    uint32_t timestamp;
    enum dnd_action action;
    bool has_data;
};

struct dnd {
    struct xwayland *server;
    struct xwm *xwm;
    xcb_atom_t xdnd_aware;
    xcb_atom_t xdnd_enter;
    xcb_atom_t xdnd_leave;
    xcb_atom_t xdnd_position;
    xcb_atom_t xdnd_drop;
    xcb_atom_t xdnd_status;
    xcb_atom_t xdnd_finished;
    xcb_atom_t xdnd_action_copy;
    xcb_atom_t xdnd_action_move;
    xcb_atom_t xdnd_action_ask;
    xcb_atom_t xdnd_action_private;
    xcb_atom_t xdnd_selection;
    xcb_atom_t xdnd_type_list;
    struct dnd_drop pending_drop;
    bool active;
    xcb_window_t source_window;
    xcb_window_t target_window;
    int32_t last_x, last_y;
    xcb_timestamp_t last_time;
};

static void dnd_send_status(struct dnd *dnd, bool accepted, enum dnd_action action) {
    xcb_client_message_event_t event;
    memset(&event, 0, sizeof(event));

    event.response_type = XCB_CLIENT_MESSAGE;
    event.window = dnd->source_window;
    event.type = dnd->xdnd_status;
    event.format = 32;
    event.data.data32[0] = dnd->target_window;
    event.data.data32[1] = accepted ? 1 : 0;
    event.data.data32[2] = 0;
    event.data.data32[3] = 0;
    event.data.data32[4] = accepted ? action : DND_ACTION_NONE;

    xcb_send_event(dnd->xwm->connection, false, dnd->source_window,
                   XCB_EVENT_MASK_NO_EVENT, (const char *)&event);
    xcb_flush(dnd->xwm->connection);
}

static void dnd_send_position(struct dnd *dnd, int32_t x, int32_t y) {
    xcb_client_message_event_t event;
    memset(&event, 0, sizeof(event));

    event.response_type = XCB_CLIENT_MESSAGE;
    event.window = dnd->target_window;
    event.type = dnd->xdnd_position;
    event.format = 32;
    event.data.data32[0] = dnd->source_window;
    event.data.data32[1] = 0;
    event.data.data32[2] = x;
    event.data.data32[3] = y;
    event.data.data32[4] = dnd->xdnd_action_copy;

    xcb_send_event(dnd->xwm->connection, false, dnd->target_window,
                   XCB_EVENT_MASK_NO_EVENT, (const char *)&event);
    xcb_flush(dnd->xwm->connection);
}

struct dnd *dnd_create(struct xwayland *server, void *seat) {
    struct dnd *dnd;

    dnd = calloc(1, sizeof(struct dnd));
    if (!dnd) {
        return NULL;
    }

    dnd->server = server;
    dnd->xwm = server ? server->xwm : NULL;
    dnd->active = false;
    dnd->source_window = XCB_NONE;
    dnd->target_window = XCB_NONE;
    dnd->last_x = 0;
    dnd->last_y = 0;
    dnd->last_time = 0;

    memset(&dnd->pending_drop, 0, sizeof(dnd->pending_drop));

    if (dnd->xwm && dnd->xwm->connection) {
        xcb_intern_atom_cookie_t cookies[12];
        const char *names[] = {
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
        };
        xcb_intern_atom_reply_t *reply;

        for (int i = 0; i < 12; i++) {
            cookies[i] = xcb_intern_atom(dnd->xwm->connection, 0,
                                          strlen(names[i]), names[i]);
        }

        for (int i = 0; i < 12; i++) {
            reply = xcb_intern_atom_reply(dnd->xwm->connection, cookies[i], NULL);
            if (reply) {
                xcb_atom_t *dest = (xcb_atom_t *)&dnd->xdnd_aware;
                dest[i] = reply->atom;
                free(reply);
            }
        }
    }

    return dnd;
}

void dnd_destroy(struct dnd *dnd) {
    if (!dnd) {
        return;
    }

    if (dnd->active && dnd->target_window != XCB_NONE) {
        dnd_finish(dnd, false);
    }

    free(dnd);
}

void dnd_handle_enter(struct dnd *dnd, xcb_window_t window, int32_t x, int32_t y) {
    xcb_client_message_event_t *event;
    uint32_t version;
    xcb_get_property_cookie_t cookie;
    xcb_get_property_reply_t *reply;

    if (!dnd || !dnd->xwm) {
        return;
    }

    cookie = xcb_get_property(dnd->xwm->connection, 0, window,
                               dnd->xdnd_aware, XCB_ATOM_ATOM, 0, 32);
    reply = xcb_get_property_reply(dnd->xwm->connection, cookie, NULL);
    if (!reply || xcb_get_property_value_length(reply) < (int)sizeof(uint32_t)) {
        free(reply);
        return;
    }
    free(reply);

    dnd->source_window = window;
    dnd->active = true;
    dnd->last_x = x;
    dnd->last_y = y;

    dnd_send_position(dnd, x, y);
}

void dnd_handle_position(struct dnd *dnd, xcb_window_t window, int32_t x, int32_t y) {
    enum dnd_action action;
    xcb_get_property_cookie_t cookie;
    xcb_get_property_reply_t *reply;

    if (!dnd || !dnd->xwm || !dnd->active) {
        return;
    }

    dnd->last_x = x;
    dnd->last_y = y;
    dnd->last_time = XCB_CURRENT_TIME;

    action = DND_ACTION_COPY;

    cookie = xcb_get_property(dnd->xwm->connection, 0, window,
                               dnd->xdnd_type_list, XCB_ATOM_ATOM, 0, 128);
    reply = xcb_get_property_reply(dnd->xwm->connection, cookie, NULL);
    if (reply && xcb_get_property_value_length(reply) > 0) {
        xcb_atom_t *atoms = xcb_get_property_value(reply);
        int count = xcb_get_property_value_length(reply) / sizeof(xcb_atom_t);
        bool has_text = false;
        bool has_uri = false;

        xcb_atom_t utf8_atom = xcb_intern_atom(dnd->xwm->connection, 0, 11, "UTF8_STRING").atom;
        xcb_atom_t text_atom = xcb_intern_atom(dnd->xwm->connection, 0, 10, "text/plain").atom;
        xcb_atom_t uri_atom = xcb_intern_atom(dnd->xwm->connection, 0, 12, "text/uri-list").atom;

        for (int i = 0; i < count; i++) {
            if (atoms[i] == utf8_atom || atoms[i] == text_atom) {
                has_text = true;
            }
            if (atoms[i] == uri_atom) {
                has_uri = true;
            }
        }

        if (has_text || has_uri) {
            action = DND_ACTION_COPY;
        }
    }
    free(reply);

    dnd_send_status(dnd, true, action);
    dnd_send_position(dnd, x, y);
}

void dnd_handle_drop(struct dnd *dnd) {
    xcb_client_message_event_t event;

    if (!dnd || !dnd->xwm || !dnd->active) {
        return;
    }

    dnd->pending_drop.source = dnd->source_window;
    dnd->pending_drop.target = dnd->target_window;
    dnd->pending_drop.x = dnd->last_x;
    dnd->pending_drop.y = dnd->last_y;
    dnd->pending_drop.timestamp = dnd->last_time;
    dnd->pending_drop.action = DND_ACTION_COPY;
    dnd->pending_drop.has_data = true;

    memset(&event, 0, sizeof(event));
    event.response_type = XCB_CLIENT_MESSAGE;
    event.window = dnd->source_window;
    event.type = dnd->xdnd_selection;
    event.format = 32;
    event.data.data32[0] = dnd->xdnd_selection;
    event.data.data32[1] = xcb_intern_atom(dnd->xwm->connection, 0,
                                             12, "text/uri-list").atom;
    event.data.data32[2] = dnd->target_window;
    event.data.data32[3] = dnd->last_time;
    event.data.data32[4] = 0;

    xcb_send_event(dnd->xwm->connection, false, dnd->source_window,
                   XCB_EVENT_MASK_NO_EVENT, (const char *)&event);
    xcb_flush(dnd->xwm->connection);
}

void dnd_handle_leave(struct dnd *dnd) {
    if (!dnd) {
        return;
    }

    dnd->active = false;
    dnd->source_window = XCB_NONE;
    dnd->target_window = XCB_NONE;
    dnd->pending_drop.has_data = false;
}

void dnd_finish(struct dnd *dnd, bool success) {
    xcb_client_message_event_t event;

    if (!dnd || !dnd->xwm) {
        return;
    }

    if (dnd->source_window != XCB_NONE) {
        memset(&event, 0, sizeof(event));
        event.response_type = XCB_CLIENT_MESSAGE;
        event.window = dnd->source_window;
        event.type = dnd->xdnd_finished;
        event.format = 32;
        event.data.data32[0] = dnd->target_window;
        event.data.data32[1] = success ? 1 : 0;
        event.data.data32[2] = success ? dnd->xdnd_action_copy : 0;

        xcb_send_event(dnd->xwm->connection, false, dnd->source_window,
                       XCB_EVENT_MASK_NO_EVENT, (const char *)&event);
        xcb_flush(dnd->xwm->connection);
    }

    dnd->active = false;
    dnd->source_window = XCB_NONE;
    dnd->target_window = XCB_NONE;
    dnd->pending_drop.has_data = false;
}
