//! Runnable entry point for the loco example.
/// Provides loco routes behavior for the loco component.
mod loco_routes;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    loco_routes::run()
}
