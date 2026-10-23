#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! `autocfg` probes compiler support for use in Cargo build scripts.
//!
//! Add to your `Cargo.toml`:
//! [build-dependencies]
//! autocfg = "1"

/// Checks for `i128` support and emits the matching custom `cfg`.
fn main() {
    let out_dir = std::env::var_os("OUT_DIR")
        .map_or_else(std::env::temp_dir, std::path::PathBuf::from);
    let ac = autocfg::AutoCfg::with_dir(out_dir).expect(
        "Cargo OUT_DIR or the system temporary directory must be writable",
    );
    ac.emit_has_type("i128");
    autocfg::rerun_path("build.rs");
}

// ANCHOR_END: example

/// Runs the compiler capability probe.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the probe runs without Cargo's build-script environment.
    #[test]
    fn probe_runs_outside_build_script() {
        main();
    }
}
