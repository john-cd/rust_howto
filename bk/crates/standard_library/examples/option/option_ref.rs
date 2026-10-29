// TODO Reviewed ??-??-2026.

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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_ref_example_entry_point() {
        main();
    }

    #[test]
    fn test_as_ref_keeps_the_original() {
        let text = Some("Hello, world!".to_owned());
        let len = text.as_ref().map(String::len);
        assert_eq!(len, Some(13));
        assert_eq!(text.as_deref(), Some("Hello, world!"));
        assert_eq!(None::<String>.as_ref().map(String::len), None);
    }

    #[test]
    fn test_as_mut_modifies_in_place() {
        let mut x = Some(2);
        if let Some(v) = x.as_mut() {
            *v += 40;
        }
        assert_eq!(x, Some(42));
        let mut none: Option<i32> = None;
        assert!(none.as_mut().is_none());
    }
}
