use clap::Parser;
use clap::Subcommand;

mod call_async_from_blocking_futures_executor;
mod call_async_from_blocking_tokio_runtime;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "call_async_from_blocking_futures_executor")]
    CallAsyncFromBlockingFuturesExecutor,
    #[command(name = "call_async_from_blocking_tokio_runtime")]
    CallAsyncFromBlockingTokioRuntime,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CallAsyncFromBlockingFuturesExecutor => {
                let _ = call_async_from_blocking_futures_executor::run();
            }
            Commands::CallAsyncFromBlockingTokioRuntime => {
                let _ = call_async_from_blocking_tokio_runtime::run();
            }
        }
    }
}
