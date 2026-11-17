// TODO Reviewed ??-??-2026.

//! Runnable entry point for the project layout example.
/// Provides reexports and prelude behavior for the project layout component.
mod reexports_and_prelude;

/// Runs the example demonstrated by this module.
fn main() {
    reexports_and_prelude::run();
}

/// Provides standard prelude behavior for the project layout component.
#[cfg(test)]
mod standard_prelude;
