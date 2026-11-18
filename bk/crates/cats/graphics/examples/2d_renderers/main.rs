//! Runnable entry point for the 2d renderers example.
/// Provides tessellate and rasterize behavior for the 2d renderers component.
mod tessellate_and_rasterize;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    tessellate_and_rasterize::run()
}
