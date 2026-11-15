//! Runs the video example.
//!
//! TODO Reviewed ??-??-2026.

/// Encodes a short synthetic video as AV1 packets using `rav1e`.
mod video;
/// Runs the video example.

/// # Errors
/// Returns an error if encoder setup or frame encoding fails.
fn main() -> anyhow::Result<()> {
    video::run()
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
