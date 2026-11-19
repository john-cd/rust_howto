//! Makes the Pico memory map available to the linker.
//!
//! TODO Reviewed ??-??-2026.

fn main() {
    println!("cargo::rustc-link-search={}", env!("CARGO_MANIFEST_DIR"));
    println!("cargo::rerun-if-changed=memory.x");
    println!("cargo::rustc-link-arg=-Tlink.x");
    println!("cargo::rustc-link-arg=-Tlink-rp.x");
}
