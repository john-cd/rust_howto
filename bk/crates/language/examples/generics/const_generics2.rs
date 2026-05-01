#![allow(unused)]
// ANCHOR: example
// Trait for a fixed-size buffer.
trait FixedSizeBuffer {
    const CAPACITY: usize;
    fn get_buffer(&self) -> &[u8];
}

// With const generics, we can implement this for any array of bytes:
impl<const N: usize> FixedSizeBuffer for [u8; N] {
    // The capacity is the array size.
    const CAPACITY: usize = N;

    fn get_buffer(&self) -> &[u8] {
        self
    }
}

fn process_buffer<T: FixedSizeBuffer>(buffer_provider: T) {
    println!("Processing buffer with capacity: {}", T::CAPACITY);
    println!("Buffer content: {:?}", buffer_provider.get_buffer());
}

fn main() {
    let buf1 = [10, 20, 30];
    let buf2 = [1, 2, 3, 4, 5, 6, 7, 8];

    process_buffer(buf1);
    process_buffer(buf2);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
