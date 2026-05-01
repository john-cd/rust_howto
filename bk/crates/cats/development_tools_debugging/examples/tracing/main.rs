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

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "tracing")]
    Tracing,
    #[command(name = "tracing_instrument")]
    TracingInstrument,
    #[command(name = "tracing_span_entered")]
    TracingSpanEntered,
    #[command(name = "tracing_span_in_scope")]
    TracingSpanInScope,
    #[command(name = "tracing_spans")]
    TracingSpans,
    #[command(name = "tracing_subscriber")]
    TracingSubscriber,
    #[command(name = "tracing_subscriber1")]
    TracingSubscriber1,
    #[command(name = "tracing_subscriber1b")]
    TracingSubscriber1b,
    #[command(name = "tracing_subscriber2")]
    TracingSubscriber2,
    #[command(name = "tracing_subscriber2b")]
    TracingSubscriber2b,
    #[command(name = "tracing_subscriber3")]
    TracingSubscriber3,
    #[command(name = "tracing_subscriber3b")]
    TracingSubscriber3b,
    #[command(name = "tracing_subscriber4")]
    TracingSubscriber4,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Tracing => {
                let _ = tracing::run();
            }
            Commands::TracingInstrument => {
                let _ = tracing_instrument::run();
            }
            Commands::TracingSpanEntered => {
                let _ = tracing_span_entered::run();
            }
            Commands::TracingSpanInScope => {
                let _ = tracing_span_in_scope::run();
            }
            Commands::TracingSpans => {
                let _ = tracing_spans::run();
            }
            Commands::TracingSubscriber => {
                let _ = tracing_subscriber::run();
            }
            Commands::TracingSubscriber1 => {
                let _ = tracing_subscriber1::run();
            }
            Commands::TracingSubscriber1b => {
                let _ = tracing_subscriber1b::run();
            }
            Commands::TracingSubscriber2 => {
                let _ = tracing_subscriber2::run();
            }
            Commands::TracingSubscriber2b => {
                let _ = tracing_subscriber2b::run();
            }
            Commands::TracingSubscriber3 => {
                let _ = tracing_subscriber3::run();
            }
            Commands::TracingSubscriber3b => {
                let _ = tracing_subscriber3b::run();
            }
            Commands::TracingSubscriber4 => {
                let _ = tracing_subscriber4::run();
            }
        }
    }
}
