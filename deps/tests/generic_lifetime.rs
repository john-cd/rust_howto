fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn test() {
    let (x, y) = ("short", "looooooong");
    println!("{}", longest(x, y));
}
