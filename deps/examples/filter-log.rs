use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;
use regex::RegexSetBuilder;

fn main() -> Result<()> {
    let log_path = "application.log";
    let buffered = BufReader::new(File::open(log_path)?);

    let set = RegexSetBuilder::new([
        r#"version "\d\.\d\.\d""#,
        r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443"#,
        r#"warning.*timeout expired"#,
    ])
    .case_insensitive(true)
    .build()?;

    buffered
        .lines()
        .filter_map(|line| line.ok())  // map_while(Result::ok)
        .filter(|line| set.is_match(line.as_str()))
        .for_each(|x| println!("{}", x));

    Ok(())
}
