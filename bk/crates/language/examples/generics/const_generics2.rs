#![allow(unused)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A trait with an associated constant, implemented for arrays of any length.

// Trait for a fixed-size buffer.
/// A buffer of fixed capacity.
trait FixedSizeBuffer {
    /// The capacity of the buffer.
    const CAPACITY: usize;

    /// Returns the content of the buffer.
    fn get_buffer(&self) -> &[u8];
}

// With const generics, we can implement this for any array of bytes:
impl<const N: usize> FixedSizeBuffer for [u8; N] {
    // The capacity is the array size.
    /// Capacity of the array implementing this trait.
    const CAPACITY: usize = N;

    fn get_buffer(&self) -> &[u8] {
        self
    }
}

/// Prints the capacity and content of the buffer.
fn process_buffer<T: FixedSizeBuffer>(buffer_provider: T) {
    println!("Processing buffer with capacity: {}", T::CAPACITY);
    println!("Buffer content: {:?}", buffer_provider.get_buffer());
}

/// Processes buffers of different sizes.
fn main() {
    let buf1 = [10, 20, 30];
    let buf2 = [1, 2, 3, 4, 5, 6, 7, 8];

    process_buffer(buf1);
    process_buffer(buf2);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
