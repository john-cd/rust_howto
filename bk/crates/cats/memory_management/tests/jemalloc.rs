// ANCHOR: example
//! Demonstrates using a custom global allocator.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [target.'cfg(not(target_env = "msvc"))'.dependencies]
//! tikv-jemallocator = "0.6"
//! tikv-jemalloc-ctl = { version = "0.6.0", features = [ "stats", "use_std" ] } # optional - for introspection.
//! ```
#![cfg(not(target_env = "msvc"))]

use tikv_jemallocator::Jemalloc;

// Once you've defined the following static, jemalloc will be used for all
// allocations requested by Rust code in the same program.
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() -> anyhow::Result<()> {
    // Allocate a large vector.
    let v: Vec<i32> = Vec::with_capacity(1_000_000);
    print_alloc()?;

    // Drop the vector.
    drop(v);
    print_alloc()?;
    Ok(())
}

fn print_alloc() -> anyhow::Result<()> {
    use tikv_jemalloc_ctl::epoch;
    use tikv_jemalloc_ctl::stats;
    // Many statistics are cached and only updated when the epoch is advanced.
    epoch::advance().unwrap();
    let allocated = stats::allocated::read().unwrap();
    let resident = stats::resident::read().unwrap();
    println!(
        "{} bytes allocated / {} bytes resident",
        allocated, resident
    );

    // Full allocator statistics:
    // tikv_jemalloc_ctl::stats_print::stats_print(std::io::stdout(),
    // tikv_jemalloc_ctl::stats_print::Options::default())?;
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
