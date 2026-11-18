//! Runs the macOS Foundation query on supported hosts.
//!
//! TODO Reviewed ??-??-2026.

/// Reads the macOS version through typed Foundation bindings.
#[cfg(target_os = "macos")]
mod macos;
/// Reports when the example is run on another operating system.
#[cfg(not(target_os = "macos"))]
mod unsupported;

/// Runs the macOS example or its unsupported-platform message.
fn main() -> std::io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        macos::run()
    }
    #[cfg(not(target_os = "macos"))]
    {
        unsupported::run()
    }
}
