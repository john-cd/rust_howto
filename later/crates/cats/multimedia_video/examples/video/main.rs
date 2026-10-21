// TODO Reviewed ??-??-2026
mod video;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    video::run()
}
