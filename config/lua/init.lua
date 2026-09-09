local M = {}

M.workspaces = {}
M.layouts = {}
M.keybinds = {}
M.theme = nil

local function setup_workspaces()
    M.workspaces = require("config.lua.workspace")
    for i = 1, 5 do
        M.workspaces.create(i)
    end
end

local function setup_layouts()
    M.layouts = require("config.lua.layout")
    local default_layout = require("config.layouts.default")
    local master_stack = require("config.layouts.master_stack")
    local grid = require("config.layouts.grid")
    local dwindle = require("config.layouts.dwindle")
    local custom = require("config.layouts.custom")
    M.layouts.register("default", default_layout)
    M.layouts.register("master_stack", master_stack)
    M.layouts.register("grid", grid)
    M.layouts.register("dwindle", dwindle)
    M.layouts.register("custom", custom)
end

local function setup_keybinds()
    M.keybinds = require("config.lua.keybinds")
    local defaults = require("config.keybinds.default")
    for combo, action in pairs(defaults) do
        M.keybinds.register(combo, action)
    end
end

local function setup_theme()
    M.theme = require("config.lua.theme")
    M.theme.load("default")
    M.theme.apply()
end

local function setup_modules()
    require("config.lua.animation")
    require("config.lua.events")
    require("config.lua.plugins")
    require("config.lua.timers")
    require("config.lua.gestures")
    require("config.lua.notifications")
    require("config.lua.menu")
    require("config.lua.bar")
    require("config.lua.rules")
    require("config.lua.power")
end

function M.setup()
    setup_workspaces()
    setup_layouts()
    setup_keybinds()
    setup_theme()
    setup_modules()
    return M
end

return M
