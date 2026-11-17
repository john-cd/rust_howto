// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates the use of the mimalloc memory allocator.
//!
//! Mimalloc is a general purpose, performance oriented allocator built by
//! Microsoft.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! mimalloc = "0.1.52"
//! ```
//!
//! A C compiler is required.
//!
//! The optional `secure` feature enables additional hardening, with a
//! workload-dependent performance cost.
//!
//! ```toml
//! [dependencies]
//! mimalloc = { version = "0.1.52", features = ["secure"] }
//! ```

use mimalloc::MiMalloc;

/// The global allocator, which handles all allocations requested by Rust code.
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Allocates a large vector with the `mimalloc` allocator.
fn main() {
    example();
}

/// Allocates a large vector using the configured global allocator.
fn example() {
    // Allocate a large vector.
    let _v = vec![0; 1024 * 1024];
    println!("Allocated 1 MiB using the mimalloc allocator");
}
// ANCHOR_END: example

/// Runs the example through its program entry point.
pub fn run() {
    example();
}

/// Allocations through the global allocator behave normally.
#[cfg(test)]
mod tests {
    use super::*;

    /// Allocations through the global allocator behave normally.
    #[test]
    fn test_mimalloc_example_entry_point() {
        main();
    }

    /// Verifies that the allocator serves large vectors.
    #[test]
    fn allocator_serves_large_vectors() {
        let v = vec![7_u8; 1024 * 1024];
        assert_eq!(v.len(), 1024 * 1024);
        assert!(v.iter().all(|&b| b == 7));
    }
}
