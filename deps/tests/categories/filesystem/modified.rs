// ANCHOR: example
use std::env;
use std::fs;

use anyhow::Result;
use anyhow::anyhow;

fn main() -> Result<()> {
    let current_dir = env::current_dir()?;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;
        if let Ok(time) = metadata.modified() {
            // Note: SystemTime.elapsed can be flaky.
            if let Ok(duration) = time.elapsed() {
                let last_modified = duration.as_secs();
                if (last_modified < 24 * 3600) && metadata.is_file() {
                    println!(
                        "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                        last_modified,
                        metadata.permissions().readonly(),
                        metadata.len(),
                        path.file_name().ok_or(anyhow!("No filename"))?
                    );
                }
            }
        } else {
            println!("Last modification time not supported on this platform");
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
// [modified: flaky test (P1)](https://github.com/john-cd/rust_howto/issues/165)
