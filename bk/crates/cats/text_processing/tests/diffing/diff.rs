// ANCHOR: example
use diff::Result;

// Add to your `Cargo.toml`:
// [dependencies]
// diff = "0.1.13" # Or latest

fn main() -> anyhow::Result<()> {
    let old_text = "Hello world\nThis is a test\nOf the diff library";
    let new_text =
        "Hello world\nThis is a sample\nOf the diff library\nWith a new line";

    // Computes the diff between the lines of two strings.
    let changeset = diff::lines(old_text, new_text);

    // Print the diff
    for chunk in changeset {
        match chunk {
            Result::Both(s, _) => println!(" {}", s),
            Result::Left(s) => println!("-{}", s),
            Result::Right(s) => println!("+{}", s),
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
