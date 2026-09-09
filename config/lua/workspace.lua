local M = {}

M.workspaces = {}
M.current = 1

function M.create(id)
    M.workspaces[id] = {
        id = id,
        windows = {},
        visible = true,
        layout = "default",
    }
end

function M.switch_to(id)
    if M.workspaces[id] then
        M.current = id
        return true
    end
    return false
end

function M.move_window(window_id, workspace_id)
    if not M.workspaces[workspace_id] then
        return false
    end

    for ws_id, ws in pairs(M.workspaces) do
        for i, wid in ipairs(ws.windows) do
            if wid == window_id then
                table.remove(ws.windows, i)
                break
            end
        end
    end

    table.insert(M.workspaces[workspace_id].windows, window_id)
    return true
end

function M.get_windows(workspace_id)
    if M.workspaces[workspace_id] then
        return M.workspaces[workspace_id].windows
    end
    return {}
end

function M.visible(workspace_id)
    if M.workspaces[workspace_id] then
        return M.workspaces[workspace_id].visible
    end
    return false
end

function M.toggle(workspace_id)
    if M.workspaces[workspace_id] then
        M.workspaces[workspace_id].visible = not M.workspaces[workspace_id].visible
    end
end

function M.add_window(window_id, workspace_id)
    workspace_id = workspace_id or M.current
    if M.workspaces[workspace_id] then
        table.insert(M.workspaces[workspace_id].windows, window_id)
    end
end

function M.remove_window(window_id)
    for ws_id, ws in pairs(M.workspaces) do
        for i, wid in ipairs(ws.windows) do
            if wid == window_id then
                table.remove(ws.windows, i)
                return ws_id
            end
        end
    end
    return nil
end

function M.window_count(workspace_id)
    workspace_id = workspace_id or M.current
    if M.workspaces[workspace_id] then
        return #M.workspaces[workspace_id].windows
    end
    return 0
end

function M.next()
    local next_id = M.current + 1
    while not M.workspaces[next_id] do
        next_id = next_id + 1
        if next_id > 32 then
            next_id = 1
        end
        if next_id == M.current then
            break
        end
    end
    M.switch_to(next_id)
    return next_id
end

function M.prev()
    local prev_id = M.current - 1
    while not M.workspaces[prev_id] do
        prev_id = prev_id - 1
        if prev_id < 1 then
            prev_id = 32
        end
        if prev_id == M.current then
            break
        end
    end
    M.switch_to(prev_id)
    return prev_id
end

return M
