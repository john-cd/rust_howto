// ANCHOR: example
// COMING SOON
// ANCHOR_END: example
//! The `bytemuck` crate provides functions for reinterpreting one type as
//! another without use of `unsafe`, whenever this is safe because both types
//! are "just bytes" (i.e. no invalid values and no padding).

use bytemuck::AnyBitPattern;
use bytemuck::NoUninit;

/// The `NoUninit` and `AnyBitPattern` traits are used to maintain memory
/// safety. `NoUninit` is a marker trait for "plain old data" types with no
/// padding bytes. `AnyBitPattern` marks "plain old data" types that are valid
/// for any bit pattern.
#[repr(C)]
#[derive(Debug, Clone, Copy, NoUninit, AnyBitPattern)]
struct MyStruct {
    x: i32,
    y: [u8; 4],
}

/// Re-interprets `&T` as `&[u8]`.
fn to_from_bytes() {
    // Create an instance of `MyStruct`.
    let my_struct = MyStruct {
        x: 42,
        y: [1, 2, 3, 4],
    };
    println!("{:?}", my_struct);

    // Convert `MyStruct` to bytes.
    // `bytes_of` re-interprets &T as &[u8], but only if T is `NoUninit` that is
    // "plain old data" types with no uninit (or padding) bytes.
    let bytes: &[u8] = bytemuck::bytes_of(&my_struct);
    // Print the bytes: [42, 0, 0, 0, 1, 2, 3, 4] (on a little-endian machine).
    println!("Bytes: {:?}", bytes);

    // Convert bytes back to `MyStruct`.
    // Re-interprets &[u8] as &T, but only if T is `AnyBitPattern`, i.e. "plain
    // old data" types that are valid for any bit pattern.
    let recovered: &MyStruct = bytemuck::from_bytes(bytes);
    println!("Recovered: {:?}", recovered);
}

/// Cast type A into type B.
///
/// To cast `T`, use `cast`.
/// For `&T`, use `cast_ref`.
/// For `&mut T`, use `cast_mut`.
/// For `&[T]`, use `cast_slice`.
/// For `&mut [T]`, use `cast_slice_mut`.
fn cast() {
    let sixteens: [u16; 4] = [1, 2, 3, 4];
    println!("Sixteens: {:?}", sixteens);
    // `cast` is purely changing the type, thus have no run-time cost.
    // - It does not reorder bytes to a specific endianness.
    // - It will panic on a size mismatch.
    let eights: [u8; 2 * 4] = bytemuck::cast(sixteens);
    println!("Eights: {:?}", eights);
}

fn main() {
    to_from_bytes();
    cast();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [finish NOW](https://github.com/john-cd/rust_howto/issues/753)
