return {
  default = "master_stack",
  layouts = {
    "master_stack",
    "dwindle",
    "grid",
    "custom",
    "dynamic",
  },
  master_stack = {
    ratio = 0.6,
    gap = 8,
    outer_gap = 16,
    new_windows = "master",
  },
  dwindle = {
    split_ratio = 0.5,
    gap = 8,
    outer_gap = 16,
    default_split = "horizontal",
  },
  grid = {
    columns = 2,
    gap = 8,
    outer_gap = 16,
  },
  custom = {
    slots = { 0.5, 0.5, 1.0 },
    gap = 8,
  },
  dynamic = {
    master = "master_stack",
    grid = "grid",
    threshold = 6,
    gap = 8,
  },
  floating = {
    enabled = true,
    apps = { "wofi", "xdg-desktop-portal", "firefox-config-editor" },
  },
}