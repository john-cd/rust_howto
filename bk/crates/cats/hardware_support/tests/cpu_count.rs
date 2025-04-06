// ANCHOR: example

/// This example demonstrates how to get the number of logical cores on the
/// system. It uses the `num_cpus` crate.
fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
