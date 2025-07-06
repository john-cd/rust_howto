#![allow(unused)]

mod directive_model;
pub use directive_model::*;

mod crate_blocks;
use crate_blocks::*;

mod conf;
use conf::*;

mod links_or_badges;
use links_or_badges::*;

mod examples;
use examples::*;

mod wikilinks;
use wikilinks::*;

/// Process all directives in all files in scope, in the given directories.
///
/// # Arguments
///
/// * `directories` - An iterator over the directories to process.
/// * `conf` - The configuration for processing directives.
pub fn process_all_directives<I>(directories: I, conf: &Config) -> anyhow::Result<()>
where
    I: IntoIterator<Item = std::path::PathBuf>,
{
    // Compile the Regex(es) only once.
    let regexes_and_replacements = get_regexes_and_replacements(conf);

    // Extensions to process and excluded files.
    let scope = core_lib::Scope::default();

    for directory in directories.into_iter() {
        let dir = directory.as_path().canonicalize()?;
        tracing::info!("Processing directory {}", dir.display());
        core_lib::walk_directory_and_process_files(&dir, &scope, |f| {
            process_all_directives_in_file(f, &regexes_and_replacements)
        })?;
    }
    Ok(())
}

/// Returns a vector of `RegexAndReplacement` based on the provided configuration.
///
/// # Arguments
///
/// * `conf` - The configuration for processing directives.
///
/// # Returns
///
/// A `Vec` of `RegexAndReplacement` structs.
fn get_regexes_and_replacements(conf: &Config) -> Vec<core_lib::RegexAndReplacement> {
    let mut rr = vec![];
    if conf.process_crate_block_directives {
        rr.append(&mut crate_block_regexes());
    }
    if conf.process_example_directives {
        rr.append(&mut example_regexes());
    }
    if conf.process_link_and_badge_directives {
        rr.append(&mut link_or_badge_regexes());
    }
    if conf.process_wikilinks {
        rr.append(&mut wikilink_regexes());
    }
    rr
}

/// Given a file, process all directives found in it.
///
/// # Arguments
///
/// * `filepath` - The path to the file to process.
/// * `rr` - The list of compiled regexes and replacements to apply.
fn process_all_directives_in_file(
    filepath: &std::path::Path,
    rr: &[core_lib::RegexAndReplacement],
) -> anyhow::Result<()> {
    use core_lib::MatchAndReplace;

    // Read a text file in memory, test if its contents should be processed, and if true, update its contents.
    core_lib::process_text_file(
        filepath,
        |s: &str| rr.has_match(s),
        |s: &str| rr.replace_all(s),
    )?;
    Ok(())
}
