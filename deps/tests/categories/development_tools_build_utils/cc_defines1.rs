// ANCHOR: example
unsafe extern "C" {
    fn print_app_info();
}

fn main() {
    unsafe {
        print_app_info();
    }
    println!("");
}
// ANCHOR_END: example

#[ignore]
#[test]
fn test() {
    main();
}
// [P1 deal with extern](https://github.com/john-cd/rust_howto/issues/901)
