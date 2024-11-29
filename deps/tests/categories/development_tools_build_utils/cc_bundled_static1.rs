// ANCHOR: example
// use std::ffi::CString;
// use std::os::raw::c_char;

// use anyhow::Result;

// fn prompt(s: &str) -> Result<String> {
//     use std::io::Write;
//     print!("{}", s);
//     std::io::stdout().flush()?;
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input)?;
//     Ok(input.trim().to_string())
// }

// extern "C" {
//     fn hello();
//     fn greet(name: *const c_char);
// }

// fn main() -> Result<()> {
//     unsafe { hello() }
//     let name = prompt("What's your name? ")?;
//     let c_name = CString::new(name)?;
//     unsafe { greet(c_name.as_ptr()) }
//     println!("");
//     Ok(())
// }
// ANCHOR_END: example

// // TODO P1
// #[test]
// #[ignore]
// fn test() {
//     println!("{:?}", main());
// }
