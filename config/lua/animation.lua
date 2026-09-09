local M = {}

M.duration = 300
M.easing = "ease_out_cubic"

M.workspace_switch = {
    duration = 400,
    easing = "ease_in_out_cubic",
}

M.window_open = {
    duration = 250,
    easing = "ease_out_cubic",
}

M.window_close = {
    duration = 200,
    easing = "ease_in_quad",
}

M.window_move = {
    duration = 300,
    easing = "ease_out_cubic",
}

M.window_resize = {
    duration = 250,
    easing = "ease_out_quad",
}

M.opacity_change = {
    duration = 200,
    easing = "linear",
}

function M.set_duration(d)
    M.duration = d
end

function M.set_easing(e)
    M.easing = e
end

function M.get_config(name)
    return M[name] or { duration = M.duration, easing = M.easing }
end

return M
