// TODO Reviewed ??-??-2026.

//! Runnable entry point for the api bindings example.
/// Provides gzip round trip behavior for the api bindings component.
mod gzip_round_trip;

/// Runs the example demonstrated by this module.
fn main() -> std::io::Result<()> {
    gzip_round_trip::run()
}
