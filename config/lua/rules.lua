local M = {}

M.rules = {}

function M.add_rule(pattern, actions)
    if type(pattern) == "string" and type(actions) == "table" then
        M.rules[pattern] = actions
    end
end

function M.remove_rule(pattern)
    M.rules[pattern] = nil
end

function M.match(app_id, title)
    local matches = {}
    for pattern, actions in pairs(M.rules) do
        local matched = false
        if app_id and string.find(app_id, pattern) then
            matched = true
        end
        if title and string.find(title, pattern) then
            matched = true
        end
        if matched then
            matches[#matches + 1] = {
                pattern = pattern,
                actions = actions,
            }
        end
    end
    return matches
end

function M.get_actions(app_id, title)
    local result = {
        floating = false,
        workspace = 1,
        size = nil,
        position = nil,
        fullscreen = false,
        opacity = 1.0,
    }
    local matches = M.match(app_id, title)
    for _, match in ipairs(matches) do
        for k, v in pairs(match.actions) do
            result[k] = v
        end
    end
    return result
end

function M.clear()
    M.rules = {}
end

function M.get_all()
    return M.rules
end

function M.load_from_table(rules_table)
    if type(rules_table) == "table" then
        for pattern, actions in pairs(rules_table) do
            M.add_rule(pattern, actions)
        end
    end
end

return M
