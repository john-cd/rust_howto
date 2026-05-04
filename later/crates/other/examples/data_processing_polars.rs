mod data_processing;

fn main() -> anyhow::Result<()> {
    data_processing::polars::run()
}
