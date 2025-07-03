use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use anyhow::Result;

/// Read a text file in memory, test if its contents should be processed, and if , update its contents.
///
/// # Arguments
///
/// * `filepath` - The path to the text file to process.
/// * `should_process` - a function that returns true if the file should be processed, given its contents.
/// * `update_contents` - a function that returns the updated contents of the file, given its contents.
///
pub fn process_file<P, F>(filepath: &Path, should_process: P, update_contents: F) -> Result<()>
where
    P: Fn(&str) -> bool,
    F: Fn(&str) -> String,
{
    // Read the file into memory.
    let mut file = File::open(filepath)?;
    let size = file.metadata()?.len() as usize;
    let mut buffer = String::with_capacity(size);
    file.read_to_string(&mut buffer)?;
    drop(file); // Close the file early.

    if should_process(&buffer) {
        let temp_filepath = filepath.with_extension(".tmp");
        let mut temp_file = File::create(&temp_filepath)?;
        buffer = update_contents(&buffer);
        temp_file.write_all(buffer.as_bytes())?;
        // Renames a file or directory to a new name, replacing the original file if it already exists.
        std::fs::rename(&temp_filepath, filepath)?;
    }
    Ok(())
}
