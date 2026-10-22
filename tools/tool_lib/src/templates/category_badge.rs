// TODO Reviewed ??-??-2026
use anyhow::Result;
use serde::Serialize;

/// Category badge
pub(super) static CAT_BADGE: &str =
    r"[![cat~{slug}][cat~{slug}~badge]][cat~{slug}]\{\{hi:{category}}} ";

/// Create a category badge.
///
/// category: category name e.g. Encoding
/// slug: category slug used by the crates.io website e.g. encoding
pub fn create_category_badge(category: &str, slug: &str) -> Result<String> {
    #[derive(Serialize)]
    struct Context<'a> {
        category: &'a str,
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

    #[test]
    fn test_create_category_badge_basic() {
        let result = create_category_badge("Encoding", "encoding").expect("input should parse");
        assert_eq!(
            result,
            "[![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} "
        );
    }

    #[test]
    fn test_create_category_badge_trimming() {
        let result = create_category_badge("  Command line interface  \n", "\tcli\t ")
            .expect("input should parse");
        assert_eq!(
            result,
            "[![cat~cli][cat~cli~badge]][cat~cli]{{hi:Command line interface}} "
        );
    }

    #[test]
    fn test_create_category_badge_empty() {
        let result = create_category_badge("", "").expect("input should parse");
        assert_eq!(result, "[![cat~][cat~~badge]][cat~]{{hi:}} ");
    }

    #[test]
    fn test_create_category_badge_whitespace_only() {
        let result = create_category_badge("   \t", " \n ").unwrap();
        assert_eq!(result, "[![cat~][cat~~badge]][cat~]{{hi:}} ");
    }

    #[test]
    fn test_create_category_badge_special_chars_and_unicode() {
        let result = create_category_badge("Asynchronous / I/O & Networking 🎉", "async-io").unwrap();
        assert_eq!(
            result,
            "[![cat~async-io][cat~async-io~badge]][cat~async-io]{{hi:Asynchronous / I/O &amp; Networking 🎉}} "
        );
    }

    #[test]
    fn test_create_category_badge_hyphens_and_underscores() {
        let result = create_category_badge("Data Structures", "data_structures-v2").unwrap();
        assert_eq!(
            result,
            "[![cat~data_structures-v2][cat~data_structures-v2~badge]][cat~data_structures-v2]{{hi:Data Structures}} "
        );
    }
}
