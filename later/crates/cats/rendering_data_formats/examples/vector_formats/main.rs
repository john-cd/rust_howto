//! Runnable entry point for the vector formats example.
/// Provides rasterize svg behavior for the vector formats component.
mod rasterize_svg;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    rasterize_svg::run()
}
