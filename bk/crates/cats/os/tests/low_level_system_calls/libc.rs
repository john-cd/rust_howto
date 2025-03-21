// ANCHOR: example
use std::ffi::CString;

use libc::c_char;

// Declare an external C function puts that takes a pointer to a c_char (C
// string) and returns an i32.
unsafe extern "C" {
    fn puts(s: *const c_char) -> i32;
}

fn main() {
    // create a CString containing the message we want to print and call the
    // puts function inside an unsafe block, since interacting with C code is
    // considered unsafe in Rust.
    let c_string = CString::new("Hello from C!").expect("CString::new failed");
    unsafe {
        puts(c_string.as_ptr());
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [NOW review](https://github.com/john-cd/rust_howto/issues/816)
