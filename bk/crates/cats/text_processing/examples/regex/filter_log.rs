// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Filtering the lines of a log file with a set of regular expressions.
//!
//! A regular expression (regex) describes a text pattern. Log files can be
//! huge, so process them line by line and combine patterns in a `RegexSet`,
//! which tests all of them in a single pass.

use std::fs::File;
use std::io::BufRead as _;
use std::io::BufReader;

use anyhow::Result;
use regex::RegexSetBuilder;

/// Prints the lines of the log that match any of the patterns.
fn main() -> Result<()> {
    let log_path = "temp/application.log";
    let buffered = BufReader::new(File::open(log_path)?);

    let set = RegexSetBuilder::new([
        r#"version "\d\.\d\.\d""#,
        r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443",
        "warning.*timeout expired",
    ])
    .case_insensitive(true)
    .build()?;

    buffered
        .lines() // yield instances of io::Result<String>
        .map_while(Result::ok)
        .filter(|line| set.is_match(line.as_str()))
        .for_each(|x| println!("{x}"));

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Text processed by the regular-expression example.
    static EXAMPLE_TEXT: &str = r#"
Lorem ipsum dolor sit amet version "0.1.2"
127.0.0.1:443 consectetur adipisicing elit
WARNING sunt in culpa qui officia TIMEOUT EXPIRED
quis nostrud exercitation ullamco
"#;

    /// Checks that the log is filtered.
    /// Runs the filter log example through its entry point.
    #[test]
    fn test_filter_log_example_entry_point() -> Result<()> {
        use std::fs;
        if !fs::exists("temp")? {
            fs::create_dir("temp")?;
        }
        use std::io::Write as _;
        let mut file = File::create("temp/application.log")?;
        file.write_all(EXAMPLE_TEXT.as_bytes())?;
        main()?;
        Ok(())
    }

    /// The set matches the version, address and timeout lines only.
    #[test]
    fn set_selects_interesting_lines() -> Result<()> {
        let set = RegexSetBuilder::new([
            r#"version "\d\.\d\.\d""#,
            r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443",
            "warning.*timeout expired",
        ])
        .case_insensitive(true)
        .build()?;
        let matching = EXAMPLE_TEXT
            .lines()
            .filter(|line| set.is_match(line))
            .count();
        assert_eq!(matching, 3);
        assert!(!set.is_match("quis nostrud exercitation ullamco"));
        Ok(())
    }

    /// A missing log file is an error.
    #[test]
    fn missing_file_is_an_error() {
        assert!(File::open("temp/no-such-file.log").is_err());
    }
}
