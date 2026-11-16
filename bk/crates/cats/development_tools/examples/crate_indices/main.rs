// TODO Reviewed ??-??-2026.

//! Demonstrates local crate-index generation with checked fixture files.

/// Generate an alphabetical crate section with published recipe links.
mod alphabetical_page;
/// Generate a category page from local TOML metadata and recipe links.
mod category_page;
/// List dependencies declared in local Cargo manifests.
mod dependencies;
/// Shared local files for the crate-index examples.
mod fixtures;
/// Collect published recipe links from the local book index.
mod published_recipes;

/// Runs each local crate-index example in sequence.
fn main() -> anyhow::Result<()> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("dependencies"));
    command = command.subcommand(clap::Command::new("published-recipes"));
    command = command.subcommand(clap::Command::new("category-page"));
    command = command.subcommand(clap::Command::new("alphabetical-page"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("dependencies", _)) => {
            dependencies::run()?;
            Ok(())
        }
        Some(("published-recipes", _)) => {
            published_recipes::run()?;
            Ok(())
        }
        Some(("category-page", _)) => {
            category_page::run()?;
            Ok(())
        }
        Some(("alphabetical-page", _)) => {
            alphabetical_page::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
