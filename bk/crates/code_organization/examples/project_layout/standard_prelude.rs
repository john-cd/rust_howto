//! Models the standard prelude's Option reexports.
//!
//! TODO Reviewed ??-??-2026.

/// Reexports Option and its constructors for a single convenient import path.
mod prelude {
    // ANCHOR: option_exports
    pub use std::option::Option::None;
    pub use std::option::Option::Some;
    pub use std::option::Option::{self};
    // ANCHOR_END: option_exports
}

/// A prelude can import both a type and its variants.
#[test]
fn imports_the_type_and_its_variants() {
    use prelude::None;
    use prelude::Option;
    use prelude::Some;

    let values: [Option<u8>; 2] = [Some(1), None];
    assert_eq!(
        values,
        [std::option::Option::Some(1), std::option::Option::None]
    );
}
