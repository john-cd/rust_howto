mod images;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    images::run()
}
