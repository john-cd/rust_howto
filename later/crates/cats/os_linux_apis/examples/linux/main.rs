//! Runs the linux example.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(target_os = "linux")]
mod statx_metadata;

/// Runs the linux example.
///
/// # Errors
/// Propagates temporary-file and Linux metadata-query failures.
#[cfg(target_os = "linux")]
fn main() -> std::io::Result<()> {
    statx_metadata::run()
}

/// Reports that the extended metadata query requires Linux.
#[cfg(not(target_os = "linux"))]
fn main() {
    println!("This statx example requires Linux.");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
