local M = {}

M.bindings = {}

function M.register(combo, action)
    if type(combo) == "string" and type(action) == "string" then
        M.bindings[combo] = action
    end
end

function M.unregister(combo)
    M.bindings[combo] = nil
end

function M.dispatch(combo)
    local action = M.bindings[combo]
    if action then
        return action
    end
    return nil
end

function M.list_actions()
    local actions = {}
    for combo, action in pairs(M.bindings) do
        actions[combo] = action
    end
    return actions
end

function M.load_from_table(table)
    if type(table) == "table" then
        for combo, action in pairs(table) do
            M.register(combo, action)
        end
    end
end

function M.clear()
    M.bindings = {}
end

function M.get_binding(action)
    for combo, act in pairs(M.bindings) do
        if act == action then
            return combo
        end
    end
    return nil
end

function M.has_binding(combo)
    return M.bindings[combo] ~= nil
end

return M
