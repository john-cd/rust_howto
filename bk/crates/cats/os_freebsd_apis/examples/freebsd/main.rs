//! Runs the FreeBSD kernel query on supported hosts.
//!
//! TODO Reviewed ??-??-2026.

/// Read-only queries of FreeBSD kernel state.
#[cfg(target_os = "freebsd")]
mod freebsd;
/// Reports when the example is run on another operating system.
#[cfg(not(target_os = "freebsd"))]
mod unsupported;

/// Runs the FreeBSD example or its unsupported-platform message.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "freebsd")]
    {
        Ok(freebsd::run()?)
    }
    #[cfg(not(target_os = "freebsd"))]
    {
        Ok(unsupported::run()?)
    }
}
