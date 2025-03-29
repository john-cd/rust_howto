use anyhow::Result;

// The master list of categories is a TOML file in the `crates.io` repo:
static CATEGORIES_URL: &str = "https://raw.githubusercontent.com/rust-lang/crates.io/refs/heads/main/src/boot/categories.toml";

pub(super) fn get_categories_toml_string() -> Result<String> {
    let response = reqwest::blocking::get(CATEGORIES_URL)?;
    let body = response.text()?;
    Ok(body)
}
// TODO unit tests; consolidate with mod.rs?
