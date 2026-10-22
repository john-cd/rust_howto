// TODO Reviewed ??-??-2026
//! Generates the crate index pages of the book.
mod cli;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;

use anyhow::Context as _;
use anyhow::Result;
use itertools::Itertools as _;
use tool_lib::Category;

/// This binary creates markdown files for the crate indices
///
/// It can create a page with crates grouped by category,
/// a page with crates grouped alphabetically, update the refdefs,
/// or expand `{{#crate <name>}}` directives in Markdown files.
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    match cli::get_cmd()? {
        cli::Cmd::None => {
            return Err(anyhow::anyhow!("You did not enter a command!"));
        }
        cli::Cmd::CategoryPage(c) => {
            let crates_with_categories: Result<Vec<(String, Vec<Category>)>> = c
                .crate_names
                .into_iter()
                .filter(|name| name != "std")
                .map(|n| {
                    let name = n.trim();
                    let mut cats = tool_lib::get_categories_for_crate(name)?;
                    if cats.is_empty() {
                        cats.push(Category {
                            category: "Uncategorized".into(),
                            slug: "uncategorized".into(),
                            description: "These crates are not added to any category on crates.io"
                                .into(),
                        });
                    }
                    Ok((name.into(), cats))
                })
                .collect();

            // Flatten the Vec((String, Vec<Cat>)) into Vec<(Cat, String)>,
            // sort and group by category
            let category_and_crates: HashMap<Category, Vec<String>> = crates_with_categories?
                .into_iter()
                .flat_map(|(name, cats)| cats.into_iter().map(move |cat| (cat, name.clone())))
                .sorted() // needs Itertools
                .into_group_map(); // needs Itertools

            for (cat, crate_names) in category_and_crates.iter().sorted_by_key(|x| x.0) {
                let markdown = tool_lib::create_category_and_crates(
                    &cat.category,
                    &cat.slug,
                    &cat.description,
                    crate_names.iter().map(AsRef::as_ref).collect(), /* convert Vec<String> into Vec<&str> */
                )?;
                println!("{markdown}");
            }
        }
        cli::Cmd::CategoryPageFromMetadata(c) => {
            let metadata = std::fs::read_to_string(&c.metadata_path)
                .with_context(|| format!("read {}", c.metadata_path.display()))?;
            let recipe_links = tool_lib::collect_published_recipe_links(&c.book_root)?;
            let markdown = tool_lib::create_category_page_from_metadata_with_recipe_links(
                &c.crate_names,
                &metadata,
                &recipe_links,
            )?;
            print!("{markdown}");
        }
        cli::Cmd::AlphabeticalCratePage(crates) => {
            let recipe_links = tool_lib::collect_published_recipe_links(&crates.book_root)?;
            print!(
                "{}",
                create_alphabetical_page(&crates.crate_names, &recipe_links)?
            );
        }
        cli::Cmd::CratePage(crates) => {
            // Group by first letter, uppercased
            let grouped = crates
                .crate_names
                .iter()
                .filter(|name| !name.is_empty() && name.as_str() != "std")
                .sorted()
                .map(|n| {
                    let f: String = n
                        .chars()
                        .next()
                        .map(|c| c.to_uppercase().collect())
                        .unwrap_or_default();
                    (f, n)
                })
                .into_group_map();

            for (first_letter, crates) in grouped.iter().sorted_by_key(|x| x.0) {
                println!("## {first_letter}\n");
                for name in crates.iter().sorted() {
                    let markdown = tool_lib::create_crate_page_section(name)?;
                    println!("{markdown}");
                }
            }
        }
        cli::Cmd::ListCrates(dircmdargs) => {
            let book_root = dircmdargs.dirpathbuf;
            let list = tool_lib::get_dependencies(book_root)?;
            for crt in list {
                println!("{crt}");
            }
        }
        cli::Cmd::UpdateRefDefs(cmdargs) => {
            let mut all_refdefs = Vec::<String>::new();
            for name in cmdargs.crate_names.iter().filter(|n| *n != "std") {
                let info = tool_lib::get_info_for_crate(name)
                    .with_context(|| format!("Unknown crate: {name}"))?;
                let refdefs = tool_lib::create_crate_block_badges_or_refdefs(
                    &info.crate_data,
                    tool_lib::GenerationMode::CrateBlockRefdefs,
                )?
                .split('\n')
                .map(std::borrow::ToOwned::to_owned)
                .collect::<Vec<String>>();
                all_refdefs.extend(refdefs);
            }
            tool_lib::merge(cmdargs.filepathbuf, all_refdefs)?;
        }
        cli::Cmd::ExpandCrateBlocks(cmdargs) => {
            tool_lib::expand_crate_block_directives_in_directory(
                &cmdargs.dirpathbuf,
                cmdargs.refdefs_filepathbuf.as_deref(),
            )?;
        }
    }
    Ok(())
}

