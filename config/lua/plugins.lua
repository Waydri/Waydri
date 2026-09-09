local M = {}

M.loaded = {}
M.plugins = {}
M.hooks = {}

function M.load(dir)
    local plugins = {}
    local lfs = nil
    pcall(function() lfs = require("lfs") end)

    if not lfs then
        local handle = io.popen('ls "' .. dir .. '" 2>/dev/null')
        if handle then
            for line in handle:lines() do
                if line:match("%.lua$") then
                    local name = line:match("(.+)%.lua$")
                    local ok, mod = pcall(require, dir .. "/" .. name)
                    if ok and type(mod) == "table" then
                        M.plugins[name] = mod
                        M.loaded[name] = true
                        plugins[#plugins + 1] = name
                    end
                end
            end
            handle:close()
        end
    else
        for file in lfs.dir(dir) do
            if file:match("%.lua$") then
                local name = file:match("(.+)%.lua$")
                local ok, mod = pcall(require, dir .. "/" .. name)
                if ok and type(mod) == "table" then
                    M.plugins[name] = mod
                    M.loaded[name] = true
                    plugins[#plugins + 1] = name
                end
            end
        end
    end

    return plugins
end

function M.list()
    local names = {}
    for name, _ in pairs(M.plugins) do
        names[#names + 1] = name
    end
    return names
end

function M.call(hook_name, ...)
    local results = {}
    for name, plugin in pairs(M.plugins) do
        if type(plugin[hook_name]) == "function" then
            local ok, result = pcall(plugin[hook_name], ...)
            if ok then
                results[name] = result
            end
        end
    end
    return results
end

function M.is_loaded(name)
    return M.loaded[name] == true
end

function M.get(name)
    return M.plugins[name]
end

function M.unload(name)
    M.plugins[name] = nil
    M.loaded[name] = nil
end

return M
