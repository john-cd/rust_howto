#![allow(dead_code)]
// ANCHOR: example
use lazy_static::lazy_static;
use regex::Regex;

fn extract_login(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
            "
        )
        .unwrap();
    }
    RE.captures(input)
        .and_then(|cap| cap.name("login").map(|login| login.as_str()))
}

fn main() {
    let login = extract_login(r"I❤email@example.com");
    println!("{:?}", login);
    assert_eq!(login, Some(r"I❤email"));

    let login = extract_login(r"sdf+sdsfsd.as.sdsd@jhkk.d.rl");
    println!("{:?}", login);
    assert_eq!(login, Some(r"sdf+sdsfsd.as.sdsd"));

    assert_eq!(extract_login(r"More@Than@One@at.com"), None);
    assert_eq!(extract_login(r"Not an email@email"), None);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
