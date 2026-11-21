// TODO Reviewed ??-??-2026.
//! Runnable entry point for the vulkan example.
/// Provides vulkan instance version behavior for the vulkan component.
mod vulkan_instance_version;

/// Runs the example demonstrated by this module.
fn main() -> impl std::process::Termination {
    vulkan_instance_version::run()
}
