// ANCHOR: example
macro_rules! print_args {
    // Rule 1: Matches one expression.
    ($e:expr) => {
        println!("Single argument: {:?}", $e);
    };
    // Rule 2: Matches two expressions (separated by a literal comma).
    ($e1:expr, $e2:expr) => {
        println!("Two arguments: {:?} and {:?}", $e1, $e2);
    };
}

fn main() {
    // Note how macros can be overloaded to accept different combinations of
    // arguments by having multiple rules:
    print_args!(10);
    print_args!("hello", true);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
