//! Runs the satellite tracking example.
//!
//! TODO Reviewed ??-??-2026.

/// Sample a satellite's orbit from a fixed two-line element set using `sgp4`.
mod aero;

/// Runs the aerospace example.
fn main() {
    aero::run();
}
