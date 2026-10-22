//! The alphabetical index of crates.
//!
//! TODO Reviewed ??-??-2026

use std::collections::BTreeMap;

use anyhow::Result;

use super::RecipeLink;

/// Returns one section of the "crates by alphabetic order" page
pub fn create_alphabetical_crate_page_section<'a>(
    first_letter: &'a str,
    crate_names: Vec<&'a str>,
    recipe_links: &BTreeMap<String, Vec<RecipeLink>>,
) -> Result<String> {
    let mut markdown = format!("## {first_letter}\n\n");
    for name in crate_names {
        markdown.push_str("- [![");
        markdown.push_str(name);
        markdown.push_str("][c~");
        markdown.push_str(name);
        markdown.push_str("~docs~badge]][c~");
        markdown.push_str(name);
        markdown.push_str("~docs]\n");
        if let Some(links) = recipe_links.get(name) {
            for link in links {
                markdown.push_str("  - [");
                markdown.push_str(&link.title);
                markdown.push_str("][");
                markdown.push_str(&link.reference);
                markdown.push_str("]\n");
            }
        }
    }
    Ok(markdown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_alphabetical_crate_page_section_empty() {
        let first_letter = "A";
        let crate_names = vec![];
        let result =
            create_alphabetical_crate_page_section(first_letter, crate_names, &BTreeMap::new())
                .expect("input should parse");
        assert_eq!(result, "## A\n\n");
    }

    #[test]
    fn test_create_alphabetical_crate_page_section_one_crate() {
        let first_letter = "A";
        let crate_names = vec!["anyhow"];
        let result =
            create_alphabetical_crate_page_section(first_letter, crate_names, &BTreeMap::new())
                .expect("input should parse");
        assert_eq!(
            result,
            "## A\n\n- [![anyhow][c~anyhow~docs~badge]][c~anyhow~docs]\n"
        );
    }

    #[test]
    fn test_create_alphabetical_crate_page_section_multiple_crates() {
        let first_letter = "A";
        let crate_names = vec!["anyhow", "assert_cmd"];
        let result =
            create_alphabetical_crate_page_section(first_letter, crate_names, &BTreeMap::new())
                .expect("input should parse");
        assert_eq!(
            result,
            "## A\n\n- [![anyhow][c~anyhow~docs~badge]][c~anyhow~docs]\n- [![assert_cmd][c~assert_cmd~docs~badge]][c~assert_cmd~docs]\n"
        );
    }

    #[test]
    fn test_create_alphabetical_crate_page_section_links_recipes() {
        let mut links = BTreeMap::new();
        links.insert(
            "anyhow".to_owned(),
            vec![RecipeLink {
                title: "Use anyhow for Application Errors".to_owned(),
                reference: "ex~error-handling~anyhow".to_owned(),
            }],
        );

        let result = create_alphabetical_crate_page_section("A", vec!["anyhow"], &links)
            .expect("input should parse");

        assert_eq!(
            result,
            "## A\n\n- [![anyhow][c~anyhow~docs~badge]][c~anyhow~docs]\n  - [Use anyhow for Application Errors][ex~error-handling~anyhow]\n"
        );
    }
}
