use anyhow::anyhow;
use tool_lib::*;
mod cli;
mod config_cmd;
pub(crate) use config_cmd::*;

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

    // Dispatch to the appropriate command handler.
    match cmd {
        // Generate badges for one or more crates.
        //
        // Include keywords, categories, description, refdefs.
        // TODO
        Cmd::Badges(b) => {
            for name in b.args {
                let (badges, refdefs) = create_crate_block(name.trim())?;
                println!("{badges}");
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
                println!("{badge}");
            }
        }
        // Generate category badges for a given crate.
        Cmd::CategoriesForCrateBadge(c) => {
            for n in c.args {
                let name = n.trim();
                for cat in tool_lib::get_categories_for_crate(name)? {
                    let markdown = create_category_badge(&cat.category, &cat.slug)?;
                    println!("{markdown}");
                }
            }
        }
        // Generate possible category badges for one or more input strings.
        Cmd::CategoryBadges(cb) => {
            let all_categories = tool_lib::get_all_categories()?;
            for cat in cb.args {
                let possible_cat_name = cat.trim().to_lowercase();
                let possible_slug = possible_cat_name.replace([' ', '_'], "-");
                let matches: Vec<_> = all_categories
                    .iter()
                    .filter(|&c| {
                        c.slug == possible_slug
                            || c.category.to_lowercase().contains(&possible_cat_name)
                            || c.description.to_lowercase().contains(&possible_cat_name)
                    })
                    .collect();
                if matches.is_empty() {
                    return Err(anyhow!("Could not find the desired category!"));
                }
                for cat in matches {
                    let markdown = create_category_badge(&cat.category, &cat.slug)?;
                    println!("{markdown}");
                }
                println!();
            }
            // all_categories.iter().for_each( |c| println!("{c:?}") )
        }
        // Returns crate information from the crates.io API for one or more crates.
        //
        Cmd::Info(i) => {
            for name in i.args {
                let info = get_info_for_crate(&name)?;
                println!("{info:#?}");
            }
        }
        _ => return Err(anyhow!("You did not enter a command!")),
    }
    Ok(())
}
