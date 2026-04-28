#![allow(dead_code)]
// ANCHOR: example
use std::marker::PhantomData;

use typenum::Prod;
use typenum::Sum;
use typenum::U2;
use typenum::U3;
use typenum::U6;
use typenum::Unsigned;

/// A type-safe fixed-size buffer whose capacity is encoded in its type.
///
/// The size `N` is a type-level unsigned integer from `typenum`, so
/// mismatched sizes are caught at compile time rather than at runtime.
struct FixedBuffer<T, N: Unsigned> {
    data: Vec<T>,
    _size: PhantomData<N>,
}

impl<T: Default + Clone, N: Unsigned> FixedBuffer<T, N> {
    fn new() -> Self {
        FixedBuffer {
            data: vec![T::default(); N::USIZE],
            _size: PhantomData,
        }
    }

    fn capacity(&self) -> usize {
        N::USIZE
    }
}

fn main() {
    // `typenum` provides type-level numbers evaluated at compile time.
    // U2 and U3 are type aliases for the compile-time unsigned integers 2 and
    // 3.
    println!("U2 = {}", U2::USIZE);
    println!("U3 = {}", U3::USIZE);

    // Compute 2 + 3 at the type level.
    type Five = Sum<U2, U3>;
    println!("U2 + U3 = {}", Five::USIZE);

    // Compute 2 * 3 at the type level.
    type Six = Prod<U2, U3>;
    println!("U2 * U3 = {}", Six::USIZE);

    // Use the computed type-level size to create a type-safe buffer.
    // `Six` is the same type as `U6`, so both are accepted.
    let buf1: FixedBuffer<f64, Six> = FixedBuffer::new();
    let buf2: FixedBuffer<f64, U6> = FixedBuffer::new();
    println!("buf1 capacity: {}", buf1.capacity());
    println!("buf2 capacity: {}", buf2.capacity());

    assert_eq!(Five::USIZE, 5);
    assert_eq!(Six::USIZE, 6);
    assert_eq!(buf1.capacity(), buf2.capacity());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
