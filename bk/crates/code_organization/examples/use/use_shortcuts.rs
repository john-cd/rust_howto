// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Shortcuts for `use`: globs, grouped imports, `self` and aliases.

// Use a glob (the `*`) to bring all public objects within a module
// (here `default`) in scope. Use sparingly.
use std::default::*;
// The most common use of globs is to import a "prelude", a group of items
// commonly used together. See the "code organization" chapter.
#[expect(
    unused_imports,
    reason = "Illustrates importing traits via standard prelude."
)]
use std::io::prelude::*;
// The `std::io::prelude::*` import above includes the common `Read` and
// `Write` traits, so they do not need a second explicit import here.
// Idiom: use `self` to refer to the module itself.
// The following is equivalent to `use std::iter; use std::iter::empty;`.
use std::iter::{self, empty};
// Use `as` to define aliases, for example in case of name conflict.
use std::mem::drop as destruct;

/// Uses items imported with the shortcuts.
fn main() {
    // Use `std::default::Default` without writing down the whole path,
    // because all public objects from the `default` module have been imported.
    let v: Vec<u8> = Default::default();

    // Use the alias defined above:
    destruct(v);

    // Use items from grouped imports:
    let _ = iter::empty::<u8>();
    let _ = empty::<u8>();

    println!(
        "use_shortcuts example: glob imports, combined imports, and aliases"
    );
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

    #[test]
    fn test_glob_import_of_default() {
        let v: Vec<u8> = Default::default();
        assert!(v.is_empty());
        destruct(v);
    }

    #[test]
    fn test_self_import_and_group() {
        assert_eq!(iter::empty::<u8>().count(), 0);
        assert_eq!(empty::<u8>().count(), 0);
    }
}
