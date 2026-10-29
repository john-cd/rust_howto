// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Borrowing the content of an `Option` with `as_ref`.

// `Option::as_ref()` allows us to get an `Option<&String>`.
// This lets us borrow the inner `String` if it exists, without moving it out of
// `maybe_name`.
/// Prints the name, if there is one, without consuming the `Option`.
fn process_optional_string(maybe_name: Option<String>) {
    let borrowed_name: Option<&String> = maybe_name.as_ref();
    // If we used `maybe_name.unwrap()`, `maybe_name` would be consumed.
    // let consumed_name = maybe_name.unwrap();

    if let Some(name_ref) = borrowed_name {
        println!("The name is: {name_ref}");
        // We can still use `maybe_name` here,
        // because `as_ref()` didn't consume it.
        println!("Original option (still valid): {maybe_name:?}");
    } else {
        println!("No name provided.");
    }

    // We can also get `Option<&str>` directly if the inner type is `String`.
    // `as_deref` is a shortcut for `opt.as_ref().map(|s| s.as_str())` or
    // equivalent `opt.as_ref().map(|s| &*s)`.
    let borrowed_str: Option<&str> = maybe_name.as_deref();

    if let Some(name_str) = borrowed_str {
        println!("The name as &str: {name_str}");
    } else {
        println!("Still no name as &str.");
    }
}

/// Processes a present and a missing name.
fn main() {
    process_optional_string(Some("Alice".to_owned()));
    process_optional_string(None);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_asref_example_entry_point() {
        main();
    }

    #[test]
    fn test_as_ref_and_as_deref_do_not_consume_the_option() {
        let name = Some("Alice".to_owned());
        let r: Option<&String> = name.as_ref();
        assert_eq!(r.map(String::len), Some(5));
        let d: Option<&str> = name.as_deref();
        assert_eq!(d, Some("Alice"));
        // The original option is still usable.
        assert_eq!(name, Some("Alice".to_owned()));
    }

    #[test]
    fn test_none_stays_none() {
        let none: Option<String> = None;
        assert_eq!(none.as_ref(), None);
        assert_eq!(none.as_deref(), None);
        process_optional_string(none);
    }
}
