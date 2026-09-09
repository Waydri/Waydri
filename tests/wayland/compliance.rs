use waydri_core::wayland::{
    Arg, ArgKind, MessageCodec, WaylandMessage, interface_count, interface_opcodes,
};
use waydri_core::wayland::protocol::{OPCODE_ATTACH, OPCODE_COMMIT, OPCODE_CREATE_REGION, OPCODE_SYNC};

#[test]
fn interface_count_matches_core() {
    assert_eq!(interface_count(), 11);
}

#[test]
fn display_opcodes_include_sync() {
    assert_eq!(interface_opcodes("wl_display"), &[OPCODE_SYNC, 1]);
}

#[test]
fn compositor_opcodes_include_create_surface() {
    assert_eq!(interface_opcodes("wl_compositor"), &[0, OPCODE_CREATE_REGION]);
}

#[test]
fn surface_opcodes_include_attach_and_commit() {
    assert_eq!(interface_opcodes("wl_surface"), &[OPCODE_ATTACH, OPCODE_COMMIT]);
}

#[test]
fn default_opcodes_all_sync() {
    assert_eq!(interface_opcodes("wl_seat"), &[OPCODE_SYNC]);
}

#[test]
fn arg_size_table() {
    assert_eq!(ArgKind::Uint.size(), 4);
    assert_eq!(ArgKind::NewId.size(), 4);
    assert_eq!(ArgKind::String.size(), 0);
    assert_eq!(ArgKind::Fd.size(), 4);
    assert_eq!(ArgKind::Array.size(), 0);
    assert_eq!(ArgKind::of(&Arg::Int(-5)), ArgKind::Int);
    assert_eq!(ArgKind::of(&Arg::Object(9)), ArgKind::Object);
    assert_eq!(ArgKind::of(&Arg::Fixed(0.5)), ArgKind::Fixed);
}

#[test]
fn no_partial_fixed_loss_offset() {
    let message = WaylandMessage::new(1, 0, vec![Arg::Fixed(1.0), Arg::Uint(2), Arg::Int(-3)]);
    let bytes = MessageCodec::encode(&message);
    assert_eq!(bytes.len(), 20);
}