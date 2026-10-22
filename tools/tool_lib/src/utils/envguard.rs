// TODO Reviewed ??-??-2026
/// Helper struct to safely set/unset environment variables during e.g. tests.
pub struct EnvGuard {
    key: String,
    original_value: Option<String>,
}

impl EnvGuard {
    /// Sets an environment variable.
    #[must_use]
    #[expect(
        unsafe_code,
        reason = "std::env::set_var is unsafe in edition 2024; callers serialize access in tests"
    )]
    pub fn set(key: &str, value: &str) -> Self {
        let key = key.to_owned();
        let original_value = std::env::var(&key).ok();
        // SAFETY: only used by tests, which serialize access to the environment.
        unsafe {
            std::env::set_var(&key, value);
        }
        Self {
            key,
            original_value,
        }
    }
}

/// `EnvGuard`'s `Drop` implementation resets the environment variable to its original value,
/// or remove it if it do not have one.
impl Drop for EnvGuard {
    #[expect(
        unsafe_code,
        reason = "std::env::set_var is unsafe in edition 2024; callers serialize access in tests"
    )]
    fn drop(&mut self) {
        if let Some(ref val) = self.original_value {
            // SAFETY: only used by tests, which serialize access to the environment.
            unsafe {
                std::env::set_var(&self.key, val);
            }
        } else {
            // SAFETY: only used by tests, which serialize access to the environment.
            unsafe {
                std::env::remove_var(&self.key);
            }
        }
    }
}
