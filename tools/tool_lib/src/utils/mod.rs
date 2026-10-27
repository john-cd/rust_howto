//! Utilities: a guard for environment variables and a macro for lazily compiled
//! regular expressions.
//!
//! TODO Reviewed ??-??-2026.

mod envguard;

pub use envguard::EnvGuard;

// By default, a macro has no path-based scope.
// However, if it has the #[macro_export] attribute, then it is declared in the crate root scope.
/// Compiles a regular expression literal once and caches it in a static.
///
/// Returns a reference to the compiled regular expression.
///
/// The pattern must be valid. Invalid patterns panic on first use because the
/// macro returns a shared `Regex` instead of a fallible result.
#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        /// Lazily compiled regular expression shared by the utility functions.
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).expect("regex! requires a valid pattern"))
    }};
}
