// TODO Reviewed ??-??-2026
mod emulator;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    emulator::run()
}
