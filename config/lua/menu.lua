local M = {}

M.items = {}
M.visible_menu = false
M.menu_x = 0
M.menu_y = 0

function M.add_item(label, action, submenu)
    local item = {
        label = label,
        action = action,
        submenu = submenu or nil,
    }
    M.items[#M.items + 1] = item
end

function M.remove_item(label)
    for i, item in ipairs(M.items) do
        if item.label == label then
            table.remove(M.items, i)
            return true
        end
    end
    return false
end

function M.show(x, y)
    M.visible_menu = true
    M.menu_x = x or 0
    M.menu_y = y or 0
end

function M.hide()
    M.visible_menu = false
end

function M.toggle(x, y)
    if M.visible_menu then
        M.hide()
    else
        M.show(x, y)
    end
end

function M.is_visible()
    return M.visible_menu
end

function M.get_items()
    return M.items
end

function M.clear()
    M.items = {}
    M.visible_menu = false
end

function M.execute(index)
    local item = M.items[index]
    if item and item.action then
        if type(item.action) == "function" then
            item.action()
        end
        return item.action
    end
    return nil
end

return M
