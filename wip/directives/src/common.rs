use std::borrow::Cow;

use core_lib::walk_directory_and_process_files;
use regex::Captures;
use regex::Regex;
use regex::Replacer;

use super::conf::*;

// Replacement closure.
type Replacement = Box<dyn Fn(&Captures) -> String>;

/// A compiled Regex and replacement string (or function).
pub struct RegexAndReplacement {
    pub re: Regex,
    // Most often set to `None` for no replacement, meaning deletion.
    pub replacement: Option<Replacement>,
}

/// Process all directives in all files in the given directories.
///
/// # Arguments
///
/// * `directories` - An iterator over the directories to process.
/// * `conf` - The configuration for processing directives.
pub fn process_all_directives<I>(directories: I, conf: &Config) -> anyhow::Result<()>
where
    I: IntoIterator<Item = std::path::PathBuf>,
{
    // Compile the replacement Regex(es) only once.
    let regexes_and_replacements = get_regexes_and_replacements(conf);

    // Extensions to process and excluded files.
    let scope = core_lib::Scope::default();

    for directory in directories.into_iter() {
        let dir = directory.as_path().canonicalize()?;
        tracing::info!("Processing directory {}", dir.display());
        walk_directory_and_process_files(&dir, &scope, |f| {
            process_all_directives_in_file(f, &regexes_and_replacements)
        }, )?;
    }
    Ok(())
}

fn get_regexes_and_replacements(conf: &Config) -> Vec<RegexAndReplacement> {
    let rr = vec![];
    // TODO
    // if conf.process_crate_directives {}
    // if conf.process_category_directives {}
    // if conf.process_page_directives {}
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
    rr: &[RegexAndReplacement],
) -> anyhow::Result<()> {
    core_lib::process_file(
        filepath,
        |s: &str| has_directives(s, rr),
        |s: &str| process(s, rr),
    )?;
    Ok(())
}

/// Returns true if at least one directive is found in the string slice.
fn has_directives(content: &str, regexes_and_replacements: &[RegexAndReplacement]) -> bool {
    if regexes_and_replacements.is_empty() {
        return false;
    }
    for rr in regexes_and_replacements.iter() {
        if rr.re.is_match(content) {
            return true;
        }
    }
    false
}

/// Replaces any directive found by the corresponding markdown, in a given string slice:
///
/// # Arguments
///
/// * `content` - The text to process.
///
/// # Returns the updated content as a `String`.
fn process<'a>(content: &'a str, regexes_and_replacements: &[RegexAndReplacement]) -> Cow<'a, str> {
    let mut result = Cow::Borrowed(content);
    if !regexes_and_replacements.is_empty() {
        for rr in regexes_and_replacements.iter() {
            result = if let Some(ref repl) = rr.replacement {
                rr.re.replace_all(content, repl)
            } else {
                // If replacement is `None`,
                // just delete the matching text.
                rr.re.replace_all(content, "")
            };
            // tracing::debug!("Content: {content}");
        }
    }
    result
}
