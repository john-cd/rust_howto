//! Read the recipe tables included in the published example index.
//!
//! TODO Reviewed ??-??-2026.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::ensure;
use regex::Regex;

/// A published recipe link associated with a crate.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RecipeLink {
    /// The title of the recipe.
    pub title: String,
    /// The reference (link target) of the recipe.
    pub reference: String,
}

/// Recipe links grouped by crate name, with stable crate and recipe ordering.
pub type RecipeLinksByCrate = BTreeMap<String, Vec<RecipeLink>>;

/// Collect crate-to-recipe links from the include fragments used by the published example index.
pub fn collect_published_recipe_links(book_root: impl AsRef<Path>) -> Result<RecipeLinksByCrate> {
    let source_root = fs::canonicalize(book_root.as_ref().join("src"))
        .with_context(|| format!("resolve book source under {}", book_root.as_ref().display()))?;
    let examples_index = source_root.join("indices/examples_index.md");
    let examples_index = fs::canonicalize(&examples_index).with_context(|| {
        format!(
            "resolve published example index {}",
            examples_index.display()
        )
    })?;
    let include_pattern = Regex::new(r"\{\{#include\s+([^}\s]+)[^}]*\}\}")?;
    let recipe_pattern = Regex::new(r"\|\s*\[(?P<title>.*?)\]\[(?P<reference>ex~[^\]]+)\]\s*\|")?;
    let crate_badge_pattern = Regex::new(
        r"\[!\[(?P<name>[^\]]+)\]\[c~(?P<badge>[^~\]]+)~docs~badge\]\]\[c~(?P<link>[^\]]+)~docs\]",
    )?;
    let crate_directive_pattern = Regex::new(r"\{\{\s*#crate\s*:?\s+(?P<name>[^\s}]+)[^}]*\}\}")?;
    let reference_pattern = Regex::new(r"(?m)^\s*\[(ex~[^\]]+)\]:\s*\S+")?;

    let mut pending = vec![examples_index];
    let mut visited = BTreeSet::<PathBuf>::new();
    let mut recipes = BTreeMap::<String, BTreeSet<RecipeLink>>::new();

    while let Some(path) = pending.pop() {
        let path = fs::canonicalize(&path)
            .with_context(|| format!("resolve published Markdown include {}", path.display()))?;
        ensure!(
            path.starts_with(&source_root),
            "published Markdown include escapes the book source: {}",
            path.display()
        );
        if !visited.insert(path.clone()) {
            continue;
        }

        let contents =
            fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let parent = path.parent().unwrap_or(&source_root);
        for include in include_pattern.captures_iter(&contents) {
            pending.push(parent.join(&include[1]));
        }

        for line in contents.lines() {
            let Some(recipe) = recipe_pattern.captures(line) else {
                continue;
            };
            let title = recipe["title"].trim().to_owned();
            let reference = recipe["reference"].trim().to_owned();
            let link = RecipeLink {
                title: title.clone(),
                reference: reference.clone(),
            };
            for badge in crate_badge_pattern.captures_iter(line) {
                let name = badge["name"].trim();
                ensure!(
                    badge
                        .name("badge")
                        .is_some_and(|value| name == value.as_str())
                        && badge
                            .name("link")
                            .is_some_and(|value| name == value.as_str()),
                    "crate badge names do not agree in {}: {line}",
                    path.display()
                );
                recipes
                    .entry(name.to_owned())
                    .or_default()
                    .insert(link.clone());
            }
            for directive in crate_directive_pattern.captures_iter(line) {
                recipes
                    .entry(directive["name"].trim().to_owned())
                    .or_default()
                    .insert(link.clone());
            }
        }
    }

    let reference_file = source_root.join("indices/refs.incl.md");
    let reference_contents = fs::read_to_string(&reference_file).with_context(|| {
        format!(
            "read recipe reference definitions {}",
            reference_file.display()
        )
    })?;
    let references: BTreeSet<String> = reference_pattern
        .captures_iter(&reference_contents)
        .map(|capture| capture[1].to_owned())
        .collect();

    for (crate_name, links) in &recipes {
        for link in links {
            ensure!(
                references.contains(&link.reference),
                "recipe reference {} for crate {} has no definition in {}",
                link.reference,
                crate_name,
                reference_file.display()
            );
        }
    }

    Ok(recipes
        .into_iter()
        .map(|(crate_name, links)| (crate_name, links.into_iter().collect()))
        .collect())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    /// Verifies that collect published recipe links deduplicates and maps multiple crates.
    #[test]
    fn collect_published_recipe_links_deduplicates_and_maps_multiple_crates() {
        let directory = tempdir().expect("input should parse");
        let source = directory.path().join("src");
        let indices = source.join("indices");
        let recipes = source.join("recipes");
        fs::create_dir_all(&indices).expect("input should parse");
        fs::create_dir_all(&recipes).expect("input should parse");
        fs::write(
            indices.join("examples_index.md"),
            "{{#include ../recipes/recipes.incl.md}}\n{{#include refs.incl.md}}\n",
        )
        .expect("input should parse");
        fs::write(
            recipes.join("recipes.incl.md"),
            concat!(
                "| Recipe | Crates |\n",
                "| --- | --- |\n",
                "| [One recipe][ex~sample~one] | [![crate-a][c~crate-a~docs~badge]][c~crate-a~docs] [![crate-b][c~crate-b~docs~badge]][c~crate-b~docs] |\n",
                "| [One recipe][ex~sample~one] | [![crate-a][c~crate-a~docs~badge]][c~crate-a~docs] |\n",
                "| [Another recipe][ex~sample~two] | [![crate-a][c~crate-a~docs~badge]][c~crate-a~docs] |\n",
                "| [Directive recipe][ex~sample~three] | {{#crate crate-c testing}} {{#categories testing=Testing}} |\n"
            ),
        )
        .expect("input should parse");
        fs::write(
            indices.join("refs.incl.md"),
            concat!(
                "[ex~sample~one]: ../recipes/recipes.md#one\n",
                "[ex~sample~two]: ../recipes/recipes.md#two\n",
                "[ex~sample~three]: ../recipes/recipes.md#three\n"
            ),
        )
        .expect("input should parse");

        let recipes = collect_published_recipe_links(directory.path()).expect("input should parse");

        assert_eq!(recipes["crate-a"].len(), 2);
        assert_eq!(recipes["crate-b"].len(), 1);
        assert_eq!(recipes["crate-c"].len(), 1);
        assert_eq!(recipes["crate-a"][0].title, "Another recipe");
        assert_eq!(recipes["crate-a"][1].reference, "ex~sample~one");
        assert_eq!(recipes["crate-c"][0].title, "Directive recipe");
    }

    /// Verifies that collect published recipe links rejects includes outside book source.
    #[test]
    fn collect_published_recipe_links_rejects_includes_outside_book_source() {
        let directory = tempdir().expect("input should parse");
        let source = directory.path().join("src");
        let indices = source.join("indices");
        fs::create_dir_all(&indices).expect("input should parse");
        fs::write(
            indices.join("examples_index.md"),
            "{{#include ../../outside.md}}\n",
        )
        .expect("input should parse");
        fs::write(directory.path().join("outside.md"), "outside").expect("input should parse");
        fs::write(indices.join("refs.incl.md"), "").expect("input should parse");

        let error =
            collect_published_recipe_links(directory.path()).expect_err("input should be rejected");

        assert!(format!("{error:#}").contains("escapes the book source"));
    }
}
