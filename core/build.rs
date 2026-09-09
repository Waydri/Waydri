use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let profile = env::var("PROFILE").unwrap_or_default();
    let rustc_version = env::var("CARGO_PKG_RUST_VERSION").unwrap_or_default();

    println!("cargo:rustc-env=WAYDRI_TARGET_OS={target_os}");
    println!("cargo:rustc-env=WAYDRI_TARGET_ARCH={target_arch}");
    println!("cargo:rustc-env=WAYDRI_PROFILE={profile}");
    println!("cargo:rustc-env=WAYDRI_RUST_VERSION={rustc_version}");
    println!("cargo:rerun-if-changed=build.rs");
}