//! Library used by the project-layout examples.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: public_modules
pub mod module1;
pub mod module2;
// ANCHOR_END: public_modules

// ANCHOR: inline_module
mod inline_layout {
    pub mod submodule1;
    pub mod submodule2;
}
// ANCHOR_END: inline_module

pub use inline_layout::submodule1::default_name;
pub use inline_layout::submodule2::format_welcome;
