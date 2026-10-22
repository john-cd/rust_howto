#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Demonstrates using a custom global allocator.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [target.'cfg(not(target_env = "msvc"))'.dependencies]
//! tikv-jemallocator = "0.6"
//! tikv-jemalloc-ctl = { version = "0.6.0", features = [ "stats", "use_std" ] } # optional - for introspection.
//! ```

/// Uses `jemalloc` as the global allocator.
#[cfg(not(target_env = "msvc"))]
mod example {

    use tikv_jemallocator::Jemalloc;

    // Once you've defined the following static, jemalloc will be used for all
    // allocations requested by Rust code in the same program.
    /// The global allocator, which handles all allocations requested by Rust
    /// code.
    #[global_allocator]
    static GLOBAL: Jemalloc = Jemalloc;

    /// Allocates, then drops a large vector, printing the allocator statistics
    /// each time.
    pub fn test_alloc() -> anyhow::Result<()> {
        // Allocate a large vector.
        let v: Vec<i32> = Vec::with_capacity(1_000_000);
        print_alloc()?;

        // Drop the vector.
        drop(v);
        print_alloc()?;
        Ok(())
    }

    /// Prints the allocated and resident memory according to `jemalloc`.
    fn print_alloc() -> anyhow::Result<()> {
        use tikv_jemalloc_ctl::epoch;
        use tikv_jemalloc_ctl::stats;
        // Many statistics are cached and only updated when the epoch is
        // advanced.
        epoch::advance().unwrap();
        let allocated = stats::allocated::read().unwrap();
        let resident = stats::resident::read().unwrap();
        println!("{allocated} bytes allocated / {resident} bytes resident");

        // Full allocator statistics:
        // tikv_jemalloc_ctl::stats_print::stats_print(std::io::stdout(),
        // tikv_jemalloc_ctl::stats_print::Options::default())?;
        Ok(())
    }
}

/// Allocates memory with `jemalloc` and prints its statistics, except on MSVC.
fn main() -> anyhow::Result<()> {
    #[cfg(not(target_env = "msvc"))]
    example::test_alloc()?;
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
