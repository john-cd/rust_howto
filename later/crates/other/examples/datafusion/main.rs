//! Runs the `datafusion` query example.
//!
//! TODO Reviewed ??-??-2026

mod datafusion;

/// Runs the example from the `datafusion` module.
fn main() {
    #[cfg(feature = "datafusion")]
    datafusion::run();
}
