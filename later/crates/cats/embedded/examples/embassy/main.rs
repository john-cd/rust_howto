// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Starts the host-side Embassy task example.

/// Runs a bounded pair of Embassy tasks on the host executor.
mod embassy;

/// Runs the bounded host example.
fn main() -> anyhow::Result<()> {
    embassy::run()
}
// ANCHOR_END: example
