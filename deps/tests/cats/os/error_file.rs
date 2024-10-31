#![cfg(target_family = "unix")]

fn main() -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::process::Command;
    use std::process::Stdio;

    let outputs = File::create("temp/out.txt")?;
    let errors = outputs.try_clone()?;

    Command::new("ls")
        .args([".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;

    Ok(())
}

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
