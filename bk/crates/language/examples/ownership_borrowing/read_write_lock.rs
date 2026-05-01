#![allow(dead_code)]
// ANCHOR: example
#[derive(Debug)]
struct MyStruct(bool);

fn main() {
    let mut s1 = MyStruct(true);

    // We can take multiple immutable (shared) references at the same time:
    let ref_s1 = &s1;
    let ref_s2 = &s1;

    // We cannot modify `s1` or obtain a mutable (exclusive) reference to it
    // when holding immutable references.
    // s1.push('!');
    // ERROR: "cannot borrow `s1` as mutable because it is also borrowed as
    // immutable."

    println!("{ref_s1:?} {ref_s2:?}"); // Last use of the `ref_*` variables.

    // However, you can reassign the variable or obtain a mutable reference,
    // once the shared references are no longer in use.
    s1 = MyStruct(false);
    s1.0 = true;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
