// ANCHOR: example
use anyhow::Result;
use walkdir::WalkDir;

fn main() -> Result<()> {
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        // `metadata()` can return errors for path values that the program
        // does not have permissions to access or if the path no longer exists.
        if let Ok(metadata) = entry.metadata() {
            let sec = metadata.modified()?;
            if let Ok(elapsed) = sec.elapsed() {
                if elapsed.as_secs() < 86400 {
                    println!("{}", f_name);
                }
            }
        }
        // You may also check for specific extensions: && f_name.ends_with(".json")
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
