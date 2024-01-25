use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cwd = env::current_dir()?;
    println!("The current directory is {}", cwd.display());
    Ok(())
}
