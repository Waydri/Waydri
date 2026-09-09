#include <stdlib.h>
#include <string.h>
#include <xcb/xcb.h>
#include <xcb/randr.h>
#include "xwayland.h"
#include "xwm.h"

struct randr_mode {
    xcb_randr_mode_t id;
    char name[64];
    uint16_t width;
    uint16_t height;
    uint32_t refresh;
    uint32_t mode_flags;
    struct randr_mode *next;
};

struct randr_output {
    xcb_randr_output_t id;
    char name[64];
    char manufacturer[64];
    uint16_t mm_width;
    uint16_t mm_height;
    bool connected;
    bool primary;
    struct randr_mode *modes;
    int mode_count;
    struct randr_output *next;
};

struct randr {
    struct xwayland *server;
    struct xwm *xwm;
    xcb_randr_output_t primary_output;
    struct randr_output *outputs;
    int output_count;
    bool has_randr;
    uint16_t screen_width;
    uint16_t screen_height;
};

static void randr_free_modes(struct randr_output *output) {
    struct randr_mode *mode, *tmp;
    mode = output->modes;
    while (mode) {
        tmp = mode->next;
        free(mode);
        mode = tmp;
    }
    output->modes = NULL;
    output->mode_count = 0;
}

static void randr_fetch_modes(struct randr *randr, struct randr_output *output) {
    xcb_randr_get_screen_resources_cookie_t res_cookie;
    xcb_randr_get_screen_resources_reply_t *res_reply;
    xcb_randr_mode_info_t *mode_infos;
    int mode_count;

    if (!randr->xwm || !randr->xwm->connection) {
        return;
    }

    res_cookie = xcb_randr_get_screen_resources(randr->xwm->connection,
                                                  randr->xwm->screen->root);
    res_reply = xcb_randr_get_screen_resources_reply(randr->xwm->connection,
                                                      res_cookie, NULL);
    if (!res_reply) {
        return;
    }

    mode_infos = xcb_randr_get_screen_resources_modes(res_reply);
    mode_count = xcb_randr_get_screen_resources_modes_length(res_reply);

    randr_free_modes(output);

    for (int i = 0; i < mode_count; i++) {
        struct randr_mode *mode = calloc(1, sizeof(struct randr_mode));
        if (!mode) {
            continue;
        }

        mode->id = mode_infos[i].id;
        mode->width = mode_infos[i].width;
        mode->height = mode_infos[i].height;
        mode->refresh = mode_infos[i].dot_clock;
        if (mode_infos[i].htotal > 0) {
            mode->refresh = mode_infos[i].dot_clock * 1000 /
                            (mode_infos[i].htotal * mode_infos[i].vtotal);
        }
        mode->mode_flags = mode_infos[i].mode_flags;

        snprintf(mode->name, sizeof(mode->name), "%dx%d@%d",
                 mode->width, mode->height, mode->refresh / 1000);

        mode->next = output->modes;
        output->modes = mode;
        output->mode_count++;
    }

    free(res_reply);
}

struct randr *randr_create(struct xwayland *server) {
    struct randr *randr;
    xcb_generic_error_t *error;

    randr = calloc(1, sizeof(struct randr));
    if (!randr) {
        return NULL;
    }

    randr->server = server;
    randr->xwm = server ? server->xwm : NULL;
    randr->outputs = NULL;
    randr->output_count = 0;
    randr->primary_output = XCB_NONE;
    randr->has_randr = false;

    if (randr->xwm && randr->xwm->connection) {
        const xcb_query_extension_reply_t *ext;
        ext = xcb_get_extension_data(randr->xwm->connection, &xcb_randr_id);

        if (ext && ext->present) {
            xcb_randr_query_version_cookie_t cookie;
            xcb_randr_query_version_reply_t *reply;

            cookie = xcb_randr_query_version(randr->xwm->connection, 1, 5);
            reply = xcb_randr_query_version_reply(randr->xwm->connection, cookie, &error);
            if (reply) {
                randr->has_randr = true;
                free(reply);
            }
        }

        if (randr->has_randr) {
            randr->screen_width = randr->xwm->screen_width;
            randr->screen_height = randr->xwm->screen_height;

            xcb_randr_select_input(randr->xwm->connection,
                                    randr->xwm->screen->root,
                                    XCB_RRandR_NOTIFY_MASK_SCREEN_CHANGE);

            xcb_randr_get_screen_resources_cookie_t res_cookie;
            xcb_randr_get_screen_resources_reply_t *res_reply;

            res_cookie = xcb_randr_get_screen_resources(randr->xwm->connection,
                                                          randr->xwm->screen->root);
            res_reply = xcb_randr_get_screen_resources_reply(randr->xwm->connection,
                                                              res_cookie, NULL);
            if (res_reply) {
                xcb_randr_output_t *outputs;
                int count;

                outputs = xcb_randr_get_screen_resources_outputs(res_reply);
                count = xcb_randr_get_screen_resources_outputs_length(res_reply);

                for (int i = 0; i < count; i++) {
                    struct randr_output *output = calloc(1, sizeof(struct randr_output));
                    if (!output) {
                        continue;
                    }

                    output->id = outputs[i];

                    xcb_randr_get_output_info_cookie_t info_cookie;
                    xcb_randr_get_output_info_reply_t *info_reply;

                    info_cookie = xcb_randr_get_output_info(randr->xwm->connection,
                                                              outputs[i],
                                                              XCB_CURRENT_TIME);
                    info_reply = xcb_randr_get_output_info_reply(randr->xwm->connection,
                                                                  info_cookie, NULL);
                    if (info_reply) {
                        char *name = xcb_randr_get_output_info_name(info_reply);
                        int name_len = xcb_randr_get_output_info_name_length(info_reply);
                        if (name_len > 63) name_len = 63;
                        memcpy(output->name, name, name_len);
                        output->name[name_len] = '\0';

                        output->mm_width = info_reply->mm_width;
                        output->mm_height = info_reply->mm_height;
                        output->connected = (info_reply->connection == XCB_RANDR_CONNECTION_CONNECTED);

                        free(info_reply);
                    }

                    output->primary = false;
                    output->modes = NULL;
                    output->mode_count = 0;

                    if (output->connected) {
                        randr_fetch_modes(randr, output);
                    }

                    output->next = randr->outputs;
                    randr->outputs = output;
                    randr->output_count++;
                }

                free(res_reply);
            }

            xcb_randr_get_output_primary_cookie_t prim_cookie;
            xcb_randr_get_output_primary_reply_t *prim_reply;

            prim_cookie = xcb_randr_get_output_primary(randr->xwm->connection,
                                                         randr->xwm->screen->root);
            prim_reply = xcb_randr_get_output_primary_reply(randr->xwm->connection,
                                                             prim_cookie, NULL);
            if (prim_reply) {
                randr->primary_output = prim_reply->output;
                free(prim_reply);
            }

            struct randr_output *out = randr->outputs;
            while (out) {
                if (out->id == randr->primary_output) {
                    out->primary = true;
                    break;
                }
                out = out->next;
            }
        }
    }

