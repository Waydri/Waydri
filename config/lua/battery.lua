local M = {}

local function read_file(path)
    local handle = io.open(path, "r")
    if handle then
        local content = handle:read("*l")
        handle:close()
        return content
    end
    return nil
end

local function find_battery_path()
    local handle = io.popen("ls /sys/class/power_supply/ 2>/dev/null")
    if handle then
        for line in handle:lines() do
            local type_handle = io.open("/sys/class/power_supply/" .. line .. "/type", "r")
            if type_handle then
                local btype = type_handle:read("*l")
                type_handle:close()
                if btype == "Battery" then
                    handle:close()
                    return "/sys/class/power_supply/" .. line
                end
            end
        end
        handle:close()
    end
    return nil
end

local battery_path = nil

local function ensure_path()
    if not battery_path then
        battery_path = find_battery_path()
    end
    return battery_path
end

function M.level()
    local path = ensure_path()
    if not path then return 0 end
    local capacity = read_file(path .. "/capacity")
    if capacity then
        local val = tonumber(capacity)
        if val then
            return math.max(0, math.min(100, val))
        end
    end
    local energy = read_file(path .. "/energy_now")
    local energy_full = read_file(path .. "/energy_full")
    if energy and energy_full then
        local e = tonumber(energy)
        local ef = tonumber(energy_full)
        if e and ef and ef > 0 then
            return math.floor(e / ef * 100)
        end
    end
    local charge = read_file(path .. "/charge_now")
    local charge_full = read_file(path .. "/charge_full")
    if charge and charge_full then
        local c = tonumber(charge)
        local cf = tonumber(charge_full)
        if c and cf and cf > 0 then
            return math.floor(c / cf * 100)
        end
    end
    return 0
end

function M.status()
    local path = ensure_path()
    if not path then return "unknown" end
    local state = read_file(path .. "/status")
    if state then
        state = state:lower()
        if state == "charging" then return "charging" end
        if state == "discharging" then return "discharging" end
        if state == "full" then return "full" end
        if state == "not charging" then return "full" end
    end
    return "unknown"
end

function M.time_to_empty()
    local path = ensure_path()
    if not path then return 0 end
    local current_now = read_file(path .. "/current_now")
    local energy_now = read_file(path .. "/energy_now")
    local voltage_now = read_file(path .. "/voltage_now")
    if current_now and energy_now and voltage_now then
        local inow = tonumber(current_now)
        local enow = tonumber(energy_now)
        local vnow = tonumber(voltage_now)
        if inow and enow and vnow and inow > 0 and vnow > 0 then
            local power = inow * vnow / 1000000
            if power > 0 then
                return math.floor(enow / 1000 / power * 3600)
            end
        end
    end
    return 0
end

function M.time_to_full()
    local path = ensure_path()
    if not path then return 0 end
    local current_now = read_file(path .. "/current_now")
    local energy_full = read_file(path .. "/energy_full")
    local energy_now = read_file(path .. "/energy_now")
    local voltage_now = read_file(path .. "/voltage_now")
    if current_now and energy_full and energy_now and voltage_now then
        local inow = tonumber(current_now)
        local efull = tonumber(energy_full)
        local enow = tonumber(energy_now)
        local vnow = tonumber(voltage_now)
        if inow and efull and enow and vnow and inow > 0 and vnow > 0 then
            local remaining = efull - enow
            if remaining < 0 then remaining = 0 end
            local power = inow * vnow / 1000000
            if power > 0 then
                return math.floor(remaining / 1000 / power * 3600)
            end
        end
    end
    return 0
end

function M.refresh()
    battery_path = nil
end

return M
