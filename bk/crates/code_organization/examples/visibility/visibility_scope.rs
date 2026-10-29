// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Restricting visibility to a scope with `pub(in path)`, `pub(super)` and
//! `pub(crate)`.

/// The module in which `pub(in ...)` items are visible.
mod a {
    /// A module that is public inside `a`.
    pub mod b {
        /// A function visible in the whole crate.
        pub(crate) fn visible_in_crate() -> &'static str {
            "visible_in_crate"
        }

        /// A function visible in the parent module only.
        pub(super) fn visible_in_parent_module() {}

        /// A public nested module.
        pub mod c {
            /// A function visible in `a` only.
            pub(in super::super) fn visible_in_a() {}
        }
    }

    /// Calls the functions visible in `a`.
    #[expect(dead_code, reason = "declared to illustrate it; never used")]
    fn use_in_a() {
        b::visible_in_crate();
        b::visible_in_parent_module();
        b::c::visible_in_a();
    }
}

/// A module that is not related to `a`.
mod d {
    /// Calls the functions visible in `d`.
    #[expect(dead_code, reason = "declared to illustrate it; never used")]
    fn use_in_d() {
        super::a::b::visible_in_crate();
        // ERROR: super::a::b::visible_in_parent_module();
        // ERROR: super::a::b::c::visible_in_a();
    }
}

/// Calls the functions that are visible from here.
fn main() {
    println!("{}", a::b::visible_in_crate());
    // ERROR: a::b::visible_in_parent_module();
    // ERROR: a::b::c::visible_in_a();
    println!(
        "visibility_scope example: pub(super) and pub(in path) restrict visibility to specific scopes"
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
    fn test_pub_crate_visible_from_other_modules() {
        assert_eq!(a::b::visible_in_crate(), "visible_in_crate");
    }
}
