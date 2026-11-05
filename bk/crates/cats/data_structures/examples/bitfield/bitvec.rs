// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! bitvec = "1.1.1"
//! ```

use bitvec::field::BitField;
use bitvec::prelude::*;

/// Demonstrates reading individual bits in a bit vector.
///
/// # Arguments
///
/// * `bv` - The bit vector to inspect.
fn bit_indexing(bv: &BitVec<u8, Msb0>) {
    println!("===== Bit Access =====");
    // Access individual bits:
    println!("First bit: {}", bv[0]);
    println!("Second bit: {}", bv[1]);
}

/// Demonstrates mutating individual bits in a bit vector.
///
/// # Arguments
///
/// * `bv` - The bit vector to update.
fn bit_mutation(bv: &mut BitVec<u8, Msb0>) {
    println!("\n===== Bit Mutation =====");
    // Modify bits. The parameters are: index, value:
    bv.set(0, true);
    bv.set(5, true);
    assert!(bv[0]);
    assert!(bv[5]);
    println!("Modified BitVec: {bv}");
}

/// Demonstrates length, all-bit, any-bit, and set-bit queries.
///
/// # Arguments
///
/// * `bv` - The bit vector to query.
fn bit_checking(bv: &BitVec<u8, Msb0>) {
    println!("\n===== Bit Checking and Querying =====");
    // Get the length:
    println!("BitVec length: {}", bv.len());

    // Check if all bits are set:
    println!("All bits set: {}", bv.all());

    // Check if any bits are set:
    println!("Any bits set: {}", bv.any());

    // Count set bits:
    println!("Number of set bits: {}", bv.count_ones());
    assert_eq!(bv.len(), 8);
    assert_eq!(bv.count_ones(), 6);
    assert!(bv.any());
    assert!(!bv.all());
}

/// Prints each bit in the supplied vector in order.
///
/// # Arguments
///
/// * `bv` - The bit vector to iterate over.
fn bit_iteration(bv: &BitVec<u8, Msb0>) {
    println!("\n===== Bit Iteration =====");
    // Iterate through bits:
    print!("Iterating through bits: ");
    for bit in bv {
        print!("{bit} ");
    }
    println!();
}

/// Creates a bit vector and demonstrates access, mutation, queries, and
/// iteration.
fn basic_operations() {
    // Create a new `BitVec` with default parameters:
    let mut bv = bitvec![u8, Msb0; 0, 1, 0, 1, 1, 0, 1, 0];
    println!("Original BitVec: {bv}");

    bit_indexing(&bv);
    bit_mutation(&mut bv);
    bit_checking(&bv);
    bit_iteration(&bv);
}

/// Demonstrates different bit orders and storage element widths.
fn endianness_and_storage() {
    println!("\n===== Different Endianness and Storage Types =====");

    // Create `BitVec`s with different storage types and endianness:
    let bv_lsb0 = bitvec![u16, Lsb0; 0, 1, 0, 1, 1, 0, 1, 0];
    println!("u16 LSB BitVec: {bv_lsb0}");

    let bv_u32 = bitvec![u32, Msb0; 0, 1, 0, 1, 1, 0, 1, 0];
    println!("u32 MSB BitVec: {bv_u32}");
}

/// Demonstrates bitwise combination and in-place operations on bit vectors.
fn bit_manipulation() {
    println!("\n===== Bit Manipulation Operations =====");

    // Create two `BitVec`s for bitwise operations:
    let mut a = bitvec![u8, Msb0; 1, 1, 0, 0, 1, 0, 1, 0];
    let b = bitvec![u8, Msb0; 0, 1, 1, 0, 0, 1, 1, 1];

    println!("a: {a}");
    println!("b: {b}");

    // Bitwise AND:
    let c = a.clone() & b.clone();
    println!("a & b: {c}");
    assert_eq!(c, bitvec![u8, Msb0; 0, 1, 0, 0, 0, 0, 1, 0]);

    // Bitwise OR:
    let d = a.clone() | b.clone();
    println!("a | b: {d}");
    assert_eq!(d, bitvec![u8, Msb0; 1, 1, 1, 0, 1, 1, 1, 1]);

    // Bitwise XOR:
    let e = a.clone() ^ b.clone();
    println!("a ^ b: {e}");
    assert_eq!(e, bitvec![u8, Msb0; 1, 0, 1, 0, 1, 1, 0, 1]);

    // Bitwise NOT:
    let f = !a.clone();
    println!("!a: {f}");
    assert_eq!(f, bitvec![u8, Msb0; 0, 0, 1, 1, 0, 1, 0, 1]);

    // In-place operations:
    a &= &b;
    println!("a &= b: {a}");
    assert_eq!(a, c);
}

/// Demonstrates reading and mutating ranges within a bit vector.
fn slicing_and_ranges() {
    println!("\n===== Slicing and Ranges =====");

    let mut bv_slice = bitvec![u8, Msb0; 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0];
    println!("Original BitVec: {bv_slice}");

    // Get a slice:
    let slice = &bv_slice[2..6];
    println!("Slice [2..6]: {slice}");
    assert_eq!(slice.count_ones(), 2);

    // Modify a slice:
    bv_slice[4..8].fill(true);
    println!("After filling [4..8] with `1`s: {bv_slice}");
    assert!(bv_slice[4..8].all());
}

/// Demonstrates flags, boolean packing, and storing values in bit ranges.
fn practical_applications() {
    println!("\n===== Practical Applications =====");

    // Example 1: Bit flags e.g. file permissions.
    let mut flags = bitvec![u8, Msb0; 0; 8];

    // Set flags:
    flags.set(0, true); // READ.
    flags.set(1, true); // WRITE.
    flags.set(2, false); // EXECUTE.

    println!("Permission flags: {flags}");
    println!("Can read: {}", flags[0]);
    println!("Can write: {}", flags[1]);
    println!("Can execute: {}", flags[2]);
    assert!(flags[0]);
    assert!(flags[1]);
    assert!(!flags[2]);

    // Example 2: Bit packing - pack multiple boolean values efficiently.
    let mut packed = BitVec::<u8, Msb0>::new();

    // Appends single bits to the vector:
    packed.push(true); // is_active.
    packed.push(false); // is_admin.
    packed.push(true); // is_verified.
    packed.push(false); // is_premium.

    println!("Packed bits: {packed}");
    assert_eq!(packed.len(), 4);
    assert_eq!(packed.count_ones(), 2);

    // Example 3: Bit-packed struct.
    let mut data = bitvec![u16, Lsb0; 0; 16];

    // Pack (small) values into specific bit ranges:
    data[0..3].store(0b101u8);
    data[3..8].store(0b10011u8);
    data[8..16].store(0b11001010u8);

    println!("Bit-packed data: {data}");

    // Extract values:
    let value1: u8 = data[0..3].load();
    let value2: u8 = data[3..8].load();
    let value3: u8 = data[8..16].load();

    println!("Extracted values: {value1}, {value2}, {value3}");
    assert_eq!((value1, value2, value3), (0b101, 0b10011, 0b11001010));
}

/// Runs each bit-vector demonstration in sequence.
fn main() {
    basic_operations();
    endianness_and_storage();
    bit_manipulation();
    slicing_and_ranges();
    practical_applications();
}

// ANCHOR_END: example

/// Runs the bit-vector example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the bitvec example through its entry point.
    #[test]
    fn test_bitvec_example_entry_point() {
        main();
    }
}
