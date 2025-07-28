#![allow(dead_code)]
// ANCHOR: example
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
fn is_string(s: &dyn Any) -> bool {
    // `is<T>` is equivalent to `TypeId::of::<T>() == s.type_id()`.
    let res = s.is::<String>();
    if res {
        print!("String: ");
    } else {
        println!("Not a string...");
    }
    if let Some(string) = s.downcast_ref::<String>() {
        println!("{string}");
    }
    res
}

// `downcast::<T>(self: Box<Self>) -> Result<Box<T>, Box<Self>>` consumes a
// `Box<dyn Any>` and attempts to downcast it to a `Box<T>`. `downcast` is
// implemented on `Box<dyn Any>`, `Box<dyn Any + Send>`, `Box<dyn Any + Send +
// Sync>`.
fn print_if_string(value: Box<dyn Any>) {
    if let Ok(string) = value.downcast::<String>() {
        println!("String: {string}");
    }
}

// Get type name for diagnostics purposes.
// Note: the returned name may vary with compiler versions.
fn get_type_name<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

fn main() {
    assert!(!is_string(&42));
    let s = "a string".to_string();
    assert!(is_string(&s));

    println!("{}", get_type_name(&s));

    print_if_string(Box::new(s));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
