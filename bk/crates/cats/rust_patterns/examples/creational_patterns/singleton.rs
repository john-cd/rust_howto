// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! The singleton pattern with a global `OnceLock`.
//!
//! A singleton is a value that exists exactly once and is shared globally, such
//! as configuration. In Rust it is built with `OnceLock` or `LazyLock`. Prefer
//! passing values explicitly where you can, since globals make testing harder.

use std::sync::Mutex;
use std::sync::OnceLock;

/// Application settings, shared as a singleton.
struct Settings {
    /// The configuration value.
    config_value: String,
}

impl Settings {
    /// Creates settings with the given configuration value.
    fn new(value: String) -> Self {
        Self {
            config_value: value,
        }
    }

    /// Returns the configuration value.
    fn get_config(&self) -> &str {
        &self.config_value
    }

    /// Replaces the configuration value.
    fn set_config(&mut self, new_value: String) {
        self.config_value = new_value;
    }
}

// OnceCell is a thread-safe cell that can be written to only once.
// We wrap our Settings instance in a Mutex to ensure thread-safe access and
// modification. Since multiple threads might try to access the singleton, we
// need to protect it from race conditions.
/// The single, lazily initialized instance of the settings.
static SETTINGS: OnceLock<Mutex<Settings>> = OnceLock::new();
// You could also use `std::sync::LazyLock`.

// Single point of access to our Settings instance.
/// Returns the singleton, creating it on first use.
fn get_settings() -> &'static Mutex<Settings> {
    // If SETTINGS hasn't been initialized yet, execute the closure.
    SETTINGS
        .get_or_init(|| Mutex::new(Settings::new("default_config".to_owned())))
}

/// Reads and updates the settings through the singleton.
fn main() {
    // `OnceLock` has not been written to yet.
    assert!(SETTINGS.get().is_none());

    // Acquire a lock.
    let settings_guard_1 =
        get_settings().lock().expect("mutex should not be poisoned");
    println!("Config 1: {}", settings_guard_1.get_config());
    drop(settings_guard_1); // Release the lock.

    {
        let mut settings_guard_2 =
            get_settings().lock().expect("mutex should not be poisoned");
        // The `MutexGuard` provides mutable access.
        settings_guard_2.set_config("new_config".to_owned());
        println!("Config 2 updated!");
    } // Lock released here.

    let settings_guard_3 =
        get_settings().lock().expect("mutex should not be poisoned");
    println!("Config 3: {}", settings_guard_3.get_config());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the singleton example through its entry point.
    #[test]
    fn test_singleton_example_entry_point() {
        main();
    }
}
