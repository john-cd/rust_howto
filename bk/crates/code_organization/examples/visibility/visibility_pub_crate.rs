// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Restricting visibility to the current crate with `pub(crate)`.

// In e.g. `lib.rs`:
/// A module with a `pub(crate)` function.
mod a_module {
    // Conservative use of `pub(crate)`:
    // Even if `a_module` is accidentally made fully public,
    // this function will remain visible only to this crate.
    /// A function visible only in the current crate.
    pub(crate) fn visible_in_crate_only() -> &'static str {
        "a_module"
    }
}

// We can also explicitly mark the top module as `pub(crate)`:
/// A `pub(crate)` module.
pub(crate) mod b_module {
    /// A function visible only in the current crate, since its module is.
    pub fn visible_in_crate_only() -> &'static str {
        "b_module"
    }
}

/// Calls the functions that are visible in the crate.
fn main() {
    println!("{}", a_module::visible_in_crate_only());
    println!("{}", b_module::visible_in_crate_only());
    println!(
        "visibility_pub_crate example: pub(crate) restricts visibility to the current crate"
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
    fn test_pub_crate_items_visible_in_crate() {
        assert_eq!(a_module::visible_in_crate_only(), "a_module");
        assert_eq!(b_module::visible_in_crate_only(), "b_module");
    }
}
