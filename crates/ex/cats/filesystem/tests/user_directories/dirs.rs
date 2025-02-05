// ANCHOR: example

use dirs;

// The `dirs` crate provides a convenient way to get paths to
// standard directories on the file system, like the user's home directory,
// the configuration directory, and more.

fn main() {
    // Get the home directory
    if let Some(home_dir) = dirs::home_dir() {
        println!("Home directory: {:?}", home_dir);
    } else {
        println!("Home directory could not be found.");
    }

    // Get the configuration directory
    if let Some(config_dir) = dirs::config_dir() {
        println!("Configuration directory: {:?}", config_dir);
    } else {
        println!("Configuration directory could not be found.");
    }

    // Get the data directory
    if let Some(data_dir) = dirs::data_dir() {
        println!("Data directory: {:?}", data_dir);
    } else {
        println!("Data directory could not be found.");
    }

    // Get the cache directory
    if let Some(cache_dir) = dirs::cache_dir() {
        println!("Cache directory: {:?}", cache_dir);
    } else {
        println!("Cache directory could not be found.");
    }

    // Get the executable directory
    if let Some(exec_dir) = dirs::executable_dir() {
        println!("Executable directory: {:?}", exec_dir);
    } else {
        println!("Executable directory could not be found.");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
