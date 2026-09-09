return {
  keys = {
    { mods = { "SUPER" }, key = "RETURN", action = "exec", arg = "alacritty" },
    { mods = { "SUPER" }, key = "Q", action = "close_window" },
    { mods = { "SUPER" }, key = "M", action = "toggle_maximize" },
    { mods = { "SUPER" }, key = "F", action = "toggle_fullscreen" },
    { mods = { "SUPER" }, key = "SPACE", action = "toggle_floating" },
    { mods = { "SUPER" }, key = "D", action = "exec", arg = "wofi --show drun" },
    { mods = { "SUPER" }, key = "T", action = "exec", arg = "kitty" },
    { mods = { "SUPER", "SHIFT" }, key = "Q", action = "quit" },
    { mods = { "SUPER" }, key = "LEFT", action = "focus_monitor", arg = "left" },
    { mods = { "SUPER" }, key = "RIGHT", action = "focus_monitor", arg = "right" },
    { mods = { "SUPER", "SHIFT" }, key = "LEFT", action = "move_to_monitor", arg = "left" },
    { mods = { "SUPER", "CTRL" }, key = "L", action = "lock" },
  },
  mouse = {
    { mods = { "SUPER" }, button = "left", action = "move_window" },
    { mods = { "SUPER" }, button = "right", action = "resize_window" },
  },
  bindings = {
    next_workspace = { mods = { "SUPER" }, key = "TAB" },
    prev_workspace = { mods = { "SUPER", "SHIFT" }, key = "TAB" },
    cycle_layout = { mods = { "SUPER" }, key = "G" },
    togglespecialworkspace = { mods = { "SUPER" }, key = "S" },
  },
}