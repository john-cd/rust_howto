// ANCHOR: example
fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
