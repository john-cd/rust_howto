//! Runs the video example.
//!
//! TODO Reviewed ??-??-2026.

mod video;

/// Runs the video example.
///
/// # Errors
/// Returns an error if encoder setup or frame encoding fails.
fn main() -> anyhow::Result<()> {
    video::run()
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
