local M = {}

M.name = "grid"
M.gap = 8
M.config = {}

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

    local cols = math.ceil(math.sqrt(count))
    local rows = math.ceil(count / cols)

    local cell_w = math.floor((area.width - M.gap * (cols + 1)) / cols)
    local cell_h = math.floor((area.height - M.gap * (rows + 1)) / rows)

    local result = {}

    for i = 1, count do
        local col = (i - 1) % cols
        local row = math.floor((i - 1) / cols)

        local win = windows[i]
        win.x = area.x + M.gap + col * (cell_w + M.gap)
        win.y = area.y + M.gap + row * (cell_h + M.gap)
        win.width = cell_w
        win.height = cell_h
        result[#result + 1] = win
    end

    return result
end

return M
