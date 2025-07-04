#![allow(dead_code)]
#![allow(unused_variables)]
// ANCHOR: example
use std::ffi::CString;
use std::os::raw::c_char;

use anyhow::Result;

fn prompt(s: &str) -> Result<String> {
    use std::io::Write;
    print!("{s}");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

unsafe extern "C" {
    fn hello();
    fn greet(name: *const c_char);
}

/// This example demonstrates how to call C functions from Rust.
/// It uses the `cc` crate to compile a C file into a static library,
/// and then links against that library.
fn main() -> Result<()> {
    // unsafe { hello() }
    let name = prompt("What's your name? ")?;
    let c_name = CString::new(name)?;
    // unsafe { greet(c_name.as_ptr()) }
    Ok(())
}
// ANCHOR_END: example

#[test]
#[ignore = "Needs review"]
fn test() {
    println!("{:?}", main());
}
// [finish; deal with extern](https://github.com/john-cd/rust_howto/issues/900)
