#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <signal.h>
#include <errno.h>
#include <fcntl.h>
#include <sys/wait.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <wayland-server-core.h>
#include "xwayland.h"
#include "xwm.h"

static int handle_xwayland_ready(int fd, uint32_t mask, void *data) {
    struct xwayland *server = data;
    char buf[64];
    ssize_t n;
    char c;

    if (mask & WL_EVENT_READABLE) {
        n = read(fd, &c, 1);
        if (n == 1 && c == 'S') {
            server->started = true;
            if (server->event_handler) {
                server->event_handler(server, WL_EVENT_READABLE);
            }
            wl_event_source_remove(server->ready_event_source);
            server->ready_event_source = NULL;
            close(server->ready_fd);
            server->ready_fd = -1;
        }
    }
    return 0;
}

static int handle_xwayland_listen(int fd, uint32_t mask, void *data) {
    struct xwayland *server = data;
    int client_fd;

    if (mask & WL_EVENT_READABLE) {
        client_fd = accept(server->listen_fd, NULL, NULL);
        if (client_fd < 0) {
            return 0;
        }
        struct xwayland_client *client = calloc(1, sizeof(struct xwayland_client));
        if (!client) {
            close(client_fd);
            return 0;
        }
        client->fd = client_fd;
        client->pid = 0;
        wl_list_insert(&server->clients, &client->link);
        if (server->event_handler) {
            server->event_handler(server, WL_EVENT_READABLE);
        }
    }
    return 0;
}

static int create_socket_pair(int fds[2]) {
    if (socketpair(AF_UNIX, SOCK_STREAM, 0, fds) < 0) {
        return -1;
    }
    for (int i = 0; i < 2; i++) {
        int flags = fcntl(fds[i], F_GETFL, 0);
        if (flags < 0) {
            close(fds[0]);
            close(fds[1]);
            return -1;
        }
        if (fcntl(fds[i], F_SETFL, flags | O_CLOEXEC) < 0) {
            close(fds[0]);
            close(fds[1]);
            return -1;
        }
    }
    return 0;
}

static int create_listen_socket(const char *path) {
    struct sockaddr_un addr;
    int fd;

    fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) {
        return -1;
    }

    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    snprintf(addr.sun_path, sizeof(addr.sun_path), "%s", path);

    unlink(path);

    if (bind(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        close(fd);
        return -1;
    }

    if (listen(fd, 1) < 0) {
        close(fd);
        unlink(path);
        return -1;
    }

    return fd;
}

struct xwayland *xwayland_create(struct wl_display *display, void *compositor) {
    struct xwayland *server;
    int wm_fds[2];
    int ready_fds[2];
    pid_t pid;
    int display_num;
    char socket_path[256];
    char display_str[32];
    int listen_fd;
    int wm_fd;
    int ready_fd;

    server = calloc(1, sizeof(struct xwayland));
    if (!server) {
        return NULL;
    }

    server->wl_display = display;
    server->event_loop = wl_display_get_event_loop(display);
    server->compositor = compositor;
    server->listen_fd = -1;
    server->wm_fd = -1;
    server->ready_fd = -1;
    server->started = false;
    server->event_handler = NULL;
    server->event_handler_data = NULL;
    server->xwm = NULL;
    wl_list_init(&server->clients);

    if (create_socket_pair(wm_fds) < 0) {
        free(server);
        return NULL;
    }

    if (create_socket_pair(ready_fds) < 0) {
        close(wm_fds[0]);
        close(wm_fds[1]);
        free(server);
        return NULL;
    }

    for (display_num = 0; display_num < 32; display_num++) {
        snprintf(socket_path, sizeof(socket_path),
                 "/tmp/.x11-unix/X%d", display_num);
        unlink(socket_path);
        listen_fd = create_listen_socket(socket_path);
        if (listen_fd >= 0) {
            break;
        }
    }

    if (display_num >= 32) {
        close(wm_fds[0]);
        close(wm_fds[1]);
        close(ready_fds[0]);
        close(ready_fds[1]);
        free(server);
        return NULL;
    }

    snprintf(display_str, sizeof(display_str), ":%d", display_num);

    pid = fork();
    if (pid < 0) {
        close(listen_fd);
        unlink(socket_path);
        close(wm_fds[0]);
        close(wm_fds[1]);
        close(ready_fds[0]);
        close(ready_fds[1]);
        free(server);
        return NULL;
    }

