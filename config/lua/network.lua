local M = {}

function M.interfaces()
    local result = {}
    local handle = io.popen("ip -o link show 2>/dev/null")
    if handle then
        for line in handle:lines() do
            local name = line:match("%d+: (%S+):")
            if name and name ~= "lo" then
                result[#result + 1] = name
            end
        end
        handle:close()
    end
    return result
end

function M.wireless_info()
    local info = {
        ssid = "",
        signal = 0,
        connected = false,
    }
    local handle = io.popen("iwgetid -r 2>/dev/null")
    if handle then
        local ssid = handle:read("*l")
        handle:close()
        if ssid and ssid ~= "" then
            info.ssid = ssid
            info.connected = true
        end
    end
    if info.connected then
        local sig_handle = io.popen("iwgetid -r 2>/dev/null && iwconfig 2>/dev/null | grep 'Signal level'")
        if sig_handle then
            local line = sig_handle:read("*l")
            while line do
                local signal = line:match("Signal level[=:](%-?%d+)")
                if signal then
                    info.signal = tonumber(signal) or 0
                end
                line = sig_handle:read("*l")
            end
            sig_handle:close()
        end
    end
    return info
end

function M.ethernet_info()
    local info = {
        connected = false,
        speed = 0,
    }
    local interfaces = M.interfaces()
    for _, iface in ipairs(interfaces) do
        if not iface:match("^wlan") and not iface:match("^wl") then
            local handle = io.popen("cat /sys/class/net/" .. iface "/operstate 2>/dev/null")
            if handle then
                local state = handle:read("*l")
                handle:close()
                if state == "up" then
                    info.connected = true
                    local speed_handle = io.popen("cat /sys/class/net/" .. iface "/speed 2>/dev/null")
                    if speed_handle then
                        local speed_str = speed_handle:read("*l")
                        speed_handle:close()
                        info.speed = tonumber(speed_str) or 0
                    end
                    break
                end
            end
        end
    end
    return info
end

function M.is_online()
    local handle = io.popen("ping -c 1 -W 2 8.8.8.8 2>/dev/null")
    if handle then
        local result = handle:read("*a")
        handle:close()
        return result and result:match("%d+ packets received") ~= nil
    end
    return false
end

return M
