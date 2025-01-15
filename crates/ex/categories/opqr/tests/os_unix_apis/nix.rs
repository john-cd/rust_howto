// // ANCHOR: example
// // ANCHOR_END: example

// use std::os::fd::AsRawFd;

// // File control options
// use nix::fcntl::OFlag;
// use nix::fcntl::open;
// use nix::sys::stat::Mode;
// // Safe wrappers around functions found in libc “unistd.h” header
// use nix::unistd::close;
// use nix::unistd::read;
// use nix::unistd::unlink;
// use nix::unistd::write;

// // Simple Rust example using the `nix` crate to handle Unix-like operating
// // system tasks: create a file, write to it, read from it, and delete it.

// fn main() {
//     // Create and open a file
//     let path = "temp/example.txt";
//     let fd = open(
//         path,
//         OFlag::O_CREAT | OFlag::O_WRONLY,
//         Mode::S_IRUSR | Mode::S_IWUSR,
//     )
//     .unwrap();

//     // Write to the file
//     let data = b"Hello, nix!";
//     write(fd.as_raw_fd(), data).unwrap();

//     // Close the file
//     close(fd.as_raw_fd()).unwrap();

//     // Open the file for reading
//     let fd = open(path, OFlag::O_RDONLY, Mode::empty()).unwrap();

//     // Read from the file
//     let mut buffer = vec![0; data.len()];
//     read(fd, &mut buffer).unwrap();

//     // Print the content of the file
//     println!("{}", String::from_utf8_lossy(&buffer));

//     // Close the file
//     close(fd).unwrap();

//     // Delete the file
//     unlink(path).unwrap();
// }

// #[test]
// fn test() {
//     main();
// }
// // [P2](https://github.com/john-cd/rust_howto/issues/820)
// // TODO P1 make sure temp folder exists
