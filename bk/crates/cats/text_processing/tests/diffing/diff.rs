// ANCHOR: example
use diff::Chunk;

// Add to your `Cargo.toml`:
// [dependencies]
// diff = "0.1.13" # Or latest

fn main() -> anyhow::Result<()> {
    let old_text = "Hello world\nThis is a test\nOf the diff library";
    let new_text =
        "Hello world\nThis is a sample\nOf the diff library\nWith a new line";

    // Generate the diff
    let changeset = diff::lines(old_text, new_text);

    // Print the diff
    for chunk in changeset.iter_all_changes() {
        match chunk {
            Chunk::Equal(s) => println!(" {}", s),
            Chunk::Delete(s) => println!("-{}", s),
            Chunk::Insert(s) => println!("+{}", s),
        }
    }

    // Get a summary of changes
    println!("\nSummary:");
    println!("  {} insertions", changeset.insertions());
    println!("  {} deletions", changeset.deletions());

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
