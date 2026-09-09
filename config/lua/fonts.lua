local M = {}

M.default = "monospace 10"
M.title = "sans-serif bold 12"
M.fallback = { "monospace", "sans-serif" }
M.sizes = {
    small = 8,
    normal = 10,
    large = 14,
}

function M.set_default(font_str)
    M.default = font_str
end

function M.set_title(font_str)
    M.title = font_str
end

function M.set_size(name, size)
    if type(size) == "number" and size > 0 then
        M.sizes[name] = size
    end
end

function M.get_size(name)
    return M.sizes[name] or M.sizes.normal
end

function M.add_fallback(font_name)
    for _, f in ipairs(M.fallback) do
        if f == font_name then
            return
        end
    end
    M.fallback[#M.fallback + 1] = font_name
end

function M.remove_fallback(font_name)
    for i, f in ipairs(M.fallback) do
        if f == font_name then
            table.remove(M.fallback, i)
            return true
        end
    end
    return false
end

function M.to_pango(family, size, weight)
    family = family or "monospace"
    size = size or M.sizes.normal
    weight = weight or "normal"
    return family .. " " .. weight .. " " .. tostring(size)
end

function M.list_fallbacks()
    local copy = {}
    for i, f in ipairs(M.fallback) do
        copy[i] = f
    end
    return copy
end

return M
