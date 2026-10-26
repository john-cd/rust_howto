//! The template of a category badge.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;
use serde::Serialize;

/// Category badge
pub(super) static CAT_BADGE: &str =
    r"[![cat~{slug}][cat~{slug}~badge]][cat~{slug}]\{\{hi:{category}}} ";

/// Renders a category badge for the supplied name and crates.io slug.
///
/// Trims leading and trailing whitespace from both values before rendering.
///
/// # Arguments
///
/// * `category` - Display name of the category, such as `Encoding`.
/// * `slug` - crates.io category slug, such as `encoding`.
///
/// # Errors
///
/// Returns an error if the category badge template cannot be loaded or rendered.
pub fn create_category_badge(category: &str, slug: &str) -> Result<String> {
    /// Template data used to render a crate category badge.
    #[derive(Serialize)]
    struct Context<'a> {
        /// The category name.
        category: &'a str,
        /// The URL-friendly category identifier.
        slug: &'a str,
    }
    let tt = super::get_template_engine()?;
    let context = Context {
        category: category.trim(),
        slug: slug.trim(),
    };
    let rendered = tt.render("CAT_BADGE", &context)?;

    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that an ordinary category name and slug produce a badge.
    #[test]
    fn test_create_category_badge_basic() {
        let result = create_category_badge("Encoding", "encoding")
            .expect("the basic category badge should render");
        assert_eq!(
            result,
            "[![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} "
        );
    }

    /// Verifies that leading and trailing whitespace is removed before rendering.
    #[test]
    fn test_create_category_badge_trimming() {
        let result = create_category_badge("  Command line interface  \n", "\tcli\t ")
            .expect("the trimmed category badge should render");
        assert_eq!(
            result,
            "[![cat~cli][cat~cli~badge]][cat~cli]{{hi:Command line interface}} "
        );
    }

    /// Verifies that empty category values render as empty badge fields.
    #[test]
    fn test_create_category_badge_empty() {
        let result = create_category_badge("", "").expect("the empty category badge should render");
        assert_eq!(result, "[![cat~][cat~~badge]][cat~]{{hi:}} ");
    }

    /// Verifies that whitespace-only category values trim to empty strings.
    #[test]
    fn test_create_category_badge_whitespace_only() {
        let result = create_category_badge("   \t", " \n ")
            .expect("the whitespace-only category badge should render");
        assert_eq!(result, "[![cat~][cat~~badge]][cat~]{{hi:}} ");
    }

    /// Verifies that Unicode and HTML-sensitive text is rendered safely.
    #[test]
    fn test_create_category_badge_special_chars_and_unicode() {
        let result = create_category_badge("Asynchronous / I/O & Networking 🎉", "async-io")
            .expect("the Unicode category badge should render");
        assert_eq!(
            result,
            "[![cat~async-io][cat~async-io~badge]][cat~async-io]{{hi:Asynchronous / I/O &amp; Networking 🎉}} "
        );
    }

    /// Verifies that hyphens and underscores in slugs are preserved.
    #[test]
    fn test_create_category_badge_hyphens_and_underscores() {
        let result = create_category_badge("Data Structures", "data_structures-v2")
            .expect("the category badge with separators should render");
        assert_eq!(
            result,
            "[![cat~data_structures-v2][cat~data_structures-v2~badge]][cat~data_structures-v2]{{hi:Data Structures}} "
        );
    }

    /// Verifies that trimming preserves repeated whitespace inside category names.
    #[test]
    fn test_create_category_badge_preserves_internal_whitespace() {
        let result =
            create_category_badge("   Web  programming:   HTTP   ", "   web-programming   ")
                .expect("the category badge with internal whitespace should render");
        assert_eq!(
            result,
            "[![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web  programming:   HTTP}} "
        );
    }
}
