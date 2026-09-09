local M = {}

M.height = 28
M.position = "top"
M.modules = { "workspaces", "clock", "battery", "network", "volume" }
M.background = "#1e1e2e"
M.text_color = "#cdd6f4"
M.font = "monospace 10"
M.padding = 8
M.spacing = 16
M.visible = true

function M.set_position(pos)
    if pos == "top" or pos == "bottom" then
        M.position = pos
    end
end

function M.set_height(h)
    if h > 0 and h <= 100 then
        M.height = h
    end
end

function M.add_module(name)
    for _, m in ipairs(M.modules) do
        if m == name then
            return
        end
    end
    M.modules[#M.modules + 1] = name
end

function M.remove_module(name)
    for i, m in ipairs(M.modules) do
        if m == name then
            table.remove(M.modules, i)
            return true
        end
    end
    return false
end

function M.toggle()
    M.visible = not M.visible
end

function M.get_module_list()
    local copy = {}
    for i, m in ipairs(M.modules) do
        copy[i] = m
    end
    return copy
end

return M
