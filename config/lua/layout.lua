local M = {}

M.layouts = {}
M.current_layout = "default"
M.master_ratio_value = 0.55

function M.register(name, layout)
    if type(layout) == "table" and layout.arrange then
        M.layouts[name] = layout
    end
end

function M.arrange(name, area, windows)
    local layout = M.layouts[name]
    if layout and layout.arrange then
        return layout.arrange(area, windows)
    end
    return windows
end

function M.available()
    local names = {}
    for name, _ in pairs(M.layouts) do
        names[#names + 1] = name
    end
    return names
end

function M.set_master_ratio(r)
    if r >= 0.1 and r <= 0.9 then
        M.master_ratio_value = r
        local layout = M.layouts[M.current_layout]
        if layout then
            layout.master_ratio = r
        end
    end
end

function M.master_ratio()
    return M.master_ratio_value
end

function M.set_current(name)
    if M.layouts[name] then
        M.current_layout = name
        return true
    end
    return false
end

function M.get_current()
    return M.current_layout
end

function M.next_layout()
    local names = M.available()
    if #names == 0 then
        return
    end
    local current_idx = 1
    for i, name in ipairs(names) do
        if name == M.current_layout then
            current_idx = i
            break
        end
    end
    local next_idx = (current_idx % #names) + 1
    M.current_layout = names[next_idx]
    return names[next_idx]
end

function M.prev_layout()
    local names = M.available()
    if #names == 0 then
        return
    end
    local current_idx = 1
    for i, name in ipairs(names) do
        if name == M.current_layout then
            current_idx = i
            break
        end
    end
    local prev_idx = current_idx - 1
    if prev_idx < 1 then
        prev_idx = #names
    end
    M.current_layout = names[prev_idx]
    return names[prev_idx]
end

function M.get_gap(name)
    local layout = M.layouts[name or M.current_layout]
    if layout then
        return layout.gap or 0
    end
    return 0
end

return M
