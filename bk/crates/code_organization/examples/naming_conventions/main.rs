// TODO Reviewed ??-??-2026.

//! Naming conventions for getters and iterators.

// ANCHOR: getters
/// A wrapper type with a getter and a mutating getter.
pub struct S {
    first: String,
}

impl S {
    /// Constructs a new instance of `S` from any type convertible into a `String`.
    ///
    /// Following the Rust API Guidelines (C-CTOR), the primary constructor is named `new`.
    ///
    /// # Parameters
    /// - `first`: The initial string value, converted into an owned `String`.
    pub fn new(first: impl Into<String>) -> Self {
        Self {
            first: first.into(),
        }
    }

    /// Returns an immutable reference to the wrapped `first` string slice.
    ///
    /// Following Rust API Guidelines (C-GETTER), getter methods omit the `get_` prefix
    /// when returning a borrowed reference to an internal field.
    /// Multiple reader threads can access this method concurrently without synchronization.
    pub fn first(&self) -> &str {
        &self.first
    }

    /// Returns a mutable reference to the wrapped `first` string.
    ///
    /// Following Rust API Guidelines (C-GETTER), mutating getters use the `_mut` suffix
    /// rather than `get_mut_first` or `mut_first`.
    /// Requires exclusive mutable access (`&mut self`), preventing concurrent data races.
    pub fn first_mut(&mut self) -> &mut String {
        &mut self.first
    }
}
// ANCHOR_END: getters

// ANCHOR: iterators
/// A container of `u32`s.
pub struct Container {
    items: Vec<u32>,
}

impl Container {
    /// The `Iter` type implements `Iterator<Item = &u32>`.
    pub fn iter(&self) -> std::slice::Iter<'_, u32> {
        self.items.iter()
    }

    /// `IterMut` implements `Iterator<Item = &mut u32>`.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, u32> {
        self.items.iter_mut()
    }
}

impl<'a> IntoIterator for &'a Container {
    type IntoIter = std::slice::Iter<'a, u32>;
    type Item = &'a u32;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Container {
    type IntoIter = std::slice::IterMut<'a, u32>;
    type Item = &'a mut u32;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl IntoIterator for Container {
    type IntoIter = std::vec::IntoIter<u32>;
    type Item = u32;

    /// `IntoIter` implements `Iterator<Item = u32>`.
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
// ANCHOR_END: iterators

fn main() {
    let mut s = S::new("a");
    s.first_mut().push('b');
    println!("{}", s.first());

    let mut c = Container {
        items: vec![1, 2, 3],
    };
    for x in &mut c {
        *x += 1;
    }
    println!("{:?}", c.iter().collect::<Vec<_>>());
    println!("{:?}", c.into_iter().collect::<Vec<_>>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_getters() {
        let mut s = S::new("a");
        s.first_mut().push('b');
        assert_eq!(s.first(), "ab");
    }

    #[test]
    fn test_iterators() {
        let mut c = Container {
            items: vec![1, 2, 3],
        };
        c.iter_mut().for_each(|x| *x *= 2);
        assert_eq!(c.iter().sum::<u32>(), 12);
        assert_eq!(c.into_iter().collect::<Vec<_>>(), [2, 4, 6]);
    }
}
