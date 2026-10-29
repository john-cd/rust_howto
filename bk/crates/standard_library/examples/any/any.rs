// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Dynamic typing with the `Any` trait.

use std::any::Any;

// - `is::<T>(&self) -> bool` checks if the underlying concrete type of the
//   trait object is `T`.
// - `downcast_ref::<T>(&self) -> Option<&T>` attempts to downcast the trait
//   object to a reference `&T`.
// It returns `Some(&T)` if successful, `None` otherwise.
// - `downcast_mut::<T>(&mut self) -> Option<&mut T>` is similar to
//   `downcast_ref`, but provides a mutable reference.
//
// All are implemented on `dyn Any`, `dyn Any + Send`, `dyn Any + Send + Sync`.
/// Returns whether `s` is a `String`.
fn is_string(s: &dyn Any) -> bool {
    // `is<T>` is equivalent to `TypeId::of::<T>() == s.type_id()`.
    let res = s.is::<String>();
    if res {
        print!("String: ");
    } else {
        println!("Not a string...");
    }
    // Get access to the `&String` if `s` is of the right type:
    if let Some(string) = s.downcast_ref::<String>() {
        println!("{string}");
    }
    res
}

// `downcast::<T>(self: Box<Self>) -> Result<Box<T>, Box<Self>>` consumes a
// `Box<dyn Any>` and attempts to downcast it to a `Box<T>`. `downcast` is
// implemented on `Box<dyn Any>`, `Box<dyn Any + Send>`, `Box<dyn Any + Send +
// Sync>`.
/// Prints `value` if it is a `String`.
fn print_if_string(value: Box<dyn Any>) {
    if let Ok(string) = value.downcast::<String>() {
        println!("String: {string}");
    }
}

/// Checks the type of values at runtime.
fn main() {
    assert!(!is_string(&42));
    let s = "a string".to_owned();
    assert!(is_string(&s));

    print_if_string(Box::new(s));
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_example_entry_point() {
        main();
    }

    #[test]
    fn test_is_string() {
        assert!(is_string(&String::from("text")));
        assert!(!is_string(&42));
        // A `&str` is not a `String`.
        assert!(!is_string(&"text"));
    }

    #[test]
    fn test_downcast_ref_and_mut() -> Result<(), Box<dyn std::error::Error>> {
        let mut value: Box<dyn Any> = Box::new(5_i32);
        assert_eq!(value.downcast_ref::<i32>(), Some(&5));
        assert_eq!(value.downcast_ref::<u32>(), None);
        *value.downcast_mut::<i32>().ok_or("not an i32")? += 1;
        assert_eq!(value.downcast_ref::<i32>(), Some(&6));
        Ok(())
    }

    #[test]
    fn test_downcast_returns_the_box_on_failure()
    -> Result<(), Box<dyn std::error::Error>> {
        let boxed: Box<dyn Any> = Box::new("text".to_owned());
        let back = boxed.downcast::<i32>().err().ok_or("unexpected i32")?;
        let text = back.downcast::<String>().map_err(|_| "not a String")?;
        assert_eq!(*text, "text");
        print_if_string(Box::new(1_u8)); // Not a `String`: prints nothing.
        print_if_string(Box::new("s".to_owned()));
        Ok(())
    }
}
