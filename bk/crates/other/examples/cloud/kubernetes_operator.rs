// TODO Reviewed ??-??-2026.

use std::path::PathBuf;

use clap::Parser;

mod kubernetes;

/// Arguments for running the Kubernetes operator or printing its CRD.
#[derive(Parser)]
struct Cli {
    /// Runs the controller with this kubeconfig file.
    #[arg(long, value_name = "FILE")]
    kubeconfig: Option<PathBuf>,
}

/// Prints the CRD or starts the controller with the supplied kubeconfig.
fn main() -> anyhow::Result<()> {
    kubernetes::run(Cli::parse().kubeconfig.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parses a path for the explicitly selected cluster credentials.
    #[test]
    fn parses_kubeconfig_argument() {
        let cli = Cli::try_parse_from(["kubernetes_operator", "--kubeconfig", "test.yaml"])
            .expect("valid kubeconfig argument");
        assert_eq!(cli.kubeconfig, Some(PathBuf::from("test.yaml")));
    }
}
