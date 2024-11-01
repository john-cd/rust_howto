// ANCHOR: example
// on structs
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
struct S(i32);

fn main() {
    println!("{:?}", S(0));
    println!("{}", S(1) == S(1));
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
