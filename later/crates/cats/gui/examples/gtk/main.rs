#[cfg(feature = "gtk")]
use clap::Parser;

#[cfg(feature = "gtk")]
mod gtk4;

#[cfg(feature = "gtk")]
mod relm4;

#[cfg(feature = "gtk")]
#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    example: String,
}

fn main() {
    #[cfg(feature = "gtk")]
    {
        let cli = Cli::parse();
        match cli.example.as_str() {
            "gtk4" => gtk4::run(),
            "relm4" => relm4::run(),
            _ => eprintln!("Unknown example. Try: gtk4, relm4"),
        }
    }
}
