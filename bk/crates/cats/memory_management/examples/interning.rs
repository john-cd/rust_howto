#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! String interning: store each distinct string once and compare, hash and
//! copy cheap handles instead of strings.
//!
//! Shown twice: a hand-written interner (`HashMap<String, u32>` plus a
//! `Vec<String>`), and the `ustr` crate, whose `Ustr` is a pointer to a global,
//! never-freed string pool (cheap equality and hashing, but the memory is not
//! reclaimed).

use std::collections::HashMap;

use ustr::Ustr;

/// A handle to an interned string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Symbol(u32);

/// Owns the interned strings.
#[derive(Default)]
struct Interner {
    ids: HashMap<String, u32>,
    strings: Vec<String>,
}

impl Interner {
    /// Returns the symbol for `s`, adding it on first use.
    fn intern(&mut self, s: &str) -> Symbol {
        if let Some(&id) = self.ids.get(s) {
            return Symbol(id);
        }
        let id = self.strings.len() as u32;
        self.strings.push(s.to_owned());
        self.ids.insert(s.to_owned(), id);
        Symbol(id)
    }

    /// Returns the string behind a symbol.
    fn resolve(&self, symbol: Symbol) -> Option<&str> {
        self.strings.get(symbol.0 as usize).map(String::as_str)
    }
}

/// Interns identifiers and compares them by handle.
fn main() {
    let mut interner = Interner::default();
    let a = interner.intern("foo");
    let b = interner.intern("bar");
    let c = interner.intern("foo");
    assert_eq!(a, c); // An integer comparison.
    assert_ne!(a, b);
    assert_eq!(interner.resolve(a), Some("foo"));
    assert_eq!(interner.strings.len(), 2); // Stored once.

    // With `ustr`: equal strings give the same pointer.
    let x = Ustr::from("foo");
    let owned = format!("{}{}", "fo", "o");
    let y = Ustr::from(&owned);
    assert_eq!(x, y);
    assert_eq!(x.as_char_ptr(), y.as_char_ptr());
    assert_eq!(x.as_str(), "foo");
}

// ANCHOR_END: example

/// Runs the example.
#[allow(clippy::main_recursion)]
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
