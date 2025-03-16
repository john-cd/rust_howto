// ANCHOR: example
use std::fmt::Debug;

use bitvec::prelude::*;

// Add this dependency to your Cargo.toml:
// [dependencies]
// bitvec = "1.0.1"

fn main() {
    // Create a new BitVec with default parameters
    let mut bv = bitvec![u8, Msb0; 0, 1, 0, 1, 1, 0, 1, 0];
    println!("Original BitVec: {}", bv);

    // Access individual bits
    println!("First bit: {}", bv[0]);
    println!("Second bit: {}", bv[1]);

    // Modify bits.
    // The parameters are: index, value.
    bv.set(0, true);
    bv.set(5, true);
    println!("Modified BitVec: {}", bv);

    // Get the length.
    println!("BitVec length: {}", bv.len());

    // Check if all bits are set.
    println!("All bits set: {}", bv.all());

    // Check if any bits are set
    println!("Any bits set: {}", bv.any());

    // Count set bits
    println!("Number of set bits: {}", bv.count_ones());

    // Iterate through bits
    print!("Iterating through bits: ");
    for bit in bv.iter() {
        print!("{} ", bit);
    }
    println!();

    println!("\n===== Different Endianness and Storage Types =====");

    // Create a BitVec with different storage types and endianness
    let bv_lsb0 = bitvec![u16, Lsb0; 0, 1, 0, 1, 1, 0, 1, 0];
    println!("LSB0 BitVec: {}", bv_lsb0);

    let bv_u32 = bitvec![u32, Msb0; 0, 1, 0, 1, 1, 0, 1, 0];
    println!("u32 BitVec: {}", bv_u32);

    println!("\n===== Bit Manipulation Operations =====");

    // Create two BitVecs for bitwise operations
    let mut a = bitvec![u8, Msb0; 1, 1, 0, 0, 1, 0, 1, 0];
    let b = bitvec![u8, Msb0; 0, 1, 1, 0, 0, 1, 1, 1];

    println!("a: {}", a);
    println!("b: {}", b);

    // Bitwise AND
    let c = a.clone() & b.clone();
    println!("a & b: {}", c);

    // Bitwise OR
    let d = a.clone() | b.clone();
    println!("a | b: {}", d);

    // Bitwise XOR
    let e = a.clone() ^ b.clone();
    println!("a ^ b: {}", e);

    // Bitwise NOT
    let f = !a.clone();
    println!("!a: {}", f);

    // In-place operations:
    a &= &b;
    println!("a &= b: {}", a);

    println!("\n===== Slicing and Ranges =====");

    let mut bv_slice = bitvec![u8, Msb0; 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0];
    println!("Original BitVec: {}", bv_slice);

    // Get a slice:
    let slice = &bv_slice[2..6];
    println!("Slice [2..6]: {}", slice);

    // Modify a slice:
    bv_slice[4..8].fill(true);
    println!("After filling [4..8] with true: {}", bv_slice);

    println!("\n===== Practical Applications =====");

    // Example 1: Bit flags
    let mut flags = bitvec![u8, Msb0; 0; 8];

    // Set flags
    flags.set(0, true); // READ
    flags.set(1, true); // WRITE
    flags.set(2, false); // EXECUTE

    println!("Permission flags: {}", flags);
    println!("Can read: {}", flags[0]);
    println!("Can write: {}", flags[1]);
    println!("Can execute: {}", flags[2]);

    // Example 2: Bit packing.
    // Pack multiple boolean values efficiently.
    let mut packed = BitVec::<u8, Msb0>::new();

    // Appends single bits to the vector:
    packed.push(true); // is_active
    packed.push(false); // is_admin
    packed.push(true); // is_verified
    packed.push(false); // is_premium

    println!("Packed bits: {}", packed);

    // Example 3: Simple bloom filter
    fn create_bloom_filter<T: Debug>(
        size: usize,
        items: &[T],
        hash_fn: impl Fn(&T) -> usize,
    ) -> BitVec<u8> {
        let mut filter = bitvec![u8, Lsb0; 0; size];

        for item in items {
            let hash = hash_fn(item) % size;
            filter.set(hash, true);
            println!("Added item {:?}, hash: {}", item, hash);
        }

        filter
    }

    let test_items = ["apple", "banana", "cherry"];
    let bloom = create_bloom_filter(10, &test_items, |s| {
        s.len() * 7 + s.chars().next().unwrap() as usize
    });

    println!("Bloom filter: {}", bloom);

    // Example 4: Bit-packed struct
    use bitvec::field::BitField;

    let mut data = bitvec![u16, Lsb0; 0; 16];

    // Pack values into specific bit ranges
    data[0..3].store(0b101u8);
    data[3..8].store(0b10011u8);
    data[8..16].store(0b11001010u8);

    println!("Bit-packed data: {}", data);

    // Extract values
    let value1: u8 = data[0..3].load();
    let value2: u8 = data[3..8].load();
    let value3: u8 = data[8..16].load();

    println!("Extracted values: {}, {}, {}", value1, value2, value3);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