/// Header of the alphabetical page.
const ALPHABETICAL_PAGE_HEADER: &str = "# Crates in Alphabetical Order\n\n\
Crates mentioned in this book, in alphabetical order. Nested links point to published recipes that use each crate.\n\n";

/// Footer of the alphabetical page, which includes the reference definitions.
const ALPHABETICAL_PAGE_FOOTER: &str = "{{#include refs.incl.md}}\n\
{{#include ../refs/link-refs.md}}\n\n\
<div class=\"hidden\">\n\
</div>\n";

/// Returns the alphabetical page of the given crates and the crates that have
/// published recipes.
fn create_alphabetical_page(
    crate_names: &[String],
    recipe_links: &tool_lib::RecipeLinksByCrate,
) -> anyhow::Result<String> {
    let crate_names: BTreeSet<String> = crate_names
        .iter()
        .map(|name| name.trim().to_owned())
        .filter(|name| !name.is_empty())
        .chain(recipe_links.keys().cloned())
        .collect();
    let mut grouped = BTreeMap::<String, Vec<String>>::new();

    for name in crate_names {
        let first_letter: String = name
            .chars()
            .next()
            .expect("empty crate names were filtered")
            .to_uppercase()
            .collect();
        grouped.entry(first_letter).or_default().push(name);
    }

    let mut markdown = String::from(ALPHABETICAL_PAGE_HEADER);
    for (first_letter, crates) in grouped {
        markdown.push_str(&tool_lib::create_alphabetical_crate_page_section(
            &first_letter,
            crates.iter().map(String::as_str).collect(),
            recipe_links,
        )?);
        markdown.push('\n');
    }
    markdown.push_str(ALPHABETICAL_PAGE_FOOTER);
    Ok(markdown)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::create_alphabetical_page;

    const MASTER_CRATES: &str = include_str!("../../../bk/master/crates.txt");

    const CATEGORY_INDEX: &str = include_str!("../../../bk/src/indices/crates_by_category.md");

    const CATEGORY_METADATA: &str = include_str!("../../../bk/master/crate_categories.toml");

    const ALPHABETICAL_INDEX: &str = include_str!("../../../bk/src/indices/crates_alphabetical.md");

    #[test]
    fn category_index_covers_master_crates() {
        let book_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../bk");
        let recipe_links =
            tool_lib::collect_published_recipe_links(book_root).expect("input should parse");
        let mut expected: BTreeSet<_> = MASTER_CRATES
            .lines()
            .map(str::trim)
            .filter(|name| !name.is_empty() && *name != "std")
            .collect();
        expected.extend(
            recipe_links
                .keys()
                .map(String::as_str)
                .filter(|name| *name != "std"),
        );
        let indexed = category_index_crates(CATEGORY_INDEX);
        let missing: Vec<_> = expected.difference(&indexed).copied().collect();

        assert!(
            missing.is_empty(),
            "crates in master/crates.txt missing from the category index: {}",
            missing.join(", ")
        );
    }

    #[test]
    fn category_index_matches_offline_generation() {
        let crate_names: Vec<_> = MASTER_CRATES.lines().map(str::to_owned).collect();
        let book_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../bk");
        let recipe_links =
            tool_lib::collect_published_recipe_links(&book_root).expect("input should parse");
        let generated = tool_lib::create_category_page_from_metadata_with_recipe_links(
            &crate_names,
            CATEGORY_METADATA,
            &recipe_links,
        )
        .expect("input should parse");

        assert_eq!(generated, CATEGORY_INDEX);
    }

    #[test]
    fn alphabetical_index_matches_published_recipe_generation() {
        let book_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../bk");
        let recipe_links =
            tool_lib::collect_published_recipe_links(book_root).expect("input should parse");
        let crate_names: Vec<_> = MASTER_CRATES.lines().map(str::to_owned).collect();
        let generated =
            create_alphabetical_page(&crate_names, &recipe_links).expect("input should parse");

        assert_eq!(generated, ALPHABETICAL_INDEX);
        for crate_name in recipe_links.keys() {
            assert!(
                generated.contains(&format!(
                    "[![{crate_name}][c~{crate_name}~docs~badge]][c~{crate_name}~docs]"
                )),
                "published recipe crate {crate_name} is missing from the alphabetical index"
            );
        }
    }

    fn category_index_crates(contents: &str) -> BTreeSet<&str> {
        contents
            .lines()
            .flat_map(|line| line.split("[!["))
            .filter_map(|badge| {
                let (label, reference) = badge.split_once("][c~")?;
                let reference = reference.split(']').next()?;
                let name = reference.strip_suffix("~docs~badge")?;
                (label == name).then_some(name)
            })
            .collect()
    }
}
