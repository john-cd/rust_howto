mod request;
use anyhow::Context;
use anyhow::Result;
use request::*;

use super::Category;

/// Returns a list of all cetagories that exist on crates.io
pub fn get_all_categories() -> Result<Vec<Category>> {
    let toml_string = get_categories_toml_string()?;
    let toml: toml::value::Table = toml::from_str(&toml_string)
        .context("Could not parse categories toml")?;

    let categories = categories_from_toml(&toml, None)?;
    Ok(categories)
}

// The categories TOML file stores child categories
// in [slug.categories.subcategory-slug] tables.
//
// Inspired by code in the crates.io repo
fn categories_from_toml(
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
            let categories = categories.as_table().with_context(|| {
                format!("child categories of {slug} were not a table")
            })?;

            result.extend(categories_from_toml(categories, Some(&category))?);
        }

        result.push(category)
    }

    Ok(result)
}

fn required_string_from_toml<'a>(
    toml: &'a toml::value::Table,
    key: &str,
) -> Result<&'a str> {
    toml.get(key)
        .and_then(toml::Value::as_str)
        .with_context(|| {
            format!("Expected category TOML attribute '{key}' to be a String")
        })
}

fn optional_string_from_toml<'a>(
    toml: &'a toml::value::Table,
    key: &str,
) -> &'a str {
    toml.get(key).and_then(toml::Value::as_str).unwrap_or("")
}

impl Category {
    // Format a child category as "parent::child"
    fn from_parent(
        slug: &str,
        category: &str,
        description: &str,
        parent: Option<&Category>,
    ) -> Category {
        match parent {
            Some(parent) => Category {
                slug: format!("{}::{}", parent.slug, slug),
                category: format!("{}::{}", parent.category, category),
                description: description.into(),
            },
            None => Category {
                slug: slug.into(),
                category: category.into(),
                description: description.into(),
            },
        }
    }
}