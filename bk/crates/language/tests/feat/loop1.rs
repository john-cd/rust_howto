// ANCHOR: example

/// Demonstrates the use of a `loop` with a `break` statement to return a value.
fn main() {
    // Initialize a mutable counter.
    let mut counter = 0;

    // The `loop` keyword creates an infinite loop.
    // Note that it is an expression.
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            // The value passed to `break` is returned by the loop.
            // `continue` and loop labels also exist.
            // See https://doc.rust-lang.org/book/ch03-05-control-flow.html
        }
    };
    println!("{}", result);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
