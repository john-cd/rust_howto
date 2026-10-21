// TODO Reviewed ??-??-2026
fn main() {
    // The Tauri build helper expects the optional Tauri dependency to be
    // enabled.
    if std::env::var_os("CARGO_FEATURE_TAURI").is_some() {
        tauri_build::build();
    }
}
