//! Build script that runs the Tauri build helper when the `tauri` feature is
//! enabled.
//!
//! TODO Reviewed ??-??-2026

/// Runs the Tauri build helper if the `tauri` feature is enabled.
fn main() {
    // The Tauri build helper expects the optional Tauri dependency to be
    // enabled.
    if std::env::var_os("CARGO_FEATURE_TAURI").is_some() {
        tauri_build::build();
    }
}
