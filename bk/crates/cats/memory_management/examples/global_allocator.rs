#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Selecting the global allocator with Cargo features.
//!
//! Enable at most one of `use-mimalloc` and `use-jemalloc`; with neither, the
//! system allocator is used. Measure before switching: the best allocator
//! depends on the workload (see the benchmarking pages).

#[cfg(all(feature = "use-mimalloc", feature = "use-jemalloc"))]
compile_error!(
    "Enable only one of the features `use-mimalloc` and `use-jemalloc`."
);

#[cfg(feature = "use-mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(feature = "use-jemalloc", not(target_env = "msvc")))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

/// The name of the allocator selected at compile time.
const fn allocator_name() -> &'static str {
    if cfg!(feature = "use-mimalloc") {
        "mimalloc"
    } else if cfg!(all(feature = "use-jemalloc", not(target_env = "msvc"))) {
        "jemalloc"
    } else {
        "system"
    }
}

/// Allocates a little, using whichever allocator is active.
fn main() {
    let v: Vec<u64> = (0..1_000).collect();
    println!(
        "Allocator: {}; sum: {}",
        allocator_name(),
        v.iter().sum::<u64>()
    );
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
        #[cfg(not(any(feature = "use-mimalloc", feature = "use-jemalloc")))]
        assert_eq!(allocator_name(), "system");
    }
}
