//! Starts the host-side Embassy task example.
// TODO Reviewed ??-??-2026.

mod embassy;

// ANCHOR: example
/// Runs the bounded host example.
fn main() -> anyhow::Result<()> {
    embassy::run()
}
// ANCHOR_END: example
