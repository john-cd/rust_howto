// TODO Reviewed ??-??-2026.
//! Fallback behavior for the FreeBSD-only example.

/// Reports that the kernel query requires FreeBSD.
pub(super) fn run() -> std::io::Result<()> {
    println!("This sysctl example requires FreeBSD.");
    Ok(())
}
