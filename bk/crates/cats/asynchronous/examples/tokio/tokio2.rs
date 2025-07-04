#![allow(dead_code)]
// ANCHOR: example
/// Example of using a single-threaded Tokio runtime.
fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("Hello world");
        })
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
