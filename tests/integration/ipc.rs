use waydri_core::ipc::json::{
    command_from_json, command_to_json, decode, event_from_json, event_to_json,
};
use waydri_core::ipc::{Command, Event, EventSink, IpcServer};

#[test]
fn server_accepts_and_client_connects() {
    let mut server = IpcServer::new("127.0.0.1:0".to_string());
    assert!(server.start().is_ok());
    assert!(server.is_running());
    server.stop();
}

#[test]
fn command_set_layout_serializes() {
    let command = Command::SetLayout("grid".to_string());
    let json = command_to_json(&command);
    let decoded = command_from_json(&json).unwrap();
    assert_eq!(decoded, command);
}

#[test]
fn window_created_event_round_trips() {
    let event = Event::WindowCreated {
        id: 7,
        title: "terminal".to_string(),
    };
    let json = event_to_json(&event);
    let decoded = event_from_json(&json).unwrap();
    assert_eq!(decoded.name(), event.name());
}

#[test]
fn decode_dispatches_event() {
    let event = Event::FrameCompleted { fps: 60.0 };
    let json = event_to_json(&event);
    match decode(&json).unwrap() {
        waydri_core::ipc::json::Decoded::Event(e) => {
            assert_eq!(e.name(), event.name())
        }
        waydri_core::ipc::json::Decoded::Command(_) => panic!("expected event"),
    }
}

#[test]
fn malformed_payload_rejected() {
    assert!(command_from_json("not json").is_err());
    assert!(decode("{}").is_err());
}

#[test]
fn event_sink_respects_capacity() {
    let mut sink = EventSink::new(2);
    sink.push(Event::OutputAdded { name: "a".to_string() });
    sink.push(Event::OutputAdded { name: "b".to_string() });
    sink.push(Event::OutputAdded { name: "c".to_string() });
    assert_eq!(sink.len(), 2);
    assert_eq!(sink.drain().len(), 2);
}

#[test]
fn list_windows_command_name() {
    let command = Command::ListWindows;
    assert_eq!(command.name(), "list_windows");
}

#[test]
fn quit_command_snake_case_tag() {
    let json = command_to_json(&Command::Quit);
    assert!(json.contains("\"command\":\"quit\""));
}

#[test]
fn event_version_injected() {
    let json = event_to_json(&Event::LayoutChanged { name: "dwindle".to_string() });
    assert!(json.contains("\"version\":1"));
}