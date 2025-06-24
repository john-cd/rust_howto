#![allow(dead_code)]
// ANCHOR: example
// The unit type is used when there is nothing to return:
#[allow(clippy::unused_unit)]
fn a_func() -> () {
    println!("a_func was called.");
}

// `()` is most commonly seen implicitly.
// Functions without a `-> ...` implicitly have return type `()`.
// The following is equivalent to the above:
fn b_func() {
    println!("b_func was called.");
}
// The semicolon ; discards the result of an expression at the end of a block,
// so that the block returns `()`.

// The `!` type, also called "never", represents the type of computations which
// never resolve to any value at all.
fn diverge() -> ! {
    panic!("This function never returns!");
}
// This function could also contain an infinite loop,
// or call the `exit` function.

fn main() {
    a_func();
    b_func();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}

#[should_panic]
#[test]
fn test_panic() {
    diverge();
}
