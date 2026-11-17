// TODO Reviewed ??-??-2026.

//! Runnable entry point for the dependencies example.
/// Provides external dependency behavior for the dependencies component.
mod external_dependency;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    external_dependency::run()
}
