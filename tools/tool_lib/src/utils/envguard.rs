/// Helper struct to safely set/unset environment variables during e.g. tests.
pub struct EnvGuard {
    key: String,
    original_value: Option<String>,
}

impl EnvGuard {
    // Set an environment variable.
    pub fn set(key: &str, value: &str) -> Self {
        let key = key.to_string();
        let original_value = std::env::var(&key).ok();
        unsafe {
            std::env::set_var(&key, value);
        }
        EnvGuard {
            key,
            original_value,
        }
    }
}

/// `EnvGuard`'s `Drop` implementation resets the environment variable to its original value,
/// or remove it if it do not have one.
impl Drop for EnvGuard {
    fn drop(&mut self) {
        if let Some(ref val) = self.original_value {
            unsafe {
                std::env::set_var(&self.key, val);
            }
        } else {
            unsafe {
                std::env::remove_var(&self.key);
            }
        }
    }
}
