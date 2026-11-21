// TODO Reviewed ??-??-2026.
//! Runnable entry point for the sensors example.
/// Provides i2c temperature behavior for the sensors component.
mod i2c_temperature;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    i2c_temperature::run()
}
