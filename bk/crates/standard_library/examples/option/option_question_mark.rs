// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! The `?` operator shines when we have a sequence of operations, each of
//! which might fail. It provides an early exit mechanism for functions that
//! return `Option`. If any intermediate step returns `None`, the function
//! immediately returns `None`.
//!
//! `?` works as well within functions that return `Result`.

/// Attempt to retrieve the first character from the input string `s`.
fn get_first_char_uppercase(input: Option<String>) -> Option<char> {
    // If `input` is `None`, the `?` operator will return `None` from this
    // function. If `input` is `Some`, it unwraps `input` and assigns it to
    // `s`.
    let s = input?;

    // If the string is empty, `chars().next()` returns `None`, indicating no
    // character is found. The `?` operator then immediately returns `None`
    // from the function, effectively propagating the absence of a character
    // up the call stack.
    let first_char = s.chars().next()?;

    // Convert the character to its uppercase equivalent.
    // `to_ascii_uppercase()` returns a char.
    Some(first_char.to_ascii_uppercase())
}

/// Applies the `?` operator to `Option` values.
fn main() {
    // `Option` contains a `String` and it's not empty.
    let s1 = Some("hello".to_owned());
    match get_first_char_uppercase(s1) {
        Some(c) => println!("First uppercase char: {c}"), // Output: H.
        None => println!("Could not process string."),
    }

    // `Option` contains an empty `String` (or is `None`).
    let s2 = Some("".to_owned());
    match get_first_char_uppercase(s2) {
        Some(c) => println!("First uppercase char: {c}"),
        None => println!("Could not process string."), /* Output: Could not
                                                        * process string. */
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_question_mark_example_entry_point() {
        main();
    }

    #[test]
    fn test_first_char_is_uppercased() {
        assert_eq!(
            get_first_char_uppercase(Some("hello".to_owned())),
            Some('H')
        );
        assert_eq!(
            get_first_char_uppercase(Some("Hello".to_owned())),
            Some('H')
        );
        assert_eq!(
            get_first_char_uppercase(Some("1abc".to_owned())),
            Some('1')
        );
    }

    #[test]
    fn test_none_and_empty_string_give_none() {
        assert_eq!(get_first_char_uppercase(None), None);
        assert_eq!(get_first_char_uppercase(Some(String::new())), None);
    }
}
