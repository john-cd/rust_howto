#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Running code when a value is dropped, with the `Drop` trait.

/// A struct that prints a message when it is dropped.
struct CustomStruct {
    /// Some data.
    data: String,
}

impl Drop for CustomStruct {
    // This is the only method required by the Drop trait. It takes a mutable
    // reference to self. The code inside this method will be executed when
    // an instance of CustomStruct is dropped.
    fn drop(&mut self) {
        println!("Dropping CustomStruct with data `{}`!", self.data);
    }
}

/// Creates and drops values.
fn main() {
    let _c = CustomStruct {
        data: String::from("my data"),
    };
    let _d = CustomStruct {
        data: String::from("another data"),
    };

    println!("CustomStructs created.");
    // `_c` and `_d` remain in scope until the end of `main`.

    let e = CustomStruct {
        data: String::from("explicit drop example"),
    };
    println!("CustomStruct 'e' created.");
    // We can explicitly drop a value using `std::mem::drop`, but usually,
    // Rust handles it automatically.
    drop(e);
    println!("CustomStruct 'e' dropped before end of main.");

    println!("End of main.");
    // After this statement, `_d` is dropped first, then `_c` (reverse
    // declaration order).
}

// ANCHOR_END: example

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
