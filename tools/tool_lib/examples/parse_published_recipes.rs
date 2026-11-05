// ANCHOR: example
//! Parsing published recipes using `tool_lib`.
//!
//! This example demonstrates how to use `tool_lib::collect_published_recipe_links`
//! to scan an mdBook structure for published recipes and collect mapping of crate
//! names to their published recipes.

use std::fs;
use std::path::Path;

use anyhow::Result;

/// Sets up a minimal mdBook folder structure in the given directory.
fn create_sample_book_structure(book_root: &Path) -> Result<()> {
    let source = book_root.join("src");
    let indices = source.join("indices");
    let recipes = source.join("recipes");

    fs::create_dir_all(&indices)?;
    fs::create_dir_all(&recipes)?;

    // Main example index pointing to includes:
    fs::write(
        indices.join("examples_index.md"),
        "{{#include ../recipes/recipes.incl.md}}\n{{#include refs.incl.md}}\n",
    )?;

    // Markdown file containing recipe table with crate badges:
    fs::write(
        recipes.join("recipes.incl.md"),
        concat!(
            "| Recipe | Crates |\n",
            "| --- | --- |\n",
            "| [JSON Serialization][ex~sample~json] | [![serde][c~serde~docs~badge]][c~serde~docs] |\n",
            "| [CLI Parsing][ex~sample~cli] | [![clap][c~clap~docs~badge]][c~clap~docs] |\n",
        ),
    )?;

    // Reference definitions for the recipes:
    fs::write(
        indices.join("refs.incl.md"),
        concat!(
            "[ex~sample~json]: ../recipes/json.md#recipe\n",
            "[ex~sample~cli]: ../recipes/cli.md#recipe\n",
        ),
    )?;

    Ok(())
}

/// Parses recipes from a sample book structure and prints the mapped links.
fn run_example() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    create_sample_book_structure(temp_dir.path())?;

    let recipes_by_crate = tool_lib::collect_published_recipe_links(temp_dir.path())?;

    println!(
        "Found published recipes for {} crates:",
        recipes_by_crate.len()
    );
    for (crate_name, recipes) in recipes_by_crate {
        println!("Crate: {crate_name}");
        for recipe in recipes {
            println!("  - {} ({})", recipe.title, recipe.reference);
        }
    }

    Ok(())
}

/// Runs the parse published recipes example binary.
fn main() -> Result<()> {
    run_example()
}
// ANCHOR_END: example

/// Runs the parse published recipes example for programmatic execution.
pub fn run() -> Result<()> {
    run_example()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the parse published recipes example runs successfully.
    #[test]
    fn test_parse_published_recipes_example_entry_point() {
        run().expect("parse published recipes example should run without errors");
    }

    /// Verifies that recipes are properly collected and mapped to crates.
    #[test]
    fn test_collect_published_recipes_from_sample() {
        let temp_dir = tempfile::tempdir().expect("temporary directory should be created");
        create_sample_book_structure(temp_dir.path())
            .expect("sample book structure should be created");

        let recipes = tool_lib::collect_published_recipe_links(temp_dir.path())
            .expect("collecting published recipe links should succeed");

        assert!(recipes.contains_key("serde"));
        assert!(recipes.contains_key("clap"));
        assert_eq!(recipes["serde"][0].title, "JSON Serialization");
        assert_eq!(recipes["serde"][0].reference, "ex~sample~json");
    }
}
