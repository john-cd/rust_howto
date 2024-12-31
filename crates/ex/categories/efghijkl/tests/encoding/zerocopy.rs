// // ANCHOR: example
// use zerocopy::FromBytes;
// use zerocopy::FromZeros;
// use zerocopy::Immutable;
// use zerocopy::IntoBytes;

// // Add this to your `Cargo.toml`:
// // zerocopy = { version = "0.8.14", features = ["derive"] } # or the latest
// version

// // Any `IntoBytes` type can be converted to an  immutable slice of
// initialized // bytes of the same size. This is useful for efficiently
// serializing structured // data as raw bytes. Do not implement this trait
// yourself! Instead, derive it.

// // Any `FromBytes` type can be created from an immutable slice of initialized
// // bytes of the same size. This is useful for efficiently deserializing
// structured data from raw bytes. // Do not implement this trait yourself!
// Instead, derive it.

// #[derive(FromBytes, IntoBytes, Immutable)]
// #[repr(C)]
// struct PacketHeader {
//     src_port: [u8; 2],
//     dst_port: [u8; 2],
//     length: [u8; 2],
//     checksum: [u8; 2],
// }

// fn into_bytes() {
//         let header = PacketHeader {
//         src_port: [0, 1],
//         dst_port: [2, 3],
//         length: [4, 5],
//         checksum: [6, 7],
//     };

//     let bytes = header.as_bytes();

//     assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6, 7]);

//     // Writes a copy to the destination byte array.
//     let mut bytes = [0, 0, 0, 0, 0, 0, 0, 0];
//     header.write_to(&mut bytes[..]);
//     assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6, 7]);
// }

// fn from_bytes() {
//     let bytes = [0, 1, 2, 3, 4, 5, 6, 7];
//     let header = PacketHeader::from_bytes(&bytes);

//     assert_eq!(header.src_port, [0, 1]);
//     assert_eq!(header.dst_port, [2, 3]);
//     assert_eq!(header.length, [4, 5]);
//     assert_eq!(header.checksum, [6, 7]);
// }

// #[derive(IntoBytes)]
// #[repr(u8)]
// enum MyEnum {
//     A(u8),
//     B(u16),
// }

// fn enums_also_work() {
//     let my_enum = MyEnum::A(42);
//     let bytes = my_enum.as_bytes();
//     assert_eq!(bytes, [42]);
// }

// // `FromZeros` indicates that a sequence of bytes all set to zero represents
// a valid instance of the type #[derive(FromZeros, Debug)]
// struct MyZeroableStruct {
//     field: [u8; 8],
// }

// fn manipulate_zero_bytes() {
//     // Creates an instance from zeroed bytes.
//     let my_struct: MyZeroableStruct = FromZeros::new_zeroed();
//     assert_eq!(my_struct.field, [0; 8]);

//     let mut my_struct = MyZeroableStruct { field: [1; 8] };
//     println!("{:?}", my_struct);
//     // Sets every byte in self to 0. While this is similar to doing *self =
// Self::new_zeroed(), it differs in that zero does not semantically drop the
// current value and replace it with a new one — it simply modifies the bytes of
// the existing value.     my_struct.zero();
//     assert_eq!(my_struct.field, [0; 8]);
// }

// // Safely transmutes a value of one type to a value of another type of the
// same size. fn transmute() {
//     let one_dimensional: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
//     let two_dimensional: [[u8; 4]; 2] =
// zerocopy::transmute!(one_dimensional);     assert_eq!(two_dimensional, [[0,
// 1, 2, 3], [4, 5, 6, 7]]); }

// fn main() {

//     into_bytes();
//     enums_also_work();

//     // // Create a byte array
//     // let mut bytes = [0u8; 9];

//     // // Create an instance of MyStruct from the byte array
//     // let my_struct = unsafe { MyStruct::from_bytes(&bytes) };
//     // println!("my_struct before: {:?}", my_struct);

//     // // Modify the struct's fields
//     // my_struct.a = 42;
//     // my_struct.b = 123;
//     // my_struct.c = 255;

//     // // Access the modified bytes directly
//     // println!("bytes after: {:?}", bytes);

//     // // Create a new instance of MyStruct from the modified bytes
//     // let my_struct_new = unsafe { MyStruct::from_bytes(&bytes) };
//     // println!("my_struct_new: {:?}", my_struct_new);

//     manipulate_zero_bytes();
//     transmute();
// }
// // ANCHOR_END: example

// #[test]
// #[ignore = "not yet implemented"]
// fn test() {
//     main();
// }
// // [P0](https://github.com/john-cd/rust_howto/issues/758)
