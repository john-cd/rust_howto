mod encoding;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    encoding::run()
}
