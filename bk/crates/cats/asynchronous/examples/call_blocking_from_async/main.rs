use clap::Parser;
use clap::Subcommand;

mod call_blocking_from_async_rayon;
mod call_blocking_from_async_spawn_blocking;
mod call_blocking_from_async_spawn_dedicated_thread;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "call_blocking_from_async_rayon")]
    CallBlockingFromAsyncRayon,
    #[command(name = "call_blocking_from_async_spawn_blocking")]
    CallBlockingFromAsyncSpawnBlocking,
    #[command(name = "call_blocking_from_async_spawn_dedicated_thread")]
    CallBlockingFromAsyncSpawnDedicatedThread,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::CallBlockingFromAsyncRayon => {
                let _ = call_blocking_from_async_rayon::run();
            }
            Commands::CallBlockingFromAsyncSpawnBlocking => {
                let _ = call_blocking_from_async_spawn_blocking::run();
            }
            Commands::CallBlockingFromAsyncSpawnDedicatedThread => {
                let _ = call_blocking_from_async_spawn_dedicated_thread::run();
            }
        }
    }
}
