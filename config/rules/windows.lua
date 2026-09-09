return {
    {
        name = "float_center",
        rule = function(w) return w.floating end,
        action = function(w)
            w:center()
        end,
    },
    {
        name = "terminal_tile",
        rule = function(w) return w.app_id == "Alacritty" and not w.floating end,
        action = function(w)
            w:set_opacity(0.95)
        end,
    },
    {
        name = "dialog_center",
        rule = function(w) return w.window_type == "dialog" end,
        action = function(w)
            w:center()
            w:set_floating(true)
        end,
    },
    {
        name = "splash_center",
        rule = function(w) return w.window_type == "splash" end,
        action = function(w)
            w:center()
            w:set_floating(true)
        end,
    },
    {
        name = "popup_center",
        rule = function(w) return w.window_type == "popup_menu" or w.window_type == "utility" end,
        action = function(w)
            w:center()
            w:set_floating(true)
        end,
    },
    {
        name = "game_fullscreen",
        rule = function(w) return w.app_id:find("steam_app") ~= nil end,
        action = function(w)
            w:toggle_fullscreen()
            w:set_opacity(1.0)
        end,
    },
}
