local Layout = {}

function Layout.new()
    local self = setmetatable({}, { __index = Layout })
    self.name = "three_column_master_stack"
    self.master_ratio = 0.5
    self.column_count = 3
    self.gaps = {
        inner = 4,
        outer = 8,
    }
    return self
end

function Layout:arrange(area, window_ids, config)
    local gaps = config.gaps or self.gaps
    local count = #window_ids
    if count == 0 then
        return {}
    end

    local rects = {}
    local x = area.x + gaps.outer
    local y = area.y + gaps.outer
    local usable_w = area.w - gaps.outer * 2
    local usable_h = area.h - gaps.outer * 2

    if count == 1 then
        rects[1] = {
            x = x,
            y = y,
            w = usable_w,
            h = usable_h,
        }
        return rects
    end

    local master_w = math.floor(usable_w * self.master_ratio)
    local stack_w = usable_w - master_w - gaps.inner
    local master_h = usable_h
    local stack_count = count - 1

    rects[1] = {
        x = x,
        y = y,
        w = master_w,
        h = master_h,
    }

    local stack_x = x + master_w + gaps.inner
    local columns = math.min(self.column_count, stack_count)
    local col_w = math.floor((stack_w - gaps.inner * (columns - 1)) / columns)
    local col_heights = {}

    for col = 1, columns do
        local items_in_col = math.floor(stack_count / columns)
        if (stack_count % columns) > 0 and col <= (stack_count % columns) then
            items_in_col = items_in_col + 1
        end
        col_heights[col] = items_in_col
    end

    local idx = 2
    for col = 1, columns do
        local items = col_heights[col]
        if items == 0 then
            break
        end
        local col_item_h = math.floor((usable_h - gaps.inner * (items - 1)) / items)
        local cx = stack_x + (col - 1) * (col_w + gaps.inner)

        for row = 1, items do
            if idx > count then
                break
            end
            local cy = y + (row - 1) * (col_item_h + gaps.inner)
            local remaining_h = usable_h - (row - 1) * (col_item_h + gaps.inner)
            local final_h = col_item_h
            if row == items then
                final_h = remaining_h
            end

            rects[idx] = {
                x = cx,
                y = cy,
                w = col_w,
                h = final_h,
            }
            idx = idx + 1
        end
    end

    while idx <= count do
        rects[idx] = rects[idx - 1] or rects[1]
        idx = idx + 1
    end

    return rects
end

function Layout:cycle_focus(direction, count)
    if direction == "next" then
        return 1
    elseif direction == "prev" then
        return count
    end
    return 1
end

function Layout:set_master_ratio(ratio)
    self.master_ratio = math.max(0.2, math.min(0.8, ratio))
end

function Layout:set_column_count(n)
    self.column_count = math.max(1, math.min(4, n))
end

return Layout
