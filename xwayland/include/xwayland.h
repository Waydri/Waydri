#ifndef WAYDRI_XWAYLAND_H
#define WAYDRI_XWAYLAND_H

#include <sys/types.h>
#include <stdint.h>
#include <stdbool.h>
#include <wayland-server-core.h>

struct xwayland;

typedef void (*xwayland_event_handler_t)(struct xwayland *server, uint32_t events);

struct xwayland_client {
    pid_t pid;
    int fd;
    struct wl_list link;
};

struct xwayland {
    struct wl_display *wl_display;
    struct wl_event_loop *event_loop;
    pid_t xwayland_pid;
    int listen_fd;
    int wm_fd;
    int ready_fd;
    char display_name[32];
    struct wl_list clients;
    struct xwm *xwm;
    void *compositor;
    struct wl_event_source *ready_event_source;
    struct wl_event_source *listen_event_source;
    bool started;
    xwayland_event_handler_t event_handler;
    void *event_handler_data;
};

struct xwayland *xwayland_create(struct wl_display *display, void *compositor);
void xwayland_destroy(struct xwayland *server);
int xwayland_handle_event(struct xwayland *server, int epoll_fd);
const char *xwayland_get_display(struct xwayland *server);
int xwayland_connect_client(struct xwayland *server);
void xwayland_set_event_handler(struct xwayland *server, xwayland_event_handler_t handler, void *data);
bool xwayland_is_ready(struct xwayland *server);

#endif
