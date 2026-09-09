return {
  apps = {
    { app = "firefox", workspace = 2, layout = "grid", follow = true },
    { app = "chromium", workspace = 2, layout = "grid", follow = true },
    { app = "alacritty", workspace = 1, follow = true },
    { app = "kitty", workspace = 1, follow = true },
    { app = "code", workspace = 3, layout = "master_stack", follow = true },
    { app = "neovim", workspace = 3, follow = true },
    { app = "wofi", floating = true },
    { app = "pavucontrol", floating = true },
  },
  monitors = {
    resume = { workspace_specific_on = true },
  },
  windows = {
    { windowrole = "pop-up", floating = true, noinitialfocus = true },
    { windowclass = "mpv", fullscreen = true },
    { windowtitle = "Picture-in-Picture", floating = true },
  },
}