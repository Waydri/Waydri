use waydri_core::wayland::protocol::{OPCODE_ATTACH, OPCODE_COMMIT, INTERFACE_SURFACE};
use waydri_core::wayland::{Arg, MessageCodec, WaylandMessage};

#[test]
fn attach_and_commit_round_trip_layout() {
    let mut message = WaylandMessage::new(5, OPCODE_ATTACH, vec![Arg::Object(6), Arg::Int(0), Arg::Int(0)]);
    message.opcode = OPCODE_COMMIT;
    let bytes = MessageCodec::encode(&message);
    assert_eq!(bytes.len() % 4, 0);
}

#[test]
fn message_exposes_fields() {
    let message = WaylandMessage::new(12, OPCODE_ATTACH, vec![Arg::Uint(0)]);
    assert_eq!(message.object_id, 12);
    assert_eq!(message.args.len(), 1);
}

#[test]
fn surface_interface_string() {
    assert_eq!(INTERFACE_SURFACE, "wl_surface");
}

#[test]
fn new_id_encoded_as_u32() {
    let message = WaylandMessage::new(2, 0, vec![Arg::NewId(33)]);
    let bytes = MessageCodec::encode(&message);
    let id = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
    assert_eq!(id, 33);
}