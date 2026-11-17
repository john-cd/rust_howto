//! Runnable entry point for the texture formats example.
/// Provides texture formats behavior for the texture formats component.
mod texture_formats;

/// Runs the example demonstrated by this module.
fn main() -> anyhow::Result<()> {
    texture_formats::run()
}
