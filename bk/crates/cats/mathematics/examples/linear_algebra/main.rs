//! Runs one of the `linear_algebra` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod add_matrices;
mod deserialize_matrix;
mod invert_matrix;
mod multiply_matrices;
mod multiply_scalar_vector_matrix;
mod nalgebra_decomposition;
mod nalgebra_transformations;
mod nalgebra_vectors;
mod vector_comparison;
mod vector_norm;

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
    /// Runs the `add_matrices` example.
    #[command(name = "add_matrices")]
    AddMatrices,
    /// Runs the `deserialize_matrix` example.
    #[command(name = "deserialize_matrix")]
    DeserializeMatrix,
    /// Runs the `invert_matrix` example.
    #[command(name = "invert_matrix")]
    InvertMatrix,
    /// Runs the `multiply_matrices` example.
    #[command(name = "multiply_matrices")]
    MultiplyMatrices,
    /// Runs the `multiply_scalar_vector_matrix` example.
    #[command(name = "multiply_scalar_vector_matrix")]
    MultiplyScalarVectorMatrix,
    /// Runs the `nalgebra_decomposition` example.
    #[command(name = "nalgebra_decomposition")]
    NalgebraDecomposition,
    /// Runs the `nalgebra_transformations` example.
    #[command(name = "nalgebra_transformations")]
    NalgebraTransformations,
    /// Runs the `nalgebra_vectors` example.
    #[command(name = "nalgebra_vectors")]
    NalgebraVectors,
    /// Runs the `vector_comparison` example.
    #[command(name = "vector_comparison")]
    VectorComparison,
    /// Runs the `vector_norm` example.
    #[command(name = "vector_norm")]
    VectorNorm,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::AddMatrices => {
                add_matrices::run();
            }
            Commands::DeserializeMatrix => {
                deserialize_matrix::run()?;
            }
            Commands::InvertMatrix => {
                invert_matrix::run();
            }
            Commands::MultiplyMatrices => {
                multiply_matrices::run();
            }
            Commands::MultiplyScalarVectorMatrix => {
                multiply_scalar_vector_matrix::run();
            }
            Commands::NalgebraDecomposition => {
                nalgebra_decomposition::run();
            }
            Commands::NalgebraTransformations => {
                nalgebra_transformations::run();
            }
            Commands::NalgebraVectors => {
                nalgebra_vectors::run();
            }
            Commands::VectorComparison => {
                vector_comparison::run();
            }
            Commands::VectorNorm => {
                vector_norm::run();
            }
        }
    }

    Ok(())
}

/// Parses the command line and runs the selected example.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &[
        "add_matrices",
        "deserialize_matrix",
        "invert_matrix",
        "multiply_matrices",
        "multiply_scalar_vector_matrix",
        "nalgebra_decomposition",
        "nalgebra_transformations",
        "nalgebra_vectors",
        "vector_comparison",
        "vector_norm",
    ];

    /// The command-line definition is internally consistent.
    #[test]
    fn test_cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Each example is reachable through its own subcommand.
    #[test]
    fn test_every_subcommand_parses() {
        for name in SUBCOMMANDS {
            let cli = Cli::try_parse_from(["example", name])
                .unwrap_or_else(|e| panic!("`{name}` should parse: {e}"));
            assert!(cli.command.is_some(), "`{name}` selected no command");
        }
    }

    /// Subcommand names are unique.
    #[test]
    fn test_subcommand_names_are_unique() {
        let mut sorted = SUBCOMMANDS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), SUBCOMMANDS.len());
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }

    /// Running without a subcommand selects nothing and succeeds.
    #[test]
    fn test_no_subcommand_runs_nothing() {
        let cli = Cli::try_parse_from(["example"])
            .expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli).expect("dispatching should succeed");
    }
}
