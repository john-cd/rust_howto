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

#[allow(clippy::enum_variant_names)]
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
                call_blocking_from_async_rayon::run();
            }
            Commands::CallBlockingFromAsyncSpawnBlocking => {
                call_blocking_from_async_spawn_blocking::run();
            }
            Commands::CallBlockingFromAsyncSpawnDedicatedThread => {
                call_blocking_from_async_spawn_dedicated_thread::run();
            }
        }
    }
}
