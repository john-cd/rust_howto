#![allow(dead_code)]
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

fn main() {
    println!("{:?}", get_data_dir());
    println!("{:?}", get_config_dir());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
