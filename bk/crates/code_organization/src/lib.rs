//! Library used by the project-layout examples.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: public_modules
/// Provides module1 behavior for the src component.
pub mod module1;
/// Provides module2 behavior for the src component.
pub mod module2;
// ANCHOR_END: public_modules

// ANCHOR: inline_module
mod inline_layout {
    /// Provides submodule1 behavior for the src component.
    pub mod submodule1;
    /// Provides submodule2 behavior for the src component.
    pub mod submodule2;
}
// ANCHOR_END: inline_module

pub use inline_layout::submodule1::default_name;
pub use inline_layout::submodule2::format_welcome;
