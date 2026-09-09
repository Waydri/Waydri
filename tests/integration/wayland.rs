use waydri_core::wayland::client::maybe_detach;
use waydri_core::wayland::protocol::vendor_interface;
use waydri_core::wayland::{
    Arg, ArgKind, ClientState, MessageCodec, WaylandMessage,
};

#[test]
fn message_encode_header() {
    let message = WaylandMessage::new(1, 0, vec![Arg::Uint(42)]);
    let bytes = MessageCodec::encode(&message);
    assert_eq!(bytes.len(), 12);
    assert_eq!(&bytes[0..4], &1u32.to_le_bytes());
    assert_eq!(&bytes[4..6], &0u16.to_le_bytes());
}

#[test]
fn string_arg_pads_to_four() {
    let message = WaylandMessage::new(2, 1, vec![Arg::String("ab".to_string())]);
    let bytes = MessageCodec::encode(&message);
    assert_eq!(bytes.len() % 4, 0);
}

#[test]
fn fixed_arg_encoding_keeps_value() {
    let message = WaylandMessage::new(3, 0, vec![Arg::Fixed(2.5)]);
    let bytes = MessageCodec::encode(&message);
    let fixed_bytes: [u8; 4] = bytes[8..12].try_into().unwrap();
    let as_i32 = i32::from_le_bytes(fixed_bytes);
    assert_eq!(as_i32, 640);
}

#[test]
fn opcodes_for_display() {
    assert_eq!(
        waydri_core::wayland::interface_opcodes("wl_display"),
        &[
            waydri_core::wayland::protocol::OPCODE_SYNC,
            waydri_core::wayland::protocol::OPCODE_GET_REGISTRY,
        ]
    );
}

#[test]
fn client_state_transitions_from_detach() {
    assert_eq!(maybe_detach(b"12345"), ClientState::Running);
    assert_eq!(maybe_detach(b""), ClientState::Running);
}

#[test]
fn registry_vendored_interface_is_stable() {
    let first = vendor_interface("zwaydri_shm", 1);
    let second = vendor_interface("zwaydri_shm", 1);
    assert_eq!(first.id, second.id);
    assert_eq!(first.version, 1);
}

#[test]
fn arg_kind_sizes() {
    assert_eq!(ArgKind::Uint.size(), 4);
    assert_eq!(ArgKind::Fixed.size(), 4);
    assert_eq!(ArgKind::String.size(), 0);
    assert_eq!(ArgKind::Array.size(), 0);
}

#[test]
fn array_args_pad_to_word() {
    let message = WaylandMessage::new(9, 3, vec![Arg::Array(vec![1, 2, 3])]);
    let bytes = MessageCodec::encode(&message);
    assert_eq!(bytes.len() % 4, 0);
}

#[test]
fn multiple_args_encode_in_order() {
    let message = WaylandMessage::new(5, 2, vec![Arg::Uint(7), Arg::String("x".to_string())]);
    let bytes = MessageCodec::encode(&message);
    assert_eq!(bytes.len() % 4, 0);
    assert_eq!(&bytes[8..12], &7u32.to_le_bytes());
}