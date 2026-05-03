mod audio;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    audio::run()?;
    Ok(())
}
