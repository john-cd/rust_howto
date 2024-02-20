// on structs
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
struct S(i32);

#[test]
fn test() {
    println!("{:?}", S(0));
    println!("{}", S(1) == S(1));
}
