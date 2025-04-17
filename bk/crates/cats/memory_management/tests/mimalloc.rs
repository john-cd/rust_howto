// ANCHOR: example
//! Demonstrates the use of the mimalloc memory allocator.
//!
//! Mimalloc is a general purpose, performance oriented allocator built by Microsoft.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! mimalloc = "0.1.46" # Or latest
//! ```
//!
//! A C compiler is required.
//!
//! Using secure mode adds guard pages, randomized allocation, encrypted free lists, etc.
//! The performance penalty is usually around 10%.
//!
//! ```toml
//! [dependencies]
//! mimalloc = { version = "*", features = ["secure"] }
//! ```

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    // Allocate a large vector.
    let _v = vec![0; 1024 * 1024];
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
