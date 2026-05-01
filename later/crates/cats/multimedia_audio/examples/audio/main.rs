mod audio;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = audio::run();
    Ok(())
}
