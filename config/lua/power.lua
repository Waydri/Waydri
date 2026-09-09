local M = {}

M.idle_timeout = 300
M.lock_before_sleep = true

function M.suspend()
    local handle = io.popen("systemctl suspend 2>/dev/null || loginctl suspend 2>/dev/null")
    if handle then
        handle:read("*a")
        handle:close()
    end
end

function M.hibernate()
    local handle = io.popen("systemctl hibernate 2>/dev/null || loginctl hibernate 2>/dev/null")
    if handle then
        handle:read("*a")
        handle:close()
    end
end

function M.shutdown()
    local handle = io.popen("systemctl poweroff 2>/dev/null || loginctl poweroff 2>/dev/null")
    if handle then
        handle:read("*a")
        handle:close()
    end
end

function M.reboot()
    local handle = io.popen("systemctl reboot 2>/dev/null || loginctl reboot 2>/dev/null")
    if handle then
        handle:read("*a")
        handle:close()
    end
end

function M.lock()
    local handle = io.popen("waydri-lock 2>/dev/null || swaylock 2>/dev/null || loginctl lock-session 2>/dev/null")
    if handle then
        handle:read("*a")
        handle:close()
    end
end

function M.set_idle_timeout(seconds)
    if seconds >= 0 then
        M.idle_timeout = seconds
    end
end

function M.get_idle_timeout()
    return M.idle_timeout
end

return M
