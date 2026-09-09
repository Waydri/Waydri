local M = {}

function M.devices()
    local result = {}
    local handle = io.popen("bluetoothctl devices 2>/dev/null")
    if handle then
        for line in handle:lines() do
            local addr, name = line:match("^Device (%S+) (.+)$")
            if addr then
                result[#result + 1] = {
                    address = addr,
                    name = name,
                }
            end
        end
        handle:close()
    end
    return result
end

function M.connect(addr)
    if not addr then return false end
    local handle = io.popen("bluetoothctl connect " .. addr .. " 2>/dev/null")
    if handle then
        local result = handle:read("*a")
        handle:close()
        return result and result:match("Connection successful") ~= nil
    end
    return false
end

function M.disconnect(addr)
    if not addr then return false end
    local handle = io.popen("bluetoothctl disconnect " .. addr .. " 2>/dev/null")
    if handle then
        local result = handle:read("*a")
        handle:close()
        return result and result:match("Successful disconnected") ~= nil
    end
    return false
end

function M.pair(addr)
    if not addr then return false end
    local handle = io.popen("bluetoothctl pair " .. addr .. " 2>/dev/null")
    if handle then
        local result = handle:read("*a")
        handle:close()
        return result and result:match("Pairing successful") ~= nil
    end
    return false
end

function M.paired_devices()
    local result = {}
    local handle = io.popen("bluetoothctl paired-devices 2>/dev/null")
    if handle then
        for line in handle:lines() do
            local addr, name = line:match("^Device (%S+) (.+)$")
            if addr then
                result[#result + 1] = {
                    address = addr,
                    name = name,
                }
            end
        end
        handle:close()
    end
    return result
end

function M.remove(addr)
    if not addr then return false end
    local handle = io.popen("bluetoothctl remove " .. addr .. " 2>/dev/null")
    if handle then
        local result = handle:read("*a")
        handle:close()
        return result and result:match("Removed") ~= nil
    end
    return false
end

return M
