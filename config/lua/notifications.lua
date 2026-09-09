local M = {}

M.notifications = {}
M.next_id = 1
M.default_timeout = 5000
M.max_history = 100
M.history = {}

function M.show(title, body, urgency)
    urgency = urgency or "normal"
    local notif = {
        id = M.next_id,
        title = title or "",
        body = body or "",
        urgency = urgency,
        timestamp = os.time(),
        dismissed = false,
    }
    M.notifications[M.next_id] = notif
    M.history[#M.history + 1] = {
        id = notif.id,
        title = notif.title,
        body = notif.body,
        urgency = notif.urgency,
        timestamp = notif.timestamp,
    }
    if #M.history > M.max_history then
        table.remove(M.history, 1)
    end
    M.next_id = M.next_id + 1
    return notif.id
end

function M.dismiss(id)
    local notif = M.notifications[id]
    if notif then
        notif.dismissed = true
        M.notifications[id] = nil
        return true
    end
    return false
end

function M.dismiss_all()
    for id, _ in pairs(M.notifications) do
        M.notifications[id].dismissed = true
        M.notifications[id] = nil
    end
end

function M.get(id)
    return M.notifications[id]
end

function M.active()
    local active_list = {}
    for id, notif in pairs(M.notifications) do
        if not notif.dismissed then
            active_list[#active_list + 1] = notif
        end
    end
    return active_list
end

function M.get_history()
    return M.history
end

function M.clear_history()
    M.history = {}
end

function M.set_timeout(timeout_ms)
    M.default_timeout = timeout_ms
end

function M.get_timeout()
    return M.default_timeout
end

function M.count()
    local c = 0
    for _ in pairs(M.notifications) do
        c = c + 1
    end
    return c
end

return M
