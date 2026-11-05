// TODO Reviewed ??-??-2026.

//! Read the macOS version through typed Foundation bindings.
//!
//! Foundation is Apple's base framework for macOS and iOS apps. The typed
//! bindings let Rust call it without writing Objective-C. It runs on macOS
//! only.

// ANCHOR: example
use std::io;

use objc2_foundation::NSProcessInfo;

/// Reads and prints the operating-system version using safe retained objects.
///
/// # Errors
/// Returns `InvalidData` if Foundation unexpectedly returns an empty version.
fn main() -> io::Result<()> {
    let process_info = NSProcessInfo::processInfo();
    let version = process_info.operatingSystemVersionString().to_string();
    if version.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Foundation returned an empty operating-system version",
        ));
    }
    println!("Operating system: {version}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the native macOS Foundation query.
///
/// # Errors
/// Propagates an empty system-version response from Foundation.
pub fn run() -> io::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the macos example through its entry point.
    #[test]
    fn test_macos_example_entry_point() {
        main().expect("Foundation should return a non-empty system version");
    }
}
