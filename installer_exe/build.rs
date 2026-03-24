/// KelpyShark build script
///
/// Locates the pre-built CLI binary and tells Cargo where to find it
/// so the installer can embed it via `include_bytes!`.

use std::env;
use std::path::PathBuf;

fn main() {
    // Point to the workspace release binary
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = PathBuf::from(&manifest_dir).parent().unwrap().to_path_buf();

    let cli_binary = if cfg!(target_os = "windows") {
        workspace_root.join("target").join("release").join("kelpyshark-cli.exe")
    } else {
        workspace_root.join("target").join("release").join("kelpyshark-cli")
    };

    println!("cargo:rustc-env=KELPY_CLI_BINARY={}", cli_binary.display());
    println!("cargo:rerun-if-changed={}", cli_binary.display());
    println!("cargo:rerun-if-changed=build.rs");
}
