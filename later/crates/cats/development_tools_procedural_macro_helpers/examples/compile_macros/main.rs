//! Runs the ahead-of-time macro helper example.

mod watt;

/// Runs the `watt` token-transformation example.
fn main() -> anyhow::Result<()> {
    watt::run()
}
