// // ANCHOR: example
// use hex_literal::hex;
// use sha1::Digest;
// use sha1::Sha1;

// SHA-1 is considered cryptographically broken and should not be used for new
// security-critical applications. It is primarily used for legacy compatibility
// or non-security-sensitive purposes.

// fn main() {
//     // Create a Sha1 object
//     let mut hasher = Sha1::new();

//     // Process input message
//     hasher.update(b"hello world");

//     // Acquire hash digest
//     let result = hasher.finalize();

//     // Assert the expected hash value
//     assert_eq!(result[..], hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));

//     println!("SHA-1 hash of 'hello world': {:x}", result);
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/702)
