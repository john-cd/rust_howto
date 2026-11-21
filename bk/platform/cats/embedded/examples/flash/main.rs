// TODO Reviewed ??-??-2026.
//! Runnable entry point for the flash example.
/// Provides nor flash storage behavior for the flash component.
mod nor_flash_storage;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    nor_flash_storage::run()
}
