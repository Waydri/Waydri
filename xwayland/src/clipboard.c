#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <errno.h>
#include <xcb/xcb.h>
#include <wayland-server-core.h>
#include "xwayland.h"
#include "xwm.h"
#include "selection.h"

struct clipboard {
    struct xwayland *server;
    struct xwm *xwm;
    struct selection *selection;
    xcb_atom_t clipboard_atom;
    xcb_atom_t targets_atom;
    xcb_atom_t atom_text_plain;
    xcb_atom_t atom_text_uri;
    xcb_atom_t atom_utf8;
    xcb_window_t owner;
    xcb_window_t requestor;
    xcb_atom_t property;
    bool owns_clipboard;
    struct wl_list pending_transfers;
};

struct pending_transfer {
    xcb_window_t requestor;
    xcb_atom_t target;
    xcb_atom_t property;
    int fd;
    struct wl_list link;
};

static const char *mime_to_atom_name(const char *mime_type) {
    if (strcmp(mime_type, "text/plain") == 0) {
        return "text/plain";
    }
    if (strcmp(mime_type, "text/plain;charset=utf-8") == 0 ||
        strcmp(mime_type, "text/plain;charset=UTF-8") == 0) {
        return "UTF8_STRING";
    }
    if (strcmp(mime_type, "text/uri-list") == 0) {
        return "text/uri-list";
    }
    if (strcmp(mime_type, "text/html") == 0) {
        return "text/html";
    }
    if (strcmp(mime_type, "image/png") == 0) {
        return "image/png";
    }
    if (strcmp(mime_type, "text/x-moz-url") == 0) {
        return "text/x-moz-url";
    }
    return "text/plain";
}

static const char *atom_name_to_mime(xcb_atom_t atom, struct clipboard *clipboard) {
    if (atom == clipboard->atom_text_plain) {
        return "text/plain";
    }
    if (atom == clipboard->atom_utf8) {
        return "text/plain;charset=utf-8";
    }
    if (atom == clipboard->atom_text_uri) {
        return "text/uri-list";
    }
    return "text/plain";
}

struct clipboard *clipboard_create(struct xwayland *server, struct selection *sel) {
    struct clipboard *clipboard;

    clipboard = calloc(1, sizeof(struct clipboard));
    if (!clipboard) {
        return NULL;
    }

    clipboard->server = server;
    clipboard->xwm = server->xwm;
    clipboard->selection = sel;
    clipboard->owns_clipboard = false;
    clipboard->owner = XCB_NONE;
    wl_list_init(&clipboard->pending_transfers);

    if (clipboard->xwm) {
        clipboard->clipboard_atom = xcb_intern_atom(
            clipboard->xwm->connection, 0, 8, "CLIPBOARD").atom;
        clipboard->targets_atom = xcb_intern_atom(
            clipboard->xwm->connection, 0, 7, "TARGETS").atom;
        clipboard->atom_text_plain = xcb_intern_atom(
            clipboard->xwm->connection, 0, 10, "text/plain").atom;
        clipboard->atom_text_uri = xcb_intern_atom(
            clipboard->xwm->connection, 0, 12, "text/uri-list").atom;
        clipboard->atom_utf8 = xcb_intern_atom(
            clipboard->xwm->connection, 0, 11, "UTF8_STRING").atom;
    }

    return clipboard;
}

void clipboard_destroy(struct clipboard *clipboard) {
    struct pending_transfer *transfer, *tmp;

    if (!clipboard) {
        return;
    }

    wl_list_for_each_safe(transfer, tmp, &clipboard->pending_transfers, link) {
        wl_list_remove(&transfer->link);
        if (transfer->fd >= 0) {
            close(transfer->fd);
        }
        free(transfer);
    }

    free(clipboard);
}

void clipboard_handle_selection_request(struct clipboard *clipboard, xcb_selection_request_event_t *event) {
    xcb_selection_notify_event_t notify;
    xcb_get_property_cookie_t prop_cookie;
    xcb_get_property_reply_t *prop_reply;

    if (!clipboard || !clipboard->xwm) {
        return;
    }

    memset(&notify, 0, sizeof(notify));
    notify.response_type = XCB_SELECTION_NOTIFY;
    notify.sequence = 0;
    notify.time = event->time;
    notify.requestor = event->requestor;
    notify.selection = event->selection;
    notify.target = event->target;
    notify.property = XCB_NONE;

    if (event->target == clipboard->targets_atom) {
        xcb_atom_t targets[6];
        int count = 0;
        targets[count++] = clipboard->targets_atom;
        targets[count++] = clipboard->atom_text_plain;
        targets[count++] = clipboard->atom_utf8;
        targets[count++] = clipboard->atom_text_uri;

        xcb_change_property(clipboard->xwm->connection,
                            XCB_PROP_MODE_REPLACE,
                            event->requestor,
                            event->property,
                            XCB_ATOM_ATOM, 32,
                            count, targets);
        notify.property = event->property;
    } else {
        prop_cookie = xcb_get_property(clipboard->xwm->connection,
                                       0, event->requestor,
                                       event->property,
                                       XCB_GET_PROPERTY_TYPE_ANY,
                                       0, 65536);
        prop_reply = xcb_get_property_reply(clipboard->xwm->connection,
                                            prop_cookie, NULL);

        if (prop_reply) {
            free(prop_reply);
            notify.property = event->property;
        }

        const char *mime = atom_name_to_mime(event->target, clipboard);
        struct pending_transfer *transfer = calloc(1, sizeof(struct pending_transfer));
        if (transfer) {
            transfer->requestor = event->requestor;
            transfer->target = event->target;
            transfer->property = event->property;
            transfer->fd = -1;
            wl_list_insert(&clipboard->pending_transfers, &transfer->link);
        }
    }

    xcb_send_event(clipboard->xwm->connection, false,
                   event->requestor, 0,
                   (const char *)&notify);
    xcb_flush(clipboard->xwm->connection);
}

