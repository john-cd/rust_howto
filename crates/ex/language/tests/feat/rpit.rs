// ANCHOR: example
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    let f = returns_closure();
    println!("{}", f(1));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
