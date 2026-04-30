// ANCHOR: example
//! Demonstrates using `bumpalo` for fast bump allocation.
//!
//! `bumpalo` is a bump allocator (also called an arena allocator). It
//! allocates memory by bumping a pointer, which is very fast - roughly
//! as fast as incrementing an integer. All memory is freed at once
//! when the `Bump` arena is dropped.
//!
//! Bump allocation is useful when you need to allocate many small,
//! short-lived objects and want to avoid the overhead of individual
//! heap allocations.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! bumpalo = "3"
//! ```

use bumpalo::Bump;

fn main() {
    // Create a new bump allocation arena.
    let bump = Bump::new();

    // Allocate a single value in the arena. The value lives until the
    // arena is dropped.
    let x = bump.alloc(42_i32);
    println!("Allocated integer: {x}");

    // Allocate a string slice by copying bytes into the arena.
    let s = bump.alloc_str("Hello, bumpalo!");
    println!("Allocated str: {s}");

    // Allocate and initialize a slice.
    let nums = bump.alloc_slice_copy(&[1_u32, 2, 3, 4, 5]);
    println!("Allocated slice: {nums:?}");

    // Allocate several values; they all live in the same arena.
    let a = bump.alloc(1.0_f64);
    let b = bump.alloc(2.0_f64);
    println!("Sum: {}", *a + *b);

    println!(
        "Total bytes allocated in this arena: {}",
        bump.allocated_bytes()
    );

    // When `bump` is dropped here, ALL allocations above are freed at once.
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
