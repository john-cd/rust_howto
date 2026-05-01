use clap::Parser;

#[cfg(feature = "floem")]
mod floem;
mod iced;
mod slint;
#[cfg(feature = "vizia")]
mod vizia;
#[cfg(feature = "xilem")]
mod xilem;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    example: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.example.as_str() {
        #[cfg(feature = "floem")]
        "floem" => floem::run(),
        "iced" => iced::run(),
        "slint" => { /* slint::run()?; */ }
        #[cfg(feature = "vizia")]
        "vizia" => vizia::run(),
        #[cfg(feature = "xilem")]
        "xilem" => xilem::run(),
        _ => eprintln!("Unknown example. Try: floem, iced, slint, vizia, xilem"),
    }
    Ok(())
}
// [finish fix](https://github.com/john-cd/rust_howto/issues/1051)
