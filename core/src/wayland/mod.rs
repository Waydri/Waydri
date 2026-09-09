pub mod client;
pub mod data_device;
pub mod output;
pub mod primary_selection;
pub mod protocol;
pub mod seat;
pub mod server;
pub mod shell;
pub mod wlr_layer_shell;
pub mod xdg_shell;

pub use client::{WlClient, ClientState};
pub use protocol::{
    Arg, ArgKind, MessageCodec, RegistryEntry, WaylandMessage, interface_opcodes, interface_count,
};
pub use seat::{Seat, SeatCapability};
pub use server::{WaylandServer, ServerConnection};
pub use shell::ShellSurface;
pub use xdg_shell::{XdgSurface, XdgSurfaceKind, XdgToplevel};
pub use wlr_layer_shell::LayerSurface;

pub const WAYLAND_VERSION: u32 = 1;