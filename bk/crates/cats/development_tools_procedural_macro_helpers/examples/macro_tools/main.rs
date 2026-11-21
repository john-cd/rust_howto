// TODO Reviewed ??-??-2026.
//! Runnable entry point for the macro tools example.
/// Provides derive expansion behavior for the macro tools component.
mod derive_expansion;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    derive_expansion::run()
}
