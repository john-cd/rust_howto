// ANCHOR: example
//! Formatting crate sections and crate blocks using `tool_lib`.
//!
//! This example demonstrates how to use `tool_lib` to format:
//! 1. Alphabetical crate index sections (`create_alphabetical_crate_page_section`).
//! 2. Crate category rows (`create_category_and_crates`).
//! 3. Markdown crate badges, descriptions, and reference definitions from crate metadata
//!    (`create_crate_block_badges_or_refdefs`).
//!
//! TODO Reviewed ??-??-2026.

use std::collections::BTreeMap;

use anyhow::Result;
use tool_lib::GenerationMode;
use tool_lib::RecipeLink;
use tool_lib::create_alphabetical_crate_page_section;
use tool_lib::create_category_and_crates;
use tool_lib::create_crate_block_badges_or_refdefs;

/// Creates a minimal `crates_io_api::Crate` struct for demonstration.
fn sample_crate_data() -> crates_io_api::Crate {
    crates_io_api::Crate {
        id: "serde".to_owned(),
        name: "serde".to_owned(),
        description: Some("A generic serialization/deserialization framework".to_owned()),
        #[expect(
            deprecated,
            reason = "Deprecated field required by Crate struct layout"
        )]
        license: None,
        documentation: Some("https://docs.rs/serde".to_owned()),
        homepage: Some("https://serde.rs".to_owned()),
        repository: Some("https://github.com/serde-rs/serde".to_owned()),
        downloads: 100_000_000,
        recent_downloads: Some(10_000_000),
        categories: Some(vec!["encoding".to_owned()]),
        keywords: Some(vec!["serde".to_owned(), "serialization".to_owned()]),
        versions: None,
        max_version: "1.0.219".to_owned(),
        max_stable_version: Some("1.0.219".to_owned()),
        links: crates_io_api::CrateLinks {
            version_downloads: String::new(),
            versions: None,
            owners: String::new(),
            owner_team: String::new(),
            owner_user: String::new(),
            reverse_dependencies: String::new(),
        },
        created_at: chrono::DateTime::default(),
        updated_at: chrono::DateTime::default(),
        exact_match: Some(true),
    }
}

/// Formats sample crate index sections and blocks, and prints them.
fn run_example() -> Result<()> {
    // 1. Format an alphabetical crate section:
    let mut links = BTreeMap::new();
    links.insert(
        "serde".to_owned(),
        vec![RecipeLink {
            title: "Serialize and deserialize JSON".to_owned(),
            reference: "ex~encoding~serde_json".to_owned(),
        }],
    );
    let alpha_section = create_alphabetical_crate_page_section("S", vec!["serde"], &links)?;
    println!("--- Alphabetical Section ---\n{alpha_section}");

    // 2. Format a category row linking crates:
    let category_row = create_category_and_crates(
        "Encoding",
        "encoding",
        "Encoding and serialization crates.",
        vec!["serde"],
    )?;
    println!("--- Category Row ---\n{category_row}\n");

    // 3. Format badges and reference definitions using crate metadata:
    let crate_data = sample_crate_data();
    let badges = create_crate_block_badges_or_refdefs(&crate_data, GenerationMode::CrateBlock)?;
    println!("--- Crate Badges ---\n{badges}\n");

    let refdefs =
        create_crate_block_badges_or_refdefs(&crate_data, GenerationMode::CrateBlockRefdefs)?;
    println!("--- Crate Reference Definitions ---\n{refdefs}");

    Ok(())
}

/// Runs the crate section formatting example binary.
fn main() -> Result<()> {
    run_example()
}
// ANCHOR_END: example

/// Runs the crate section formatting example for programmatic execution.
pub fn run() -> Result<()> {
    run_example()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the crate section formatting example runs successfully.
    #[test]
    fn test_format_crate_section_example_entry_point() {
        run().expect("format crate section example should run without errors");
    }

    /// Verifies the output structure of formatted crate badges.
    #[test]
    fn test_format_crate_section_output() {
        let crate_data = sample_crate_data();
        let badges = create_crate_block_badges_or_refdefs(&crate_data, GenerationMode::CrateBlock)
            .expect("crate block badges rendering should succeed");
        assert!(badges.contains("[![serde][c~serde~docs~badge]][c~serde~docs]"));
        assert!(
            badges.contains("[![serde~crates.io][c~serde~crates.io~badge]][c~serde~crates.io]")
        );
    }
}
