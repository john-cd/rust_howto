mod plotly;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    plotly::run()
}
