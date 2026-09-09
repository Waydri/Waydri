local M = {}

M.current_theme = nil
M.colors = {
    primary = "#4c7899",
    secondary = "#5a9fd4",
    accent = "#8ab4f8",
    background = "#1e1e2e",
    surface = "#2a2a3e",
    text = "#cdd6f4",
    border = "#45475a",
    shadow = "#000000",
}

M.font = {
    family = "monospace",
    size = 10,
}

M.border = {
    width = 1,
    radius = 0,
}

M.shadow = {
    enabled = false,
    offset_x = 0,
    offset_y = 2,
    blur = 8,
    color = "#000000",
}

local theme_dir = os.getenv("WAYDRI_CONFIG_DIR") or "config"
local themes_dir = theme_dir .. "/themes"

function M.load(name)
    local path = themes_dir .. "/" .. name .. ".json"
    local file = io.open(path, "r")
    if not file then
        return false
    end
    local content = file:read("*a")
    file:close()

    if not content or content == "" then
        return false
    end

    local theme = M.parse_json(content)
    if theme then
        M.current_theme = theme
        if theme.colors then
            for k, v in pairs(theme.colors) do
                M.colors[k] = v
            end
        end
        if theme.font then
            for k, v in pairs(theme.font) do
                M.font[k] = v
            end
        end
        if theme.border then
            for k, v in pairs(theme.border) do
                M.border[k] = v
            end
        end
        if theme.shadow then
            for k, v in pairs(theme.shadow) do
                M.shadow[k] = v
            end
        end
        return true
    end
    return false
end

function M.parse_json(str)
    local result = {}
    local stack = {result}
    local current = result
    local key = nil

    local i = 1
    local len = #str
    while i <= len do
        local c = str:sub(i, i)
        if c == '{' then
            local new_obj = {}
            current[key or #current + 1] = new_obj
            stack[#stack + 1] = new_obj
            current = new_obj
            key = nil
        elseif c == '}' then
            stack[#stack] = nil
            current = stack[#stack] or result
            key = nil
        elseif c == '"' then
            local j = i + 1
            while j <= len and str:sub(j, j) ~= '"' do
                if str:sub(j, j) == '\\' then j = j + 1 end
                j = j + 1
            end
            local s = str:sub(i + 1, j - 1)
            if key == nil then
                key = s
            else
                current[key] = s
                key = nil
            end
            i = j
        elseif c == ':' then
        elseif c == ',' then
        elseif c >= '0' and c <= '9' then
            local j = i
            while j <= len and ((str:sub(j, j) >= '0' and str:sub(j, j) <= '9') or str:sub(j, j) == '.') do
                j = j + 1
            end
            local num = tonumber(str:sub(i, j - 1))
            if num then
                if key then
                    current[key] = num
                    key = nil
                else
                    current[#current + 1] = num
                end
            end
            i = j - 1
        elseif str:sub(i, i + 3) == 'true' then
            if key then
                current[key] = true
                key = nil
            end
            i = i + 3
        elseif str:sub(i, i + 4) == 'false' then
            if key then
                current[key] = false
                key = nil
            end
            i = i + 4
        elseif str:sub(i, i + 3) == 'null' then
            if key then
                current[key] = nil
                key = nil
            end
            i = i + 3
        end
        i = i + 1
    end

    return result
end

function M.apply()
    if not M.current_theme then
        return false
    end
    return true
end

function M.get_color(name)
    return M.colors[name] or "#000000"
end

function M.set_color(name, value)
    M.colors[name] = value
end

return M
