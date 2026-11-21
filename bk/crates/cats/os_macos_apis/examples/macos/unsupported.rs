// TODO Reviewed ??-??-2026.
//! Fallback behavior for the macOS-only example.

/// Reports that the Foundation query requires macOS.
pub(super) fn run() -> std::io::Result<()> {
    println!("This Foundation example requires macOS.");
    Ok(())
}
