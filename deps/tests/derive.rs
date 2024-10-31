// on structs
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
struct S(i32);

fn main() {
    println!("{:?}", S(0));
    println!("{}", S(1) == S(1));
}

#[test]
fn test() {
    main();
}
