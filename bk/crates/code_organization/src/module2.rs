//! Uses public items from a sibling module's children.

// ANCHOR: sibling_imports
use super::module1::submodule1;
use super::module1::submodule2::Greeting;

/// Formats the name in a greeting.
pub fn message(greeting: Greeting<'_>) -> String {
    submodule1::public_function(greeting.0)
}
// ANCHOR_END: sibling_imports
