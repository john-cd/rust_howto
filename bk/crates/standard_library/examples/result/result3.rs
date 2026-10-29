// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates the use of `as_ref`, `as_deref`, and `as_mut` methods on
//! `Result`.

/// Returns a message on success, or an error.
fn get_message(success: bool) -> Result<String, &'static str> {
    if success {
        Ok(String::from("Rust"))
    } else {
        Err("Failed to get message")
    }
}

/// Converts a `Result` with `as_ref`, `as_deref` and `as_mut`.
fn main() {
    let result: Result<String, &'static str> = get_message(true);

    // `as_ref` converts from `&Result<T, E>` to Result<&T, &E>`.
    let ref_result: Result<&String, &&str> = result.as_ref();
    println!("as_ref(): {ref_result:?}");

    // Use `as_deref` to coerce the `Ok` variant of the original `Result` via
    // `Deref`. Here, `&String` is coerced to `&str`.
    let deref_result: Result<&str, &&str> = result.as_deref();
    println!("as_deref(): {deref_result:?}");

    // Use `as_mut` to get a mutable reference:
    let mut result_mut = get_message(true);
    let mut_ref: Result<&mut String, &mut &str> = result_mut.as_mut();
    if let Ok(s) = mut_ref {
        s.push_str(" is awesome!");
    }
    println!("as_mut(): {result_mut:?}");

    // There is also a `as_deref_mut` method.
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result3_example_entry_point() {
        main();
    }

    #[test]
    fn test_get_message() {
        assert_eq!(get_message(true), Ok("Rust".to_owned()));
        assert_eq!(get_message(false), Err("Failed to get message"));
    }

    #[test]
    fn test_as_ref_and_as_deref() {
        let result = get_message(true);
        assert_eq!(result.as_ref().map(String::len), Ok(4));
        assert_eq!(result.as_deref(), Ok("Rust"));
        // The original is still usable.
        assert_eq!(result, Ok("Rust".to_owned()));
    }

    #[test]
    fn test_as_mut_modifies_in_place() {
        let mut result = get_message(true);
        if let Ok(s) = result.as_mut() {
            s.push_str(" is awesome!");
        }
        assert_eq!(result, Ok("Rust is awesome!".to_owned()));
    }

    #[test]
    fn test_as_mut_on_err_does_nothing() {
        let mut result = get_message(false);
        assert!(result.as_mut().is_err());
        assert_eq!(result, Err("Failed to get message"));
    }
}
