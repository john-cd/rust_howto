// ANCHOR: example

// Note the `&mut` in the function's signature.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{some_string}");
}

fn main() {
    let mut s = String::from("hello"); // note the `mut`
    change(&mut s);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
