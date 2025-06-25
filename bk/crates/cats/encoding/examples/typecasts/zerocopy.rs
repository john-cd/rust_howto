#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates zero-cost memory manipulation with `zerocopy`.
//!
//! First, add the following to your `Cargo.toml`:
//! ```toml
//! zerocopy = { version = "0.8.14", features = ["derive"] } # or latest version
//! ```
//!
//! For performance reasons, CPUs often strongly prefer or even mandate storing
//! data in memory at addresses that are multiples of the word size (usually 4
//! or 8 bytes). For example, on a x86 architecture, `u64` and `f64` are often
//! aligned to 4 bytes (32 bits).
//!
//! In order to not waste space while still respecting the CPU's preferred
//! alignment, the Rust compiler optimizes the layout of composite data
//! structures (e.g. structs, tuples, arrays, enums...) when storing them in
//! memory. It may reorder their fields, and may insert "gaps" of one or more
//! bytes (often called padding) before, between, and after the fields.
//! As a result, structs, enums... can't, in general, be treated as contiguous
//! blocks of bytes.
//!
//! However, _some can_, given proper restrictions on layout (aka
//! "representation") and field types. `zerocopy` performs a sophisticated,
//! compile-time safety analysis to determine whether such zero-cost, zero-copy
//! conversions are safe. See e.g. <https://docs.rs/zerocopy/0.8.14/zerocopy/derive.IntoBytes.html#analysis>
//!
//! For that purpose, Zerocopy provides several derivable traits: `FromBytes`,
//! `TryFromBytes`, `FromZeros`, `Immutable`...

use std::mem::size_of;

// `FromBytes` indicates that a type may safely be converted from an
// arbitrary byte sequence This is useful for efficiently deserializing
// structured data from raw bytes. Do not implement this trait yourself!
// Instead, derive it, as we do below.
use zerocopy::FromBytes;
// There is also a `TryFromBytes` trait for types that may safely be
// converted from certain byte sequences (conditional on runtime checks).

// `FromZeros` indicates that a sequence of zero bytes represents a valid
// instance of a type.
use zerocopy::FromZeros;
// `Immutable` is a marker trait that flags types which are free from
// interior mutability.
use zerocopy::Immutable;
// `Immutable` is required to call certain methods provided by the
// conversion traits: `IntoBytes` indicates that a type may safely be
// converted to a byte sequence of initialized bytes of the same size.
// This is useful for efficiently serializing structured data as raw bytes.
// Do not implement this trait yourself! Instead, derive it.
use zerocopy::IntoBytes;
// `KnownLayout` is a marker trait that indicates that zerocopy can reason
// about certain aspects of a type's layout.
use zerocopy::KnownLayout;

// 1. Let's first demonstrate `FromZeros`:

#[derive(FromZeros, Debug)]
struct MyZeroableStruct {
    field: [u8; 8],
}

fn manipulate_zero_bytes() {
    // Creates an instance from zeroed bytes.
    let my_struct: MyZeroableStruct = FromZeros::new_zeroed();
    assert_eq!(my_struct.field, [0; 8]);

    let mut my_struct2 = MyZeroableStruct { field: [1; 8] };
    println!("{:?}", my_struct2);

    // Sets every byte in self to 0. While this is similar to doing
    // *self = Self::new_zeroed(), it differs in that zero does not semantically
    // drop the current value and replace it with a new one - it simply
    // modifies the bytes of the existing value.
    my_struct2.zero();
    assert_eq!(my_struct2.field, [0; 8]);
}

// 2. We then define a fictive network `PacketHeader` structure.

// As discussed above, all user-defined composite types (structs, enums,
// unions...) have a representation that specifies what the layout (its size,
// alignment, and the order / relative offsets of its fields) is for the type:
// <https://doc.rust-lang.org/reference/type-layout.html#representations>
//
// Types you expect to pass through an FFI / network boundary most often are
// most often `repr(C)`, as C is the lingua-franca of the programming world.
// It means that their layout is exactly that C or C++ expect.
#[repr(C)]
// We derive the Zerocopy traits that are required to convert from / to bytes.
#[derive(FromBytes, IntoBytes, Immutable, KnownLayout, PartialEq, Debug)]
struct PacketHeader {
    src_port: [u8; 2],
    dst_port: [u8; 2],
    length: [u8; 2],
    checksum: [u8; 2],
}

