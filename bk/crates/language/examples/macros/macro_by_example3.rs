#![allow(unused)]
// ANCHOR: example
macro_rules! sum {
    // `$x:expr` matches an expression.
    // `$($x:expr),*` matches zero or more occurrences of `$x:expr` separated by commas.
    //
    // To allow for an optional trailing comma, add `$(,)?`. The `?` makes the comma pattern optional.
    ( $( $x:expr ),* ) => {
        {
            let mut temp = 0;
            $(
                temp += $x;
            )*
            temp
        }
    };
}
// Note the double set of { } above.
// One is part of the macro-by-example syntax,
// the other is a block that scopes the `temp` variable
// and avoid name collisions.

fn main() {
    let s = sum!(1, 2, 3, 4);
    println!("{s}");
    assert_eq!(s, 10);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
