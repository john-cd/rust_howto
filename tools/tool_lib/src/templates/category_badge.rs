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
        let result = create_category_badge("Encoding", "encoding").unwrap();
        assert_eq!(
            result,
            "[![cat~encoding][cat~encoding~badge]][cat~encoding]{{hi:Encoding}} "
        );
    }

    #[test]
    fn test_create_category_badge_trimming() {
        let result = create_category_badge("  Command line interface  \n", "\tcli\t ").unwrap();
        assert_eq!(
            result,
            "[![cat~cli][cat~cli~badge]][cat~cli]{{hi:Command line interface}} "
        );
    }

    #[test]
    fn test_create_category_badge_empty() {
        let result = create_category_badge("", "").unwrap();
        assert_eq!(result, "[![cat~][cat~~badge]][cat~]{{hi:}} ");
    }

    #[test]
    fn test_create_category_badge_whitespace_only() {
        let result = create_category_badge("   ", "\t \n").unwrap();
        assert_eq!(result, "[![cat~][cat~~badge]][cat~]{{hi:}} ");
    }

    #[test]
    fn test_create_category_badge_hyphenated_slug() {
        let result = create_category_badge("Command-line Interface", "command-line-interface").unwrap();
        assert_eq!(
            result,
            "[![cat~command-line-interface][cat~command-line-interface~badge]][cat~command-line-interface]{{hi:Command-line Interface}} "
        );
    }

    #[test]
    fn test_create_category_badge_unicode_and_special_chars() {
        let result = create_category_badge("GUI & Web 🚀", "gui-web").unwrap();
        assert_eq!(
            result,
            "[![cat~gui-web][cat~gui-web~badge]][cat~gui-web]{{hi:GUI &amp; Web 🚀}} "
        );
    }
}
