use clap::Parser;

#[cfg(feature = "candle")]
mod candle;
mod linfa;
mod smartcore;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    example: String,
}

fn main() {
    let cli = Cli::parse();
    match cli.example.as_str() {
        #[cfg(feature = "candle")]
        "candle" => candle::run(),
        "linfa" => linfa::run(),
        "smartcore" => smartcore::run(),
        _ => eprintln!("Unknown example. Try: candle, linfa, smartcore"),
    }
}
