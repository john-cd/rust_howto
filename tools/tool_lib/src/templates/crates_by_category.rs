use anyhow::Result;
use serde::Serialize;

pub(super) static CATEGORY_ROW: &str = r"| [![cat~{slug}][cat~{slug}~badge]][cat~{slug}]\{\{hi:{category}}} | {description} | {{ for name in crate_names }}[![{name}][c~{name}~docs~badge]][c~{name}~docs]\{\{hi:{name}}} {{ endfor }}|";

/// Context for rendering a category row in the crates by category table.
///
/// category: category name e.g. Encoding
/// slug: category slug used by the crates.io website e.g. encoding
/// description: description of the category
/// crate_names: list of the names of the crates to include in the category (per
/// lib.rs)
#[derive(Serialize)]
struct Context<'a> {
    category: &'a str,
    slug: &'a str,
    description: &'a str,
    crate_names: Vec<&'a str>,
}

/// Create_category_and_crates
/// Returns one line in the "crates organized by category" table:
/// | <category badge> | <description> | <crate badge> <crate badge>... |
///
/// category: category name e.g. Encoding
/// slug: category slug used by the crates.io website e.g. encoding
/// description: description of the category
/// crate_names: list of the names of the crates to include in the category (per
/// lib.rs)
pub fn create_category_and_crates(
    category: &str,
    slug: &str,
    description: &str,
    crate_names: Vec<&str>,
) -> Result<String> {
    let tt = super::get_template_engine()?;
    let context = Context {
        category: category.trim(),
        slug: slug.trim(),
        description: description.trim(),
        crate_names,
    };
    let rendered = tt.render("CATEGORY_ROW", &context)?;
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_category_and_crates() {
        let category = " Encoding ";
        let slug = " encoding ";
        let description = " Description of the category ";
        let crate_names = vec!["base64", "hex"];

        let result = create_category_and_crates(category, slug, description, crate_names).unwrap();

        // Check for the category badge and name
        assert!(result.contains("[![cat~encoding][cat~encoding~badge]][cat~encoding]"));
        assert!(result.contains("{{hi:Encoding}}"));

        // Check for the description
        assert!(result.contains("Description of the category"));

        // Check for the crate badges
        assert!(result.contains("[![base64][c~base64~docs~badge]][c~base64~docs]"));
        assert!(result.contains("{{hi:base64}}"));
        assert!(result.contains("[![hex][c~hex~docs~badge]][c~hex~docs]"));
        assert!(result.contains("{{hi:hex}}"));
    }

    #[test]
    fn test_create_category_and_crates_empty() {
        let category = "Empty";
        let slug = "empty";
        let description = "Empty category";
        let crate_names = vec![];

        let result = create_category_and_crates(category, slug, description, crate_names).unwrap();

        assert!(result.contains("[![cat~empty][cat~empty~badge]][cat~empty]"));
        assert!(result.contains("{{hi:Empty}}"));
        assert!(result.contains("Empty category"));

        // Ensure it doesn't contain crate badges (it should end with a '|')
        assert!(result.ends_with("|"));
    }
}
