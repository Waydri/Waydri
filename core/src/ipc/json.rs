use serde_json::{Map, Value};

use super::{Command, Event};

pub const PROTOCOL_VERSION: u32 = 1;

fn command_object(command: &Command) -> Value {
    let mut object = Map::new();
    object.insert("version".to_string(), Value::from(PROTOCOL_VERSION));
    object.insert("command".to_string(), Value::from(command.name()));
    match command {
        Command::SetLayout(layout) => {
            object.insert("layout".to_string(), Value::from(layout.as_str()));
        }
        Command::SwitchWorkspace(index) => {
            object.insert("index".to_string(), Value::from(*index));
        }
        Command::FocusWindow(id) | Command::CloseWindow(id) => {
            object.insert("id".to_string(), Value::from(*id));
        }
        Command::SetOpacity { window, alpha } => {
            object.insert("window".to_string(), Value::from(*window));
            object.insert("alpha".to_string(), serde_json::Number::from_f64(*alpha as f64).map(|n| Value::Number(n)).unwrap_or(Value::Null));
        }
        _ => {}
    }
    Value::Object(object)
}

pub fn command_to_json(command: &Command) -> String {
    serde_json::to_string(&command_object(command)).unwrap_or_else(|_| "null".to_string())
}

fn event_object(event: &Event) -> Value {
    let mut object = Map::new();
    object.insert("version".to_string(), Value::from(PROTOCOL_VERSION));
    object.insert("event".to_string(), Value::from(event.name()));
    match event {
        Event::WindowCreated { id, title } => {
            object.insert("id".to_string(), Value::from(*id));
            object.insert("title".to_string(), Value::from(title.as_str()));
        }
        Event::WindowDestroyed { id } | Event::WindowFocusChanged { id } => {
            object.insert("id".to_string(), Value::from(*id));
        }
        Event::WorkspaceChanged { index } => {
            object.insert("index".to_string(), Value::from(*index));
        }
        Event::LayoutChanged { name } => {
            object.insert("name".to_string(), Value::from(name.as_str()));
        }
        Event::OutputAdded { name } | Event::OutputRemoved { name } => {
            object.insert("name".to_string(), Value::from(name.as_str()));
        }
        Event::FrameCompleted { fps } => {
            object.insert("fps".to_string(), serde_json::Number::from_f64(*fps as f64).map(|n| Value::Number(n)).unwrap_or(Value::Null));
        }
        Event::DamagedRegion { x, y, width, height } => {
            object.insert("x".to_string(), Value::from(*x));
            object.insert("y".to_string(), Value::from(*y));
            object.insert("width".to_string(), Value::from(*width));
            object.insert("height".to_string(), Value::from(*height));
        }
    }
    Value::Object(object)
}

pub fn event_to_json(event: &Event) -> String {
    serde_json::to_string(&event_object(event)).unwrap_or_else(|_| "null".to_string())
}

pub fn command_from_json(input: &str) -> Result<Command, String> {
    let value: Value = serde_json::from_str(input)
        .map_err(|error| format!("invalid command: {error}"))?;
    let name = value
        .get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "command is missing a name".to_string())?;
    match name {
        "quit" => Ok(Command::Quit),
        "reload_config" => Ok(Command::ReloadConfig),
        "set_layout" => Ok(Command::SetLayout(
            value
                .get("layout")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "set_layout requires a layout".to_string())?
                .to_string(),
        )),
        "next_workspace" => Ok(Command::NextWorkspace),
        "prev_workspace" => Ok(Command::PrevWorkspace),
        "switch_workspace" => Ok(Command::SwitchWorkspace(
            value
                .get("index")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| "switch_workspace requires an index".to_string())?,
        )),
        "next_window" => Ok(Command::NextWindow),
        "prev_window" => Ok(Command::PrevWindow),
        "focus_window" => Ok(Command::FocusWindow(
            value
                .get("id")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| "focus_window requires an id".to_string())?,
        )),
        "close_window" => Ok(Command::CloseWindow(
            value
                .get("id")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| "close_window requires an id".to_string())?,
        )),
        "set_opacity" => {
            let window = value
                .get("window")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| "set_opacity requires a window".to_string())?;
            let alpha = value
                .get("alpha")
                .and_then(|v| v.as_f64())
                .ok_or_else(|| "set_opacity requires an alpha".to_string())? as f32;
            Ok(Command::SetOpacity { window, alpha })
        }
        "toggle_fullscreen" => Ok(Command::ToggleFullscreen),
        "toggle_floating" => Ok(Command::ToggleFloating),
        "list_windows" => Ok(Command::ListWindows),
        "get_version" => Ok(Command::GetVersion),
        _ => Err(format!("unknown command {name}")),
    }
}

pub fn event_from_json(input: &str) -> Result<Event, String> {
    let value: Value = serde_json::from_str(input)
        .map_err(|error| format!("invalid event: {error}"))?;
    let name = value
        .get("event")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "event is missing a name".to_string())?;
    match name {
        "window_created" => {
            let id = value.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
            let title = value.get("title").and_then(|v| v.as_str()).unwrap_or_default().to_string();
            Ok(Event::WindowCreated { id, title })
        }
        "window_destroyed" => Ok(Event::WindowDestroyed { id: value.get("id").and_then(|v| v.as_u64()).unwrap_or(0) }),
        "window_focus_changed" => Ok(Event::WindowFocusChanged { id: value.get("id").and_then(|v| v.as_u64()).unwrap_or(0) }),
        "workspace_changed" => Ok(Event::WorkspaceChanged { index: value.get("index").and_then(|v| v.as_u64()).unwrap_or(0) as usize }),
        "layout_changed" => {
            let name = value.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
            Ok(Event::LayoutChanged { name })
        }
        "output_added" => {
            let name = value.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
            Ok(Event::OutputAdded { name })
        }
        "output_removed" => {
            let name = value.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
            Ok(Event::OutputRemoved { name })
        }
        "frame_completed" => Ok(Event::FrameCompleted { fps: value.get("fps").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32 }),
        "damaged_region" => Ok(Event::DamagedRegion {
            x: value.get("x").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            y: value.get("y").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            width: value.get("width").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            height: value.get("height").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
        }),
        _ => Err(format!("unknown event {name}")),
    }
}

pub fn decode(input: &str) -> Result<Decoded, String> {
    let value: Value = serde_json::from_str(input)
        .map_err(|error| format!("invalid message: {error}"))?;
    if let Some(version) = value.get("version").and_then(|v| v.as_u64()) {
        if version != PROTOCOL_VERSION as u64 {
            return Err(format!("unsupported protocol version {version}"));
        }
    }
    if value.get("command").is_some() {
        command_from_json(input).map(Decoded::Command)
    } else if value.get("event").is_some() {
        event_from_json(input).map(Decoded::Event)
    } else {
        Err("message must contain a command or event field".to_string())
    }
}

pub fn ipc_decode(input: &str) -> Result<Decoded, String> {
    decode(input)
}

pub enum Decoded {
    Command(Command),
    Event(Event),
}

impl Decoded {
    pub fn as_command(&self) -> Option<&Command> {
        match self {
            Decoded::Command(command) => Some(command),
            Decoded::Event(_) => None,
        }
    }

    pub fn as_event(&self) -> Option<&Event> {
        match self {
            Decoded::Event(event) => Some(event),
            Decoded::Command(_) => None,
        }
    }
}