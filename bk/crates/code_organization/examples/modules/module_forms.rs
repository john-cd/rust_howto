//! Exercises inline modules, external files, and qualified paths.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: declarations
// The contents come from module_forms/a_file.rs.
mod a_file;

// The contents are written inline.
mod inline_module {
    /// Returns a value through an inline module.
    pub(super) fn some_function() -> u8 {
        2
    }
}
// ANCHOR_END: declarations

// ANCHOR: nested
mod a {
    // The contents come from module_forms/a/b.rs.
    pub(super) mod b;

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

enum Direction {
    North,
}

impl Direction {
    const DEFAULT: Self = Self::North;

    /// Constructs the north direction.
    fn north() -> Self {
        Self::North
    }
}

trait Measurement {
    type Unit;
}

impl Measurement for Direction {
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
