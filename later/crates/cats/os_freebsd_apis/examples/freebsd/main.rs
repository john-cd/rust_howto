//! Runs the freebsd example.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(target_os = "freebsd")]
mod freebsd;

/// Runs the freebsd example.
///
/// # Errors
/// Propagates failures reading FreeBSD kernel information.
#[cfg(target_os = "freebsd")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(freebsd::run()?)
}

/// Reports that the kernel query requires FreeBSD.
#[cfg(not(target_os = "freebsd"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    run()
}

/// Provides a harmless fallback on hosts without the FreeBSD sysctl interface.
#[cfg(not(target_os = "freebsd"))]
fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("This sysctl example requires FreeBSD.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
