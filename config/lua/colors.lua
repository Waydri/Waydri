local M = {}

function M.parse(hex)
    hex = hex:gsub("^#", "")
    if #hex == 3 then
        hex = hex:sub(1,1) .. hex:sub(1,1) .. hex:sub(2,2) .. hex:sub(2,2) .. hex:sub(3,3) .. hex:sub(3,3)
    end
    local r = tonumber(hex:sub(1, 2), 16) or 0
    local g = tonumber(hex:sub(3, 4), 16) or 0
    local b = tonumber(hex:sub(5, 6), 16) or 0
    local a = 255
    if #hex == 8 then
        a = tonumber(hex:sub(7, 8), 16) or 255
    end
    return { r = r, g = g, b = b, a = a }
end

function M.to_hex(color)
    return string.format("#%02x%02x%02x%02x", color.r, color.g, color.b, color.a or 255)
end

function M.blend(c1, c2, t)
    local r = math.floor(c1.r + (c2.r - c1.r) * t)
    local g = math.floor(c1.g + (c2.g - c1.g) * t)
    local b = math.floor(c1.b + (c2.b - c1.b) * t)
    local a = math.floor((c1.a or 255) + ((c2.a or 255) - (c1.a or 255)) * t)
    r = math.max(0, math.min(255, r))
    g = math.max(0, math.min(255, g))
    b = math.max(0, math.min(255, b))
    a = math.max(0, math.min(255, a))
    return { r = r, g = g, b = b, a = a }
end

function M.brighten(c, factor)
    local r = math.floor(c.r + (255 - c.r) * factor)
    local g = math.floor(c.g + (255 - c.g) * factor)
    local b = math.floor(c.b + (255 - c.b) * factor)
    r = math.max(0, math.min(255, r))
    g = math.max(0, math.min(255, g))
    b = math.max(0, math.min(255, b))
    return { r = r, g = g, b = b, a = c.a or 255 }
end

function M.darken(c, factor)
    local r = math.floor(c.r * (1 - factor))
    local g = math.floor(c.g * (1 - factor))
    local b = math.floor(c.b * (1 - factor))
    r = math.max(0, math.min(255, r))
    g = math.max(0, math.min(255, g))
    b = math.max(0, math.min(255, b))
    return { r = r, g = g, b = b, a = c.a or 255 }
end

function M.to_rgba(color, alpha)
    return string.format("rgba(%d, %d, %d, %.2f)",
        color.r, color.g, color.b, (alpha or (color.a or 255)) / 255)
end

function M.to_rgb(color)
    return string.format("rgb(%d, %d, %d)", color.r, color.g, color.b)
end

function M.equal(c1, c2)
    return c1.r == c2.r and c1.g == c2.g and c1.b == c2.b and (c1.a or 255) == (c2.a or 255)
end

function M.copy(c)
    return { r = c.r, g = c.g, b = c.b, a = c.a or 255 }
end

return M
