use std::process::Command;

use anyhow::bail;
use anyhow::Result;
use regex::Regex;

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

fn main() -> Result<()> {
    let output = Command::new("git").arg("log").arg("--oneline").output()?;

    if !output.status.success() {
        bail!("Command executed with failing error code");
    }

    let pattern = Regex::new(
        r"(?x)
                               ([0-9a-fA-F]+) # commit hash
                               (.*)           # The commit message",
    )?;

    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| Commit {
            hash: cap[1].to_string(),
            message: cap[2].trim().to_string(),
        })
        .take(5)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}

// TODO
// this test should not run during CI
#[cfg_attr(feature = "ci", ignore)]
#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
