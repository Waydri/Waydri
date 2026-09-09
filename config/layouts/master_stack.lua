local M = {}

M.name = "master_stack"
M.gap = 8
M.master_ratio = 0.55
M.master_area_side = "Left"

M.config = {
    master_area_side = "Left",
    allow_split_change = true,
}

function M.arrange(area, windows)
    local count = #windows
    if count == 0 then
        return {}
    end

    if count == 1 then
        local win = windows[1]
        win.x = area.x + M.gap
        win.y = area.y + M.gap
        win.width = area.width - M.gap * 2
        win.height = area.height - M.gap * 2
        return {win}
    end

    local master_count = math.max(1, math.floor(count * M.master_ratio))
    local slave_count = count - master_count

    local side = M.master_area_side or "Left"
    local master_w, master_h, slave_w, slave_h
    local master_x, master_y, slave_x, slave_y

    if side == "Left" or side == "Right" then
        master_w = math.floor((area.width - M.gap * 3) * M.master_ratio)
        slave_w = area.width - master_w - M.gap * 3
        master_h = area.height - M.gap * 2
        slave_h = slave_count > 0 and math.floor((area.height - M.gap * (slave_count + 1)) / slave_count) or master_h

        if side == "Left" then
            master_x = area.x + M.gap
            slave_x = area.x + master_w + M.gap * 2
        else
            slave_x = area.x + M.gap
            master_x = area.x + slave_w + M.gap * 2
        end
        master_y = area.y + M.gap
    else
        master_h = math.floor((area.height - M.gap * 3) * M.master_ratio)
        slave_h = area.height - master_h - M.gap * 3
        master_w = area.width - M.gap * 2
        slave_w = slave_count > 0 and math.floor((area.width - M.gap * (slave_count + 1)) / slave_count) or master_w

        master_x = area.x + M.gap
        if side == "Top" then
            master_y = area.y + M.gap
            slave_y = area.y + master_h + M.gap * 2
        else
            slave_y = area.y + M.gap
            master_y = area.y + slave_h + M.gap * 2
        end
    end

    local result = {}

    for i = 1, master_count do
        local win = windows[i]
        win.x = master_x
        win.y = master_y
        win.width = master_w
        win.height = master_h
        result[#result + 1] = win
    end

    local offset = 0
    for i = master_count + 1, count do
        local win = windows[i]
        if side == "Left" or side == "Right" then
            win.x = slave_x
            win.y = area.y + M.gap + offset
            win.width = slave_w
            win.height = math.floor((area.height - M.gap * (slave_count + 1)) / slave_count)
        else
            win.x = area.x + M.gap + offset
            win.y = slave_y
            win.width = math.floor((area.width - M.gap * (slave_count + 1)) / slave_count)
            win.height = slave_h
        end
        offset = offset + (side == "Left" or side == "Right" and slave_h + M.gap or slave_w + M.gap)
        result[#result + 1] = win
    end

    return result
end

return M
