mod categories_from_toml;
mod request;

use anyhow::Context;
use anyhow::Result;

use super::Category;

/// Fetches and parses the categories TOML file from `crates.io`.
///
/// This function retrieves the TOML data, parses it, and then
/// extracts the category information into a vector of `Category` structs.
/// Returns a list of all categories that exist on `crates.io`.
pub fn get_all_categories() -> Result<Vec<Category>> {
    let toml_string = request::get_categories_toml_string()?;
    let toml: toml::value::Table =
        toml::from_str(&toml_string).context("Could not parse categories toml")?;

    let categories = categories_from_toml::categories_from_toml(&toml, None)?;
    Ok(categories)
}
