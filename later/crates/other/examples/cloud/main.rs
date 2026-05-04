use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

mod aws_lambda;
mod aws_sdk;
mod shuttle;

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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AwsLambda => aws_lambda::run()?,
            Commands::AwsSdk => aws_sdk::run()?,
            Commands::Shuttle => shuttle::run(),
        }
    }

    Ok(())
}
