// TODO Reviewed ??-??-2026
/// Restores an environment variable to its original state when dropped.
///
/// Callers must serialize access to the process environment while a guard is active.
pub struct EnvGuard {
    /// Name of the environment variable.
    key: String,
    /// Value of the variable when the guard was created, if it was set.
    original_value: Option<String>,
}

impl EnvGuard {
    /// Sets an environment variable until this guard is dropped.
    ///
    /// Dropping the guard restores the previous value or removes the variable if it was absent.
    /// Callers must serialize access to the process environment while the guard is active.
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

    /// Removes an environment variable until this guard is dropped.
    ///
    /// Dropping the guard restores the previous value if the variable was present.
    /// Callers must serialize access to the process environment while the guard is active.
    #[must_use]
    #[expect(
        unsafe_code,
        reason = "std::env::remove_var is unsafe in edition 2024; callers serialize access in tests"
    )]
    pub fn remove(key: &str) -> Self {
        let key = key.to_owned();
        let original_value = std::env::var(&key).ok();
        // SAFETY: only used by tests, which serialize access to the environment.
        unsafe {
            std::env::remove_var(&key);
        }
        Self {
            key,
            original_value,
        }
    }
}

/// `EnvGuard`'s `Drop` implementation resets the environment variable to its original value,
/// or removes it if it did not have one.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ENV_MUTEX;

    /// Verifies that dropping a guard removes a variable that was initially unset.
    #[test]
    fn test_env_guard_unset_variable() {
        let _lock = ENV_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let key = "TEST_ENV_GUARD_UNSET_VAR";

        let _restore_guard = EnvGuard::remove(key);
        assert_eq!(std::env::var_os(key), None);

        {
            let _guard = EnvGuard::set(key, "temporary_value");
            assert_eq!(
                std::env::var(key).expect("the guard should set the test variable"),
                "temporary_value"
            );
        }

        assert_eq!(std::env::var_os(key), None);
    }

    /// Verifies that dropping a guard restores the variable's previous value.
    #[test]
    fn test_env_guard_existing_variable() {
        let _lock = ENV_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let key = "TEST_ENV_GUARD_EXISTING_VAR";

        let _original_guard = EnvGuard::set(key, "original_value");
        assert_eq!(
            std::env::var(key).expect("the outer guard should set the test variable"),
            "original_value"
        );

        {
            let _guard = EnvGuard::set(key, "new_value");
            assert_eq!(
                std::env::var(key).expect("the inner guard should set the test variable"),
                "new_value"
            );
        }

        assert_eq!(
            std::env::var(key).expect("the outer guard should restore the test variable"),
            "original_value"
        );
    }

    /// Verifies that nested guards restore each immediately preceding value.
    #[test]
    fn test_env_guard_nested_overrides() {
        let _lock = ENV_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let key = "TEST_ENV_GUARD_NESTED_VAR";

        let _guard0 = EnvGuard::set(key, "level_0");

        {
            let _guard1 = EnvGuard::set(key, "level_1");
            assert_eq!(
                std::env::var(key).expect("the first nested guard should set the test variable"),
                "level_1"
            );

            {
                let _guard2 = EnvGuard::set(key, "level_2");
                assert_eq!(
                    std::env::var(key)
                        .expect("the second nested guard should set the test variable"),
                    "level_2"
                );
            }

            assert_eq!(
                std::env::var(key)
                    .expect("dropping the second nested guard should restore the first value"),
                "level_1"
            );
        }

        assert_eq!(
            std::env::var(key).expect("dropping the first nested guard should restore level 0"),
            "level_0"
        );
    }
}
