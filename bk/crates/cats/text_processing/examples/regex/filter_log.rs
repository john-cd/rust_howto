#![allow(dead_code)]
// ANCHOR: example
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Result;
use regex::RegexSetBuilder;

fn main() -> Result<()> {
    let log_path = "temp/application.log";
    let buffered = BufReader::new(File::open(log_path)?);

    let set = RegexSetBuilder::new([
        r#"version "\d\.\d\.\d""#,
        r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443"#,
        r#"warning.*timeout expired"#,
    ])
    .case_insensitive(true)
    .build()?;

    buffered
        .lines()                // yield instances of io::Result<String>
        .map_while(Result::ok)
        .filter(|line| set.is_match(line.as_str()))
        .for_each(|x| println!("{x}"));

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    use std::fs;
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }
    use std::io::Write;
    let mut file = File::create("temp/application.log")?;
    file.write_all(EXAMPLE_TEXT.as_bytes())?;
    main()?;
    Ok(())
}

static EXAMPLE_TEXT: &str = r#"
Lorem ipsum dolor sit amet version "0.1.2"
127.0.0.1:443 consectetur adipisicing elit
WARNING sunt in culpa qui officia TIMEOUT EXPIRED
quis nostrud exercitation ullamco
"#;
