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

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_env_guard_unset_variable() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let key = "TEST_ENV_GUARD_UNSET_VAR";

        // Ensure key is initially unset
        unsafe {
            std::env::remove_var(key);
        }
        assert!(std::env::var(key).is_err());

        {
            let _guard = EnvGuard::set(key, "temporary_value");
            assert_eq!(std::env::var(key).unwrap(), "temporary_value");
        }

        // After guard drops, the env var should be removed
        assert!(std::env::var(key).is_err());
    }

    #[test]
    fn test_env_guard_existing_variable() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let key = "TEST_ENV_GUARD_EXISTING_VAR";

        // Set initial value
        unsafe {
            std::env::set_var(key, "original_value");
        }
        assert_eq!(std::env::var(key).unwrap(), "original_value");

        {
            let _guard = EnvGuard::set(key, "new_value");
            assert_eq!(std::env::var(key).unwrap(), "new_value");
        }

        // After guard drops, original value should be restored
        assert_eq!(std::env::var(key).unwrap(), "original_value");

        // Clean up
        unsafe {
            std::env::remove_var(key);
        }
    }

    #[test]
    fn test_env_guard_nested_overrides() {
        let _lock = ENV_MUTEX.lock().unwrap();
        let key = "TEST_ENV_GUARD_NESTED_VAR";

        unsafe {
            std::env::set_var(key, "level_0");
        }

        {
            let _guard1 = EnvGuard::set(key, "level_1");
            assert_eq!(std::env::var(key).unwrap(), "level_1");

            {
                let _guard2 = EnvGuard::set(key, "level_2");
                assert_eq!(std::env::var(key).unwrap(), "level_2");
            }

            assert_eq!(std::env::var(key).unwrap(), "level_1");
        }

        assert_eq!(std::env::var(key).unwrap(), "level_0");

        unsafe {
            std::env::remove_var(key);
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
