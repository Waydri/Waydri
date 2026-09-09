local M = {}

M.name = "custom"
M.gap = 8
M.config = {
    splits = {
        { axis = "horizontal", ratio = 0.55 },
        { axis = "vertical", ratio = 0.5 },
        { axis = "horizontal", ratio = 0.5 },
    },
}

local function build_tree(splits, area, windows, index)
    if #windows == 0 then
        return {}, index
    end

    if #splits == 0 or #windows <= 1 then
        local results = {}
        for i = index, index + #windows - 1 do
            if i <= #windows then
                local win = windows[i]
                win.x = area.x + M.gap
                win.y = area.y + M.gap
                win.width = area.width - M.gap * 2
                win.height = area.height - M.gap * 2
                results[#results + 1] = win
            end
        end
        return results, index + #windows
    end

    local split = splits[1]
    local remaining_splits = {}
    for i = 2, #splits do
        remaining_splits[#remaining_splits + 1] = splits[i]
    end

    local half = math.max(1, math.floor(#windows * split.ratio))
    local first_count = half
    local second_count = #windows - first_count

    local result = {}

    if split.axis == "horizontal" then
        local split_x = math.floor(area.width * split.ratio)

        local left_area = {
            x = area.x,
            y = area.y,
            width = split_x,
            height = area.height,
        }
        local right_area = {
            x = area.x + split_x,
            y = area.y,
            width = area.width - split_x,
            height = area.height,
        }

        local left_wins = {}
        for i = 1, first_count do
            left_wins[#left_wins + 1] = windows[i]
        end
        local right_wins = {}
        for i = first_count + 1, #windows do
            right_wins[#right_wins + 1] = windows[i]
        end

        local left_result
        left_result, index = build_tree(remaining_splits, left_area, left_wins, index)
        for _, w in ipairs(left_result) do
            result[#result + 1] = w
        end

        local right_result
        right_result, index = build_tree(remaining_splits, right_area, right_wins, index)
        for _, w in ipairs(right_result) do
            result[#result + 1] = w
        end
    else
        local split_y = math.floor(area.height * split.ratio)

        local top_area = {
            x = area.x,
            y = area.y,
            width = area.width,
            height = split_y,
        }
        local bottom_area = {
            x = area.x,
            y = area.y + split_y,
            width = area.width,
            height = area.height - split_y,
        }

        local top_wins = {}
        for i = 1, first_count do
            top_wins[#top_wins + 1] = windows[i]
        end
        local bottom_wins = {}
        for i = first_count + 1, #windows do
            bottom_wins[#bottom_wins + 1] = windows[i]
        end

        local top_result
        top_result, index = build_tree(remaining_splits, top_area, top_wins, index)
        for _, w in ipairs(top_result) do
            result[#result + 1] = w
        end

        local bottom_result
        bottom_result, index = build_tree(remaining_splits, bottom_area, bottom_wins, index)
        for _, w in ipairs(bottom_result) do
            result[#result + 1] = w
        end
    end

    return result, index
end

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

    local splits = M.config.splits or {}
    local result
    result, _ = build_tree(splits, area, windows, 1)
    return result
end

return M
