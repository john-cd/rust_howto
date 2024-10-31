fn change(some_string: &mut String) {
    // note the `&mut`
    some_string.push_str(", world");
}

fn main() {
    let mut s = String::from("hello"); // note the `mut`
    change(&mut s);
}

#[test]
fn test() {
    main();
}
