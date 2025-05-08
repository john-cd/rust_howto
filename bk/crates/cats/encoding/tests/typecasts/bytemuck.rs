// ANCHOR: example
//! The `bytemuck` crate provides functions for reinterpreting one type as
//! another without use of `unsafe`, whenever this is safe e.g. because both
//! types are "just bytes" (i.e. no invalid values and no padding).
//!
//! - To cast `T`, use `try_cast` or `cast`.
//! - For `&T`, use `(try_)cast_ref`.
//! - For `&mut T`, use `(try_)cast_mut`.
//! - For `&[T]`, use `(try_)cast_slice`.
//! - For `&mut [T]`, use `(try_)cast_slice_mut`.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! bytemuck = { version = "1.21.0", features = ["derive"] }
//! ```
//!
//! The crate offers a number of additional features to work with allocated
//! types (`Box`, `Vec`...), `Zeroable` types, SIMD types, and to ensure at
//! compile time that a cast will work at runtime.
//!
//! It also offers casting for types that have some invalid bit patterns by
//! performing a runtime check. This is particularly useful for types like
//! fieldless ('C-style') enums, char, bool, and structs containing them.

use bytemuck::AnyBitPattern;
use bytemuck::NoUninit;

/// Cast type A into type B, here an array into another.
fn cast() {
    let sixteens: [u16; 4] = [1, 2, 3, 4];
    println!("Sixteens: {:?}", sixteens);

    // `cast` is purely changing the type, thus have no run-time cost.
    // - It does not reorder bytes to a specific endianness.
    // - It will panic on a size mismatch.
    let eights: [u8; 2 * 4] = bytemuck::cast(sixteens);
    println!("Eights: {:?}", eights);
}

/// You can also cast complex types, like a `struct`, provided that it has been
/// properly annotated.
///
/// The `NoUninit` and `AnyBitPattern` traits are used to maintain memory
/// safety. `NoUninit` is a marker trait for "plain old data" types with no
/// (uninitialized) padding bytes. `AnyBitPattern` marks "plain old data" types
/// that are valid for any bit pattern.
#[repr(C)]
#[derive(Debug, Clone, Copy, NoUninit, AnyBitPattern)]
struct MyStruct {
    x: i32,
    y: [u8; 4],
}

/// Cast the `struct` to an array.
fn cast_complex_type() {
    // Create an instance of `MyStruct`.
    let my_struct = MyStruct {
        x: 42,
        y: [1, 2, 3, 4],
    };
    println!("\n{:?}", my_struct);
    let my_ref = &my_struct;
    let r: Result<&[i32; 2], _> = bytemuck::try_cast_ref(my_ref);
    println!("{:?}", r);
}

/// You can also re-interpret `&T` as `&[u8]` and vice-versa.
fn to_from_bytes() {
    let my_struct = MyStruct {
        x: 42,
        y: [1, 2, 3, 4],
    };

    // `bytes_of` re-interprets `&T` as `&[u8]`, but again only if `T` is
    // `NoUninit` that is "plain old data" types with no uninit (or padding)
    // bytes.
    let bytes: &[u8] = bytemuck::bytes_of(&my_struct);
    // Print the bytes: [42, 0, 0, 0, 1, 2, 3, 4] (on a little-endian machine).
    println!("Bytes: {:?}", bytes);

    // Re-interprets `&[u8]` as `&T`, but only if `T` is `AnyBitPattern`, i.e.
    // "plain old data" types that are valid for any bit pattern.
    let recovered: &MyStruct = bytemuck::from_bytes(bytes);
    println!("Recovered: {:?}", recovered);
}

fn main() {
    cast();
    cast_complex_type();
    to_from_bytes();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
