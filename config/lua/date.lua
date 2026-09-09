local M = {}

function M.now()
    local t = os.date("*t")
    return {
        year = t.year,
        month = t.month,
        day = t.day,
        hour = t.hour,
        min = t.min,
        sec = t.sec,
    }
end

function M.format(template)
    template = template or "%Y-%m-%d %H:%M:%S"
    return os.date(template)
end

function M.weekday()
    local t = os.date("*t")
    local days = {
        "Sunday", "Monday", "Tuesday", "Wednesday",
        "Thursday", "Friday", "Saturday",
    }
    return days[t.wday]
end

function M.iso8601()
    return os.date("!%Y-%m-%dT%H:%M:%SZ")
end

function M.timestamp()
    return os.time()
end

function M.format_time(seconds)
    if seconds < 0 then seconds = 0 end
    local hours = math.floor(seconds / 3600)
    local mins = math.floor((seconds % 3600) / 60)
    local secs = math.floor(seconds % 60)
    if hours > 0 then
        return string.format("%d:%02d:%02d", hours, mins, secs)
    else
        return string.format("%02d:%02d", mins, secs)
    end
end

function M.format_relative(seconds)
    if seconds < 60 then
        return tostring(seconds) .. "s ago"
    elseif seconds < 3600 then
        return tostring(math.floor(seconds / 60)) .. "m ago"
    elseif seconds < 86400 then
        return tostring(math.floor(seconds / 3600)) .. "h ago"
    else
        return tostring(math.floor(seconds / 86400)) .. "d ago"
    end
end

function M.is_leap_year(year)
    return (year % 4 == 0 and year % 100 ~= 0) or (year % 400 == 0)
end

function M.days_in_month(month, year)
    local days = { 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31 }
    if month == 2 and M.is_leap_year(year) then
        return 29
    end
    return days[month] or 30
end

function M.is_24h()
    local handle = io.popen("locale time_fmt 2>/dev/null || echo 24")
    if handle then
        local fmt = handle:read("*l")
        handle:close()
        return not (fmt and fmt:match("12"))
    end
    return true
end

return M
