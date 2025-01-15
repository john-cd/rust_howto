// // ANCHOR: example
// // ANCHOR_END: example

// // `rustix` is a library that provides a safe and idiomatic Rust interface to
// // low-level system calls.

// use std::ffi::CString;

// use rustix::fs::AtFlags;
// use rustix::fs::Mode;
// use rustix::fs::OFlags;
// use rustix::fs::openat;
// use rustix::fs::read;
// use rustix::fs::write;
// use rustix::path::CStr;

// fn main() {
//     // Create a CString for the file path
//     let path = CString::new("example.txt").unwrap();

//     // Open the file for writing
//     let fd = openat(
//         rustix::fs::CWD,
//         &CStr::from_bytes_with_nul(b"example.txt\0").unwrap(),
//         OFlags::CREATE | OFlags::WRONLY | OFlags::TRUNC,
//         Mode::IRUSR | Mode::IWUSR,
//     )
//     .unwrap();

//     // Write to the file
//     let data = b"Hello, rustix!";
//     write(fd, data).unwrap();

//     // Close the file
//     rustix::fs::close(fd).unwrap();

//     // Open the file for reading
//     let fd = openat(
//         rustix::fs::cwd(),
//         &CStr::from_bytes_with_nul(b"example.txt\0").unwrap(),
//         OFlags::RDONLY,
//         Mode::empty(),
//     )
//     .unwrap();

//     // Read from the file
//     let mut buffer = vec![0; data.len()];
//     read(fd, &mut buffer).unwrap();

//     // Print the content of the file
//     println!("{}", String::from_utf8_lossy(&buffer));

//     // Close the file
//     rustix::fs::close(fd).unwrap();
// }

// #[test]
// fn test() {
//     main();
// }
// // [P2](https://github.com/john-cd/rust_howto/issues/821) fix