    return randr;
}

void randr_destroy(struct randr *randr) {
    struct randr_output *output, *tmp;

    if (!randr) {
        return;
    }

    output = randr->outputs;
    while (output) {
        tmp = output->next;
        randr_free_modes(output);
        free(output);
        output = tmp;
    }

    free(randr);
}

struct randr_mode *randr_get_modes(struct randr *randr, xcb_randr_output_t output_id) {
    struct randr_output *output;

    if (!randr) {
        return NULL;
    }

    output = randr->outputs;
    while (output) {
        if (output->id == output_id) {
            return output->modes;
        }
        output = output->next;
    }

    return NULL;
}

int randr_set_mode(struct randr *randr, xcb_randr_output_t output_id,
                    xcb_randr_mode_t mode_id) {
    xcb_randr_set_config_cookie_t cookie;
    xcb_randr_set_config_reply_t *reply;
    xcb_generic_error_t *error;

    if (!randr || !randr->xwm || !randr->xwm->connection) {
        return -1;
    }

    cookie = xcb_randr_set_screen_config(randr->xwm->connection,
                                           randr->xwm->screen->root,
                                           XCB_CURRENT_TIME,
                                           randr->xwm->screen->root,
                                           mode_id,
                                           randr->xwm->screen->root,
                                           randr->screen_width,
                                           randr->screen_height,
                                           randr->xwm->screen->root);
    reply = xcb_randr_set_screen_config_reply(randr->xwm->connection,
                                               cookie, &error);
    free(reply);

    if (error) {
        free(error);
        return -1;
    }

    return 0;
}

struct randr_output *randr_get_output_info(struct randr *randr,
                                            xcb_randr_output_t output_id) {
    struct randr_output *output;

    if (!randr) {
        return NULL;
    }

    output = randr->outputs;
    while (output) {
        if (output->id == output_id) {
            return output;
        }
        output = output->next;
    }

    return NULL;
}

int randr_add_output(struct randr *randr, const char *name,
                      uint16_t width, uint16_t height, uint32_t refresh) {
    xcb_randr_add_output_cookie_t cookie;
    xcb_randr_add_output_reply_t *reply;
    xcb_generic_error_t *error;
    xcb_randr_output_t output_id;

    if (!randr || !randr->xwm || !randr->xwm->connection || !name) {
        return -1;
    }

    output_id = xcb_generate_id(randr->xwm->connection);

    cookie = xcb_randr_add_output(randr->xwm->connection,
                                    output_id,
                                    randr->xwm->screen->root,
                                    strlen(name), name);
    reply = xcb_randr_add_output_reply(randr->xwm->connection,
                                        cookie, &error);

    if (error) {
        free(error);
        return -1;
    }
    free(reply);

    struct randr_mode *mode = calloc(1, sizeof(struct randr_mode));
    if (mode) {
        xcb_randr_create_mode_cookie_t mode_cookie;
        xcb_randr_create_mode_reply_t *mode_reply;

        xcb_randr_mode_info_t mode_info;
        memset(&mode_info, 0, sizeof(mode_info));
        mode_info.width = width;
        mode_info.height = height;
        mode_info.dot_clock = refresh * (uint32_t)width * (uint32_t)height;
        mode_info.htotal = width;
        mode_info.vtotal = height;

        mode_cookie = xcb_randr_create_mode(randr->xwm->connection,
                                              output_id,
                                              strlen(name), name,
                                              &mode_info);
        mode_reply = xcb_randr_create_mode_reply(randr->xwm->connection,
                                                  mode_cookie, NULL);
        if (mode_reply) {
            mode->id = mode_reply->mode;
            free(mode_reply);
        }

        mode->width = width;
        mode->height = height;
        mode->refresh = refresh;
        snprintf(mode->name, sizeof(mode->name), "%s", name);

        struct randr_output *output = calloc(1, sizeof(struct randr_output));
        if (output) {
            output->id = output_id;
            snprintf(output->name, sizeof(output->name), "%s", name);
            output->connected = true;
            output->primary = false;
            output->modes = mode;
            output->mode_count = 1;
            output->next = randr->outputs;
            randr->outputs = output;
            randr->output_count++;
        } else {
            free(mode);
        }
    }

    xcb_flush(randr->xwm->connection);
    return 0;
}

xcb_randr_output_t randr_get_primary(struct randr *randr) {
    if (!randr) {
        return XCB_NONE;
    }
    return randr->primary_output;
}
