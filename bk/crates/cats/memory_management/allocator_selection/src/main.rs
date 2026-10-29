// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Selecting the global allocator with Cargo features.
//!
//! Enable one of `use-mimalloc` and `use-jemalloc`; with neither, the system
//! allocator is used. Enabling both is a compile error. Measure before
//! switching: the best allocator depends on the workload (see the
//! benchmarking pages).

#[cfg(all(feature = "use-mimalloc", feature = "use-jemalloc"))]
compile_error!("select only one global allocator feature");

#[cfg(feature = "use-mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    feature = "use-jemalloc",
    not(feature = "use-mimalloc"),
    not(target_env = "msvc")
))]
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

#[allow(clippy::main_recursion)]
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the global allocator example through its entry point.
    #[test]
    fn test_global_allocator_example_entry_point() {
        main();
        #[cfg(feature = "use-mimalloc")]
        assert_eq!(allocator_name(), "mimalloc");
        #[cfg(all(feature = "use-jemalloc", not(target_env = "msvc")))]
        assert_eq!(allocator_name(), "jemalloc");
        #[cfg(any(
            not(any(feature = "use-mimalloc", feature = "use-jemalloc")),
            all(feature = "use-jemalloc", target_env = "msvc")
        ))]
        assert_eq!(allocator_name(), "system");
    }
}
