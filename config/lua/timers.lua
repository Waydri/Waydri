local M = {}

M.active_timers = {}
M.next_id = 1

function M.create(interval_ms, callback)
    if type(callback) ~= "function" then
        return nil
    end
    local timer = {
        id = M.next_id,
        interval = interval_ms,
        callback = callback,
        active = true,
    }
    M.active_timers[M.next_id] = timer
    M.next_id = M.next_id + 1
    return timer.id
end

function M.cancel(timer_id)
    local timer = M.active_timers[timer_id]
    if timer then
        timer.active = false
        M.active_timers[timer_id] = nil
        return true
    end
    return false
end

function M.after(delay_ms, callback)
    if type(callback) ~= "function" then
        return nil
    end
    local timer = {
        id = M.next_id,
        interval = delay_ms,
        callback = function()
            callback()
        end,
        active = true,
        one_shot = true,
    }
    M.active_timers[M.next_id] = timer
    M.next_id = M.next_id + 1
    return timer.id
end

function M.is_active(timer_id)
    local timer = M.active_timers[timer_id]
    return timer ~= nil and timer.active
end

function M.get_info(timer_id)
    local timer = M.active_timers[timer_id]
    if timer then
        return {
            id = timer.id,
            interval = timer.interval,
            active = timer.active,
            one_shot = timer.one_shot or false,
        }
    end
    return nil
end

function M.cancel_all()
    for id, timer in pairs(M.active_timers) do
        timer.active = false
    end
    M.active_timers = {}
end

function M.count()
    local c = 0
    for _ in pairs(M.active_timers) do
        c = c + 1
    end
    return c
end

return M
