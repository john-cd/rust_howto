// Must use the results of the fn; also applies to traits, structs,
// enums...
#[must_use]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test() {
    println!("{}", add(1, 2));
}
