local M = {}

local function run_cmd(cmd)
    local handle = io.popen(cmd)
    if handle then
        local result = handle:read("*a")
        handle:close()
        return result
    end
    return nil
end

local function parse_sinks()
    local sinks = {}
    local output = run_cmd("pactl list sinks short 2>/dev/null")
    if output then
        for line in output:gmatch("[^\n]+") do
            local id, name, driver, state = line:match("^(%d+)%s+(%S+)%s+(%S+)%s+(%S+)")
            if id then
                sinks[#sinks + 1] = {
                    id = tonumber(id),
                    name = name,
                    driver = driver,
                    state = state,
                }
            end
        end
    end
    return sinks
end

function M.volume()
    local output = run_cmd("pactl get-sink-volume @DEFAULT_SINK@ 2>/dev/null")
    if output then
        local vol = output:match("(%d+)%%")
        if vol then
            return tonumber(vol) or 0
        end
    end
    return 0
end

function M.set_volume(v)
    v = math.max(0, math.min(100, v))
    run_cmd("pactl set-sink-volume @DEFAULT_SINK@ " .. v .. "% 2>/dev/null")
end

function M.mute(state)
    if state then
        run_cmd("pactl set-sink-mute @DEFAULT_SINK@ 1 2>/dev/null")
    else
        run_cmd("pactl set-sink-mute @DEFAULT_SINK@ 0 2>/dev/null")
    end
end

function M.is_muted()
    local output = run_cmd("pactl get-sink-mute @DEFAULT_SINK@ 2>/dev/null")
    if output then
        return output:match("yes") ~= nil
    end
    return false
end

function M.sinks()
    return parse_sinks()
end

function M.set_default_sink(sink_id)
    if type(sink_id) == "number" then
        run_cmd("pactl set-default-sink " .. sink_id .. " 2>/dev/null")
    elseif type(sink_id) == "string" then
        run_cmd("pactl set-default-sink " .. sink_id .. " 2>/dev/null")
    end
end

function M.volume_up(step)
    step = step or 5
    local vol = M.volume()
    M.set_volume(vol + step)
end

function M.volume_down(step)
    step = step or 5
    local vol = M.volume()
    M.set_volume(vol - step)
end

function M.toggle_mute()
    run_cmd("pactl set-sink-mute @DEFAULT_SINK@ toggle 2>/dev/null")
end

return M
