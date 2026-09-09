local M = {}

M.swipe_threshold = 50
M.pinch_threshold = 0.1
M.long_press_duration = 500

M.three_finger_actions = {
    swipe_up = "overview",
    swipe_down = "taskbar",
    swipe_left = "prev_workspace",
    swipe_right = "next_workspace",
}

M.four_finger_actions = {
    swipe_up = "overview_all",
    swipe_down = "show_desktop",
    swipe_left = "next_workspace",
    swipe_right = "prev_workspace",
}

M.two_finger_actions = {
    swipe_up = "scroll_up",
    swipe_down = "scroll_down",
    swipe_left = "scroll_left",
    swipe_right = "scroll_right",
}

M.pinch_actions = {
    zoom_in = "zoom_in",
    zoom_out = "zoom_out",
}

M.set_threshold = function(threshold)
    if threshold > 0 then
        M.swipe_threshold = threshold
    end
end

M.set_long_press = function(duration)
    if duration > 0 then
        M.long_press_duration = duration
    end
end

M.get_action = function(fingers, direction)
    local actions = nil
    if fingers == 2 then
        actions = M.two_finger_actions
    elseif fingers == 3 then
        actions = M.three_finger_actions
    elseif fingers == 4 then
        actions = M.four_finger_actions
    end
    if actions then
        return actions[direction]
    end
    return nil
end

M.get_pinch_action = function(scale)
    if scale > 1 + M.pinch_threshold then
        return M.pinch_actions.zoom_in
    elseif scale < 1 - M.pinch_threshold then
        return M.pinch_actions.zoom_out
    end
    return nil
end

return M
