use core_lib::walk_directory_and_process_files;
use regex::Captures;
use regex::Regex;
use regex::Replacer;

use super::conf::*;

// Replacement closure.
type Replacement = Box<dyn Fn(&Captures) -> String>;

pub struct RegexAndReplacement {
    pub re: Regex,
    // Most often set to `None` for no replacement, meaning deletion.
    pub replacement: Option<Replacement>,
}

pub fn get_regexes_and_replacements(conf: &Config) -> Vec<RegexAndReplacement> {
    let rr = vec![];
    // TODO
    if conf.process_crate_directives {}
    if conf.process_category_directives {}
    if conf.process_page_directives {}
    rr
}

pub fn process_all_directives<I>(directories: I, conf: &Config) -> anyhow::Result<()>
where
    I: IntoIterator<Item = std::path::PathBuf>,
{
    let conf: Config = Config::default();
    // Compile the replacement Regex(es) only once.
    let regexes_and_replacements = get_regexes_and_replacements(&conf);
    for directory in directories {
        let dir = directory.as_path().canonicalize()?;
        println!("Processing {}", dir.display());
        walk_directory_and_process_files(&dir, |f| {
            process_all_directives_in_file(f, &regexes_and_replacements)
        })?;
    }
    Ok(())
}

fn process_all_directives_in_file(
    filepath: &std::path::Path,
    regexes_and_replacements: &[RegexAndReplacement],
) -> anyhow::Result<()> {
    core_lib::process_file(filepath, has_directive, |s: &str| {
        process(s, regexes_and_replacements)
    })?;
    Ok(())
}

fn has_directive(_content: &str) -> bool {
    // TODO
    true
}

fn process(content: &str, regexes_and_replacements: &[RegexAndReplacement]) -> String {
    let mut content = content.to_string();
    // If the configuration is fully disabled,
    // don't do anything.
    if !regexes_and_replacements.is_empty() {
        // core_lib::process_file(filepath, has_crate_badge, )?;
        for rr in regexes_and_replacements.iter() {
            content = if let Some(ref repl) = rr.replacement {
                rr.re.replace_all(&content, repl).into_owned()
            } else {
                // If replacement is `None`,
                // just delete the matching text.
                rr.re.replace_all(&content, "").into_owned()
            };
            // tracing::debug!(content);
        }
    }
    content
}
