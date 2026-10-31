// ANCHOR: example
//! Naming conventions for types, functions, getters, and iterator methods.

/// A sample type illustrating getter and conversion method conventions.
#[derive(Debug, PartialEq, Eq)]
pub struct First {
    /// Inner numeric payload.
    pub value: u32,
}

/// A container struct demonstrating getter naming conventions (`foo` and
/// `foo_mut`).
pub struct S {
    first: First,
}

impl S {
    /// Creates a new instance of `S`.
    pub fn new(first: First) -> Self {
        Self { first }
    }

    /// Returns an immutable reference to `first` (without a `get_` prefix).
    pub fn first(&self) -> &First {
        &self.first
    }

    /// Returns a mutable reference to `first` (following the field name with
    /// `_mut`).
    pub fn first_mut(&mut self) -> &mut First {
        &mut self.first
    }

    /// Consumes the wrapper and returns the wrapped value.
    pub fn into_inner(self) -> First {
        self.first
    }
}

/// Demonstrates getter methods and conversion conventions.
fn main() {
    let mut s = S::new(First { value: 10 });
    println!("Initial value: {}", s.first().value);

    s.first_mut().value = 20;
    println!("Updated value: {}", s.first().value);

    let inner = s.into_inner();
    println!("Consumed wrapper into inner: {inner:?}");
}

// ANCHOR_END: example

/// Runs the naming conventions example by calling `main()`.
pub fn run() {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the naming conventions example runs through its entry point.
    #[test]
    fn test_naming_conventions_example_entry_point() {
        let entry = main;
        entry();
    }

    /// Verifies getters, mutable getters, and consumption conventions.
    #[test]
    fn test_getter_conventions() {
        let mut s = S::new(First { value: 42 });
        assert_eq!(s.first(), &First { value: 42 });

        s.first_mut().value = 99;
        assert_eq!(s.first(), &First { value: 99 });

        let inner = s.into_inner();
        assert_eq!(inner, First { value: 99 });
    }
}
