// ANCHOR: example
#![allow(clippy::single_match)]
#![cfg(target_family = "unix")]

fn main() -> anyhow::Result<()> {
    use std::process::Command;
    use std::process::Stdio;

    let directory = std::env::current_dir()?;
    let mut du_output_child = Command::new("du")
        .arg("-ah")
        .arg(&directory)
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(du_output) = du_output_child.stdout.take() {
        let mut sort_output_child = Command::new("sort")
            .arg("-hr")
            .stdin(du_output)
            .stdout(Stdio::piped())
            .spawn()?;

        du_output_child.wait()?;

        match sort_output_child.stdout.take() {
            Some(sort_output) => {
                let head_output_child = Command::new("head")
                    .args(["-n", "10"])
                    .stdin(sort_output)
                    .stdout(Stdio::piped())
                    .spawn()?;

                let head_stdout = head_output_child.wait_with_output()?;

                sort_output_child.wait()?;

                println!(
                    "Top 10 biggest files and directories in '{}':\n{}",
                    directory.display(),
                    String::from_utf8(head_stdout.stdout).unwrap()
                );
            }
            _ => {}
        }
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
