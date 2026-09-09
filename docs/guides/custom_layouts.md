# Custom Layouts Guide

## Overview

Waydri supports custom layouts via Lua scripts. A layout defines how windows are arranged within the available screen area.

## Creating a Layout

Create a Lua file in `config/layouts/`:

```lua
local Layout = {}

function Layout.new()
    return setmetatable({}, { __index = Layout })
end

function Layout:name()
    return "my_layout"
end

function Layout:arrange(area, window_ids, gaps)
    local count = #window_ids
    if count == 0 then return {} end

    local rects = {}
    local gap = gaps.inner
    local outer = gaps.outer
    local x = area.x + outer
    local y = area.y + outer
    local w = area.w - outer * 2
    local h = area.h - outer * 2

    for i, id in ipairs(window_ids) do
        local cols = math.ceil(math.sqrt(count))
        local rows = math.ceil(count / cols)
        local col = (i - 1) % cols
        local row = math.floor((i - 1) / cols)

        local cell_w = math.floor((w - gap * (cols - 1)) / cols)
        local cell_h = math.floor((h - gap * (rows - 1)) / rows)

        rects[id] = {
            x = x + col * (cell_w + gap),
            y = y + row * (cell_h + gap),
            w = cell_w,
            h = cell_h,
        }
    end

    return rects
end

return Layout
```

## Layout API

### Required Methods

#### `name()`

Returns the layout name as a string. This name is used in configuration and IPC.

#### `arrange(area, window_ids, gaps)`

Computes window positions. Parameters:

- `area`: Screen rectangle `{x, y, w, h}`.
- `window_ids`: Array of window IDs to arrange.
- `gaps`: Gap configuration `{inner, outer}`.

Returns a table mapping window IDs to `{x, y, w, h}` rectangles.

### Optional Methods

#### `cycle_focus(direction)`

Returns the index of the next window to focus. `direction` is `"next"` or `"prev"`.

#### `resize_master(ratio)`

Adjusts the master/stack ratio for layouts that support it.

## Registering a Layout

After creating the layout file, register it in your config:

```lua
local my_layout = require("layouts.my_custom")

return {
    layouts = {
        custom = my_layout,
    },
    general = {
        default_layout = "tile",
    },
}
```

## Gap Configuration

The `gaps` parameter passed to `arrange` contains:

```lua
{
    inner = 4,    -- Space between windows
    outer = 8,    -- Space around screen edges
}
```

Apply gaps by offsetting window rectangles:

```lua
rect.x = rect.x + gap.inner
rect.y = rect.y + gap.inner
rect.w = rect.w - gap.inner * 2
rect.h = rect.h - gap.inner * 2
```

## Common Patterns

### Master-Stack

```lua
function Layout:arrange(area, window_ids, gaps)
    if #window_ids <= 1 then
        return { [window_ids[1]] = area }
    end

    local master_w = math.floor(area.w * self.ratio)
    local rects = {}

    rects[window_ids[1]] = {
        x = area.x + gaps.outer,
        y = area.y + gaps.outer,
        w = master_w - gaps.inner,
        h = area.h - gaps.outer * 2,
    }

    local stack_w = area.w - master_w - gaps.outer
    local stack_count = #window_ids - 1
    local stack_h = math.floor((area.h - gaps.outer * 2) / stack_count)

    for i = 2, #window_ids do
        rects[window_ids[i]] = {
            x = area.x + master_w + gaps.outer,
            y = area.y + gaps.outer + (i - 2) * stack_h,
            w = stack_w - gaps.inner,
            h = stack_h - gaps.inner,
        }
    end

    return rects
end
```

### Fibonacci

```lua
function Layout:arrange(area, window_ids, gaps)
    local rects = {}
    local remaining = {
        x = area.x + gaps.outer,
        y = area.y + gaps.outer,
        w = area.w - gaps.outer * 2,
        h = area.h - gaps.outer * 2,
    }

    for i, id in ipairs(window_ids) do
        if i == #window_ids then
            rects[id] = remaining
        elseif i % 2 == 1 then
            local hw = math.floor(remaining.w * 0.6)
              �‘                33    2  _,       33    ' 。       0,33           3, |。2,0,3.5). -.  223 with8 (_w_w7、032。42取取:72w32323 400,00,,0, )
0}
: rects30,,</03' then
                rects[id] = {
                    x = remaining.x,
                    y = remaining.y,
                    w = remaining.w,
                    h = math.floor(remaining.h * 0.618),
                }
                remaining.y = remaining.y + rects[id].h + gaps.inner
                remaining.h = remaining.h - rects[id].h - gaps.inner
            else
                rects[id] = {
                    x = remaining.x,
                    y = remaining.y,
                    w = math.floor(remaining.w * 0.618),
                    h = remaining.h,
                }
                remaining.x = remaining.x + rects[id].w + gaps.inner
                remaining.w = remaining.w - rects[id].w - gaps.inner
            end
        end
    end

    return rects
end
```

## Testing Layouts

Use the IPC interface to test layouts without restarting:

```bash
echo '{"type":"command","id":"1","command":"set_layout","args":{"layout":"my_layout"}}' | socat - UNIX-CONNECT:/run/user/1000/waydri/ipc.sock
```
