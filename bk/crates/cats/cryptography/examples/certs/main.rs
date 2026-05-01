use clap::Parser;
use clap::Subcommand;

mod der;
mod pem_rfc7468;
mod pkcs8;
mod x509_cert;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "der")]
    Der,
    #[command(name = "pem_rfc7468")]
    PemRfc7468,
    #[command(name = "pkcs8")]
    Pkcs8,
    #[command(name = "x509_cert")]
    X509Cert,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Der => {
                der::run();
            }
            Commands::PemRfc7468 => {
                pem_rfc7468::run();
            }
            Commands::Pkcs8 => {
                pkcs8::run();
            }
            Commands::X509Cert => {
                x509_cert::run();
            }
        }
    }
}
