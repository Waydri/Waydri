local M = {}

function M.deep_merge(a, b)
    if type(a) ~= "table" then return b end
    if type(b) ~= "table" then return a end
    local result = {}
    for k, v in pairs(a) do
        if type(v) == "table" and type(b[k]) == "table" then
            result[k] = M.deep_merge(v, b[k])
        else
            result[k] = v
        end
    end
    for k, v in pairs(b) do
        if result[k] == nil then
            result[k] = v
        elseif type(v) == "table" and type(result[k]) == "table" then
            result[k] = M.deep_merge(result[k], v)
        end
    end
    return result
end

function M.shallow_copy(t)
    if type(t) ~= "table" then return t end
    local copy = {}
    for k, v in pairs(t) do
        copy[k] = v
    end
    return copy
end

function M.serialize(t)
    if type(t) == "string" then
        return '"' .. t:gsub('"', '\\"'):gsub('\n', '\\n'):gsub('\t', '\\t') .. '"'
    elseif type(t) == "number" then
        return tostring(t)
    elseif type(t) == "boolean" then
        return tostring(t)
    elseif type(t) == "nil" then
        return "nil"
    elseif type(t) == "table" then
        local parts = {}
        local is_array = #t > 0
        if is_array then
            for _, v in ipairs(t) do
                parts[#parts + 1] = M.serialize(v)
            end
            return "{" .. table.concat(parts, ", ") .. "}"
        else
            for k, v in pairs(t) do
                local key
                if type(k) == "string" and k:match("^[%a_][%w_]*$") then
                    key = k
                else
                    key = "[" .. M.serialize(k) .. "]"
                end
                parts[#parts + 1] = key .. " = " .. M.serialize(v)
            end
            return "{" .. table.concat(parts, ", ") .. "}"
        end
    end
    return "nil"
end

function M.deserialize(s)
    if type(s) ~= "string" or s == "" then
        return nil
    end
    local func, err = load("return " .. s)
    if func then
        local ok, result = pcall(func)
        if ok then
            return result
        end
    end
    return nil
end

function M.log(msg)
    io.write("[Waydri] " .. tostring(msg) .. "\n")
    io.flush()
end

function M.clamp(v, min_val, max_val)
    if v < min_val then return min_val end
    if v > max_val then return max_val end
    return v
end

function M.lerp(a, b, t)
    return a + (b - a) * t
end

function M.table_length(t)
    local count = 0
    for _ in pairs(t) do
        count = count + 1
    end
    return count
end

function M.round(v)
    return math.floor(v + 0.5)
end

function M.split(str, sep)
    local result = {}
    local pattern = "([^" .. sep .. "]+)"
    for match in str:gmatch(pattern) do
        result[#result + 1] = match
    end
    return result
end

function M.trim(s)
    return s:match("^%s*(.-)%s*$")
end

return M
