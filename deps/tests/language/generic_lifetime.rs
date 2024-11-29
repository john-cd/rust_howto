// ANCHOR: example
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let (x, y) = ("short", "looooooong");
    println!("{}", longest(x, y));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
