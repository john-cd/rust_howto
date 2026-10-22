#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Borrowing the content of an `Option` with `as_ref` and `as_mut`.

/// Reads an `Option<String>` without consuming it.
fn as_ref() {
    let text: Option<String> = Some("Hello, world!".to_owned());
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume that with `map`, leaving `text` on the stack.
    let text_length: Option<usize> =
        text.as_ref().map(std::string::String::len);
    println!("We still can print `text`: {text:?}. Length: {text_length:?}");
}

/// Modifies the content of an `Option` in place.
fn as_mut() {
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => {
            *v = 42;
            println!("x is Some({v})");
        }
        None => {
            println!("x is None");
        }
    }
    assert_eq!(x, Some(42));
}

/// Runs both examples.
fn main() {
    as_ref();
    as_mut();
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
