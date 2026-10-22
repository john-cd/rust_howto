// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod tracing;
mod tracing_instrument;
mod tracing_span_entered;
mod tracing_span_in_scope;
mod tracing_spans;
mod tracing_subscriber;
mod tracing_subscriber1;
mod tracing_subscriber1b;
mod tracing_subscriber2;
mod tracing_subscriber2b;
mod tracing_subscriber3;
mod tracing_subscriber3b;
mod tracing_subscriber4;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `tracing` example.
    #[command(name = "tracing")]
    Tracing,
    /// Runs the `tracing_instrument` example.
    #[command(name = "tracing_instrument")]
    TracingInstrument,
    /// Runs the `tracing_span_entered` example.
    #[command(name = "tracing_span_entered")]
    TracingSpanEntered,
    /// Runs the `tracing_span_in_scope` example.
    #[command(name = "tracing_span_in_scope")]
    TracingSpanInScope,
    /// Runs the `tracing_spans` example.
    #[command(name = "tracing_spans")]
    TracingSpans,
    /// Runs the `tracing_subscriber` example.
    #[command(name = "tracing_subscriber")]
    TracingSubscriber,
    /// Runs the `tracing_subscriber1` example.
    #[command(name = "tracing_subscriber1")]
    TracingSubscriber1,
    /// Runs the `tracing_subscriber1b` example.
    #[command(name = "tracing_subscriber1b")]
    TracingSubscriber1b,
    /// Runs the `tracing_subscriber2` example.
    #[command(name = "tracing_subscriber2")]
    TracingSubscriber2,
    /// Runs the `tracing_subscriber2b` example.
    #[command(name = "tracing_subscriber2b")]
    TracingSubscriber2b,
    /// Runs the `tracing_subscriber3` example.
    #[command(name = "tracing_subscriber3")]
    TracingSubscriber3,
    /// Runs the `tracing_subscriber3b` example.
    #[command(name = "tracing_subscriber3b")]
    TracingSubscriber3b,
    /// Runs the `tracing_subscriber4` example.
    #[command(name = "tracing_subscriber4")]
    TracingSubscriber4,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Tracing => {
                tracing::run();
            }
            Commands::TracingInstrument => {
                tracing_instrument::run();
            }
            Commands::TracingSpanEntered => {
                tracing_span_entered::run();
            }
            Commands::TracingSpanInScope => {
                tracing_span_in_scope::run();
            }
            Commands::TracingSpans => {
                tracing_spans::run();
            }
            Commands::TracingSubscriber => {
                tracing_subscriber::run();
            }
            Commands::TracingSubscriber1 => {
                tracing_subscriber1::run();
            }
            Commands::TracingSubscriber1b => {
                tracing_subscriber1b::run();
            }
            Commands::TracingSubscriber2 => {
                tracing_subscriber2::run()
                    .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            }
            Commands::TracingSubscriber2b => {
                tracing_subscriber2b::run();
            }
            Commands::TracingSubscriber3 => {
                tracing_subscriber3::run()?;
            }
            Commands::TracingSubscriber3b => {
                tracing_subscriber3b::run()?;
            }
            Commands::TracingSubscriber4 => {
                tracing_subscriber4::run();
            }
        }
    }
    Ok(())
}
