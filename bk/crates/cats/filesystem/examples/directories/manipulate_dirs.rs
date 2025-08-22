#![allow(dead_code)]
// ANCHOR: example
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = Path::new("./example_dir");

    // 1. Create a directory:
    if !path.exists() {
        fs::create_dir(path)?;
        // This function will return an error e.g. if the parent directory
        // doesn't exist, if the directory already exists, or if
        // permissions are lacking.
        println!("Directory 'example_dir' created successfully!");
    } else {
        println!("Directory 'example_dir' already exists.");
    }
    // 2. Remove it. The directory must be empty:
    fs::remove_dir(path)?;
    println!("'{}' successfully removed.", path.display());

    // 3. Create a directory and any of its missing parents:
    let path = "./temp/my_files/subfolder";
    fs::create_dir_all(path)?; // Creates 'temp', 'temp/my_files', and 'temp/my_files/subfolder' if they don't exist.

    // 4. Create a few files for the next step.
    // `write` creates a file if it does not exist, and will entirely replace
    // its contents if it does.
    fs::write("./temp/my_files/file1.txt", "Hello")?;
    fs::write("./temp/my_files/subfolder/file2.log", "World")?;

    // 5. List the contents of a directory:
    println!("Reading contents of 'my_files':");
    // `read_dir` returns an iterator over the directory's entries.
    // Note that the order in which `read_dir` returns entries is not
    // guaranteed.
    for entry in fs::read_dir("./temp/my_files")? {
        let entry = entry?; // Handle potential error for each entry.
        let path = entry.path();

        if path.is_dir() {
            println!("- (Directory) {}", path.display());
        } else {
            println!("- (File)      {}", path.display());
        }
    }

    // 6. `std::fs::remove_dir_all` removes a directory at a given path, after
    // removing all its contents - similar to `rm -rf` on Unix.
    // Note that `remove_dir_all` will fail if `remove_dir` or `remove_file`
    // fail on any constituent paths. The directory you are deleting must
    // therefore exist. This function does not follow symbolic links and
    // will simply remove the symbolic link itself.
    fs::remove_dir_all("./temp/my_files")?;
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
