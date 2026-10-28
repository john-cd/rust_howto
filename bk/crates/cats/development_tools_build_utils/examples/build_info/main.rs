//! Runs the build information example.

mod build_info;

/// Runs the build information example.
fn main() -> anyhow::Result<()> {
    build_info::run()
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
