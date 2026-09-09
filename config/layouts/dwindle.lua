local M = {}

M.name = "dwindle"
M.gap = 8
M.ratio = 0.55
M.config = {
    ratio = 0.55,
    recursive = true,
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

    local result = {}
    local current_area = {
        x = area.x,
        y = area.y,
        width = area.width,
        height = area.height,
    }
    local horizontal = true
    local ratio = M.ratio

    for i = 1, count do
        local win = windows[i]

        if i == count then
            win.x = current_area.x + M.gap
            win.y = current_area.y + M.gap
            win.width = current_area.width - M.gap * 2
            win.height = current_area.height - M.gap * 2
        else
            if horizontal then
                local split_w = math.floor(current_area.width * ratio)
                win.x = current_area.x + M.gap
                win.y = current_area.y + M.gap
                win.width = split_w - M.gap * 2
                win.height = current_area.height - M.gap * 2

                current_area = {
                    x = current_area.x + split_w,
                    y = current_area.y,
                    width = current_area.width - split_w,
                    height = current_area.height,
                }
            else
                local split_h = math.floor(current_area.height * ratio)
                win.x = current_area.x + M.gap
                win.y = current_area.y + M.gap
                win.width = current_area.width - M.gap * 2
                win.height = split_h - M.gap * 2

                current_area = {
                    x = current_area.x,
                    y = current_area.y + split_h,
                    width = current_area.width,
                    height = current_area.height - split_h,
                }
            end

            horizontal = not horizontal
            ratio = ratio * 0.75
        end

        result[#result + 1] = win
    end

    return result
end

return M
