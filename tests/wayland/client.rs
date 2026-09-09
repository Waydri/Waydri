use waydri_core::wayland::client::{maybe_detach, WlClient};
use waydri_core::wayland::{Arg, ClientState, MessageCodec, WaylandMessage};

#[test]
fn client_tracks_objects() {
    let mut client = WlClient::new(1, 3, 100, 1000);
    client.add_object(4);
    client.add_object(8);
    assert!(client.owns(4));
    assert!(client.owns(8));
    assert!(client.remove_object(4));
    assert!(!client.owns(4));
}

#[test]
fn client_ping_scheduled() {
    let mut client = WlClient::new(2, 5, 200, 1000);
    assert_eq!(client.ping_due, 0);
    client.schedule_ping(7);
    assert_eq!(client.ping_serial, 7);
    assert!(client.ping_due > 0);
}

#[test]
fn client_starts_connected() {
    let client = WlClient::new(3, 7, 300, 1000);
    assert_eq!(client.state, ClientState::Connected);
    assert!(client.objects.is_empty());
}

#[test]
fn sync_dispatched_to_display() {
    let client = WlClient::new(4, 8, 0, 0);
    let message = WaylandMessage::new(1, 0, vec![Arg::Uint(7)]);
    assert!(client.dispatch(&message).is_some());
}

#[test]
fn detachable_streams() {
    assert_eq!(maybe_detach(b"unrelated"), ClientState::Running);
    assert_eq!(maybe_detach(b""), ClientState::Running);
}

#[test]
fn display_messages_encode_cleanly() {
    let message = WaylandMessage::new(1, 1, vec![]);
    let bytes = MessageCodec::encode(&message);
    assert_eq!(bytes.len(), 8);
    assert_eq!(&bytes[6..8], &8u16.to_le_bytes());
}