    if (pid == 0) {
        close(wm_fds[0]);
        close(ready_fds[0]);
        close(listen_fd);

        char wm_fd_str[16];
        char ready_fd_str[16];
        snprintf(wm_fd_str, sizeof(wm_fd_str), "%d", wm_fds[1]);
        snprintf(ready_fd_str, sizeof(ready_fd_str), "%d", ready_fds[1]);

        setenv("DISPLAY", display_str, 1);

        const char *wayland_display = getenv("WAYLAND_DISPLAY");
        if (!wayland_display) {
            wayland_display = "wayland-0";
        }
        setenv("WAYLAND_DISPLAY", wayland_display, 1);

        setenv("XWAYLAND_WM_FD", wm_fd_str, 1);
        setenv("XWAYLAND_READY_FD", ready_fd_str, 1);

        const char *xwayland_path = getenv("XWAYLAND_PATH");
        if (!xwayland_path) {
            xwayland_path = "/usr/bin/Xwayland";
        }

        close(wm_fds[1]);
        close(ready_fds[1]);

        execlp(xwayland_path, xwayland_path,
               display_str,
               "-rootless",
               "-listen", socket_path,
               "-wm", wm_fd_str,
               "-noreset",
               "+extension", "GLX",
               "+extension", "Composite",
               "+extension", "RENDER",
               "+extension", "RANDR",
               "+extension", "FIXES",
               "+extension", "SHAPE",
               "+extension", "XTEST",
               NULL);

        _exit(1);
    }

    close(wm_fds[1]);
    close(ready_fds[1]);

    wm_fd = wm_fds[0];
    ready_fd = ready_fds[0];

    server->xwayland_pid = pid;
    server->wm_fd = wm_fd;
    server->ready_fd = ready_fd;
    server->listen_fd = listen_fd;
    snprintf(server->display_name, sizeof(server->display_name), "%s", display_str);

    server->ready_event_source = wl_event_loop_add_fd(
        server->event_loop, ready_fd, WL_EVENT_READABLE,
        handle_xwayland_ready, server);

    server->listen_event_source = wl_event_loop_add_fd(
        server->event_loop, listen_fd, WL_EVENT_READABLE,
        handle_xwayland_listen, server);

    server->xwm = xwm_create(server, compositor);

    return server;
}

void xwayland_destroy(struct xwayland *server) {
    struct xwayland_client *client, *tmp;
    int status;

    if (!server) {
        return;
    }

    if (server->xwm) {
        xwm_destroy(server->xwm);
        server->xwm = NULL;
    }

    wl_list_for_each_safe(client, tmp, &server->clients, link) {
        wl_list_remove(&client->link);
        close(client->fd);
        free(client);
    }

    if (server->ready_event_source) {
        wl_event_source_remove(server->ready_event_source);
        server->ready_event_source = NULL;
    }

    if (server->listen_event_source) {
        wl_event_source_remove(server->listen_event_source);
        server->listen_event_source = NULL;
    }

    if (server->ready_fd >= 0) {
        close(server->ready_fd);
        server->ready_fd = -1;
    }

    if (server->wm_fd >= 0) {
        close(server->wm_fd);
        server->wm_fd = -1;
    }

    if (server->listen_fd >= 0) {
        close(server->listen_fd);
        server->listen_fd = -1;
        char socket_path[256];
        snprintf(socket_path, sizeof(socket_path),
                 "/tmp/.x11-unix/X%s", server->display_name + 1);
        unlink(socket_path);
    }

    if (server->xwayland_pid > 0) {
        kill(server->xwayland_pid, SIGTERM);
        waitpid(server->xwayland_pid, &status, 0);
        server->xwayland_pid = 0;
    }

    free(server);
}

int xwayland_handle_event(struct xwayland *server, int epoll_fd) {
    char buf[4096];
    ssize_t n;

    if (!server || server->wm_fd < 0) {
        return -1;
    }

    n = read(server->wm_fd, buf, sizeof(buf));
    if (n < 0) {
        if (errno == EAGAIN || errno == EWOULDBLOCK) {
            return 0;
        }
        return -1;
    }

    if (n == 0) {
        return -1;
    }

    if (server->xwm && server->started) {
        return xwm_handle_event(server->xwm);
    }

    return 0;
}

const char *xwayland_get_display(struct xwayland *server) {
    if (!server) {
        return NULL;
    }
    return server->display_name;
}

int xwayland_connect_client(struct xwayland *server) {
    struct sockaddr_un addr;
    int fd;

    if (!server || server->listen_fd < 0) {
        return -1;
    }

    fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0) {
        return -1;
    }

    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    snprintf(addr.sun_path, sizeof(addr.sun_path),
             "/tmp/.x11-unix/X%s", server->display_name + 1);

    if (connect(fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        close(fd);
        return -1;
    }

    return fd;
}

void xwayland_set_event_handler(struct xwayland *server, xwayland_event_handler_t handler, void *data) {
    if (!server) {
        return;
    }
    server->event_handler = handler;
    server->event_handler_data = data;
}

bool xwayland_is_ready(struct xwayland *server) {
    if (!server) {
        return false;
    }
    return server->started;
}
