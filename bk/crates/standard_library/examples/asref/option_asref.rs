#![allow(dead_code)]
// ANCHOR: example
// `Option::as_ref()` allows us to get an `Option<&String>`.
// This lets us borrow the inner `String` if it exists, without moving it out of
// `maybe_name`.
fn process_optional_string(maybe_name: Option<String>) {
    let borrowed_name: Option<&String> = maybe_name.as_ref();
    // If you used `maybe_name.unwrap()`, `maybe_name` would be consumed.
    // let consumed_name = maybe_name.unwrap();

    if let Some(name_ref) = borrowed_name {
        println!("The name is: {}", name_ref);
        // We can still use `maybe_name` here,
        // because `as_ref()` didn't consume it.
        println!("Original option (still valid): {:?}", maybe_name);
    } else {
        println!("No name provided.");
    }

    // You can also get `Option<&str>` directly if the inner type is `String`.
    // `as_deref` is a shortcut for `opt.as_ref().map(|s| s.as_str())` or
    // equivalent `opt.as_ref().map(|s| &*s)`.
    let borrowed_str: Option<&str> = maybe_name.as_deref();

    if let Some(name_str) = borrowed_str {
        println!("The name as &str: {}", name_str);
    } else {
        println!("Still no name as &str.");
    }
}

fn main() {
    process_optional_string(Some("Alice".to_string()));
    process_optional_string(None);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
