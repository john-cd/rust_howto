//! Parses category hierarchy data from TOML.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Context as _;
use anyhow::Result;

use super::Category;

/// Recursively parses a TOML table representing categories and their subcategories.
///
/// The categories TOML file stores child categories in nested tables like
/// `[slug.categories.subcategory-slug]`. This function recursively traverses
/// these tables to extract all categories and their hierarchical relationships.
///
/// Inspired by code in the `crates.io` repository.
///
/// # Arguments:
///
/// * `categories` - A reference to the TOML table containing category data.
/// * `parent` - The enclosing category, if this table contains child categories.
///
/// # Errors
///
/// Returns an error when a category entry is not a table, a required name is missing or not a
/// string, or a child-category entry is not a table.
pub(super) fn categories_from_toml(
    categories: &toml::value::Table,
    parent: Option<&Category>,
) -> Result<Vec<Category>> {
    let mut result = vec![];

    for (slug, details) in categories {
        let details = details
            .as_table()
            .with_context(|| format!("category {slug} was not a TOML table"))?;

        let category = Category::from_parent(
            slug,
            required_string_from_toml(details, "name")?,
            optional_string_from_toml(details, "description"),
            parent,
        );

        if let Some(categories) = details.get("categories") {
            let categories = categories
                .as_table()
                .with_context(|| format!("child categories of {slug} were not a table"))?;

            result.extend(categories_from_toml(categories, Some(&category))?);
        }

        result.push(category);
    }

    Ok(result)
}

/// Extracts a required string value from a TOML table.
///
/// This function retrieves a value associated with a given key from a TOML table
/// and ensures it is a string. If the key is missing or the value is not a string,
/// it returns an error.
///
/// # Arguments:
///
/// * `toml` - A reference to the TOML table.
/// * `key` - The key to look up in the TOML table.
fn required_string_from_toml<'a>(toml: &'a toml::value::Table, key: &str) -> Result<&'a str> {
    toml.get(key)
        .and_then(toml::Value::as_str)
        .with_context(|| format!("Expected category TOML attribute '{key}' to be a String"))
}

/// Extracts an optional string value from a TOML table.
///
/// This function retrieves a value associated with a given key from a TOML table.
/// If the key is missing or the value is not a string, it returns an empty string.
///
/// # Arguments:
///
/// * `toml` - A reference to the TOML table.
/// * `key` - The key to look up in the TOML table.
fn optional_string_from_toml<'a>(toml: &'a toml::value::Table, key: &str) -> &'a str {
    toml.get(key).and_then(toml::Value::as_str).unwrap_or("")
}

impl Category {
    /// Format a child category as "`parent::child`".
    fn from_parent(slug: &str, category: &str, description: &str, parent: Option<&Self>) -> Self {
        match parent {
            Some(parent) => Self {
                slug: format!("{}::{slug}", parent.slug),
                category: format!("{}::{category}", parent.category),
                description: description.into(),
            },
            None => Self {
                slug: slug.into(),
                category: category.into(),
                description: description.into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that a single category preserves its slug, name, and description.
    #[test]
    fn test_categories_from_toml_single_category() -> Result<()> {
        let toml_str = r#"
            [command-line-interface]
            name = "Command Line Interface"
            description = "CLI utilities and apps"
        "#;
        let table: toml::value::Table = toml::from_str(toml_str)?;
        let categories = categories_from_toml(&table, None)?;

        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].slug, "command-line-interface");
        assert_eq!(categories[0].category, "Command Line Interface");
        assert_eq!(categories[0].description, "CLI utilities and apps");
        Ok(())
    }

    /// Verifies that nested categories inherit the parent name and slug.
    #[test]
    fn test_categories_from_toml_nested_categories() -> Result<()> {
        let toml_str = r#"
            [parent-cat]
            name = "Parent Category"
            description = "Parent description"

            [parent-cat.categories.child-cat]
            name = "Child Category"
            description = "Child description"
        "#;
        let table: toml::value::Table = toml::from_str(toml_str)?;
        let categories = categories_from_toml(&table, None)?;

        assert_eq!(categories.len(), 2);

        let child = categories
            .iter()
            .find(|c| c.slug == "parent-cat::child-cat")
            .expect("child category found");
        assert_eq!(child.category, "Parent Category::Child Category");
        assert_eq!(child.description, "Child description");

        let parent = categories
            .iter()
            .find(|c| c.slug == "parent-cat")
            .expect("parent category found");
        assert_eq!(parent.category, "Parent Category");
        assert_eq!(parent.description, "Parent description");

        Ok(())
    }

    /// Verifies that a category without a description uses an empty description.
    #[test]
    fn test_categories_from_toml_optional_description() -> Result<()> {
        let toml_str = r#"
            [no-desc]
            name = "No Description Category"
        "#;
        let table: toml::value::Table = toml::from_str(toml_str)?;
        let categories = categories_from_toml(&table, None)?;

        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].slug, "no-desc");
        assert_eq!(categories[0].category, "No Description Category");
        assert_eq!(categories[0].description, "");
        Ok(())
    }

    /// Verifies that a category entry with a scalar value is rejected.
    #[test]
    fn test_categories_from_toml_invalid_details_table() -> Result<()> {
        let toml_str = r#"
            invalid-cat = "not a table"
        "#;
        let table: toml::value::Table = toml::from_str(toml_str)?;
        let error = categories_from_toml(&table, None)
            .expect_err("a scalar category entry should be rejected");

        let err_msg = error.to_string();
        assert!(err_msg.contains("category invalid-cat was not a TOML table"));
        Ok(())
    }

    /// Verifies that a category without a required name is rejected.
    #[test]
    fn test_categories_from_toml_missing_required_name() -> Result<()> {
        let toml_str = r#"
            [missing-name]
            description = "Missing name field"
        "#;
        let table: toml::value::Table = toml::from_str(toml_str)?;
        let error = categories_from_toml(&table, None)
            .expect_err("a category without a name should be rejected");

        let err_msg = error.to_string();
        assert!(err_msg.contains("Expected category TOML attribute 'name' to be a String"));
        Ok(())
    }

    /// Verifies that a scalar child-category field is rejected.
    #[test]
    fn test_categories_from_toml_invalid_child_categories_table() -> Result<()> {
        let toml_str = r#"
            [bad-children]
            name = "Bad Children"
            categories = "not a table"
        "#;
        let table: toml::value::Table = toml::from_str(toml_str)?;
        let error = categories_from_toml(&table, None)
            .expect_err("a scalar child-category entry should be rejected");

        let err_msg = error.to_string();
        assert!(err_msg.contains("child categories of bad-children were not a table"));
        Ok(())
    }
}
