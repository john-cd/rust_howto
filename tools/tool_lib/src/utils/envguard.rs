//! Provides a test-only environment-variable guard for process-wide changes.
//!
//! Mutating the process environment is unsound while any other thread reads or
//! writes it (on Unix, `getenv` is not synchronized with `setenv`). A mutex
//! shared among the tests does not cover threads started by libraries (for
//! example the HTTP server or client threads), so the constructors are `unsafe`
//! and the guard is only compiled for this crate's tests. Prefer injecting typed
//! configuration or running a subprocess fixture with an explicit environment.
//!
//! TODO Reviewed ??-??-2026.

use std::ffi::OsString;

/// Restores an environment variable to its original state when dropped.
///
/// The guard changes process-wide state for its whole lifetime, so it must be
/// dropped on the thread and in the test that created it. Restoration is
/// lossless: non-Unicode original values are preserved as `OsString`.
pub(crate) struct EnvGuard {
    /// Name of the environment variable.
    key: String,
    /// Value of the variable when the guard was created, if it was set.
    original_value: Option<OsString>,
}

impl EnvGuard {
    /// Sets an environment variable until this guard is dropped.
    ///
    /// Dropping the guard restores the previous value, byte for byte, or removes the
    /// variable if it was absent.
    ///
    /// # Safety
    ///
    /// No other thread may read or write the process environment (directly, or
    /// through libraries such as resolvers and HTTP clients) from this call until
    /// the guard is dropped. Holding a mutex shared only among tests is not sufficient.
    #[must_use]
    #[expect(
        unsafe_code,
        reason = "std::env::set_var is unsafe in edition 2024; the caller upholds the contract"
    )]
    pub(crate) unsafe fn set(key: &str, value: &str) -> Self {
        let key = key.to_owned();
        let original_value = std::env::var_os(&key);
        // SAFETY: the caller guarantees exclusive access to the process environment.
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
    /// Dropping the guard restores the previous value, byte for byte, if the variable was present.
    ///
    /// # Safety
    ///
    /// Same contract as [`EnvGuard::set`].
    #[must_use]
    #[expect(
        unsafe_code,
        reason = "std::env::remove_var is unsafe in edition 2024; the caller upholds the contract"
    )]
    pub(crate) unsafe fn remove(key: &str) -> Self {
        let key = key.to_owned();
        let original_value = std::env::var_os(&key);
        // SAFETY: the caller guarantees exclusive access to the process environment.
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
        reason = "std::env::set_var is unsafe in edition 2024; the constructor's contract covers the guard lifetime"
    )]
    fn drop(&mut self) {
        if let Some(ref val) = self.original_value {
            // SAFETY: the constructor's caller guaranteed exclusive access until drop.
            unsafe {
                std::env::set_var(&self.key, val);
            }
        } else {
            // SAFETY: the constructor's caller guaranteed exclusive access until drop.
            unsafe {
                std::env::remove_var(&self.key);
            }
        }
    }
}

#[cfg(test)]
#[expect(
    unsafe_code,
    reason = "tests call the unsafe guard constructors under ENV_MUTEX"
)]
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

        // SAFETY: serialized by `ENV_MUTEX`; no other thread uses the environment.
        let _restore_guard = unsafe { EnvGuard::remove(key) };
        assert_eq!(std::env::var_os(key), None);

        {
            // SAFETY: serialized by `ENV_MUTEX`; no other thread uses the environment.
            let _guard = unsafe { EnvGuard::set(key, "temporary_value") };
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

        // SAFETY: serialized by `ENV_MUTEX`; no other thread uses the environment.
        let _original_guard = unsafe { EnvGuard::set(key, "original_value") };
        assert_eq!(
            std::env::var(key).expect("the outer guard should set the test variable"),
            "original_value"
        );

        {
            // SAFETY: serialized by `ENV_MUTEX`; no other thread uses the environment.
            let _guard = unsafe { EnvGuard::set(key, "new_value") };
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

        // SAFETY: serialized by `ENV_MUTEX`; no other thread uses the environment.
        let _guard0 = unsafe { EnvGuard::set(key, "level_0") };

        {
            // SAFETY: serialized by `ENV_MUTEX`; no other thread uses the environment.
            let _guard1 = unsafe { EnvGuard::set(key, "level_1") };
            assert_eq!(
                std::env::var(key).expect("the first nested guard should set the test variable"),
                "level_1"
            );

            {
                // SAFETY: serialized by `ENV_MUTEX`; no other thread uses the environment.
                let _guard2 = unsafe { EnvGuard::set(key, "level_2") };
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

    /// Verifies that a non-Unicode original value is restored losslessly.
    #[cfg(any(unix, windows))]
    #[test]
    fn test_env_guard_restores_non_unicode_value() {
        let _lock = ENV_MUTEX
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let key = "TEST_ENV_GUARD_NON_UNICODE_VAR";

        #[cfg(unix)]
        let original: OsString = {
            use std::os::unix::ffi::OsStringExt as _;
            OsString::from_vec(vec![b'a', 0xFF, b'b'])
        };
        #[cfg(windows)]
        let original: OsString = {
            use std::os::windows::ffi::OsStringExt as _;
            // An unpaired surrogate is not valid Unicode.
            OsString::from_wide(&[u16::from(b'a'), 0xD800, u16::from(b'b')])
        };
        assert!(original.to_str().is_none());

        // SAFETY: serialized by `ENV_MUTEX`; no other thread touches the environment.
        unsafe {
            std::env::set_var(key, &original);
        }
        {
            // SAFETY: serialized by `ENV_MUTEX`; no other thread uses the environment.
            let _guard = unsafe { EnvGuard::set(key, "temporary_value") };
            assert_eq!(
                std::env::var_os(key),
                Some(OsString::from("temporary_value"))
            );
        }
        assert_eq!(std::env::var_os(key), Some(original));
        // SAFETY: as above.
        unsafe {
            std::env::remove_var(key);
        }
    }
}
