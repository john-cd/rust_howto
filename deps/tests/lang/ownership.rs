// ANCHOR: example
fn main() {
    let s1 = String::from("hello"); // On the heap
    let _s2 = s1; // s1 was MOVED into s2 - NOT a shallow copy
                  // Rust invalidates s1
                  // ERROR println!("{}, world!", s1);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
