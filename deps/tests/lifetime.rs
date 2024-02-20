#![allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

#[test]
fn test() {
    let ie = ImportantExcerpt { part: "a part" };
    println!("{}", ie.level());
}
