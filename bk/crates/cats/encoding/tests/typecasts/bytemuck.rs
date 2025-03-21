// ANCHOR: example
// COMING SOON
// ANCHOR_END: example
// use bytemuck::Zeroable;
// use bytemuck::AnyBitPattern;
use bytemuck::NoUninit;

// The bytemuck crate provides functions for reinterpreting one type as another
// without use of `unsafe`, whenever this is safe because both types are “just
// bytes” (no invalid values and no padding).

//#[derive(AnyBitPattern)]

#[repr(C)]
#[derive(Debug, Clone, Copy, NoUninit)]
struct MyStruct {
    x: i32,
    y: [u8; 4],
}

fn to_from_bytes() {
    // Create an instance of MyStruct
    let my_struct = MyStruct {
        x: 42,
        y: [1, 2, 3, 4],
    };
    // MyStruct { x: 42, y: [1, 2, 3, 4] }
    println!("{:?}", my_struct);

    // Convert `MyStruct` to bytes
    // `bytes_of` re-interprets &T as &[u8], but only if T is `NoUninit` that is
    // "plain old data" types with no uninit (or padding) bytes
    let bytes: &[u8] = bytemuck::bytes_of(&my_struct);
    // Bytes: [42, 0, 0, 0, 1, 2, 3, 4] (on a little-endian machine)
    println!("Bytes: {:?}", bytes);

    // FIXME
    // Convert bytes back to `MyStruct`
    // Re-interprets &[u8] as &T, but only if T is `AnyBitPattern`, i.e. "plain
    // old data" types that are valid for any bit pattern. let recovered:
    // &MyStruct = bytemuck::from_bytes(bytes); println!("{:?}", recovered);
}

fn cast() {
    let sixteens: [u16; 4] = [1, 2, 3, 4];
    println!("Sixteens: {:?}", sixteens);
    // `cast` is purely changing the type, thus have no run-time cost.
    // It does not reorder bytes to a specific endianness
    let eights: [u8; 2 * 4] = bytemuck::cast(sixteens);
    println!("Eights: {:?}", eights);
}

fn main() {
    to_from_bytes();
    cast();
}

#[test]
fn test() {
    main();
}
// [finish NOW](https://github.com/john-cd/rust_howto/issues/753)
