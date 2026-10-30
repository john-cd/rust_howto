//! Loads the Vulkan library with `ash` and reports the instance version.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use ash::Entry;
use ash::vk;

/// Returns the instance-level Vulkan version, or `None` when the machine has
/// no Vulkan loader (for example in a container or on a headless CI runner).
fn instance_version() -> anyhow::Result<Option<(u32, u32, u32)>> {
    // SAFETY: loading a shared library runs its initialization code; the
    // Vulkan loader is trusted system software.
    let Ok(entry) = (unsafe { Entry::load() }) else {
        return Ok(None);
    };
    // Vulkan 1.0 loaders do not implement the version query.
    let version = match unsafe { entry.try_enumerate_instance_version() }? {
        Some(version) => version,
        None => vk::API_VERSION_1_0,
    };
    Ok(Some((
        vk::api_version_major(version),
        vk::api_version_minor(version),
        vk::api_version_patch(version),
    )))
}

/// Prints the Vulkan version, or explains that no loader is available.
fn main() -> anyhow::Result<()> {
    match instance_version()? {
        Some((major, minor, patch)) => {
            println!("Vulkan instance version {major}.{minor}.{patch}");
        }
        None => println!("no Vulkan loader found; skipping the device query"),
    }
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point, with or without a loader.
    #[test]
    fn test_main_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Verifies the version encoding helpers used by the example.
    #[test]
    fn decodes_packed_api_versions() {
        let version = vk::make_api_version(0, 1, 3, 250);
        assert_eq!(
            (
                vk::api_version_major(version),
                vk::api_version_minor(version),
                vk::api_version_patch(version)
            ),
            (1, 3, 250)
        );
    }

    /// Verifies that, when Vulkan is present, it is at least version 1.0.
    #[test]
    fn reports_a_plausible_version_when_available() -> anyhow::Result<()> {
        if let Some((major, _, _)) = instance_version()? {
            assert!(major >= 1);
        }
        Ok(())
    }
}
