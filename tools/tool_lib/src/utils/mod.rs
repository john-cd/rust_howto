mod envguard;
pub use envguard::EnvGuard;

// By default, a macro has no path-based scope.
// However, if it has the #[macro_export] attribute, then it is declared in the crate root scope.
#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}
