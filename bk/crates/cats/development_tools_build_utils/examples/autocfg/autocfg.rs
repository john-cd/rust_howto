// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! `autocfg` probes compiler support for use in Cargo build scripts.
//!
//! Add to your `Cargo.toml`:
//! [build-dependencies]
//! autocfg = "1"

/// Checks whether the active compiler supports `i128`.
fn main() {
    let out_dir =
        std::env::var_os("OUT_DIR").map_or_else(std::env::temp_dir, std::path::PathBuf::from);
    let ac = autocfg::AutoCfg::with_dir(out_dir)
        .expect("Cargo OUT_DIR or the system temporary directory must be writable");
    println!("compiler supports i128: {}", ac.probe_type("i128"));
}
// ANCHOR_END: example

/// Runs the compiler capability probe.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the probe runs outside build script.
    #[test]
    fn probe_runs_outside_build_script() {
        main();
    }

    /// Reports which types the compiler supports for the probing.
    #[test]
    fn probing_reports_which_types_the_compiler_supports() {
        let dir = std::env::temp_dir();
        let ac = autocfg::AutoCfg::with_dir(dir).expect("temp dir is writable");
        assert!(ac.probe_type("i128"));
        assert!(!ac.probe_type("this_type_does_not_exist"));
        assert!(ac.probe_path("std::collections::HashMap"));
        assert!(!ac.probe_path("std::collections::NoSuchMap"));
    }
}