void clipboard_handle_selection_notify(struct clipboard *clipboard, xcb_selection_notify_event_t *event) {
    xcb_get_property_cookie_t prop_cookie;
    xcb_get_property_reply_t *prop_reply;
    struct pending_transfer *transfer, *tmp;

    if (!clipboard || !clipboard->xwm) {
        return;
    }

    if (event->property == XCB_NONE) {
        return;
    }

    prop_cookie = xcb_get_property(clipboard->xwm->connection,
                                   0, event->requestor,
                                   event->property,
                                   XCB_GET_PROPERTY_TYPE_ANY,
                                   0, 65536);
    prop_reply = xcb_get_property_reply(clipboard->xwm->connection,
                                        prop_cookie, NULL);
    if (!prop_reply) {
        return;
    }

    int value_len = xcb_get_property_value_length(prop_reply);
    if (value_len <= 0) {
        free(prop_reply);
        return;
    }

    void *value = xcb_get_property_value(prop_reply);

    wl_list_for_each_safe(transfer, tmp, &clipboard->pending_transfers, link) {
        if (transfer->target == event->target) {
            wl_list_remove(&transfer->link);
            free(transfer);
            break;
        }
    }

    free(prop_reply);
}

void clipboard_set_from_wayland(struct clipboard *clipboard, const char *mime_type, int fd) {
    xcb_atom_t target_atom;
    const char *atom_name;
    char *data;
    ssize_t total = 0;
    ssize_t n;
    char buf[4096];

    if (!clipboard || !clipboard->xwm || fd < 0) {
        if (fd >= 0) {
            close(fd);
        }
        return;
    }

    atom_name = mime_to_atom_name(mime_type);
    target_atom = xcb_intern_atom(clipboard->xwm->connection,
                                  0, strlen(atom_name), atom_name).atom;

    data = malloc(65536);
    if (!data) {
        close(fd);
        return;
    }

    while ((n = read(fd, buf, sizeof(buf))) > 0) {
        if (total + n > 65536) {
            break;
        }
        memcpy(data + total, buf, n);
        total += n;
    }
    close(fd);

    if (total > 0) {
        xcb_change_property(clipboard->xwm->connection,
                            XCB_PROP_MODE_REPLACE,
                            clipboard->xwm->screen->root,
                            clipboard->clipboard_atom,
                            target_atom, 8,
                            total, data);

        xcb_set_selection_owner(clipboard->xwm->connection,
                                clipboard->xwm->grab_window,
                                clipboard->clipboard_atom,
                                XCB_CURRENT_TIME);
        clipboard->owns_clipboard = true;
    }

    free(data);
    xcb_flush(clipboard->xwm->connection);
}

void clipboard_get_to_wayland(struct clipboard *clipboard) {
    xcb_get_selection_owner_cookie_t cookie;
    xcb_get_selection_owner_reply_t *reply;
    xcb_selection_request_event_t event;

    if (!clipboard || !clipboard->xwm) {
        return;
    }

    cookie = xcb_get_selection_owner(clipboard->xwm->connection,
                                     clipboard->clipboard_atom);
    reply = xcb_get_selection_owner_reply(clipboard->xwm->connection,
                                         cookie, NULL);
    if (!reply) {
        return;
    }

    xcb_window_t owner = reply->owner;
    free(reply);

    if (owner == XCB_NONE) {
        return;
    }

    memset(&event, 0, sizeof(event));
    event.response_type = XCB_SELECTION_REQUEST;
    event.time = XCB_CURRENT_TIME;
    event.owner = owner;
    event.requestor = clipboard->xwm->grab_window;
    event.selection = clipboard->clipboard_atom;
    event.target = clipboard->atom_utf8;
    event.property = xcb_intern_atom(clipboard->xwm->connection,
                                     0, 14, "WL_DATA_PROPERTY").atom;

    xcb_send_event(clipboard->xwm->connection, false, owner,
                   XCB_EVENT_MASK_PROPERTY_CHANGE,
                   (const char *)&event);
    xcb_flush(clipboard->xwm->connection);
}
