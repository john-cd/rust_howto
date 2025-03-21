#[allow(dead_code)]
// ANCHOR: example
unsafe extern "C" {
    fn print_app_info();
}

fn main() {
    // unsafe {
    //     print_app_info();
    // }
    println!("Printed app info.");
}
// ANCHOR_END: example

#[ignore = "Needs review"]
#[test]
fn test() {
    main();
}
// [finish; deal with extern](https://github.com/john-cd/rust_howto/issues/901)
