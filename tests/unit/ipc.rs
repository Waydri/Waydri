use waydri_core::ipc::json::{
    command_from_json, command_to_json, decode as ipc_decode, event_from_json, event_to_json,
};
use waydri_core::ipc::{Command, Event, EventSink, IpcClient, IpcServer};

#[test]
fn command_json_round_trip() {
    let command = Command::SetLayout("grid".into());
    let json = command_to_json(&command);
    let decoded = command_from_json(&json).unwrap();
    assert_eq!(decoded, command);
}

#[test]
fn all_command_variants_round_trip() {
    let commands = vec![
        Command::Quit,
        Command::ReloadConfig,
        Command::SetLayout("dwindle".into()),
        Command::NextWorkspace,
        Command::PrevWorkspace,
        Command::SwitchWorkspace(3),
        Command::NextWindow,
        Command::PrevWindow,
        Command::FocusWindow(7),
        Command::CloseWindow(9),
        Command::SetOpacity { window: 2, alpha: 0.4 },
        Command::ToggleFullscreen,
        Command::ToggleFloating,
        Command::ListWindows,
        Command::GetVersion,
    ];
    for command in commands {
        let json = command_to_json(&command);
        let decoded = command_from_json(&json).unwrap();
        assert_eq!(decoded, command);
    }
}

#[test]
fn command_names() {
    assert_eq!(Command::Quit.name(), "quit");
    assert_eq!(Command::SwitchWorkspace(0).name(), "switch_workspace");
}

#[test]
fn event_json_round_trip() {
    let event = Event::WindowCreated { id: 5, title: "Terminal".into() };
    let json = event_to_json(&event);
    let decoded = event_from_json(&json).unwrap();
    assert_eq!(decoded, event);
}

#[test]
fn decode_dispatches_both() {
    let command_str = command_to_json(&Command::NextWorkspace);
    assert!(ipc_decode(&command_str).unwrap().as_command().is_some());
    assert!(ipc_decode(&command_str).unwrap().as_event().is_none());

    let event_str = event_to_json(&Event::FrameCompleted { fps: 120.0 });
    assert!(ipc_decode(&event_str).unwrap().as_event().is_some());
    assert!(ipc_decode(&event_str).unwrap().as_command().is_none());
}

#[test]
fn malformed_json_fails() {
    assert!(command_from_json("not json").is_err());
    assert!(event_from_json("[]").is_err());
}

#[test]
fn event_sink_capacity() {
    let mut sink = EventSink::new(2);
    sink.push(Event::WindowCreated { id: 1, title: "A".into() });
    sink.push(Event::WindowCreated { id: 2, title: "B".into() });
    sink.push(Event::WindowCreated { id: 3, title: "C".into() });
    assert_eq!(sink.len(), 2);
    let drained = sink.drain();
    assert_eq!(drained.len(), 2);
    assert!(sink.is_empty());
}

#[test]
fn server_lifecycle() {
    let mut server = IpcServer::new("127.0.0.1:0".into());
    assert!(!server.is_running());
    assert!(server.start().is_ok());
    assert!(server.is_running());
    server.stop();
    assert!(!server.is_running());
}

#[test]
fn client_connect_missing_server_fails() {
    let result = IpcClient::connect("127.0.0.1:1");
    assert!(result.is_err());
}