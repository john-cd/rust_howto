//! Runs the macos example.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(target_os = "macos")]
mod macos;

/// Runs the macos example.
///
/// # Errors
/// Propagates an empty native operating-system version string.
#[cfg(target_os = "macos")]
fn main() -> std::io::Result<()> {
    macos::run()
}

/// Reports that the Foundation system-information query requires macOS.
#[cfg(not(target_os = "macos"))]
fn main() -> std::io::Result<()> {
    run()
}

/// Provides a message-only fallback without linking Apple's frameworks.
#[cfg(not(target_os = "macos"))]
fn run() -> std::io::Result<()> {
    println!("This Foundation example requires macOS.");
    Ok(())
}
