#![allow(dead_code)]
// ANCHOR: example
unsafe extern "C" {
    fn multiply(x: i32, y: i32) -> i32;
}

fn main() {
    // unsafe {
    //     println!("{}", multiply(5, 7));
    // }
}
// ANCHOR_END: example

#[test]
#[ignore = "WIP"]
fn test() {
    main();
}
// [WIP finish; deal with extern](https://github.com/john-cd/rust_howto/issues/898)
