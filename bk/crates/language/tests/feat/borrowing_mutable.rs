// ANCHOR: example
/// This function takes a mutable reference to a String and appends ", world" to
/// it.
fn change(some_string: &mut String) {
    some_string.push_str(", world"); // Modifies the string in place.
    println!("{some_string}");
}

fn main() {
    let mut s = String::from("hello"); // Note the `mut` keyword.
    change(&mut s);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