/// Define a `Packet` struct.
/// Note that, in this case, the Packet's `body` is a slice, which can have
/// different lengths at runtime. Zerocopy can handle a "slice-based dynamically
/// sized type". A slice DST is a type whose trailing field is either a slice or
/// another slice DST, rather than a type with fixed size.
#[repr(C)]
#[derive(FromBytes, Immutable, KnownLayout)]
struct Packet {
    header: PacketHeader,
    body: [u8],
}

/// Convert the `PacketHeader` into bytes.
fn into_bytes() -> anyhow::Result<()> {
    let mut header = PacketHeader {
        src_port: [0, 1],
        dst_port: [2, 3],
        length: [4, 5],
        checksum: [6, 7],
    };

    let bytes: &mut [u8] = header.as_mut_bytes();
    // Note: there is also an `as_bytes` method.

    assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6, 7]);
    // Note that `bytes` and `header` share the same memory.

    bytes.reverse();

    assert_eq!(
        header,
        PacketHeader {
            src_port: [7, 6],
            dst_port: [5, 4],
            length: [3, 2],
            checksum: [1, 0],
        }
    );

    // You can also write a copy to a destination byte array.
    // If too many or too few target bytes are provided, `write_to` returns
    // `Err` and leaves the target bytes unmodified.
    let mut buf = [0, 0, 0, 0, 0, 0, 0, 0];
    header
        .write_to(&mut buf[..])
        .map_err(|_| anyhow::anyhow!("write_to error!"))?;
    assert_eq!(buf, [7, 6, 5, 4, 3, 2, 1, 0]);
    // There are also methods to write to the beginning or end of a byte buffer
    // or to IO.

    Ok(())
}

/// Convert bytes into a `Packet`.
fn from_bytes() -> anyhow::Result<()> {
    // These bytes encode a `Packet`.
    let bytes: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11][..];
    let packet = Packet::ref_from_bytes(bytes)?;
    assert_eq!(packet.header.src_port, [0, 1]);
    assert_eq!(packet.header.dst_port, [2, 3]);
    assert_eq!(packet.header.length, [4, 5]);
    assert_eq!(packet.header.checksum, [6, 7]);
    // The bytes beyond the 8th are the body:
    assert_eq!(packet.body, [8, 9, 10, 11]);

    Ok(())
}

// 3. Enums can also be used.

/// An enum can implement `IntoBytes`, if it has
/// - a defined representation (reprs C, u8, u16, u32, u64, usize, i8, i16, i32,
///   i64, or isize).
/// - no padding bytes; and
/// - `IntoBytes` fields.
///
/// Here, the enum is field-less. It stores 0 for A, 1 for B, etc... in a byte.
#[repr(u8)]
#[derive(IntoBytes, Immutable)]
enum MyEnum {
    #[allow(dead_code)]
    A,
    B,
}

fn enums_also_work() {
    assert_eq!(size_of::<MyEnum>(), 1);

    let my_enum = MyEnum::B;
    let bytes = my_enum.as_bytes();
    assert_eq!(bytes, [1]);
}

/// 4. Safely transmutes a value of one type to a value of another type of the
///    same size.
fn transmute() {
    let one_dimensional: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let two_dimensional: [[u8; 4]; 2] = zerocopy::transmute!(one_dimensional);
    assert_eq!(two_dimensional, [[0, 1, 2, 3], [4, 5, 6, 7]]);
}

fn main() -> anyhow::Result<()> {
    manipulate_zero_bytes();
    into_bytes()?;
    from_bytes()?;
    enums_also_work();
    transmute();
    Ok(())
}
// Examples adapted from <https://docs.rs/zerocopy/>
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
