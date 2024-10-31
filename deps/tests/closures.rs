fn find_emails(list: Vec<String>) -> Vec<String> {
    list.into_iter()
        .filter(|s| s.contains('@')) // <-- closure
        .collect()
}

fn main() {
    for s in find_emails(vec![
        String::from("example"),
        String::from("example@example.com"),
    ]) {
        println!("{}", s);
    }
}

#[test]
fn test() {
    main();
}
