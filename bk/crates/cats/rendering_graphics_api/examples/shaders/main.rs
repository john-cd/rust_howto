//! Runnable entry point for the shaders example.
/// Provides wgsl to spirv behavior for the shaders component.
mod wgsl_to_spirv;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    wgsl_to_spirv::run()
}
