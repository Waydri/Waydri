use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Command {
    Quit,
    ReloadConfig,
    SetLayout(String),
    NextWorkspace,
    PrevWorkspace,
    SwitchWorkspace(u64),
    NextWindow,
    PrevWindow,
    FocusWindow(u64),
    CloseWindow(u64),
    SetOpacity { window: u64, alpha: f32 },
    ToggleFullscreen,
    ToggleFloating,
    ListWindows,
    GetVersion,
}

impl Command {
    pub fn name(&self) -> &'static str {
        match self {
            Command::Quit => "quit",
            Command::ReloadConfig => "reload_config",
            Command::SetLayout(_) => "set_layout",
            Command::NextWorkspace => "next_workspace",
            Command::PrevWorkspace => "prev_workspace",
            Command::SwitchWorkspace(_) => "switch_workspace",
            Command::NextWindow => "next_window",
            Command::PrevWindow => "prev_window",
            Command::FocusWindow(_) => "focus_window",
            Command::CloseWindow(_) => "close_window",
            Command::SetOpacity { .. } => "set_opacity",
            Command::ToggleFullscreen => "toggle_fullscreen",
            Command::ToggleFloating => "toggle_floating",
            Command::ListWindows => "list_windows",
            Command::GetVersion => "get_version",
        }
    }
}

pub trait CommandHandler {
    fn handle(&mut self, command: Command) -> String;
}

pub struct NullHandler;

impl CommandHandler for NullHandler {
    fn handle(&mut self, _command: Command) -> String {
        "received".to_string()
    }
}