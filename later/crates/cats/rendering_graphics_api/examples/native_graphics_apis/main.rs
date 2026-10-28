//! Runs the native graphics apis example.
//!
//! TODO Reviewed ??-??-2026.

mod wgpu_adapter_info;

/// Reports the native backend selected for a headless GPU adapter.
///
/// # Errors
///
/// Returns an error when no compatible GPU adapter is available.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    wgpu_adapter_info::run()
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
