//! Exercises inline modules, external files, and qualified paths.
//!
//! TODO Reviewed ??-??-2026.

#![expect(
    clippy::self_named_module_files,
    reason = "This example teaches the named-file module form with a sibling module directory"
)]

// ANCHOR: declarations
/// Loads a sibling module from module_forms/a_file.rs.
mod a_file;

/// Defines a module directly in the containing source file.
mod inline_module {
    /// Returns a value through an inline module.
    pub(super) fn some_function() -> u8 {
        2
    }
}
// ANCHOR_END: declarations

// ANCHOR: nested
/// Groups nested external-file and inline modules under a common parent.
mod a {
    /// Loads the nested file module and exposes it to the parent module.
    pub(super) mod b;

    /// Inline sibling of b demonstrating visibility restricted to the enclosing example.
    pub(super) mod c {
        /// Returns a value from a nested inline module.
        pub(in crate::module_forms) fn value() -> u8 {
            4
        }
    }
}
// ANCHOR_END: nested

// ANCHOR: relative
/// Calls the same item with two relative paths.
fn relative_paths() -> (u8, u8) {
    (
        inline_module::some_function(),
        self::inline_module::some_function(),
    )
}
// ANCHOR_END: relative

// ANCHOR: parent
/// Demonstrates using super to call a private function in the parent module.
mod child {
    /// Calls a function in the enclosing module.
    pub(super) fn in_child() -> u8 {
        super::in_parent_module()
    }
}

/// Returns the parent module's value.
fn in_parent_module() -> u8 {
    5
}
// ANCHOR_END: parent

// ANCHOR: absolute
/// Starts the path at the crate root.
fn absolute_path() -> u8 {
    crate::module_forms::inline_module::some_function()
}
// ANCHOR_END: absolute

/// Example enum used to resolve variant, constructor, constant, and type paths.
enum Direction {
    /// Represents the north case of direction.
    North,
}

impl Direction {
    /// Associated constant naming the example's default direction.
    const DEFAULT: Self = Self::North;

    /// Constructs the north direction.
    fn north() -> Self {
        Self::North
    }
}

/// Defines the operations required for measurement behavior.
trait Measurement {
    /// Type of the unit values produced or consumed by this trait.
    type Unit;
}

impl Measurement for Direction {
    /// Alias for `u8` used to represent unit.
    type Unit = u8;
}

/// Paths can name an enum variant, associated function, type, or constant.
#[test]
fn resolves_associated_items() {
    // ANCHOR: associated_items
    let variant = Direction::North;
    let constructed = Direction::north();
    let default = Direction::DEFAULT;
    let bearing: <Direction as Measurement>::Unit = 0;
    // ANCHOR_END: associated_items

    assert!(matches!(
        (variant, constructed, default),
        (Direction::North, Direction::North, Direction::North)
    ));
    assert_eq!(bearing, 0);
}

/// Resolves module names through imports and reaches both module layouts.
#[test]
fn resolves_module_paths() {
    // ANCHOR: imports
    use self::a::b;
    use self::a_file::file_value;
    use self::inline_module;
    use crate::module_forms::a::c;
    // ANCHOR_END: imports

    assert_eq!(
        (
            b::value(),
            file_value(),
            inline_module::some_function(),
            c::value()
        ),
        (3, 1, 2, 4)
    );
}

/// Explicit and implicit relative paths resolve to the same item.
#[test]
fn resolves_relative_and_absolute_paths() {
    assert_eq!(relative_paths(), (2, 2));
    assert_eq!(absolute_path(), 2);
    assert_eq!(child::in_child(), 5);
}
