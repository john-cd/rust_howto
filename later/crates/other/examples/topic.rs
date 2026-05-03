use clap::Parser;
use clap::Subcommand;

mod cloud;
mod cross_platform;
mod data_processing;
mod gpu;
mod scripting;
mod written_in_rust;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "aws_lambda")]
    AwsLambda,
    #[command(name = "aws_sdk")]
    AwsSdk,
    #[command(name = "shuttle")]
    Shuttle,
    #[command(name = "crux")]
    Crux,
    #[cfg(feature = "arrow")]
    #[command(name = "arrow")]
    Arrow,
    #[cfg(feature = "datafusion")]
    #[command(name = "datafusion")]
    Datafusion,
    #[cfg(feature = "polars")]
    #[command(name = "polars")]
    Polars,
    #[command(name = "rust_gpu")]
    RustGpu,
    #[command(name = "rhai")]
    Rhai,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AwsLambda => {
                cloud::aws_lambda::run();
            }
            Commands::AwsSdk => {
                cloud::aws_sdk::run();
            }
            Commands::Shuttle => {
                cloud::shuttle::run();
            }
            Commands::Crux => {
                cross_platform::crux::run();
            }
            #[cfg(feature = "arrow")]
            Commands::Arrow => {
                data_processing::arrow::run();
            }
            #[cfg(feature = "datafusion")]
            Commands::Datafusion => {
                data_processing::datafusion::run();
            }
            #[cfg(feature = "polars")]
            Commands::Polars => {
                data_processing::polars::run();
            }
            Commands::RustGpu => {
                gpu::rust_gpu::run();
            }
            Commands::Rhai => {
                scripting::rhai::run()
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?;
            }
        }
    }

    Ok(())
}
