// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use the `directories` crate to find the
//! user's data and config directories for a specific application.

use std::path::PathBuf;

use anyhow::Result;
use anyhow::anyhow;
use directories::ProjectDirs;

/// Returns the user's data directory.
pub fn get_data_dir() -> Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("APP_DATA_DIR") {
        PathBuf::from(s)
    } else if let Some(proj_dirs) =
        ProjectDirs::from("com", "Foo Corp", "Bar App")
    {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        return Err(anyhow!("Unable to find the data directory"));
    };
    Ok(directory)
}

/// Returns the user's config directory.
pub fn get_config_dir() -> Result<PathBuf> {
    let directory = if let Ok(s) = std::env::var("APP_CONFIG_DIR") {
        PathBuf::from(s)
    } else if let Some(proj_dirs) =
        ProjectDirs::from("com", "Foo Corp", "Bar App")
    {
        proj_dirs.config_local_dir().to_path_buf()
    } else {
        return Err(anyhow!("Unable to find the config directory"));
    };
    Ok(directory)
}
// Look into `BaseDirs` and `UserDirs` for additional directories as well.

/// Prints the data and configuration directories of the application.
fn main() {
    println!("{:?}", get_data_dir());
    println!("{:?}", get_config_dir());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the directories example through its entry point.
    #[test]
    fn test_directories_example_entry_point() {
        main();
    }

    /// The application directories are absolute and distinct when no
    /// override is set, and name the application.
    #[test]
    fn application_dirs_name_the_app() -> Result<()> {
        if std::env::var_os("APP_DATA_DIR").is_some()
            || std::env::var_os("APP_CONFIG_DIR").is_some()
        {
            return Ok(()); // The overrides decide the result.
        }
        let (data, config) = (get_data_dir()?, get_config_dir()?);
        for dir in [&data, &config] {
            assert!(dir.is_absolute(), "{dir:?}");
            assert!(dir.to_string_lossy().to_lowercase().contains("bar"));
        }
        Ok(())
    }
}
