// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Reports the native graphics API behind a headless `wgpu` adapter.

use std::error::Error;

use pollster::block_on;
use wgpu::Instance;
use wgpu::InstanceDescriptor;
use wgpu::RequestAdapterOptions;

/// Selects an adapter without creating a window or requesting a device.
///
/// # Errors
///
/// Returns an error if the installed drivers expose no compatible adapter.
fn main() -> Result<(), Box<dyn Error>> {
    let mut descriptor = InstanceDescriptor::new_without_display_handle();
    descriptor.backends = wgpu::Backends::PRIMARY;
    let instance = Instance::new(descriptor);
    let adapter = block_on(instance.request_adapter(&RequestAdapterOptions::default()))
        .map_err(|error| {
            std::io::Error::other(format!(
                "could not select a GPU adapter; check the installed graphics drivers: {error}"
            ))
        })?;

    let info = adapter.get_info();
    println!("Adapter: {}", info.name);
    println!("Native backend: {:?}", info.backend);
    println!("Device type: {:?}", info.device_type);
    Ok(())
}
// ANCHOR_END: example

/// Runs the adapter query from the example target.
///
/// # Errors
///
/// Returns an error when a compatible adapter cannot be selected.
pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Queries the graphics backend when a compatible GPU adapter is present.
    #[test]
    #[ignore = "needs a GPU adapter"]
    fn test_graphics_example_entry_point() -> Result<(), Box<dyn Error>> {
        main()
    }
}
