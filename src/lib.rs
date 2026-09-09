pub use waydri_core::*;

pub const CRATE_NAME: &str = "waydri";
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn describe() -> &'static str {
    "Waydri root crate re-exporting the compositor core"
}