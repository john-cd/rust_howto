#![cfg(target_family = "unix")]
// ANCHOR: example

fn main() -> Result<(), std::io::Error> {
    use std::fs;
    use std::fs::File;
    use std::process::Command;
    use std::process::Stdio;

    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }
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
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [error_file: print (P1)](https://github.com/john-cd/rust_howto/issues/167)
