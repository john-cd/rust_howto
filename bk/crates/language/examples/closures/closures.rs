#![allow(dead_code)]
// ANCHOR: example
/// Finds all strings in a list that contain the '@' character.
fn find_emails(list: Vec<String>) -> Vec<String> {
    list.into_iter()
        // `filter` takes a predicate, a closure that returns a `bool`.
        .filter(|s| s.contains('@')) // Closure with a simple expression as its body. Note the `| |`.
        .collect()
}

fn main() {
    for s in find_emails(vec![
        String::from("example"),
        String::from("example@example.com"),
    ]) {
        println!("{s}");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
