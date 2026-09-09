local M = {}

M.handlers = {}

function M.on(event, handler)
    if type(event) ~= "string" or type(handler) ~= "function" then
        return false
    end
    if not M.handlers[event] then
        M.handlers[event] = {}
    end
    M.handlers[event][#M.handlers[event] + 1] = handler
    return true
end

function M.emit(event, ...)
    if not M.handlers[event] then
        return 0
    end
    local count = 0
    for _, handler in ipairs(M.handlers[event]) do
        handler(...)
        count = count + 1
    end
    return count
end

function M.off(event, handler)
    if not M.handlers[event] then
        return false
    end
    for i, h in ipairs(M.handlers[event]) do
        if h == handler then
            table.remove(M.handlers[event], i)
            return true
        end
    end
    return false
end

function M.off_all(event)
    if event then
        M.handlers[event] = nil
    else
        M.handlers = {}
    end
end

function M.has_listeners(event)
    return M.handlers[event] ~= nil and #M.handlers[event] > 0
end

function M.listener_count(event)
    if M.handlers[event] then
        return #M.handlers[event]
    end
    return 0
end

function M.list_events()
    local events = {}
    for event, _ in pairs(M.handlers) do
        events[#events + 1] = event
    end
    return events
end

return M
