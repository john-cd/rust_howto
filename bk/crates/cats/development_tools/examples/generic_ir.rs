//! Gives cargo-llvm-lines a generic function with two concrete instantiations.

// ANCHOR: example
use std::hint::black_box;
use std::ops::Add;

/// Doubles a value using its type's addition operation.
#[inline(never)]
fn double<T: Copy + Add<Output = T>>(value: T) -> T {
    value + value
}

/// Calls the same generic function with two concrete integer types.
fn main() {
    assert_eq!(double(black_box(21_u32)), 42);
    assert_eq!(double(black_box(21_u64)), 42);
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Checks that both concrete instantiations double their input.
    #[test]
    fn doubles_both_integer_types() {
        super::main();
    }
}
