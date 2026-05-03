mod cryptocurrencies;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    cryptocurrencies::run()
}
