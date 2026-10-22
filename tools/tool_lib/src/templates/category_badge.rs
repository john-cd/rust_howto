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
    fn test_create_category_badge_special_characters_and_numbers() {
        let result =
            create_category_badge("GUI & 2D Graphics / C++", "gui-2d-graphics-cpp").unwrap();
        assert_eq!(
            result,
            "[![cat~gui-2d-graphics-cpp][cat~gui-2d-graphics-cpp~badge]][cat~gui-2d-graphics-cpp]{{hi:GUI &amp; 2D Graphics / C++}} "
        );
    }

    #[test]
    fn test_create_category_badge_internal_whitespace() {
        let result = create_category_badge(
            "   Web  programming:   HTTP   ",
            "   web-programming   ",
        )
        .unwrap();
        assert_eq!(
            result,
            "[![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web  programming:   HTTP}} "
        );
    }

    #[test]
    fn test_create_category_badge_unicode() {
        let result = create_category_badge("国际化 & 本地化", "i18n").unwrap();
        assert_eq!(
            result,
            "[![cat~i18n][cat~i18n~badge]][cat~i18n]{{hi:国际化 &amp; 本地化}} "
        );
    }
}
