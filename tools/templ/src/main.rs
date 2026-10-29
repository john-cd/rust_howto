//! Generates files from templates for the book.
//!
//! TODO Reviewed ??-??-2026.

use std::io::Write;

use anyhow::anyhow;
use tool_lib::Category;
use tool_lib::create_category_badge;
use tool_lib::create_crate_block;
use tool_lib::create_rbe_badge;
use tool_lib::get_info_for_crate;

mod cli;
mod config_cmd;

pub(crate) use config_cmd::Cmd;
pub(crate) use config_cmd::CmdArgs;
pub(crate) use config_cmd::CmdArgs2;
pub(crate) use config_cmd::Config;

/// The main function of the `templ` tool.
///
/// This function parses command-line arguments, sets up logging, and dispatches
/// to the appropriate command handler based on the user's input.
fn main() -> anyhow::Result<()> {
    let (config, cmd) = cli::run()?;

    // Set the log level based on the verbose flag.
    let log_level = if config.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::ERROR
    };
    tracing_subscriber::fmt().with_max_level(log_level).init();

    dispatch(cmd, &mut std::io::stdout())
}

/// Returns the categories whose slug, name or description match a word.
fn matching_categories<'a>(all_categories: &'a [Category], word: &str) -> Vec<&'a Category> {
    let possible_cat_name = word.trim().to_lowercase();
    let possible_slug = possible_cat_name.replace([' ', '_'], "-");
    all_categories
        .iter()
        .filter(|&c| {
            c.slug == possible_slug
                || c.category.to_lowercase().contains(&possible_cat_name)
                || c.description.to_lowercase().contains(&possible_cat_name)
        })
        .collect()
}

/// Runs the selected command and writes its output to `out`.
fn dispatch(cmd: Cmd, out: &mut impl Write) -> anyhow::Result<()> {
    dispatch_with_categories(cmd, out, tool_lib::get_all_categories)
}

/// Runs a command with an injected category catalog loader.
fn dispatch_with_categories(
    cmd: Cmd,
    out: &mut impl Write,
    load_categories: impl FnOnce() -> anyhow::Result<Vec<Category>>,
) -> anyhow::Result<()> {
    match cmd {
        // Generate badges for one or more crates.
        //
        // `create_crate_block` returns the badges and description (printed) and the reference
        // definitions (merged into the file given with `-a`). Category badges come from the
        // separate category command, so keywords and categories are not repeated here.
        Cmd::Badges(b) => {
            for name in b.args {
                let (badges, refdefs) = create_crate_block(name.trim())?;
                writeln!(out, "{badges}")?;
                // -a FILE (or simply -a) was passed as an argument.
                if let Some(ref pathbuf) = b.file {
                    tool_lib::merge(pathbuf, refdefs)?;
                }
            }
        }
        // Generate badge(s) for the Rust-by-example book.
        Cmd::Rbe(r) => {
            for concept in r.args {
                let badge = create_rbe_badge(&concept)?;
                writeln!(out, "{badge}")?;
            }
        }
        // Generate category badges for a given crate.
        Cmd::CategoriesForCrateBadge(c) => {
            for n in c.args {
                let name = n.trim();
                for cat in tool_lib::get_categories_for_crate(name)? {
                    let markdown = create_category_badge(&cat.category, &cat.slug)?;
                    writeln!(out, "{markdown}")?;
                }
            }
        }
        // Generate possible category badges for one or more input strings.
        Cmd::CategoryBadges(cb) => {
            let all_categories = load_categories()?;
            for cat in cb.args {
                let matches = matching_categories(&all_categories, &cat);
                if matches.is_empty() {
                    return Err(anyhow!("Could not find the desired category!"));
                }
                for cat in matches {
                    let markdown = create_category_badge(&cat.category, &cat.slug)?;
                    writeln!(out, "{markdown}")?;
                }
                writeln!(out)?;
            }
            // all_categories.iter().for_each( |c| println!("{c:?}") )
        }
        // Returns crate information from the crates.io API for one or more crates.
        //
        Cmd::Info(i) => {
            for name in i.args {
                let info = get_info_for_crate(&name)?;
                writeln!(out, "{info:#?}")?;
            }
        }
        _ => return Err(anyhow!("You did not enter a command!")),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a category for the tests.
    fn category(name: &str, slug: &str, description: &str) -> Category {
        Category {
            category: name.to_owned(),
            slug: slug.to_owned(),
            description: description.to_owned(),
        }
    }

    /// Builds the categories used by the tests.
    fn sample() -> Vec<Category> {
        vec![
            category("Encoding", "encoding", "Encoding and decoding data"),
            category("Web programming", "web-programming", "Create servers"),
        ]
    }

    /// Checks the match by slug, with underscores or spaces.
    #[test]
    fn matches_slug_variants() {
        let all = sample();
        assert_eq!(matching_categories(&all, "web_programming").len(), 1);
        assert_eq!(matching_categories(&all, " Web Programming ").len(), 1);
    }

    /// Checks the match by a word of the name or the description.
    #[test]
    fn matches_name_and_description() {
        let all = sample();
        assert_eq!(matching_categories(&all, "ENCOD")[0].slug, "encoding");
        assert_eq!(
            matching_categories(&all, "servers")[0].slug,
            "web-programming"
        );
    }

    /// Checks that unknown words match nothing.
    #[test]
    fn no_match() {
        assert!(matching_categories(&sample(), "zzz").is_empty());
    }

    /// Checks that the offline `rbe` command writes a badge for each concept.
    #[test]
    fn rbe_command_writes_badges() {
        let mut out = Vec::new();
        let cmd = Cmd::Rbe(CmdArgs {
            args: vec!["attributes".to_owned(), "traits".to_owned()],
        });
        dispatch(cmd, &mut out).expect("rbe should succeed");
        let text = String::from_utf8(out).expect("output should be UTF-8");
        assert!(text.contains("rust-by-example/attributes.html"));
        assert!(text.contains("rust-by-example/traits.html"));
    }

    /// Checks that a missing command is an error.
    #[test]
    fn no_command_is_an_error() {
        let mut out = Vec::new();
        let error = dispatch(Cmd::None, &mut out).expect_err("no command");
        assert!(error.to_string().contains("did not enter a command"));
        assert!(out.is_empty());
    }

    /// Category dispatch accepts an injected catalog and does not fetch it in the test.
    #[test]
    fn category_dispatch_uses_injected_catalog() {
        let categories = vec![category(
            "Text Processing",
            "text-processing",
            "Transform words",
        )];
        let command = Cmd::CategoryBadges(CmdArgs {
            args: vec!["text".to_owned()],
        });
        let mut out = Vec::new();

        dispatch_with_categories(command, &mut out, || Ok(categories))
            .expect("category dispatch should succeed with the injected catalog");

        let output = String::from_utf8(out).expect("the generated badge should be UTF-8");
        assert!(output.contains("text-processing"));
    }

    /// Runs `main`, which parses the real process arguments (not valid for this CLI under the test harness).
    #[test]
    #[ignore = "main() parses the real process arguments, which belong to the test harness"]
    fn main_runs() {
        main().expect("main should succeed");
    }
}
