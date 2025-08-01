#![allow(dead_code)]
// ANCHOR: example
use std::sync::Mutex;
use std::sync::OnceLock;

struct Settings {
    config_value: String,
}

impl Settings {
    fn new(value: String) -> Self {
        Settings {
            config_value: value,
        }
    }

    fn get_config(&self) -> &str {
        &self.config_value
    }

    fn set_config(&mut self, new_value: String) {
        self.config_value = new_value;
    }
}

// OnceCell is a thread-safe cell that can be written to only once.
// We wrap our Settings instance in a Mutex to ensure thread-safe access and
// modification. Since multiple threads might try to access the singleton, we
// need to protect it from race conditions.
static SETTINGS: OnceLock<Mutex<Settings>> = OnceLock::new();
// You could also use `std::sync::LazyLock`.

// Single point of access to our Settings instance.
fn get_settings() -> &'static Mutex<Settings> {
    // If SETTINGS hasn't been initialized yet, execute the closure.
    SETTINGS
        .get_or_init(|| Mutex::new(Settings::new("default_config".to_string())))
}

fn main() {
    // `OnceLock` has not been written to yet.
    assert!(SETTINGS.get().is_none());

    // Acquire a lock.
    let settings_guard_1 = get_settings().lock().unwrap();
    println!("Config 1: {}", settings_guard_1.get_config());
    drop(settings_guard_1); // Release the lock.

    {
        let mut settings_guard_2 = get_settings().lock().unwrap();
        // The MutexGuard provides mutable access.
        settings_guard_2.set_config("new_config".to_string());
        println!("Config 2 updated!");
    } // Lock released here.

    let settings_guard_3 = get_settings().lock().unwrap();
    println!("Config 3: {}", settings_guard_3.get_config());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
