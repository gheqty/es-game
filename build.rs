use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let client_manifest = manifest_dir.join("client").join("Cargo.toml");
    let target_dir = manifest_dir.join("target").join("client-wasm");

    println!("cargo:rerun-if-changed=client/src/lib.rs");
    println!("cargo:rerun-if-changed=client/src/math.rs");
    println!("cargo:rerun-if-changed=client/src/ffi.rs");
    println!("cargo:rerun-if-changed=client/src/state.rs");
    println!("cargo:rerun-if-changed=client/src/world.rs");
    println!("cargo:rerun-if-changed=client/src/story.rs");
    println!("cargo:rerun-if-changed=client/src/dialogue.rs");
    println!("cargo:rerun-if-changed=client/src/actor.rs");
    println!("cargo:rerun-if-changed=client/src/ui.rs");
    println!("cargo:rerun-if-changed=client/Cargo.toml");

    let status = Command::new(env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()))
        .arg("build")
        .arg("--manifest-path")
        .arg(&client_manifest)
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release")
        .arg("--target-dir")
        .arg(&target_dir)
        .env("RUSTFLAGS", "-C link-arg=--import-undefined")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("RUSTC_WRAPPER")
        .status()
        .expect("failed to invoke cargo to build the wasm client");

    if !status.success() {
        panic!(
            "building the wasm client failed.\n\
             Make sure the wasm32-unknown-unknown target is installed:\n\
             rustup target add wasm32-unknown-unknown"
        );
    }
}
